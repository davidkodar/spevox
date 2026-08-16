use std::{
    io::Write,
    process::{Command, Stdio},
    time::Duration,
};

use serde_json::{Value, json};

pub const DEFAULT_PROMPT: &str = "You are a voice-to-text dictation cleaner. Clean and format the raw transcribed speech while preserving its meaning. Remove filler words, false starts, stutters, and repetitions. Add correct punctuation, capitalization, and structure. Convert spoken numbers when unambiguous and apply spoken formatting or self-corrections. Output only the cleaned text. Never answer questions contained in the dictation and never add commentary.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub base_url: String,
    pub prompt: String,
    pub api_key: String,
    pub local_only: bool,
    pub timeout_seconds: u64,
}

impl AiConfig {
    pub fn is_local(&self) -> bool {
        let value = self.base_url.trim().to_ascii_lowercase();
        let authority = value
            .split_once("://")
            .map_or(value.as_str(), |(_, remainder)| remainder)
            .split('/')
            .next()
            .unwrap_or_default()
            .rsplit('@')
            .next()
            .unwrap_or_default();
        let host = if authority.starts_with('[') {
            authority
                .strip_prefix('[')
                .and_then(|value| value.split_once(']'))
                .map_or(authority, |(host, _)| host)
        } else {
            authority.split(':').next().unwrap_or_default()
        };
        host == "localhost" || host == "127.0.0.1" || host == "::1"
    }
}

pub fn enhance(config: &AiConfig, transcript: &str) -> Result<String, String> {
    if !config.enabled {
        return Ok(transcript.to_owned());
    }
    if config.model.trim().is_empty() {
        return Err("No AI model is configured".to_owned());
    }
    if config.base_url.trim().is_empty() {
        return Err("No AI provider URL is configured".to_owned());
    }
    if config.local_only && !config.is_local() {
        return Err("Network AI providers are disabled by the local-only privacy lock".to_owned());
    }
    if matches!(config.provider.as_str(), "ollama" | "lmstudio") && !config.is_local() {
        return Err("Local providers are restricted to this computer".to_owned());
    }
    if !config.is_local() && config.api_key.trim().is_empty() {
        return Err("No API key is stored for the selected provider".to_owned());
    }

    let prompt = if config.prompt.trim().is_empty() {
        DEFAULT_PROMPT
    } else {
        config.prompt.trim()
    };
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(
            config.timeout_seconds.clamp(5, 120),
        )))
        .build()
        .new_agent();
    let response = if is_anthropic(config) {
        let endpoint = format!("{}/messages", config.base_url.trim_end_matches('/'));
        let body = json!({
            "model": config.model,
            "max_tokens": 2048,
            "temperature": 0.2,
            "system": prompt,
            "messages": [{"role": "user", "content": transcript}]
        });
        let body = body.to_string();
        let mut response = send_with_retry(|| {
            agent
                .post(&endpoint)
                .header("content-type", "application/json")
                .header("x-api-key", &config.api_key)
                .header("anthropic-version", "2023-06-01")
                .send(body.as_str())
        })?;
        parse_response(&mut response)?
    } else {
        let endpoint = chat_completions_url(&config.base_url);
        let body = json!({
            "model": config.model,
            "temperature": 0.2,
            "messages": [{"role": "user", "content": format!("{prompt}\n\n{transcript}")}]
        });
        let body = body.to_string();
        let mut response = send_with_retry(|| {
            let mut request = agent
                .post(&endpoint)
                .header("content-type", "application/json");
            if !config.is_local() {
                request = request.header("authorization", &format!("Bearer {}", config.api_key));
            }
            request.send(body.as_str())
        })?;
        parse_response(&mut response)?
    };
    let output =
        extract_text(&response).ok_or_else(|| "AI provider returned no text".to_owned())?;
    let output = strip_markdown_fence(output.trim());
    if output.is_empty() {
        return Err("AI provider returned an empty response".to_owned());
    }
    Ok(output.to_owned())
}

