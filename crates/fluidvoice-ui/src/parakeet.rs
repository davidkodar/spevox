use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use sha2::{Digest, Sha256};
use tungstenite::{Message, connect, stream::MaybeTlsStream};

use fluidvoice_audio::{AudioBuffer, StreamingAsrConverter};

pub const RUNTIME_REVISION: &str = "9bc876635af36df537d9bc6d3f57ad1b76e4f74a";
const REALTIME_FRAME_SAMPLES: usize = 2_560; // 160 ms at 16 kHz.

const SOURCE_URL: &str = "https://github.com/NVIDIA/NeMo-Speech.cpp.git";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Model {
    pub id: &'static str,
    pub name: &'static str,
    pub detail: &'static str,
    pub file: &'static str,
    pub bytes: u64,
    pub sha256: &'static str,
    pub url: &'static str,
    pub realtime: bool,
}

pub const PARAKEET_V3: Model = Model {
    id: "parakeet-v3",
    name: "Parakeet TDT v3",
    detail: "25 European languages · full-utterance/offline · 681 MiB",
    file: "parakeet-tdt-0.6b-v3.q8_0.gguf",
    bytes: 713_975_456,
    sha256: "e3880d0aaaaf2c308ea2c35016b2b895c423eb3fda924c1b463d1c19b7f4d32e",
    url: "https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3/resolve/main/parakeet-tdt-0.6b-v3.q8_0.gguf",
    realtime: false,
};
pub const NEMOTRON_35: Model = Model {
    id: "nemotron-35",
    name: "Nemotron 3.5 Multilingual",
    detail: "40+ language-locales · streaming RNNT · 707 MiB",
    file: "nemotron-3.5-asr-streaming-0.6b.q8_0.gguf",
    bytes: 741_548_352,
    sha256: "a5c435f294eea8f88ce68dd27b8c3bfea7f777cb2fbba04fcd30eaa555f429ae",
    url: "https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b/resolve/main/nemotron-3.5-asr-streaming-0.6b.q8_0.gguf",
    realtime: true,
};
pub const NEMOTRON_EN: Model = Model {
    id: "nemotron-en",
    name: "Nemotron Streaming English",
    detail: "Low-latency English RNNT · 668 MiB",
    file: "nemotron-speech-streaming-en-0.6b.q8_0.gguf",
    bytes: 699_872_960,
    sha256: "d9a01898d2a611c8764e23a1c2f45e70bbd5a425dc4de93692ac951dd603812d",
    url: "https://huggingface.co/nvidia/nemotron-speech-streaming-en-0.6b/resolve/main/nemotron-speech-streaming-en-0.6b.q8_0.gguf",
    realtime: true,
};
pub const PARAKEET_CTC: Model = Model {
    id: "parakeet-ctc",
    name: "Parakeet CTC 1.1B",
    detail: "High-throughput English · 1.10 GiB",
    file: "parakeet-ctc-1.1b.q8_0.gguf",
    bytes: 1_178_100_960,
    sha256: "6584fc0fdacf1c220401ea4c3a1d5b44454b655c141cb8672178072c203d92b8",
    url: "https://huggingface.co/nvidia/parakeet-ctc-1.1b/resolve/main/parakeet-ctc-1.1b.q8_0.gguf",
    realtime: true,
};
pub const SORTFORMER_V2: Model = Model {
    id: "sortformer-v2",
    name: "Sortformer Diarizer 4-speaker v2",
    detail: "Experimental speaker diarization · up to 4 speakers · 140 MiB",
    file: "sortformer-v2-q8_0.gguf",
    bytes: 147_075_776,
    sha256: "0679cfeb1ce356d0dea9470b31274f4bfc7eb927497d82005483770666da998a",
    url: "https://github.com/davidkodar/fluidvoice-linux/releases/download/models-sortformer-v2-q8_0/sortformer-v2-q8_0.gguf",
    realtime: true,
};

