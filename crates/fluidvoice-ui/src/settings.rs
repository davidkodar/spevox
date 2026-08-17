use super::{
    PathBuf, ai_profiles_path, atomic_write_private, escape_setting, fs, preferences_path,
    unescape_setting,
};

// Persisted feature switches are independent user choices rather than one
// state machine, so explicit booleans make migrations and defaults auditable.
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Preferences {
    pub(super) onboarding_completed: bool,
    pub(super) language: String,
    pub(super) model: PathBuf,
    pub(super) shortcut: String,
    pub(super) input: String,
    pub(super) gain_db: f32,
    pub(super) overlay_enabled: bool,
    pub(super) overlay_size: i32,
    pub(super) overlay_position: i32,
    pub(super) overlay_show_text: bool,
    pub(super) overlay_opacity: f32,
    pub(super) command_mode_enabled: bool,
    pub(super) compute_backend: i32,
    pub(super) theme: i32,
    pub(super) accent: i32,
    pub(super) ai_enabled: bool,
    pub(super) ai_provider: i32,
    pub(super) ai_model: String,
    pub(super) ai_base_url: String,
    pub(super) ai_prompt: String,
    pub(super) ai_local_only: bool,
    pub(super) auto_profiles_enabled: bool,
    pub(super) typing_wpm: i32,
    pub(super) skip_weekends: bool,
    pub(super) audio_history_enabled: bool,
    pub(super) audio_history_budget_mb: i32,
    pub(super) local_api_enabled: bool,
    pub(super) local_api_port: i32,
    pub(super) speech_engine: i32,
    pub(super) local_speech_url: String,
    pub(super) diarization_enabled: bool,
}

#[derive(Clone)]
pub(super) struct AiProfile {
    pub(super) name: String,
    pub(super) prompt: String,
    pub(super) application_match: String,
}

#[derive(Clone)]
pub(super) enum WriteModeJob {
    Rewrite {
        instruction: String,
        selected: String,
    },
    Draft {
        instruction: String,
    },
}

pub(super) fn load_ai_profiles() -> Vec<AiProfile> {
    let Ok(contents) = fs::read_to_string(ai_profiles_path()) else {
        return Vec::new();
    };
    serde_json::from_str::<serde_json::Value>(&contents)
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| {
            Some(AiProfile {
                name: value.get("name")?.as_str()?.to_owned(),
                prompt: value.get("prompt")?.as_str()?.to_owned(),
                application_match: value
                    .get("application_match")
                    .and_then(|value| value.as_str())
                    .unwrap_or_default()
                    .to_owned(),
            })
        })
        .filter(|profile| !profile.name.trim().is_empty() && !profile.prompt.trim().is_empty())
        .collect()
}

pub(super) fn save_ai_profiles(profiles: &[AiProfile]) -> Result<(), String> {
    let values = profiles
        .iter()
        .map(|profile| {
            serde_json::json!({
                "name": profile.name,
                "prompt": profile.prompt,
                "application_match": profile.application_match
            })
        })
        .collect::<Vec<_>>();
    let path = ai_profiles_path();
    let parent = path
        .parent()
        .ok_or_else(|| "profile path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    atomic_write_private(
        &path,
        serde_json::to_string_pretty(&values)
            .map_err(|error| error.to_string())?
            .as_bytes(),
    )
}

pub(super) fn profile_matches_application(
    profile: &AiProfile,
    lowercase_window_identity: &str,
) -> bool {
    let matcher = profile.application_match.trim().to_lowercase();
    !matcher.is_empty()
        && matcher
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .any(|part| lowercase_window_identity.contains(part))
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            onboarding_completed: false,
            language: "en".to_owned(),
            model: PathBuf::new(),
            shortcut: "CTRL+ALT+D".to_owned(),
            input: String::new(),
            gain_db: 0.0,
            overlay_enabled: true,
            overlay_size: 1,
            overlay_position: 0,
            overlay_show_text: true,
            overlay_opacity: 0.98,
            command_mode_enabled: false,
            compute_backend: 0,
            theme: 0,
            accent: 0,
            ai_enabled: false,
            ai_provider: 7,
            ai_model: String::new(),
            ai_base_url: String::new(),
            ai_prompt: String::new(),
            ai_local_only: true,
            auto_profiles_enabled: false,
            typing_wpm: 40,
            skip_weekends: true,
            audio_history_enabled: false,
            audio_history_budget_mb: 500,
            local_api_enabled: false,
            local_api_port: 43_128,
            speech_engine: 0,
            local_speech_url: "http://127.0.0.1:8080".to_owned(),
            diarization_enabled: false,
        }
    }
}