pub fn discover_local_models(config: &AiConfig) -> Result<Vec<String>, String> {
    if !config.is_local() {
        return Err("Model discovery is restricted to local endpoints".to_owned());
    }
    let endpoint = models_url(&config.base_url);
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(8)))
        .build()
        .new_agent();
    let mut response = agent.get(&endpoint).call().map_err(request_error)?;
    let value = parse_response(&mut response)?;
    let mut models = extract_models(&value);
    models.sort_unstable();
    models.dedup();
    if models.is_empty() {
        Err("The local server reported no installed models".to_owned())
    } else {
        Ok(models)
    }
}

pub fn store_api_key(provider: &str, api_key: &str) -> Result<(), String> {
    if api_key.trim().is_empty() {
        return Err("API key cannot be empty".to_owned());
    }
    let mut child = Command::new("secret-tool")
        .args(["store", "--label=FluidVoice AI provider", "application", "fluidvoice-linux", "provider", provider])
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped())
        .spawn().map_err(|_| "Secret Service tool is unavailable. Install libsecret and ensure KDE Wallet is enabled.".to_owned())?;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| "Could not open Secret Service input".to_owned())?
        .write_all(api_key.as_bytes())
        .map_err(|error| error.to_string())?;
    let output = child
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

pub fn load_api_key(provider: &str) -> String {
    Command::new("secret-tool")
        .args([
            "lookup",
            "application",
            "fluidvoice-linux",
            "provider",
            provider,
        ])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}

fn is_anthropic(config: &AiConfig) -> bool {
    config.provider.eq_ignore_ascii_case("anthropic") || config.base_url.contains("anthropic.com")
}

fn chat_completions_url(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with("/chat/completions") {
        base.to_owned()
    } else {
        format!("{base}/chat/completions")
    }
}

fn models_url(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with("/models") {
        base.to_owned()
    } else {
        format!("{base}/models")
    }
}