#[cfg(test)]
const MODELS: [Model; 5] = [
    PARAKEET_V3,
    NEMOTRON_35,
    NEMOTRON_EN,
    PARAKEET_CTC,
    SORTFORMER_V2,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Backend {
    Cpu,
    Vulkan,
}

impl Backend {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Vulkan => "vulkan",
        }
    }
}

#[must_use]
pub fn data_directory() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME").map_or_else(
        || {
            std::env::var_os("HOME").map_or_else(
                || std::env::temp_dir().join(format!("fluidvoice-{}", std::process::id())),
                |home| PathBuf::from(home).join(".local/share/fluidvoice"),
            )
        },
        |path| PathBuf::from(path).join("fluidvoice"),
    )
}

#[must_use]
pub fn model_path(model: Model) -> PathBuf {
    if model == PARAKEET_V3 {
        // Preserve the location used by the original one-model preview.
        data_directory().join("models/parakeet").join(model.file)
    } else if model == SORTFORMER_V2 {
        data_directory().join("models/diarization").join(model.file)
    } else {
        data_directory().join("models/native-asr").join(model.file)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiarizationSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speaker: u32,
}

pub fn diarize_file(
    backend: Backend,
    wav_path: &std::path::Path,
) -> Result<Vec<DiarizationSegment>, String> {
    let executable = runtime_executable(backend)
        .ok_or_else(|| format!("{} NeMo-Speech.cpp runtime is not installed", backend.id()))?;
    if !model_installed(SORTFORMER_V2) {
        return Err(format!("{} is not installed", SORTFORMER_V2.name));
    }
    let device = if backend == Backend::Cpu {
        "cpu"
    } else {
        "vulkan"
    };
    let output = Command::new(executable)
        .args(["--json", "diarize"])
        .arg(wav_path)
        .args(["--model"])
        .arg(model_path(SORTFORMER_V2))
        .args(["--backend", device, "--format", "json"])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("could not start speaker diarization: {error}"))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "speaker diarization failed: {}",
            detail.lines().last().unwrap_or("native runtime exited")
        ));
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid diarization response: {error}"))?;
    value
        .get("segments")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "diarization response did not contain segments".to_owned())?
        .iter()
        .map(|segment| {
            let start_seconds = segment
                .get("start")
                .and_then(serde_json::Value::as_f64)
                .ok_or_else(|| "diarization segment has no start time".to_owned())?;
            let end_seconds = segment
                .get("end")
                .and_then(serde_json::Value::as_f64)
                .ok_or_else(|| "diarization segment has no end time".to_owned())?;
            let speaker = segment
                .get("speaker")
                .and_then(serde_json::Value::as_u64)
                .and_then(|speaker| u32::try_from(speaker).ok())
                .ok_or_else(|| "diarization segment has no speaker label".to_owned())?;
            Ok(DiarizationSegment {
                start_seconds,
                end_seconds,
                speaker,
            })
        })
        .collect()
}

#[must_use]
pub fn runtime_prefix(backend: Backend) -> PathBuf {
    data_directory()
        .join("runtimes")
        .join(format!("nemo-speech-{}-{RUNTIME_REVISION}", backend.id()))
}

#[must_use]
pub fn runtime_executable(backend: Backend) -> Option<PathBuf> {
    let managed = runtime_prefix(backend).join("bin/nemo-speech");
    if managed.is_file() {
        return Some(managed);
    }
    executable_on_path("nemo-speech")
}

#[must_use]
pub fn runtime_installed(backend: Backend) -> bool {
    runtime_executable(backend).is_some()
}

#[must_use]
pub fn model_installed(model: Model) -> bool {
    let path = model_path(model);
    fs::metadata(&path).is_ok_and(|metadata| metadata.is_file() && metadata.len() == model.bytes)
        && fs::read_to_string(verification_path(&path))
            .is_ok_and(|digest| digest.trim() == model.sha256)
}

