use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    time::Duration,
};

use serde_json::{Value, json};
use url::{Host, Url};

const PROVIDER_TIMEOUT_MIN_SECONDS: u64 = 5;
const PROVIDER_TIMEOUT_MAX_SECONDS: u64 = 120;
const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(8);

pub const DEFAULT_PROMPT: &str = "You are a conservative voice-dictation editor. Return only the edited dictation, never an answer or commentary. Preserve the speaker's language, meaning, intent, names, technical terms, and level of certainty. Remove filler words, stutters, abandoned false starts, and accidental repetitions. Apply explicit self-corrections. Add capitalization, sentence boundaries, commas, and question or exclamation marks when clearly implied by the utterance. Convert spoken formatting commands and unambiguous spoken numbers. Repair only grammar needed to make an obviously incomplete dictated sentence readable; never invent facts, arguments, greetings, conclusions, or missing details. Keep deliberate repetition and informal style. Do not translate. Do not use Markdown fences.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderId {
    OpenAi,
    Anthropic,
    Xai,
    Groq,
    Cerebras,
    Google,
    OpenRouter,
    Ollama,
    LmStudio,
    Custom,
}

impl ProviderId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Anthropic => "anthropic",
            Self::Xai => "xai",
            Self::Groq => "groq",
            Self::Cerebras => "cerebras",
            Self::Google => "google",
            Self::OpenRouter => "openrouter",
            Self::Ollama => "ollama",
            Self::LmStudio => "lmstudio",
            Self::Custom => "custom",
        }
    }

    pub const fn is_local(self) -> bool {
        matches!(self, Self::Ollama | Self::LmStudio)
    }

    pub const fn preference_index(self) -> i32 {
        match self {
            Self::OpenAi => 0,
            Self::Anthropic => 1,
            Self::Xai => 2,
            Self::Groq => 3,
            Self::Cerebras => 4,
            Self::Google => 5,
            Self::OpenRouter => 6,
            Self::Ollama => 7,
            Self::LmStudio => 8,
            Self::Custom => 9,
        }
    }

    pub const fn from_preference_index(index: i32) -> Self {
        match index {
            0 => Self::OpenAi,
            1 => Self::Anthropic,
            2 => Self::Xai,
            3 => Self::Groq,
            4 => Self::Cerebras,
            5 => Self::Google,
            6 => Self::OpenRouter,
            8 => Self::LmStudio,
            9 => Self::Custom,
            _ => Self::Ollama,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: ProviderId,
    pub model: String,
    pub base_url: String,
    pub prompt: String,
    pub language: String,
    pub api_key: String,
    pub local_only: bool,
    pub timeout_seconds: u64,
}

impl AiConfig {
    pub fn new(
        provider: ProviderId,
        model: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        Self {
            enabled: true,
            provider,
            model: model.into(),
            base_url: base_url.into(),
            prompt: DEFAULT_PROMPT.to_owned(),
            language: String::new(),
            api_key: String::new(),
            local_only: false,
            timeout_seconds: 45,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = api_key.into();
        self
    }

    pub fn with_local_only(mut self, local_only: bool) -> Self {
        self.local_only = local_only;
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    pub fn is_local(&self) -> bool {
        Url::parse(self.base_url.trim()).is_ok_and(|url| match url.host() {
            Some(Host::Domain(host)) => host.eq_ignore_ascii_case("localhost"),
            Some(Host::Ipv4(ip)) => ip.is_loopback(),
            Some(Host::Ipv6(ip)) => ip.is_loopback(),
            None => false,
        })
    }
}

fn provider_agent(config: &AiConfig, timeout: Duration) -> ureq::Agent {
    let mut builder = ureq::Agent::config_builder().timeout_global(Some(timeout));
    if config.is_local() {
        // Local-only means local at the transport layer too: never inherit an
        // HTTP proxy and never permit a redirect away from loopback.
        builder = builder.proxy(None).max_redirects(0);
    }
    builder.build().new_agent()
}

pub fn enhance(config: &AiConfig, transcript: &str) -> Result<String, String> {
    if !config.enabled {
        return Ok(transcript.to_owned());
    }
    validate_config(config)?;
    let prompt = effective_prompt(config);
    let agent = provider_agent(config, provider_timeout(config));
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
        let mut request = agent
            .post(&endpoint)
            .header("content-type", "application/json")
            .header("anthropic-version", "2023-06-01");
        if !config.api_key.trim().is_empty() {
            request = request.header("x-api-key", &config.api_key);
        }
        let mut response = request
            .send(body.as_str())
            .map_err(|error| request_error(&error))?;
        parse_response(&mut response)?
    } else {
        let endpoint = chat_completions_url(&config.base_url);
        let body = json!({
            "model": config.model,
            "temperature": 0.2,
            "messages": [
                {"role": "system", "content": prompt},
                {"role": "user", "content": transcript}
            ]
        });
        let body = body.to_string();
        let mut response = {
            let mut request = agent
                .post(&endpoint)
                .header("content-type", "application/json");
            if !config.api_key.trim().is_empty() {
                request = request.header("authorization", &format!("Bearer {}", config.api_key));
            }
            request
                .send(body.as_str())
                .map_err(|error| request_error(&error))?
        };
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

pub fn enhance_streaming(
    config: &AiConfig,
    transcript: &str,
    mut on_update: impl FnMut(&str),
) -> Result<String, String> {
    validate_config(config)?;
    let prompt = effective_prompt(config);
    let agent = provider_agent(config, provider_timeout(config));
    let mut response = if is_anthropic(config) {
        let endpoint = format!("{}/messages", config.base_url.trim_end_matches('/'));
        let body = json!({
            "model": config.model, "max_tokens": 2048, "temperature": 0.2,
            "stream": true, "system": prompt,
            "messages": [{"role": "user", "content": transcript}]
        })
        .to_string();
        let mut request = agent
            .post(&endpoint)
            .header("content-type", "application/json")
            .header("anthropic-version", "2023-06-01");
        if !config.api_key.trim().is_empty() {
            request = request.header("x-api-key", &config.api_key);
        }
        request
            .send(body.as_str())
            .map_err(|error| request_error(&error))?
    } else {
        let endpoint = chat_completions_url(&config.base_url);
        let body = json!({
            "model": config.model, "temperature": 0.2, "stream": true,
            "messages": [
                {"role": "system", "content": prompt},
                {"role": "user", "content": transcript}
            ]
        })
        .to_string();
        {
            let mut request = agent
                .post(&endpoint)
                .header("content-type", "application/json");
            if !config.api_key.trim().is_empty() {
                request = request.header("authorization", &format!("Bearer {}", config.api_key));
            }
            request
                .send(body.as_str())
                .map_err(|error| request_error(&error))?
        }
    };
    parse_stream(&mut response, &mut on_update)
}

fn validate_config(config: &AiConfig) -> Result<(), String> {
    if !config.enabled {
        return Err("AI enhancement is disabled".to_owned());
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
    if config.provider.is_local() && !config.is_local() {
        return Err("Local providers are restricted to this computer".to_owned());
    }
    if !config.is_local() && config.api_key.trim().is_empty() {
        return Err("No API key is stored for the selected provider".to_owned());
    }
    Ok(())
}

fn effective_prompt(config: &AiConfig) -> String {
    let base = if config.prompt.trim().is_empty() {
        DEFAULT_PROMPT
    } else {
        config.prompt.trim()
    };
    let language = config.language.trim();
    if language.is_empty() {
        format!(
            "{base}\n\nLanguage contract: infer the input language and keep the entire output in that language. For mixed-language dictation, preserve intentional code-switching."
        )
    } else {
        format!(
            "{base}\n\nLanguage contract: the dictation language is ISO code {language}. Keep the output in that language and preserve intentional foreign words or technical terms."
        )
    }
}

fn provider_timeout(config: &AiConfig) -> Duration {
    Duration::from_secs(
        config
            .timeout_seconds
            .clamp(PROVIDER_TIMEOUT_MIN_SECONDS, PROVIDER_TIMEOUT_MAX_SECONDS),
    )
}

pub fn discover_local_models(config: &AiConfig) -> Result<Vec<String>, String> {
    if !config.is_local() {
        return Err("Model discovery is restricted to local endpoints".to_owned());
    }
    let endpoint = models_url(&config.base_url);
    let agent = provider_agent(config, DISCOVERY_TIMEOUT);
    let mut response = agent
        .get(&endpoint)
        .call()
        .map_err(|error| request_error(&error))?;
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
    config.provider == ProviderId::Anthropic || config.base_url.contains("anthropic.com")
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
    let Some(inner) = value
        .strip_prefix("```")
        .and_then(|value| value.strip_suffix("```"))
    else {
        return value;
    };
    let inner = inner.trim();
    inner
        .strip_prefix("text\n")
        .or_else(|| inner.strip_prefix("plaintext\n"))
        .unwrap_or(inner)
        .trim()
}

fn request_error(error: &ureq::Error) -> String {
    format!("AI provider request failed: {error}")
}

fn parse_response(response: &mut ureq::http::Response<ureq::Body>) -> Result<Value, String> {
    const MAX_RESPONSE_BYTES: usize = 1_048_576;
    let body = response
        .body_mut()
        .with_config()
        .limit(MAX_RESPONSE_BYTES as u64)
        .read_to_string()
        .map_err(|error| format!("AI provider response invalid or exceeded 1 MiB: {error}"))?;
    serde_json::from_str(&body)
        .map_err(|error| format!("AI provider returned invalid JSON: {error}"))
}

fn parse_stream(
    response: &mut ureq::http::Response<ureq::Body>,
    on_update: &mut impl FnMut(&str),
) -> Result<String, String> {
    const MAX_RESPONSE_BYTES: usize = 1_048_576;
    let mut output = String::new();
    let reader = response
        .body_mut()
        .with_config()
        .limit(MAX_RESPONSE_BYTES as u64)
        .reader();
    let mut received = 0_usize;
    for line in BufReader::new(reader).lines() {
        let line = line.map_err(|error| error.to_string())?;
        received = received.saturating_add(line.len());
        if received > MAX_RESPONSE_BYTES {
            return Err("AI provider response exceeded the 1 MiB safety limit".to_owned());
        }
        let payload = line.strip_prefix("data:").map_or(line.trim(), str::trim);
        if payload.is_empty() || payload == "[DONE]" {
            continue;
        }
        let Ok(event) = serde_json::from_str::<Value>(payload) else {
            continue;
        };
        let delta = event
            .pointer("/choices/0/delta/content")
            .and_then(Value::as_str)
            .or_else(|| event.pointer("/delta/text").and_then(Value::as_str));
        if let Some(delta) = delta {
            output.push_str(delta);
            on_update(output.trim());
        } else if output.is_empty()
            && let Some(text) = extract_text(&event)
        {
            output.push_str(text);
            on_update(output.trim());
        }
    }
    let output = strip_markdown_fence(output.trim());
    if output.is_empty() {
        Err("AI provider returned no streamed text".to_owned())
    } else {
        Ok(output.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    use super::*;

    fn read_complete_request(stream: &mut std::net::TcpStream) {
        let mut request = Vec::new();
        let mut chunk = [0_u8; 2048];
        loop {
            let count = stream.read(&mut chunk).expect("read request");
            if count == 0 {
                return;
            }
            request.extend_from_slice(&chunk[..count]);
            let Some(header_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..header_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            if request.len() >= header_end + 4 + content_length {
                return;
            }
        }
    }

    #[test]
    fn provider_ids_preserve_persisted_preference_indexes() {
        for index in 0..=9 {
            assert_eq!(
                ProviderId::from_preference_index(index).preference_index(),
                index
            );
        }
        assert_eq!(
            ProviderId::from_preference_index(i32::MAX),
            ProviderId::Ollama
        );
    }

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
    fn cleanup_contract_is_language_aware_and_forbids_answering() {
        let fixed =
            AiConfig::new(ProviderId::Ollama, "m", "http://localhost:11434").with_language("sv");
        let prompt = effective_prompt(&fixed);
        assert!(prompt.contains("ISO code sv"));
        assert!(prompt.contains("never an answer"));
        assert!(prompt.contains("never invent"));

        let automatic = AiConfig::new(ProviderId::Ollama, "m", "http://localhost:11434");
        assert!(effective_prompt(&automatic).contains("infer the input language"));
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
        let config = |url: &str| AiConfig::new(ProviderId::Custom, "m", url);
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
        let config = AiConfig::new(
            ProviderId::Ollama,
            "qwen2.5:7b",
            format!("http://{address}/v1"),
        )
        .with_local_only(true);
        assert_eq!(
            discover_local_models(&config).expect("discover local models"),
            ["llama3.2", "qwen2.5:7b"]
        );
        server.join().expect("join local test server");
    }

    #[test]
    fn local_only_lock_rejects_remote_provider_before_request() {
        let config = AiConfig::new(
            ProviderId::OpenAi,
            "gpt-test",
            "https://api.example.invalid/v1",
        )
        .with_api_key("unused")
        .with_local_only(true)
        .with_timeout(5);
        assert_eq!(
            enhance(&config, "hello").expect_err("privacy lock must reject cloud provider"),
            "Network AI providers are disabled by the local-only privacy lock"
        );
    }

    #[test]
    fn does_not_retry_non_idempotent_enhancement_posts() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
        let address = listener.local_addr().expect("read local server address");
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept local request");
            read_complete_request(&mut stream);
            let (status, body) = ("500 Internal Server Error", "{}");
            write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .expect("write response");
        });
        let config = AiConfig::new(
            ProviderId::Ollama,
            "local-test",
            format!("http://{address}/v1"),
        )
        .with_local_only(true)
        .with_timeout(5);
        assert!(enhance(&config, "raw").is_err());
        server.join().expect("join local test server");
    }

    #[test]
    fn streams_openai_compatible_text_updates() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
        let address = listener.local_addr().expect("read local server address");
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept local request");
            read_complete_request(&mut stream);
            let body = "data: {\"choices\":[{\"delta\":{\"content\":\"clean \"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\"text\"}}]}\n\ndata: [DONE]\n\n";
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).expect("write stream");
        });
        let config = AiConfig::new(
            ProviderId::Ollama,
            "local-test",
            format!("http://{address}/v1"),
        )
        .with_local_only(true)
        .with_timeout(5);
        let mut updates = Vec::new();
        let result = enhance_streaming(&config, "raw", |text| updates.push(text.to_owned()))
            .expect("stream enhancement");
        assert_eq!(result, "clean text");
        assert_eq!(updates, ["clean", "clean text"]);
        server.join().expect("join local test server");
    }
}
