use super::{
    PathBuf, ProviderId, ai_profiles_path, atomic_write_private, fs, preferences_path,
    unescape_setting,
};
use serde::{Deserialize, Serialize};

const PREFERENCES_VERSION: u32 = 1;

// Persisted feature switches are independent user choices rather than one
// state machine, so explicit booleans make migrations and defaults auditable.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
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

impl WriteModeJob {
    const REWRITE_PROMPT: &'static str = "Rewrite the selected text according to the user's instruction. Preserve meaning unless the instruction asks otherwise. Output only the replacement text, with no explanation or markdown fences.";
    const DRAFT_PROMPT: &'static str = "Write the requested text. Follow the user's instruction precisely. Output only the finished text, with no explanation or markdown fences.";

    pub(super) fn rewrite(instruction: String, selected: String) -> Self {
        Self::Rewrite {
            instruction,
            selected,
        }
    }

    pub(super) fn draft(instruction: String) -> Self {
        Self::Draft { instruction }
    }

    pub(super) const fn prompt(&self) -> &'static str {
        match self {
            Self::Rewrite { .. } => Self::REWRITE_PROMPT,
            Self::Draft { .. } => Self::DRAFT_PROMPT,
        }
    }

    pub(super) fn input(&self) -> String {
        match self {
            Self::Rewrite {
                instruction,
                selected,
            } => format!("User instruction: {instruction}\n\nSelected text:\n{selected}"),
            Self::Draft { instruction } => instruction.clone(),
        }
    }

    pub(super) const fn paste_result(&self) -> bool {
        matches!(self, Self::Rewrite { .. })
    }

    pub(super) const fn retry_success_status(&self) -> &'static str {
        match self {
            Self::Rewrite { .. } => "Rewrite retry pasted or left on clipboard",
            Self::Draft { .. } => "Draft retry copied to the clipboard",
        }
    }
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
            ai_provider: ProviderId::Ollama.preference_index(),
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
        Self::parse(&contents)
    }

    fn parse(contents: &str) -> Self {
        if let Ok(document) = serde_json::from_str::<PreferencesDocument>(contents) {
            return document.preferences.sanitized();
        }
        Self::parse_legacy(contents).sanitized()
    }

    fn parse_legacy(contents: &str) -> Self {
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
                preferences.ai_provider = value
                    .parse()
                    .unwrap_or_else(|_| ProviderId::Ollama.preference_index());
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

    fn sanitized(mut self) -> Self {
        self.gain_db = self.gain_db.clamp(-24.0, 24.0);
        self.overlay_size = self.overlay_size.clamp(0, 2);
        self.overlay_position = self.overlay_position.clamp(0, 2);
        self.overlay_opacity = self.overlay_opacity.clamp(0.55, 1.0);
        self.typing_wpm = self.typing_wpm.clamp(10, 250);
        self.audio_history_budget_mb = self.audio_history_budget_mb.clamp(100, 10_000);
        self.local_api_port = self.local_api_port.clamp(1024, 65_535);
        self.speech_engine = self.speech_engine.clamp(0, 5);
        self
    }

    pub(super) fn save(&self) -> Result<(), String> {
        let path = preferences_path();
        let parent = path
            .parent()
            .ok_or_else(|| "preferences path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let contents = serde_json::to_vec_pretty(&PreferencesDocumentRef {
            version: PREFERENCES_VERSION,
            preferences: self,
        })
        .map_err(|error| error.to_string())?;
        atomic_write_private(&path, &contents)
    }
}

#[derive(Deserialize)]
struct PreferencesDocument {
    #[allow(dead_code)]
    version: u32,
    preferences: Preferences,
}

#[derive(Serialize)]
struct PreferencesDocumentRef<'a> {
    version: u32,
    preferences: &'a Preferences,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_legacy_preferences_without_losing_escaped_values() {
        let parsed = Preferences::parse(
            "language=sv\noverlay_opacity=0.8\nai_prompt=one\\ntwo\nonboarding_completed=false\n",
        );
        assert_eq!(parsed.language, "sv");
        assert_eq!(parsed.ai_prompt, "one\ntwo");
        assert!(!parsed.onboarding_completed);
    }

    #[test]
    fn versioned_preferences_round_trip() {
        let preferences = Preferences {
            language: "de".to_owned(),
            ai_prompt: "Preserve = and newlines\nexactly".to_owned(),
            ..Preferences::default()
        };
        let serialized = serde_json::to_string(&PreferencesDocumentRef {
            version: PREFERENCES_VERSION,
            preferences: &preferences,
        })
        .unwrap();
        let parsed = Preferences::parse(&serialized);
        assert_eq!(parsed.language, "de");
        assert_eq!(parsed.ai_prompt, preferences.ai_prompt);
    }
}