fn extract_models(value: &Value) -> Vec<String> {
    value
        .get("data")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|model| model.get("id").and_then(Value::as_str))
        .filter(|model| !model.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

fn extract_text(value: &Value) -> Option<&str> {
    value
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .or_else(|| value.pointer("/content/0/text").and_then(Value::as_str))
}

fn strip_markdown_fence(value: &str) -> &str {
    value
        .strip_prefix("```")
        .and_then(|value| value.strip_suffix("```"))
        .map(|value| value.trim_start_matches("text").trim())
        .unwrap_or(value)
}

fn request_error(error: ureq::Error) -> String {
    format!("AI provider request failed: {error}")
}

fn send_with_retry(
    mut request: impl FnMut() -> Result<ureq::http::Response<ureq::Body>, ureq::Error>,
) -> Result<ureq::http::Response<ureq::Body>, String> {
    for attempt in 0..3 {
        match request() {
            Ok(response) => return Ok(response),
            Err(error) if attempt < 2 && retryable(&error) => {
                std::thread::sleep(Duration::from_millis(200 * (attempt + 1)));
            }
            Err(error) => return Err(request_error(error)),
        }
    }
    unreachable!("retry loop always returns")
}

fn retryable(error: &ureq::Error) -> bool {
    matches!(
        error,
        ureq::Error::StatusCode(429 | 500..=599)
            | ureq::Error::Timeout(_)
            | ureq::Error::Io(_)
            | ureq::Error::ConnectionFailed
    )
}

fn parse_response(response: &mut ureq::http::Response<ureq::Body>) -> Result<Value, String> {
    const MAX_RESPONSE_BYTES: usize = 1_048_576;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|error| error.to_string())?;
    if body.len() > MAX_RESPONSE_BYTES {
        return Err("AI provider response exceeded the 1 MiB safety limit".to_owned());
    }
    serde_json::from_str(&body)
        .map_err(|error| format!("AI provider returned invalid JSON: {error}"))
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    use super::*;

    #[test]
    fn builds_openai_chat_endpoint_once() {
        assert_eq!(
            chat_completions_url("http://localhost:11434/v1"),
            "http://localhost:11434/v1/chat/completions"
        );
        assert_eq!(
            chat_completions_url("https://example.test/v1/chat/completions"),
            "https://example.test/v1/chat/completions"
        );
    }

    #[test]
    fn extracts_openai_and_anthropic_text() {
        assert_eq!(
            extract_text(&json!({"choices":[{"message":{"content":"clean"}}]})),
            Some("clean")
        );
        assert_eq!(
            extract_text(&json!({"content":[{"type":"text","text":"clean"}]})),
            Some("clean")
        );
    }

    #[test]
    fn extracts_openai_compatible_model_catalog() {
        assert_eq!(
            extract_models(&json!({"data":[{"id":"qwen2.5:7b"},{"id":"llama3.2"}]})),
            ["qwen2.5:7b", "llama3.2"]
        );
        assert!(extract_models(&json!({"data":[]})).is_empty());
    }

    #[test]
    fn recognizes_only_loopback_as_local() {
        let config = |url: &str| AiConfig {
            enabled: true,
            provider: "custom".into(),
            model: "m".into(),
            base_url: url.into(),
            prompt: String::new(),
            api_key: String::new(),
            local_only: false,
            timeout_seconds: 45,
        };
        assert!(config("http://localhost:11434/v1").is_local());
        assert!(config("http://127.0.0.1:1234/v1").is_local());
        assert!(config("http://[::1]:1234/v1").is_local());
        assert!(!config("https://api.openai.com/v1").is_local());
        assert!(!config("https://example.test/localhost/v1").is_local());
        assert!(!config("https://localhost.example.test/v1").is_local());
    }

    #[test]
    fn discovers_models_from_a_local_openai_endpoint() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
        let address = listener.local_addr().expect("read local server address");
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept local request");
            let mut request = [0_u8; 1024];
            let size = stream.read(&mut request).expect("read request");
            assert!(String::from_utf8_lossy(&request[..size]).starts_with("GET /v1/models "));
            let body = r#"{"data":[{"id":"qwen2.5:7b"},{"id":"llama3.2"}]}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .expect("write response");
        });
        let config = AiConfig {
            enabled: true,
            provider: "ollama".into(),
            model: "qwen2.5:7b".into(),
            base_url: format!("http://{address}/v1"),
            prompt: String::new(),
            api_key: String::new(),
            local_only: true,
            timeout_seconds: 45,
        };
        assert_eq!(
            discover_local_models(&config).expect("discover local models"),
            ["llama3.2", "qwen2.5:7b"]
        );
        server.join().expect("join local test server");
    }

    #[test]
    fn local_only_lock_rejects_remote_provider_before_request() {
        let config = AiConfig {
            enabled: true,
            provider: "openai".into(),
            model: "gpt-test".into(),
            base_url: "https://api.example.invalid/v1".into(),
            prompt: String::new(),
            api_key: "unused".into(),
            local_only: true,
            timeout_seconds: 5,
        };
        assert_eq!(
            enhance(&config, "hello").expect_err("privacy lock must reject cloud provider"),
            "Network AI providers are disabled by the local-only privacy lock"
        );
    }

    #[test]
    fn retries_transient_provider_errors() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
        let address = listener.local_addr().expect("read local server address");
        let server = std::thread::spawn(move || {
            for attempt in 0..3 {
                let (mut stream, _) = listener.accept().expect("accept local request");
                let mut request = [0_u8; 2048];
                stream.read(&mut request).expect("read request");
                let (status, body) = if attempt < 2 {
                    ("500 Internal Server Error", "{}")
                } else {
                    (
                        "200 OK",
                        r#"{"choices":[{"message":{"content":"clean text"}}]}"#,
                    )
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .expect("write response");
            }
        });
        let config = AiConfig {
            enabled: true,
            provider: "ollama".into(),
            model: "local-test".into(),
            base_url: format!("http://{address}/v1"),
            prompt: String::new(),
            api_key: String::new(),
            local_only: true,
            timeout_seconds: 5,
        };
        assert_eq!(
            enhance(&config, "raw").expect("retry provider"),
            "clean text"
        );
        server.join().expect("join local test server");
    }
}