/// Streams captured audio chunks to the managed `NeMo` server and publishes
/// cumulative partial text until the sender is dropped.
pub fn stream_transcript(
    endpoint: &str,
    receiver: &std::sync::mpsc::Receiver<AudioBuffer>,
    language: String,
    gain: f32,
    mut publish: impl FnMut(String),
) -> Result<(), String> {
    let realtime_endpoint = format!("{}/v1/realtime", endpoint.replacen("http://", "ws://", 1));
    let (mut socket, _) = connect(realtime_endpoint)
        .map_err(|error| format!("could not connect realtime transcription: {error}"))?;
    if let MaybeTlsStream::Plain(stream) = socket.get_mut() {
        stream
            .set_read_timeout(Some(Duration::from_millis(10)))
            .map_err(|error| error.to_string())?;
    }
    let mut session = serde_json::json!({
        "type": "session.update",
        "session": {
            "sample_rate": 16_000,
            "automatic_punctuation": true
        }
    });
    if !language.is_empty() {
        session["session"]["language"] = serde_json::Value::String(language);
    }
    socket
        .send(Message::Text(session.to_string().into()))
        .map_err(|error| format!("could not configure realtime transcription: {error}"))?;

    let mut accumulated = String::new();
    let mut pending = Vec::<i16>::with_capacity(REALTIME_FRAME_SAMPLES * 2);
    let mut converter = StreamingAsrConverter::default();
    while let Ok(chunk) = receiver.recv() {
        let mono = converter.process(&chunk).amplified(gain);
        for sample in mono.samples() {
            #[allow(clippy::cast_possible_truncation)]
            let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
            pending.push(value);
        }
        while pending.len() >= REALTIME_FRAME_SAMPLES {
            send_pcm16(&mut socket, pending.drain(..REALTIME_FRAME_SAMPLES))?;
            drain_realtime_events(&mut socket, &mut accumulated, &mut publish)?;
        }
    }
    if !pending.is_empty() {
        send_pcm16(&mut socket, pending.drain(..))?;
        drain_realtime_events(&mut socket, &mut accumulated, &mut publish)?;
    }
    // The regular HTTP transcription below this preview is authoritative.
    // Discard the realtime session instead of running the same final inference
    // twice and blocking the model worker after the shortcut is released.
    socket
        .send(Message::Text(r#"{"type":"response.cancel"}"#.into()))
        .map_err(|error| format!("could not cancel realtime preview: {error}"))?;
    socket.close(None).ok();
    Ok(())
}

fn send_pcm16(
    socket: &mut tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    samples: impl Iterator<Item = i16>,
) -> Result<(), String> {
    let mut pcm = Vec::with_capacity(REALTIME_FRAME_SAMPLES * 2);
    for sample in samples {
        pcm.extend_from_slice(&sample.to_le_bytes());
    }
    socket
        .send(Message::Binary(pcm.into()))
        .map_err(|error| format!("could not send realtime audio: {error}"))
}

fn drain_realtime_events(
    socket: &mut tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    accumulated: &mut String,
    publish: &mut impl FnMut(String),
) -> Result<(), String> {
    loop {
        match socket.read() {
            Ok(Message::Text(text)) => {
                let event: serde_json::Value = serde_json::from_str(&text)
                    .map_err(|error| format!("invalid realtime response: {error}"))?;
                if let Some(text) = apply_realtime_event(&event, accumulated)? {
                    publish(text);
                }
            }
            Ok(Message::Close(_)) => return Ok(()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                return Ok(());
            }
            Err(error) => return Err(format!("realtime transcription failed: {error}")),
        }
    }
}

fn apply_realtime_event(
    event: &serde_json::Value,
    accumulated: &mut String,
) -> Result<Option<String>, String> {
    match event
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
    {
        "conversation.item.input_audio_transcription.delta" => {
            if let Some(delta) = event.get("delta").and_then(serde_json::Value::as_str) {
                accumulated.push_str(delta);
                Ok(Some(accumulated.clone()))
            } else {
                Ok(None)
            }
        }
        "conversation.item.input_audio_transcription.completed" => {
            if let Some(text) = event.get("transcript").and_then(serde_json::Value::as_str) {
                text.clone_into(accumulated);
                Ok(Some(accumulated.clone()))
            } else {
                Ok(None)
            }
        }
        "error" => Err(event
            .pointer("/error/message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown realtime transcription error")
            .to_owned()),
        _ => Ok(None),
    }
}

pub fn install_runtime(backend: Backend) -> Result<(), String> {
    for tool in ["git", "cmake", "ninja", "bash", "cc", "c++"] {
        if executable_on_path(tool).is_none() {
            return Err(format!(
                "{tool} is required to build the pinned native runtime; install it and retry"
            ));
        }
    }
    if backend == Backend::Vulkan && executable_on_path("glslc").is_none() {
        return Err("glslc and the Vulkan development files are required for the Vulkan runtime; install them or select CPU".to_owned());
    }
    let sources = data_directory().join("sources");
    fs::create_dir_all(&sources).map_err(|error| error.to_string())?;
    let checkout = sources.join(format!("nemo-speech-{RUNTIME_REVISION}"));
    if !checkout.join(".git").is_dir() {
        if checkout.exists() {
            fs::remove_dir_all(&checkout).map_err(|error| error.to_string())?;
        }
        run(
            Command::new("git")
                .args(["clone", "--filter=blob:none", "--no-checkout", SOURCE_URL])
                .arg(&checkout),
            "clone NeMo-Speech.cpp",
        )?;
    }
    run(
        Command::new("git").arg("-C").arg(&checkout).args([
            "fetch",
            "--depth",
            "1",
            "origin",
            RUNTIME_REVISION,
        ]),
        "fetch pinned NeMo-Speech.cpp revision",
    )?;
    run(
        Command::new("git").arg("-C").arg(&checkout).args([
            "checkout",
            "--detach",
            RUNTIME_REVISION,
        ]),
        "check out pinned NeMo-Speech.cpp revision",
    )?;
    run(
        Command::new("git")
            .arg("-C")
            .arg(&checkout)
            .args(["submodule", "update", "--init", "--depth", "1"])
            .args(["ggml", "third_party/cpp-httplib"]),
        "prepare NeMo-Speech.cpp dependencies",
    )?;
    let sentencepiece_build = checkout.join(".deps/sentencepiece-build/build");
    let sentencepiece_library = checkout.join(".deps/sentencepiece/lib/libsentencepiece.a");
    if !sentencepiece_library.is_file() && sentencepiece_build.is_dir() {
        fs::remove_dir_all(&sentencepiece_build).map_err(|error| error.to_string())?;
    }
    run(
        Command::new(checkout.join("scripts/build_sentencepiece_static.sh"))
            .current_dir(&checkout)
            // Current Arch toolchains use CMake 4 and GCC 16. The pinned
            // dependency predates both defaults but is source-compatible.
            .env("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
            .env("CXXFLAGS", "-include cstdint"),
        "build the pinned SentencePiece dependency",
    )?;
    let preset = format!("{}-server", backend.id());
    run(
        Command::new(checkout.join("scripts/configure.sh"))
            .current_dir(&checkout)
            .arg(&preset),
        "configure NeMo-Speech.cpp",
    )?;
    run(
        Command::new("cmake")
            .current_dir(&checkout)
            .args(["--build", "--preset"])
            .arg(&preset),
        "build NeMo-Speech.cpp",
    )?;
    let prefix = runtime_prefix(backend);
    fs::create_dir_all(&prefix).map_err(|error| error.to_string())?;
    run(
        Command::new("cmake")
            .current_dir(&checkout)
            .arg("--install")
            .arg(checkout.join("build").join(&preset))
            .arg("--prefix")
            .arg(&prefix),
        "install NeMo-Speech.cpp",
    )?;
    if !prefix.join("bin/nemo-speech").is_file() {
        return Err("runtime build completed without installing bin/nemo-speech".to_owned());
    }
    Ok(())
}

pub fn download_model(
    model: Model,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32),
) -> Result<(), String> {
    let destination = model_path(model);
    let parent = destination
        .parent()
        .ok_or_else(|| "invalid native model destination".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let partial = destination.with_extension("gguf.part");
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_mins(30)))
        .timeout_recv_body(Some(Duration::from_secs(30)))
        .build()
        .new_agent();
    let response = agent
        .get(model.url)
        .call()
        .map_err(|error| format!("{} request failed: {error}", model.name))?;
    let mut reader = response.into_body().into_reader();
    let mut output = fs::File::create(&partial).map_err(|error| error.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 256 * 1024];
    let mut downloaded = 0_u64;
    loop {
        if cancel.load(Ordering::Relaxed) {
            drop(output);
            fs::remove_file(&partial).ok();
            return Err("cancelled".to_owned());
        }
        let count = reader
            .read(&mut buffer)
            .map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        output
            .write_all(&buffer[..count])
            .map_err(|error| error.to_string())?;
        hasher.update(&buffer[..count]);
        downloaded = downloaded.saturating_add(u64::try_from(count).unwrap_or_default());
        progress((downloaded as f64 / model.bytes as f64).clamp(0.0, 1.0) as f32);
    }
    output.sync_all().map_err(|error| error.to_string())?;
    drop(output);
    let digest = format!("{:x}", hasher.finalize());
    if downloaded != model.bytes || digest != model.sha256 {
        fs::remove_file(&partial).ok();
        return Err(format!(
            "model verification failed (bytes {downloaded}/{}, sha256 {digest})",
            model.bytes
        ));
    }
    fs::rename(&partial, &destination).map_err(|error| error.to_string())?;
    fs::write(
        verification_path(&destination),
        format!("{}\n", model.sha256),
    )
    .map_err(|error| error.to_string())
}

pub fn delete_model(model: Model) -> Result<(), String> {
    let path = model_path(model);
    if path.is_file() {
        fs::remove_file(&path).map_err(|error| error.to_string())?;
    }
    fs::remove_file(verification_path(&path)).ok();
    Ok(())
}

fn verification_path(model: &std::path::Path) -> PathBuf {
    model.with_extension("gguf.sha256")
}

pub struct Supervisor {
    child: Option<Child>,
    executable: Option<PathBuf>,
    model: Option<Model>,
    port: u16,
}

impl Supervisor {
    #[must_use]
    pub fn new() -> Self {
        Self {
            child: None,
            executable: None,
            model: None,
            port: reserve_loopback_port().unwrap_or(8179),
        }
    }

    #[must_use]
    pub fn endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub fn ensure_ready(&mut self, backend: Backend, model: Model) -> Result<(), String> {
        let executable = runtime_executable(backend)
            .ok_or_else(|| format!("{} NeMo-Speech.cpp runtime is not installed", backend.id()))?;
        if !model_installed(model) {
            return Err(format!("{} is not installed", model.name));
        }
        if self.executable.as_ref() == Some(&executable)
            && self.model == Some(model)
            && self
                .child
                .as_mut()
                .is_some_and(|child| child.try_wait().ok().flatten().is_none())
            && ready(self.port)
        {
            return Ok(());
        }
        self.stop();
        let log_dir = data_directory().join("logs");
        fs::create_dir_all(&log_dir).map_err(|error| error.to_string())?;
        let stdout =
            fs::File::create(log_dir.join("nemo-speech.log")).map_err(|error| error.to_string())?;
        let stderr = stdout.try_clone().map_err(|error| error.to_string())?;
        let thread_count = std::thread::available_parallelism()
            .map_or(1, std::num::NonZero::get)
            .to_string();
        let mut command = Command::new(&executable);
        command
            .args(["serve", "--asr-model"])
            .arg(model_path(model))
            .args(["--host", "127.0.0.1", "--port"])
            .arg(self.port.to_string())
            .args(["--no-ui", "--max-upload-mb", "16", "--threads"])
            .arg(thread_count);
        // FluidVoice uses Nemotron for completed dictation rather than a
        // latency-critical voice-agent stream. NVIDIA's largest trained
        // right-context geometry materially improves broad-coverage languages
        // such as Swedish, so prefer accuracy over the runtime's 160 ms default.
        if model == NEMOTRON_35 {
            command.args([
                "--asr.streaming.chunk_size",
                "1.12",
                "--asr.streaming.rnnt_right_context",
                "13",
            ]);
        }
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .map_err(|error| format!("could not start NeMo-Speech.cpp: {error}"))?;
        self.child = Some(child);
        self.executable = Some(executable);
        self.model = Some(model);
        for _ in 0..120 {
            if let Some(status) = self
                .child
                .as_mut()
                .and_then(|child| child.try_wait().ok().flatten())
            {
                self.child = None;
                return Err(format!("NeMo-Speech.cpp exited during startup ({status})"));
            }
            if ready(self.port) {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(500));
        }
        self.stop();
        Err("NeMo-Speech.cpp did not become ready within 60 seconds".to_owned())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().ok();
            child.wait().ok();
        }
        self.executable = None;
        self.model = None;
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        self.stop();
    }
}

