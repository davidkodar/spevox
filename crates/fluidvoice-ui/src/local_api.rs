use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
    mpsc::Sender,
};
use std::time::{Duration, Instant};

const MAX_REQUEST_BYTES: usize = 16 * 1024;
const MAX_CONNECTIONS: usize = 8;

struct ConnectionSlot(Arc<AtomicUsize>);

impl Drop for ConnectionSlot {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

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
        .fold(String::with_capacity(64), |mut token, byte| {
            write!(token, "{byte:02x}").expect("writing to a String cannot fail");
            token
        });
    let temporary = parent.join(format!(".local-api.token.{}.tmp", std::process::id()));
    // A crash can leave this same-process-name temporary behind; PID reuse
    // must not permanently prevent secure token rotation.
    fs::remove_file(&temporary).ok();
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&temporary)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file
        .write_all(token.as_bytes())
        .and_then(|()| file.sync_all())
    {
        fs::remove_file(&temporary).ok();
        return Err(error.to_string());
    }
    drop(file);
    fs::rename(&temporary, path).map_err(|error| {
        fs::remove_file(&temporary).ok();
        error.to_string()
    })?;
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
        let active = Arc::new(AtomicUsize::new(0));
        for connection in listener.incoming() {
            match connection {
                Ok(mut stream) => {
                    if active
                        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                            (count < MAX_CONNECTIONS).then_some(count + 1)
                        })
                        .is_err()
                    {
                        respond(&mut stream, 503, r#"{"error":"server busy"}"#);
                        continue;
                    }
                    let active = Arc::clone(&active);
                    let token = token.clone();
                    let actions = actions.clone();
                    std::thread::spawn(move || {
                        let _slot = ConnectionSlot(active);
                        handle_connection(stream, &token, &actions);
                    });
                }
                Err(error) => eprintln!("Local API connection failed: {error}"),
            }
        }
    });
    Ok(())
}

// Keeps parsing, authentication, and routing in one bounded request lifecycle;
// splitting it would require carrying partially validated HTTP state.
#[allow(clippy::too_many_lines)]
fn handle_connection(mut stream: TcpStream, token_path: &Path, actions: &Sender<LocalApiAction>) {
    let deadline = Instant::now() + Duration::from_secs(3);
    stream.set_write_timeout(Some(Duration::from_secs(3))).ok();
    let mut request = Vec::new();
    let mut buffer = [0_u8; 2048];
    while request.len() <= MAX_REQUEST_BYTES {
        let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
            respond(&mut stream, 408, r#"{"error":"request timed out"}"#);
            return;
        };
        stream.set_read_timeout(Some(remaining)).ok();
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                ) =>
            {
                respond(&mut stream, 408, r#"{"error":"request timed out"}"#);
                return;
            }
            Err(_) => {
                respond(&mut stream, 400, r#"{"error":"request could not be read"}"#);
                return;
            }
            Ok(count) => {
                request.extend_from_slice(&buffer[..count]);
                if request.windows(4).any(|part| part == b"\r\n\r\n") {
                    break;
                }
            }
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
    let supplied = lines.find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("authorization")
            .then(|| value.trim().strip_prefix("Bearer "))
            .flatten()
    });
    let expected = match fs::read_to_string(token_path) {
        Ok(expected)
            if expected.trim().len() == 64
                && expected.trim().bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            expected
        }
        _ => {
            respond(
                &mut stream,
                503,
                r#"{"error":"authentication token unavailable"}"#,
            );
            return;
        }
    };
    if !tokens_equal(
        supplied.unwrap_or_default().as_bytes(),
        expected.trim().as_bytes(),
    ) {
        respond(&mut stream, 401, r#"{"error":"unauthorized"}"#);
        return;
    }
    if method == "GET" && path == "/v1/health" {
        respond(&mut stream, 200, r#"{"status":"ok"}"#);
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
        408 => "Request Timeout",
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
    fn health_and_status_require_bearer_token() {
        let path = std::env::temp_dir().join(format!("fluidvoice-api-http-{}", std::process::id()));
        let token = "a".repeat(64);
        fs::write(&path, &token).unwrap();
        let (unauthorized_health, _) = round_trip(
            "GET /v1/health HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(unauthorized_health.starts_with("HTTP/1.1 401"));
        let (health, _) = round_trip(
            &format!(
                "GET /v1/health HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {token}\r\n\r\n"
            ),
            path.clone(),
        );
        assert!(health.starts_with("HTTP/1.1 200"));
        let (unauthorized, _) = round_trip(
            "GET /v1/status HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(unauthorized.starts_with("HTTP/1.1 401"));
        let (authorized, _) = round_trip(
            &format!(
                "GET /v1/status HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {token}\r\n\r\n"
            ),
            path.clone(),
        );
        assert!(authorized.starts_with("HTTP/1.1 200"));
        fs::remove_file(path).ok();
    }

    #[test]
    fn authenticated_toggle_dispatches_exactly_one_action() {
        let path =
            std::env::temp_dir().join(format!("fluidvoice-api-toggle-{}", std::process::id()));
        let token = "b".repeat(64);
        fs::write(&path, &token).unwrap();
        let (response, events) = round_trip(
            &format!(
                "POST /v1/dictation/toggle HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {token}\r\nContent-Length: 0\r\n\r\n"
            ),
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

    #[test]
    fn missing_or_empty_token_fails_closed() {
        let path = std::env::temp_dir().join(format!(
            "fluidvoice-api-missing-token-{}",
            std::process::id()
        ));
        fs::remove_file(&path).ok();
        let (missing, _) = round_trip(
            "GET /v1/status HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(missing.starts_with("HTTP/1.1 503"));
        fs::write(&path, "").unwrap();
        let (empty, _) = round_trip(
            "POST /v1/dictation/toggle HTTP/1.1\r\nHost: localhost\r\n\r\n",
            path.clone(),
        );
        assert!(empty.starts_with("HTTP/1.1 503"));
        fs::remove_file(path).ok();
    }
}
