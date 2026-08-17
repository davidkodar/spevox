/// Converts bounded counters to a UI progress ratio. Catalog sizes and audio
/// counters stay far below the integer precision limits of `f64`/`f32`.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub(crate) fn progress_ratio(numerator: u64, denominator: u64) -> f32 {
    (numerator as f64 / denominator.max(1) as f64).clamp(0.0, 1.0) as f32
}

#[allow(clippy::cast_precision_loss)]
pub(super) fn display_ratio(numerator: u64, denominator: u64) -> f64 {
    numerator as f64 / denominator.max(1) as f64
}

pub(super) fn shortcut_triggers() -> &'static [(&'static str, &'static str)] {
    &[
        ("Ctrl  Alt  D", "CTRL+ALT+D"),
        ("Ctrl  Alt  Space", "CTRL+ALT+SPACE"),
        ("Meta  Alt  D", "META+ALT+D"),
        ("Meta  Shift  Space", "META+SHIFT+SPACE"),
    ]
}

pub(super) fn selected_language_code(controller: &SpevoxControllerRust) -> String {
    usize::try_from(controller.selected_language)
        .ok()
        .and_then(|index| controller.language_codes.get(index))
        .cloned()
        .unwrap_or_else(|| "en".to_owned())
}

pub(super) fn parakeet_backend(compute_backend: i32) -> ParakeetBackend {
    if compute_backend == 2 {
        ParakeetBackend::Cpu
    } else {
        ParakeetBackend::Vulkan
    }
}

pub(super) fn compute_backend_summary(compute_backend: i32) -> String {
    let cpu = parakeet::runtime_installed(ParakeetBackend::Cpu);
    let vulkan = parakeet::runtime_installed(ParakeetBackend::Vulkan);
    let runtime = match (vulkan, cpu) {
        (true, true) => "native Vulkan and CPU runtimes installed",
        (true, false) => "native Vulkan runtime installed; CPU fallback not installed",
        (false, true) => "native CPU runtime installed; Vulkan runtime unavailable",
        (false, false) => "native runtimes not installed",
    };
    match compute_backend {
        2 => format!("CPU only · Whisper requests CPU · {runtime}"),
        1 => format!("Vulkan only · Whisper requests GPU · {runtime}"),
        _ if vulkan => format!(
            "Automatic · Whisper requests GPU · native engines use Vulkan · CPU fallback {}",
            if cpu { "ready" } else { "not installed" }
        ),
        _ if cpu => "Automatic · Whisper requests GPU with library fallback · native engines use installed CPU runtime".to_owned(),
        _ => "Automatic · Whisper requests GPU with library fallback · native runtime setup required".to_owned(),
    }
}

pub(super) fn native_model_for_engine(engine: i32) -> Option<parakeet::Model> {
    match engine {
        1 => Some(parakeet::PARAKEET_V3),
        2 => Some(parakeet::NEMOTRON_35),
        3 => Some(parakeet::NEMOTRON_EN),
        4 => Some(parakeet::PARAKEET_CTC),
        _ => None,
    }
}

pub(super) fn native_language_for_model(model: parakeet::Model, language: &str) -> String {
    if model == parakeet::NEMOTRON_EN || model == parakeet::PARAKEET_CTC {
        return "en-US".to_owned();
    }
    if model != parakeet::NEMOTRON_35 || language.is_empty() {
        return language.to_owned();
    }

    let locale = match language {
        "ar" => "ar-SA",
        "bg" => "bg-BG",
        "ca" => "ca-ES",
        "cs" => "cs-CZ",
        "da" => "da-DK",
        "de" => "de-DE",
        "el" => "el-GR",
        "en" => "en-US",
        "es" => "es-ES",
        "et" => "et-EE",
        "fa" => "fa-IR",
        "fi" => "fi-FI",
        "fr" => "fr-FR",
        "he" => "he-IL",
        "hi" => "hi-IN",
        "hr" => "hr-HR",
        "hu" => "hu-HU",
        "id" => "id-ID",
        "it" => "it-IT",
        "ja" => "ja-JP",
        "ko" => "ko-KR",
        "lt" => "lt-LT",
        "lv" => "lv-LV",
        "ms" => "ms-MY",
        "nl" => "nl-NL",
        "no" => "nb-NO",
        "pl" => "pl-PL",
        "pt" => "pt-BR",
        "ro" => "ro-RO",
        "ru" => "ru-RU",
        "sk" => "sk-SK",
        "sl" => "sl-SI",
        "sv" => "sv-SE",
        "th" => "th-TH",
        "tr" => "tr-TR",
        "uk" => "uk-UA",
        "vi" => "vi-VN",
        "zh" => "zh-CN",
        _ => return String::new(),
    };
    locale.to_owned()
}

