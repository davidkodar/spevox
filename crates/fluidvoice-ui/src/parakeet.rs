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

pub const RUNTIME_REVISION: &str = "9bc876635af36df537d9bc6d3f57ad1b76e4f74a";
pub const MODEL_FILE: &str = "parakeet-tdt-0.6b-v3.q8_0.gguf";
pub const MODEL_BYTES: u64 = 713_975_456;
pub const MODEL_SHA256: &str = "e3880d0aaaaf2c308ea2c35016b2b895c423eb3fda924c1b463d1c19b7f4d32e";
pub const ENDPOINT: &str = "http://127.0.0.1:8179";

const SOURCE_URL: &str = "https://github.com/NVIDIA/NeMo-Speech.cpp.git";
const MODEL_URL: &str = "https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3/resolve/main/parakeet-tdt-0.6b-v3.q8_0.gguf";

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
                || PathBuf::from(".local/share/fluidvoice"),
                |home| PathBuf::from(home).join(".local/share/fluidvoice"),
            )
        },
        |path| PathBuf::from(path).join("fluidvoice"),
    )
}

#[must_use]
pub fn model_path() -> PathBuf {
    data_directory().join("models/parakeet").join(MODEL_FILE)
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
pub fn model_installed() -> bool {
    let path = model_path();
    fs::metadata(&path).is_ok_and(|metadata| metadata.is_file() && metadata.len() == MODEL_BYTES)
        && fs::read_to_string(verification_path(&path))
            .is_ok_and(|digest| digest.trim() == MODEL_SHA256)
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

pub fn download_model(cancel: &AtomicBool, mut progress: impl FnMut(f32)) -> Result<(), String> {
    let destination = model_path();
    let parent = destination
        .parent()
        .ok_or_else(|| "invalid Parakeet model destination".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let partial = destination.with_extension("gguf.part");
    let response = ureq::get(MODEL_URL)
        .call()
        .map_err(|error| format!("Parakeet model request failed: {error}"))?;
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
        progress((downloaded as f64 / MODEL_BYTES as f64).clamp(0.0, 1.0) as f32);
    }
    output.sync_all().map_err(|error| error.to_string())?;
    drop(output);
    let digest = format!("{:x}", hasher.finalize());
    if downloaded != MODEL_BYTES || digest != MODEL_SHA256 {
        fs::remove_file(&partial).ok();
        return Err(format!(
            "model verification failed (bytes {downloaded}/{MODEL_BYTES}, sha256 {digest})"
        ));
    }
    fs::rename(&partial, &destination).map_err(|error| error.to_string())?;
    fs::write(verification_path(&destination), format!("{MODEL_SHA256}\n"))
        .map_err(|error| error.to_string())
}

pub fn delete_model() -> Result<(), String> {
    let path = model_path();
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
}

impl Supervisor {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            child: None,
            executable: None,
        }
    }

    pub fn ensure_ready(&mut self, backend: Backend) -> Result<(), String> {
        let executable = runtime_executable(backend)
            .ok_or_else(|| format!("{} NeMo-Speech.cpp runtime is not installed", backend.id()))?;
        if !model_installed() {
            return Err("Parakeet v3 model is not installed".to_owned());
        }
        if self.executable.as_ref() == Some(&executable)
            && self
                .child
                .as_mut()
                .is_some_and(|child| child.try_wait().ok().flatten().is_none())
            && ready()
        {
            return Ok(());
        }
        self.stop();
        let log_dir = data_directory().join("logs");
        fs::create_dir_all(&log_dir).map_err(|error| error.to_string())?;
        let stdout =
            fs::File::create(log_dir.join("nemo-speech.log")).map_err(|error| error.to_string())?;
        let stderr = stdout.try_clone().map_err(|error| error.to_string())?;
        let child = Command::new(&executable)
            .args(["serve", "--asr-model"])
            .arg(model_path())
            .args([
                "--host",
                "127.0.0.1",
                "--port",
                "8179",
                "--no-ui",
                "--max-upload-mb",
                "16",
                "--threads",
                "2",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .map_err(|error| format!("could not start NeMo-Speech.cpp: {error}"))?;
        self.child = Some(child);
        self.executable = Some(executable);
        for _ in 0..120 {
            if ready() {
                return Ok(());
            }
            if let Some(status) = self
                .child
                .as_mut()
                .and_then(|child| child.try_wait().ok().flatten())
            {
                self.child = None;
                return Err(format!("NeMo-Speech.cpp exited during startup ({status})"));
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
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        self.stop();
    }
}

fn ready() -> bool {
    ureq::get(&format!("{ENDPOINT}/ready"))
        .config()
        .timeout_global(Some(Duration::from_secs(1)))
        .build()
        .call()
        .is_ok()
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
        assert_eq!(MODEL_SHA256.len(), 64);
        assert!(MODEL_BYTES > 600_000_000);
        assert!(MODEL_URL.starts_with("https://huggingface.co/nvidia/"));
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
}