impl Preferences {
    pub(super) fn load() -> Self {
        let Ok(contents) = fs::read_to_string(preferences_path()) else {
            return Self::default();
        };
        let mut preferences = Self {
            onboarding_completed: true,
            ..Self::default()
        };
        // Existing installations predate onboarding and must not be interrupted
        // by a first-run dialog after upgrading. An explicit saved false value
        // still allows users to reopen the guide from Getting Started.
        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("language=") {
                value.clone_into(&mut preferences.language);
            } else if let Some(value) = line.strip_prefix("model=") {
                preferences.model = PathBuf::from(value);
            } else if let Some(value) = line.strip_prefix("shortcut=") {
                value.clone_into(&mut preferences.shortcut);
            } else if let Some(value) = line.strip_prefix("input=") {
                value.clone_into(&mut preferences.input);
            } else if let Some(value) = line.strip_prefix("gain_db=") {
                preferences.gain_db = value.parse().unwrap_or(0.0);
            } else if let Some(value) = line.strip_prefix("overlay_enabled=") {
                preferences.overlay_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("overlay_size=") {
                preferences.overlay_size = value.parse().unwrap_or(1).clamp(0, 2);
            } else if let Some(value) = line.strip_prefix("overlay_position=") {
                preferences.overlay_position = value.parse().unwrap_or(0).clamp(0, 2);
            } else if let Some(value) = line.strip_prefix("overlay_show_text=") {
                preferences.overlay_show_text = value == "true";
            } else if let Some(value) = line.strip_prefix("overlay_opacity=") {
                preferences.overlay_opacity = value.parse::<f32>().unwrap_or(0.98).clamp(0.55, 1.0);
            } else if let Some(value) = line.strip_prefix("command_mode_enabled=") {
                preferences.command_mode_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("compute_backend=") {
                preferences.compute_backend = value.parse().unwrap_or(0);
            } else if let Some(value) = line.strip_prefix("theme=") {
                preferences.theme = value.parse().unwrap_or(0);
            } else if let Some(value) = line.strip_prefix("accent=") {
                preferences.accent = value.parse().unwrap_or(0);
            } else if let Some(value) = line.strip_prefix("ai_enabled=") {
                preferences.ai_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("ai_provider=") {
                preferences.ai_provider = value.parse().unwrap_or(7);
            } else if let Some(value) = line.strip_prefix("ai_model=") {
                preferences.ai_model = unescape_setting(value);
            } else if let Some(value) = line.strip_prefix("ai_base_url=") {
                preferences.ai_base_url = unescape_setting(value);
            } else if let Some(value) = line.strip_prefix("ai_prompt=") {
                preferences.ai_prompt = unescape_setting(value);
            } else if let Some(value) = line.strip_prefix("ai_local_only=") {
                preferences.ai_local_only = value == "true";
            } else if let Some(value) = line.strip_prefix("auto_profiles_enabled=") {
                preferences.auto_profiles_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("typing_wpm=") {
                preferences.typing_wpm = value.parse().unwrap_or(40).clamp(10, 250);
            } else if let Some(value) = line.strip_prefix("skip_weekends=") {
                preferences.skip_weekends = value == "true";
            } else if let Some(value) = line.strip_prefix("audio_history_enabled=") {
                preferences.audio_history_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("audio_history_budget_mb=") {
                preferences.audio_history_budget_mb =
                    value.parse().unwrap_or(500).clamp(100, 10_000);
            } else if let Some(value) = line.strip_prefix("local_api_enabled=") {
                preferences.local_api_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("local_api_port=") {
                preferences.local_api_port = value.parse().unwrap_or(43_128).clamp(1024, 65_535);
            } else if let Some(value) = line.strip_prefix("speech_engine=") {
                preferences.speech_engine = value.parse().unwrap_or(0).clamp(0, 5);
            } else if let Some(value) = line.strip_prefix("local_speech_url=") {
                preferences.local_speech_url = unescape_setting(value);
            } else if let Some(value) = line.strip_prefix("diarization_enabled=") {
                preferences.diarization_enabled = value == "true";
            } else if let Some(value) = line.strip_prefix("onboarding_completed=") {
                preferences.onboarding_completed = value == "true";
            }
        }
        preferences
    }

    pub(super) fn save(&self) -> Result<(), String> {
        let path = preferences_path();
        let parent = path
            .parent()
            .ok_or_else(|| "preferences path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let contents = format!(
            "language={}\nmodel={}\nshortcut={}\ninput={}\ngain_db={}\noverlay_enabled={}\noverlay_size={}\noverlay_position={}\noverlay_show_text={}\noverlay_opacity={}\ncommand_mode_enabled={}\ncompute_backend={}\ntheme={}\naccent={}\nai_enabled={}\nai_provider={}\nai_model={}\nai_base_url={}\nai_prompt={}\nai_local_only={}\nauto_profiles_enabled={}\ntyping_wpm={}\nskip_weekends={}\naudio_history_enabled={}\naudio_history_budget_mb={}\nlocal_api_enabled={}\nlocal_api_port={}\nspeech_engine={}\nlocal_speech_url={}\ndiarization_enabled={}\nonboarding_completed={}\n",
            self.language,
            self.model.display(),
            self.shortcut,
            self.input,
            self.gain_db,
            self.overlay_enabled,
            self.overlay_size,
            self.overlay_position,
            self.overlay_show_text,
            self.overlay_opacity,
            self.command_mode_enabled,
            self.compute_backend,
            self.theme,
            self.accent,
            self.ai_enabled,
            self.ai_provider,
            escape_setting(&self.ai_model),
            escape_setting(&self.ai_base_url),
            escape_setting(&self.ai_prompt),
            self.ai_local_only,
            self.auto_profiles_enabled,
            self.typing_wpm,
            self.skip_weekends,
            self.audio_history_enabled,
            self.audio_history_budget_mb,
            self.local_api_enabled,
            self.local_api_port,
            self.speech_engine,
            escape_setting(&self.local_speech_url),
            self.diarization_enabled,
            self.onboarding_completed
        );
        atomic_write_private(&path, contents.as_bytes())
    }
}