pub(super) fn native_model_supports_language(model: parakeet::Model, language: &str) -> bool {
    let language = language.trim();
    language.is_empty()
        || (model != parakeet::NEMOTRON_EN && model != parakeet::PARAKEET_CTC)
        || language.eq_ignore_ascii_case("en")
        || language.to_ascii_lowercase().starts_with("en-")
}

pub(super) fn effective_parakeet_backend(compute_backend: i32) -> ParakeetBackend {
    if compute_backend == 0
        && !parakeet::runtime_installed(ParakeetBackend::Vulkan)
        && parakeet::runtime_installed(ParakeetBackend::Cpu)
    {
        ParakeetBackend::Cpu
    } else {
        parakeet_backend(compute_backend)
    }
}

pub(super) fn parakeet_runtime_available(compute_backend: i32) -> bool {
    parakeet::runtime_installed(effective_parakeet_backend(compute_backend))
}

pub(super) fn friendly_runtime_error(error: &str) -> String {
    if error.contains("SPIRV-Headers") || error.contains("spirv-headers") {
        "Vulkan development package SPIRV-Headers is missing".to_owned()
    } else if error.contains("glslc") {
        "the Vulkan shader compiler (glslc) is missing".to_owned()
    } else {
        error.lines().next().unwrap_or(error).trim().to_owned()
    }
}

pub(super) fn language_display_name(code: &str) -> Option<&'static str> {
    supported_languages()
        .iter()
        .find_map(|(name, candidate)| (*candidate == code).then_some(*name))
}

pub(super) fn selected_model_path(controller: &SpevoxControllerRust) -> Option<PathBuf> {
    let index = usize::try_from(controller.selected_model).ok()?;
    let path = controller.model_paths.get(index)?;
    model_file_valid(path, whisper_model_catalog().get(index)?).then(|| path.clone())
}

pub(super) fn selected_shortcut_trigger(controller: &SpevoxControllerRust) -> String {
    usize::try_from(controller.selected_shortcut)
        .ok()
        .and_then(|index| shortcut_triggers().get(index))
        .map_or("CTRL+ALT+D", |(_, trigger)| trigger)
        .to_owned()
}

pub(super) fn valid_index(index: i32, length: usize) -> bool {
    usize::try_from(index).is_ok_and(|index| index < length)
}

pub(super) fn dump_asr_audio(audio: &spevox_audio::MonoAudioBuffer) -> Result<(), String> {
    let Some(path) = std::env::var_os("SPEVOX_ASR_DUMP").map(PathBuf::from) else {
        return Ok(());
    };
    let wav = spevox_audio::encode_pcm16_wav(audio).map_err(|error| error.to_string())?;
    fs::write(&path, wav).map_err(|error| format!("{}: {error}", path.display()))
}

