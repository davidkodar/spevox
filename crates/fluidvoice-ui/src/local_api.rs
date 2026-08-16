use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::time::Duration;

const MAX_REQUEST_BYTES: usize = 16 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalApiAction {
    ToggleDictation,
}

pub fn token_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("fluidvoice").join("local-api.token")
}

pub fn ensure_token(path: &Path) -> Result<String, String> {
    if let Ok(value) = fs::read_to_string(path) {
        let value = value.trim();
        if value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            fs::set_permissions(path, fs::Permissions::from_mode(0o600))
                .map_err(|error| error.to_string())?;
            return Ok(value.to_owned());
        }
    }
    rotate_token(path)
}

pub fn rotate_token(path: &Path) -> Result<String, String> {
    let parent = path
        .parent()
        .ok_or_else(|| "token path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let mut bytes = [0_u8; 32];
    fs::File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .map_err(|error| format!("secure random source unavailable: {error}"))?;
    let token = bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .map_err(|error| error.to_string())?;
    file.write_all(token.as_bytes())
        .map_err(|error| error.to_string())?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
        .map_err(|error| error.to_string())?;
    Ok(token)
}

pub fn start(port: u16, actions: Sender<LocalApiAction>) -> Result<(), String> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
        .map_err(|error| format!("could not bind 127.0.0.1:{port}: {error}"))?;
    let token = token_path();
    ensure_token(&token)?;
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            match connection {
                Ok(stream) => handle_connection(stream, &token, &actions),
                Err(error) => eprintln!("Local API connection failed: {error}"),
            }
        }
    });
    Ok(())
}

fn handle_connection(mut stream: TcpStream, token_path: &Path, actions: &Sender<LocalApiAction>) {
    stream.set_read_timeout(Some(Duration::from_secs(3))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(3))).ok();
    let mut request = Vec::new();
    let mut buffer = [0_u8; 2048];
    while request.len() <= MAX_REQUEST_BYTES {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                request.extend_from_slice(&buffer[..count]);
                if request.windows(4).any(|part| part == b"\r\n\r\n") {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    if request.len() > MAX_REQUEST_BYTES {
        respond(&mut stream, 413, r#"{"error":"request too large"}"#);
        return;
    }
    let Ok(request) = std::str::from_utf8(&request) else {
        respond(&mut stream, 400, r#"{"error":"invalid request"}"#);
        return;
    };
    let mut lines = request.split("\r\n");
    let first = lines.next().unwrap_or_default();
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let path = parts.next().unwrap_or_default();
    if lines
        .clone()
        .any(|line| line.to_ascii_lowercase().starts_with("origin:"))
    {
        respond(
            &mut stream,
            403,
            r#"{"error":"browser origins are not accepted"}"#,
        );
        return;
    }
    if method == "GET" && path == "/v1/health" {
        respond(&mut stream, 200, r#"{"status":"ok"}"#);
        return;
    }
    let supplied = lines.find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("authorization")
            .then(|| value.trim().strip_prefix("Bearer "))
            .flatten()
    });
    let expected = fs::read_to_string(token_path).unwrap_or_default();
    if !tokens_equal(
        supplied.unwrap_or_default().as_bytes(),
        expected.trim().as_bytes(),
    ) {
        respond(&mut stream, 401, r#"{"error":"unauthorized"}"#);
        return;
    }
    match (method, path) {
        ("GET", "/v1/status") => respond(
            &mut stream,
            200,
            r#"{"status":"ready","privacy":"loopback-only"}"#,
        ),
        ("POST", "/v1/dictation/toggle") => {
            if actions.send(LocalApiAction::ToggleDictation).is_ok() {
                respond(&mut stream, 202, r#"{"accepted":true}"#);
            } else {
                respond(
                    &mut stream,
                    503,
                    r#"{"error":"desktop controller unavailable"}"#,
                );
            }
        }
        _ => respond(&mut stream, 404, r#"{"error":"not found"}"#),
    }
}

fn tokens_equal(left: &[u8], right: &[u8]) -> bool {
    let mut difference = left.len() ^ right.len();
    for index in 0..left.len().max(right.len()) {
        difference |= usize::from(*left.get(index).unwrap_or(&0) ^ *right.get(index).unwrap_or(&0));
    }
    difference == 0
}

fn respond(stream: &mut TcpStream, status: u16, body: &str) {
    let label = match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        413 => "Payload Too Large",
        _ => "Service Unavailable",
    };
    let response = format!(
        "HTTP/1.1 {status} {label}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(
        request: &str,
        token_path: PathBuf,
    ) -> (String, std::sync::mpsc::Receiver<LocalApiAction>) {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let (actions, events) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(stream, &token_path, &actions);
        });
        let mut client = TcpStream::connect(address).unwrap();
        client.write_all(request.as_bytes()).unwrap();
        client.shutdown(std::net::Shutdown::Write).unwrap();
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        (response, events)
    }

    #[test]
    fn token_comparison_checks_length_and_content() {
        assert!(tokens_equal(b"secret", b"secret"));
        assert!(!tokens_equal(b"secret", b"secrex"));
        assert!(!tokens_equal(b"secret", b"secret-long"));
    }

    #[test]
    fn generated_token_is_strong_and_reusable() {
        let path =
            std::env::temp_dir().join(format!("fluidvoice-api-token-{}", std::process::id()));
        fs::remove_file(&path).ok();
        let first = ensure_token(&path).unwrap();
        let second = ensure_token(&path).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
        fs::remove_file(path).ok();
    }

    #[test]
    fn health_is_public_but_status_requires_bearer_token() {
        let path = std::env::temp_dir().join(format!("fluidvoice-api-http-{}", std::process::id()));
        fs::write(&path, "valid-token").unwrap();
        let (health, _) = round_trip(
            "GET /v1/health HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(health.starts_with("HTTP/1.1 200"));
        let (unauthorized, _) = round_trip(
            "GET /v1/status HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(unauthorized.starts_with("HTTP/1.1 401"));
        let (authorized, _) = round_trip(
            "GET /v1/status HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer valid-token\r\n\r\n",
            path.clone(),
        );
        assert!(authorized.starts_with("HTTP/1.1 200"));
        fs::remove_file(path).ok();
    }

    #[test]
    fn authenticated_toggle_dispatches_exactly_one_action() {
        let path =
            std::env::temp_dir().join(format!("fluidvoice-api-toggle-{}", std::process::id()));
        fs::write(&path, "valid-token").unwrap();
        let (response, events) = round_trip(
            "POST /v1/dictation/toggle HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer valid-token\r\nContent-Length: 0\r\n\r\n",
            path.clone(),
        );
        assert!(response.starts_with("HTTP/1.1 202"));
        assert_eq!(
            events.recv_timeout(Duration::from_secs(1)).unwrap(),
            LocalApiAction::ToggleDictation
        );
        assert!(events.try_recv().is_err());
        fs::remove_file(path).ok();
    }
}