fn ready(port: u16) -> bool {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(1)))
        .timeout_recv_body(Some(Duration::from_secs(1)))
        .proxy(None)
        .max_redirects(0)
        .build()
        .new_agent()
        .get(&format!("http://127.0.0.1:{port}/ready"))
        .call()
        .is_ok()
}

fn reserve_loopback_port() -> Result<u16, String> {
    std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
        .and_then(|listener| listener.local_addr())
        .map(|address| address.port())
        .map_err(|error| format!("could not select a private native-speech port: {error}"))
}

fn executable_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join(name))
        .find(|candidate| candidate.is_file())
}

fn run(command: &mut Command, description: &str) -> Result<(), String> {
    let output = command
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("could not {description}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let detail = stderr
        .lines()
        .rev()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");
    Err(format!(
        "could not {description} ({}): {detail}",
        output.status
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_model_metadata_is_complete() {
        for model in MODELS {
            assert_eq!(model.sha256.len(), 64);
            assert!(model.bytes > 100_000_000);
            assert!(
                model.url.starts_with("https://huggingface.co/")
                    || model
                        .url
                        .starts_with("https://github.com/davidkodar/fluidvoice-linux/")
            );
        }
        assert!(!PARAKEET_V3.realtime);
        assert!(NEMOTRON_35.realtime);
        assert!(NEMOTRON_EN.realtime);
        assert!(PARAKEET_CTC.realtime);
        assert!(SORTFORMER_V2.realtime);
        assert!(SORTFORMER_V2.url.contains("models-sortformer-v2-q8_0"));
    }

    #[test]
    fn runtime_revision_is_a_full_git_object_id() {
        assert_eq!(RUNTIME_REVISION.len(), 40);
        assert!(
            RUNTIME_REVISION
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        );
    }

    #[test]
    fn accumulates_realtime_deltas_and_accepts_authoritative_final_text() {
        let mut text = String::new();
        let first = apply_realtime_event(
            &serde_json::json!({
                "type": "conversation.item.input_audio_transcription.delta",
                "delta": "hello "
            }),
            &mut text,
        )
        .expect("delta");
        assert_eq!(first.as_deref(), Some("hello "));
        let final_text = apply_realtime_event(
            &serde_json::json!({
                "type": "conversation.item.input_audio_transcription.completed",
                "transcript": "Hello world."
            }),
            &mut text,
        )
        .expect("completed");
        assert_eq!(final_text.as_deref(), Some("Hello world."));
        assert_eq!(text, "Hello world.");
    }

    #[test]
    fn surfaces_realtime_server_errors() {
        let error = apply_realtime_event(
            &serde_json::json!({"type": "error", "error": {"message": "bad locale"}}),
            &mut String::new(),
        )
        .expect_err("server error");
        assert_eq!(error, "bad locale");
    }

    #[test]
    #[ignore = "requires a managed NeMo server on 127.0.0.1:8179"]
    fn realtime_endpoint_accepts_framed_pcm_and_cancel() {
        let (sender, receiver) = std::sync::mpsc::channel();
        sender
            .send(AudioBuffer::new(vec![0.0; 3_200], 16_000, 1, false).expect("audio"))
            .expect("send audio");
        drop(sender);
        stream_transcript(
            "http://127.0.0.1:8179",
            &receiver,
            "sv-SE".to_owned(),
            1.0,
            |_| {},
        )
        .expect("realtime session");
    }
}