pub(super) fn meter_level(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return 0.0;
    }
    ((20.0 * peak.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

pub(super) fn peak_db(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return -60.0;
    }
    (20.0 * peak.log10()).clamp(-60.0, 0.0)
}

pub(super) fn asr_gain(peak: f32, user_gain: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.000_5 {
        return 1.0;
    }
    // Normalize ordinary speech to a conservative peak, then treat the UI gain
    // as an adjustment. Always retain headroom so a high setting cannot turn
    // the buffer sent to Whisper into a clipped square wave.
    let automatic = (0.35 / peak).clamp(1.0, 64.0);
    let requested = automatic * user_gain.max(0.0);
    let headroom_limit = (0.85 / peak).min(64.0);
    requested.min(headroom_limit).max(1.0)
}

pub(super) fn suspicious_single_word(text: &str, duration: Duration) -> bool {
    if duration < Duration::from_secs(2) {
        return false;
    }
    let normalized = text
        .trim()
        .trim_matches(|character: char| !character.is_alphanumeric())
        .to_ascii_lowercase();
    matches!(normalized.as_str(), "you" | "thanks" | "thank you")
}
use super::{
    AtomicBool, Duration, Instant, ParakeetBackend, PathBuf, SpevoxControllerRust, fs,
    model_file_valid, parakeet, supported_languages, whisper_model_catalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct NativeRuntimeInstall {
    pub(super) backend: ParakeetBackend,
    pub(super) fell_back: bool,
}

pub(super) fn install_native_runtime(compute_backend: i32) -> Result<NativeRuntimeInstall, String> {
    install_native_runtime_backend(compute_backend, true)
}

pub(super) fn prepare_native_model(
    compute_backend: i32,
    model: parakeet::Model,
    cancel: &AtomicBool,
    force_download: bool,
    mut progress: impl FnMut(f32),
) -> Result<NativeRuntimeInstall, String> {
    let runtime = install_native_runtime_backend(compute_backend, false)?;
    if force_download || !parakeet::model_installed(model) {
        let mut last_progress = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .unwrap_or_else(Instant::now);
        parakeet::download_model(model, cancel, move |value| {
            if !native_progress_update_due(value, last_progress.elapsed()) {
                return;
            }
            last_progress = Instant::now();
            progress(value);
        })?;
    }
    Ok(runtime)
}

fn native_progress_update_due(value: f32, elapsed: Duration) -> bool {
    value >= 1.0 || elapsed >= Duration::from_millis(50)
}

fn install_native_runtime_backend(
    compute_backend: i32,
    force: bool,
) -> Result<NativeRuntimeInstall, String> {
    let backend = parakeet_backend(compute_backend);
    if !force && parakeet::runtime_installed(backend) {
        return Ok(NativeRuntimeInstall {
            backend,
            fell_back: false,
        });
    }
    parakeet::install_runtime(backend)
        .map(|()| NativeRuntimeInstall {
            backend,
            fell_back: false,
        })
        .or_else(|backend_error| {
            if compute_backend == 0 && backend == ParakeetBackend::Vulkan {
                if !force && parakeet::runtime_installed(ParakeetBackend::Cpu) {
                    return Ok(NativeRuntimeInstall {
                        backend: ParakeetBackend::Cpu,
                        fell_back: true,
                    });
                }
                parakeet::install_runtime(ParakeetBackend::Cpu)
                    .map(|()| NativeRuntimeInstall {
                        backend: ParakeetBackend::Cpu,
                        fell_back: true,
                    })
                    .map_err(|cpu_error| {
                        format!(
                            "Vulkan setup failed: {} CPU fallback also failed: {}",
                            friendly_runtime_error(&backend_error),
                            friendly_runtime_error(&cpu_error)
                        )
                    })
            } else {
                Err(friendly_runtime_error(&backend_error))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::native_progress_update_due;
    use std::time::Duration;

    #[test]
    fn native_progress_is_bounded_but_always_publishes_completion() {
        assert!(!native_progress_update_due(0.5, Duration::from_millis(49)));
        assert!(native_progress_update_due(0.5, Duration::from_millis(50)));
        assert!(native_progress_update_due(1.0, Duration::ZERO));
    }
}
