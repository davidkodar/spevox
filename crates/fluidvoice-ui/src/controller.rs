#![allow(clippy::unnecessary_box_returns)]
#![allow(clippy::float_cmp)] // Generated Q_PROPERTY setter compares the value.

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, status_text, cxx_name = "statusText")]
        #[qproperty(QString, text_delivery_status, cxx_name = "textDeliveryStatus")]
        #[qproperty(QString, microphone_name, cxx_name = "microphoneName")]
        #[qproperty(QString, model_name, cxx_name = "modelName")]
        #[qproperty(bool, recording)]
        #[qproperty(bool, overlay_visible, cxx_name = "overlayVisible")]
        #[qproperty(bool, overlay_enabled, cxx_name = "overlayEnabled")]
        #[qproperty(QStringList, overlay_sizes, cxx_name = "overlaySizes")]
        #[qproperty(i32, selected_overlay_size, cxx_name = "selectedOverlaySize")]
        #[qproperty(QStringList, overlay_positions, cxx_name = "overlayPositions")]
        #[qproperty(i32, selected_overlay_position, cxx_name = "selectedOverlayPosition")]
        #[qproperty(bool, overlay_show_text, cxx_name = "overlayShowText")]
        #[qproperty(f32, overlay_opacity, cxx_name = "overlayOpacity")]
        #[qproperty(bool, overlay_result_available, cxx_name = "overlayResultAvailable")]
        #[qproperty(QString, last_raw_text, cxx_name = "lastRawText")]
        #[qproperty(f32, audio_level, cxx_name = "audioLevel")]
        #[qproperty(f32, input_db, cxx_name = "inputDb")]
        #[qproperty(i32, audio_updates, cxx_name = "audioUpdates")]
        #[qproperty(QStringList, input_sources, cxx_name = "inputSources")]
        #[qproperty(i32, selected_input, cxx_name = "selectedInput")]
        #[qproperty(f32, gain_db, cxx_name = "gainDb")]
        #[qproperty(bool, transcribing)]
        #[qproperty(QString, transcript_text, cxx_name = "transcriptText")]
        #[qproperty(QString, live_transcript, cxx_name = "liveTranscript")]
        #[qproperty(QStringList, languages)]
        #[qproperty(i32, selected_language, cxx_name = "selectedLanguage")]
        #[qproperty(QStringList, models)]
        #[qproperty(QStringList, model_states, cxx_name = "modelStates")]
        #[qproperty(QStringList, model_details, cxx_name = "modelDetails")]
        #[qproperty(i32, selected_model, cxx_name = "selectedModel")]
        #[qproperty(i32, downloading_model, cxx_name = "downloadingModel")]
        #[qproperty(f32, model_download_progress, cxx_name = "modelDownloadProgress")]
        #[qproperty(QStringList, shortcuts)]
        #[qproperty(i32, selected_shortcut, cxx_name = "selectedShortcut")]
        #[qproperty(QStringList, dictionary_terms, cxx_name = "dictionaryTerms")]
        #[qproperty(QStringList, history_entries, cxx_name = "historyEntries")]
        #[qproperty(bool, audio_history_enabled, cxx_name = "audioHistoryEnabled")]
        #[qproperty(i32, audio_history_budget_mb, cxx_name = "audioHistoryBudgetMb")]
        #[qproperty(QString, audio_history_status, cxx_name = "audioHistoryStatus")]
        #[qproperty(i32, transcript_count, cxx_name = "transcriptCount")]
        #[qproperty(i32, dictated_word_count, cxx_name = "dictatedWordCount")]
        #[qproperty(i32, typing_wpm, cxx_name = "typingWpm")]
        #[qproperty(bool, skip_weekends, cxx_name = "skipWeekends")]
        #[qproperty(bool, command_mode_enabled, cxx_name = "commandModeEnabled")]
        #[qproperty(QString, command_output, cxx_name = "commandOutput")]
        #[qproperty(QString, pending_command, cxx_name = "pendingCommand")]
        #[qproperty(QStringList, command_history, cxx_name = "commandHistory")]
        #[qproperty(
            QString,
            file_transcription_status,
            cxx_name = "fileTranscriptionStatus"
        )]
        #[qproperty(f32, meeting_progress, cxx_name = "meetingProgress")]
        #[qproperty(QStringList, meeting_segments, cxx_name = "meetingSegments")]
        #[qproperty(QStringList, compute_backends, cxx_name = "computeBackends")]
        #[qproperty(i32, selected_compute_backend, cxx_name = "selectedComputeBackend")]
        #[qproperty(QStringList, theme_options, cxx_name = "themeOptions")]
        #[qproperty(i32, selected_theme, cxx_name = "selectedTheme")]
        #[qproperty(QStringList, accent_options, cxx_name = "accentOptions")]
        #[qproperty(i32, selected_accent, cxx_name = "selectedAccent")]
        #[qproperty(bool, ai_enabled, cxx_name = "aiEnabled")]
        #[qproperty(QStringList, ai_providers, cxx_name = "aiProviders")]
        #[qproperty(i32, selected_ai_provider, cxx_name = "selectedAiProvider")]
        #[qproperty(QString, ai_model, cxx_name = "aiModel")]
        #[qproperty(QString, ai_base_url, cxx_name = "aiBaseUrl")]
        #[qproperty(QString, ai_prompt, cxx_name = "aiPrompt")]
        #[qproperty(QString, ai_status, cxx_name = "aiStatus")]
        #[qproperty(i32, write_mode_activation, cxx_name = "writeModeActivation")]
        #[qproperty(bool, ai_key_configured, cxx_name = "aiKeyConfigured")]
        #[qproperty(QStringList, ai_local_models, cxx_name = "aiLocalModels")]
        #[qproperty(bool, ai_local_endpoint, cxx_name = "aiLocalEndpoint")]
        #[qproperty(bool, ai_local_only, cxx_name = "aiLocalOnly")]
        #[qproperty(QString, ollama_status, cxx_name = "ollamaStatus")]
        #[qproperty(bool, ollama_installed, cxx_name = "ollamaInstalled")]
        #[qproperty(bool, ollama_busy, cxx_name = "ollamaBusy")]
        #[qproperty(QStringList, ai_profile_names, cxx_name = "aiProfileNames")]
        #[qproperty(i32, selected_ai_profile, cxx_name = "selectedAiProfile")]
        #[qproperty(QString, ai_profile_prompt, cxx_name = "aiProfilePrompt")]
        #[qproperty(QString, ai_profile_name, cxx_name = "aiProfileName")]
        #[qproperty(QString, ai_profile_match, cxx_name = "aiProfileMatch")]
        #[qproperty(bool, auto_profiles_enabled, cxx_name = "autoProfilesEnabled")]
        #[qproperty(QString, active_application, cxx_name = "activeApplication")]
        #[qproperty(QString, app_version, cxx_name = "appVersion")]
        #[qproperty(QString, update_status, cxx_name = "updateStatus")]
        type FluidVoiceController = super::FluidVoiceControllerRust;

        #[qinvokable]
        #[cxx_name = "toggleRecording"]
        fn toggle_recording(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "initializeAudio"]
        fn initialize_audio(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "initializeDesktopRuntime"]
        fn initialize_desktop_runtime(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "diagnoseTextDelivery"]
        fn diagnose_text_delivery(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectInput"]
        fn select_input(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "selectLanguage"]
        fn select_language(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "selectModel"]
        fn select_model(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "downloadModel"]
        fn download_model(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "cancelModelDownload"]
        fn cancel_model_download(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "deleteModel"]
        fn delete_model(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "selectShortcut"]
        fn select_shortcut(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "updateGainDb"]
        fn update_gain_db(self: Pin<&mut Self>, gain: f32);

        #[qinvokable]
        #[cxx_name = "updateOverlayEnabled"]
        fn update_overlay_enabled(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "updateOverlayPreferences"]
        fn update_overlay_preferences(
            self: Pin<&mut Self>,
            size: i32,
            position: i32,
            show_text: bool,
            opacity: f32,
        );

        #[qinvokable]
        #[cxx_name = "copyLastResult"]
        fn copy_last_result(self: Pin<&mut Self>, raw: bool);

        #[qinvokable]
        #[cxx_name = "undoLastAi"]
        fn undo_last_ai(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "retryLastAi"]
        fn retry_last_ai(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "dismissOverlay"]
        fn dismiss_overlay(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);

        #[qinvokable]
        #[cxx_name = "addDictionaryTerm"]
        fn add_dictionary_term(self: Pin<&mut Self>, term: &QString);

        #[qinvokable]
        #[cxx_name = "addDictionaryReplacement"]
        fn add_dictionary_replacement(self: Pin<&mut Self>, spoken: &QString, preferred: &QString);

        #[qinvokable]
        #[cxx_name = "importDictionary"]
        fn import_dictionary(self: Pin<&mut Self>, path: &QString, conflict_mode: i32);

        #[qinvokable]
        #[cxx_name = "exportDictionary"]
        fn export_dictionary(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "removeDictionaryTerm"]
        fn remove_dictionary_term(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "clearHistory"]
        fn clear_history(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "updateStatsPreferences"]
        fn update_stats_preferences(self: Pin<&mut Self>, typing_wpm: i32, skip_weekends: bool);

        #[qinvokable]
        #[cxx_name = "updateAudioHistory"]
        fn update_audio_history(self: Pin<&mut Self>, enabled: bool, budget_mb: i32);

        #[qinvokable]
        #[cxx_name = "deleteHistoryAudio"]
        fn delete_history_audio(self: Pin<&mut Self>, entry: &QString);

        #[qinvokable]
        #[cxx_name = "exportAudioHistory"]
        fn export_audio_history(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "exportHistory"]
        fn export_history(self: Pin<&mut Self>, path: &QString, format: &QString);

        #[qinvokable]
        #[cxx_name = "copyHistoryText"]
        fn copy_history_text(self: Pin<&mut Self>, entry: &QString, mode: i32);

        #[qinvokable]
        #[cxx_name = "updateCommandModeEnabled"]
        fn update_command_mode_enabled(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "submitCommand"]
        fn submit_command(self: Pin<&mut Self>, command: &QString);

        #[qinvokable]
        #[cxx_name = "approvePendingCommand"]
        fn approve_pending_command(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "cancelPendingCommand"]
        fn cancel_pending_command(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "transcribeFile"]
        fn transcribe_file(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "cancelMeetingTranscription"]
        fn cancel_meeting_transcription(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "exportMeeting"]
        fn export_meeting(self: Pin<&mut Self>, path: &QString, format: &QString);

        #[qinvokable]
        #[cxx_name = "selectComputeBackend"]
        fn select_compute_backend(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "selectTheme"]
        fn select_theme(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "selectAccent"]
        fn select_accent(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "updateAiEnabled"]
        fn update_ai_enabled(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "selectAiProvider"]
        fn select_ai_provider(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "updateAiModel"]
        fn update_ai_model(self: Pin<&mut Self>, value: &QString);

        #[qinvokable]
        #[cxx_name = "updateAiBaseUrl"]
        fn update_ai_base_url(self: Pin<&mut Self>, value: &QString);

        #[qinvokable]
        #[cxx_name = "updateAiPrompt"]
        fn update_ai_prompt(self: Pin<&mut Self>, value: &QString);

        #[qinvokable]
        #[cxx_name = "saveAiApiKey"]
        fn save_ai_api_key(self: Pin<&mut Self>, value: &QString);

        #[qinvokable]
        #[cxx_name = "testAiProvider"]
        fn test_ai_provider(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "discoverLocalAiModels"]
        fn discover_local_ai_models(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectLocalAiModel"]
        fn select_local_ai_model(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "diagnoseOllama"]
        fn diagnose_ollama(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "startOllama"]
        fn start_ollama(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "pullOllamaModel"]
        fn pull_ollama_model(self: Pin<&mut Self>, model: &QString);

        #[qinvokable]
        #[cxx_name = "updateAiLocalOnly"]
        fn update_ai_local_only(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "selectAiProfile"]
        fn select_ai_profile(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "saveAiProfile"]
        fn save_ai_profile(
            self: Pin<&mut Self>,
            name: &QString,
            prompt: &QString,
            application_match: &QString,
        );

        #[qinvokable]
        #[cxx_name = "updateAutoProfilesEnabled"]
        fn update_auto_profiles_enabled(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "deleteAiProfile"]
        fn delete_ai_profile(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "rewriteSelectedText"]
        fn rewrite_selected_text(self: Pin<&mut Self>, instruction: &QString);

        #[qinvokable]
        #[cxx_name = "writeFromInstruction"]
        fn write_from_instruction(self: Pin<&mut Self>, instruction: &QString);

        #[qinvokable]
        #[cxx_name = "retryWriteMode"]
        fn retry_write_mode(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "checkForUpdates"]
        fn check_for_updates(self: Pin<&mut Self>);
    }

    impl cxx_qt::Threading for FluidVoiceController {}
}

use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    pin::Pin,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QString, QStringList};
use fluidvoice_audio::{AudioBuffer, AudioDevice, CaptureStopToken, PipeWireCapture};
use fluidvoice_delivery::ClipboardDelivery;
use fluidvoice_portal::{
    ActiveApplication, GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutEvent,
    TextInputSession, run_profile_bridge,
};
use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};
use tokio::sync::mpsc;

use crate::ai::{self, AiConfig};

pub struct FluidVoiceControllerRust {
    status_text: QString,
    text_delivery_status: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
    overlay_enabled: bool,
    overlay_sizes: QStringList,
    selected_overlay_size: i32,
    overlay_positions: QStringList,
    selected_overlay_position: i32,
    overlay_show_text: bool,
    overlay_opacity: f32,
    overlay_result_available: bool,
    last_raw_text: QString,
    audio_level: f32,
    input_db: f32,
    audio_updates: i32,
    input_sources: QStringList,
    selected_input: i32,
    gain_db: f32,
    transcribing: bool,
    transcript_text: QString,
    live_transcript: QString,
    stop_token: Option<CaptureStopToken>,
    capture_target: Option<String>,
    devices: Vec<AudioDevice>,
    clipboard: Option<ClipboardDelivery>,
    desktop_sender: Option<mpsc::UnboundedSender<DesktopCommand>>,
    languages: QStringList,
    selected_language: i32,
    language_codes: Vec<String>,
    models: QStringList,
    model_states: QStringList,
    model_details: QStringList,
    selected_model: i32,
    model_paths: Vec<PathBuf>,
    downloading_model: i32,
    model_download_progress: f32,
    model_download_cancel: Option<Arc<AtomicBool>>,
    shortcuts: QStringList,
    selected_shortcut: i32,
    dictionary_terms: QStringList,
    history_entries: QStringList,
    audio_history_enabled: bool,
    audio_history_budget_mb: i32,
    audio_history_status: QString,
    transcript_count: i32,
    dictated_word_count: i32,
    typing_wpm: i32,
    skip_weekends: bool,
    command_mode_enabled: bool,
    command_output: QString,
    pending_command: QString,
    pending_desktop_action: Option<DesktopAction>,
    command_history: QStringList,
    file_transcription_status: QString,
    meeting_progress: f32,
    meeting_segments: QStringList,
    meeting_results: Vec<MeetingSegment>,
    meeting_cancel: Option<Arc<AtomicBool>>,
    compute_backends: QStringList,
    selected_compute_backend: i32,
    theme_options: QStringList,
    selected_theme: i32,
    accent_options: QStringList,
    selected_accent: i32,
    ai_enabled: bool,
    ai_providers: QStringList,
    selected_ai_provider: i32,
    ai_model: QString,
    ai_base_url: QString,
    ai_prompt: QString,
    ai_status: QString,
    write_mode_activation: i32,
    ai_key_configured: bool,
    ai_local_models: QStringList,
    ai_local_endpoint: bool,
    ai_local_only: bool,
    ollama_status: QString,
    ollama_installed: bool,
    ollama_busy: bool,
    ai_profile_names: QStringList,
    selected_ai_profile: i32,
    ai_profile_prompt: QString,
    ai_profile_name: QString,
    ai_profile_match: QString,
    auto_profiles_enabled: bool,
    active_application: QString,
    ai_profiles: Vec<AiProfile>,
    last_write_instruction: String,
    app_version: QString,
    update_status: QString,
}

impl Default for FluidVoiceControllerRust {
    fn default() -> Self {
        let preferences = Preferences::load();
        let language_codes = supported_languages()
            .iter()
            .map(|(_, code)| (*code).to_owned())
            .collect::<Vec<_>>();
        let languages = supported_languages()
            .iter()
            .map(|(name, _)| QString::from(*name))
            .collect::<QStringList>();
        let selected_language = language_codes
            .iter()
            .position(|code| code == &preferences.language)
            .and_then(|index| i32::try_from(index).ok())
            .unwrap_or(0);
        let model_paths = whisper_model_catalog()
            .iter()
            .map(resolve_model_path)
            .collect::<Vec<_>>();
        let models = whisper_model_catalog()
            .iter()
            .map(|model| QString::from(model.display_name))
            .collect::<QStringList>();
        let selected_model = model_paths
            .iter()
            .position(|path| {
                path == &preferences.model
                    || path.file_name().is_some_and(|name| {
                        preferences
                            .model
                            .file_name()
                            .is_some_and(|saved| saved == name)
                    })
            })
            .and_then(|index| i32::try_from(index).ok())
            .unwrap_or(0);
        let selected_shortcut = shortcut_triggers()
            .iter()
            .position(|(_, trigger)| *trigger == preferences.shortcut)
            .and_then(|index| i32::try_from(index).ok())
            .unwrap_or(0);
        let model_name = model_paths
            .get(usize::try_from(selected_model).unwrap_or_default())
            .map_or_else(
                || QString::from("No Whisper model installed"),
                |_| {
                    QString::from(
                        whisper_model_catalog()
                            [usize::try_from(selected_model).unwrap_or_default()]
                        .display_name,
                    )
                },
            );
        let (model_states, model_details) = model_ui_lists(&model_paths);
        let dictionary = load_lines(&dictionary_path());
        clear_missing_audio_history_references().ok();
        let history = load_lines(&history_path());
        let selected_ai_provider = if preferences.ai_local_only {
            let saved = preferences.ai_provider.clamp(0, 9);
            if ai_provider(saved).local { saved } else { 7 }
        } else {
            preferences.ai_provider.clamp(0, 9)
        };
        let provider = ai_provider(selected_ai_provider);
        let ai_model = if preferences.ai_model.is_empty() {
            provider.default_model.to_owned()
        } else {
            preferences.ai_model.clone()
        };
        let mut ai_base_url = if preferences.ai_base_url.is_empty() {
            provider.default_url.to_owned()
        } else {
            preferences.ai_base_url.clone()
        };
        if provider.local
            && !(AiConfig {
                enabled: false,
                provider: provider.id.to_owned(),
                model: String::new(),
                base_url: ai_base_url.clone(),
                prompt: String::new(),
                api_key: String::new(),
                local_only: false,
                timeout_seconds: 45,
            })
            .is_local()
        {
            ai_base_url = provider.default_url.to_owned();
        }
        let provider_key = provider.id;
        let ai_key_configured = provider.local || !ai::load_api_key(provider_key).is_empty();
        let dictated_word_count = history
            .iter()
            .map(|entry| history_field(entry, 1).unwrap_or(entry))
            .map(|text| text.split_whitespace().count())
            .sum::<usize>();
        let ai_profiles = load_ai_profiles();
        Self {
            status_text: QString::from("Ready"),
            text_delivery_status: QString::from(
                "Clipboard recovery is ready. Portal paste has not been tested yet.",
            ),
            microphone_name: QString::from("Detecting PipeWire inputs…"),
            model_name,
            recording: false,
            overlay_visible: false,
            overlay_enabled: preferences.overlay_enabled,
            overlay_sizes: ["Compact", "Standard", "Expanded"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_overlay_size: preferences.overlay_size.clamp(0, 2),
            overlay_positions: ["Top center", "Bottom center", "Screen center"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_overlay_position: preferences.overlay_position.clamp(0, 2),
            overlay_show_text: preferences.overlay_show_text,
            overlay_opacity: preferences.overlay_opacity.clamp(0.55, 1.0),
            overlay_result_available: false,
            last_raw_text: QString::default(),
            audio_level: 0.0,
            input_db: -60.0,
            audio_updates: 0,
            input_sources: QStringList::default(),
            selected_input: -1,
            gain_db: preferences.gain_db,
            transcribing: false,
            transcript_text: QString::default(),
            live_transcript: QString::default(),
            stop_token: None,
            capture_target: None,
            devices: Vec::new(),
            clipboard: None,
            desktop_sender: None,
            languages,
            selected_language,
            language_codes,
            models,
            model_states,
            model_details,
            selected_model,
            model_paths,
            downloading_model: -1,
            model_download_progress: 0.0,
            model_download_cancel: None,
            shortcuts: shortcut_triggers()
                .iter()
                .map(|(label, _)| QString::from(*label))
                .collect(),
            selected_shortcut,
            dictionary_terms: dictionary
                .iter()
                .map(|line| QString::from(&dictionary_display(line)))
                .collect(),
            history_entries: history.iter().rev().map(QString::from).collect(),
            audio_history_enabled: preferences.audio_history_enabled,
            audio_history_budget_mb: preferences.audio_history_budget_mb,
            audio_history_status: QString::from(audio_history_summary()),
            transcript_count: i32::try_from(history.len()).unwrap_or(i32::MAX),
            dictated_word_count: i32::try_from(dictated_word_count).unwrap_or(i32::MAX),
            typing_wpm: preferences.typing_wpm,
            skip_weekends: preferences.skip_weekends,
            command_mode_enabled: preferences.command_mode_enabled,
            command_output: QString::from(
                "Ask a question or request an allowlisted desktop action.",
            ),
            pending_command: QString::default(),
            pending_desktop_action: None,
            command_history: load_lines(&command_history_path())
                .iter()
                .rev()
                .map(QString::from)
                .collect(),
            file_transcription_status: QString::from("Choose a WAV file to transcribe locally."),
            meeting_progress: 0.0,
            meeting_segments: QStringList::default(),
            meeting_results: Vec::new(),
            meeting_cancel: None,
            compute_backends: ["Automatic (Vulkan)", "GPU preferred (Vulkan)", "CPU"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_compute_backend: preferences.compute_backend.clamp(0, 2),
            theme_options: ["System", "FluidVoice Dark", "FluidVoice Light"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_theme: preferences.theme.clamp(0, 2),
            accent_options: ["KDE system accent", "FluidVoice Cyan", "Green", "Purple"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_accent: preferences.accent.clamp(0, 3),
            ai_enabled: preferences.ai_enabled,
            ai_providers: ai_provider_catalog()
                .iter()
                .map(|provider| QString::from(provider.name))
                .collect(),
            selected_ai_provider,
            ai_model: QString::from(&ai_model),
            ai_base_url: QString::from(&ai_base_url),
            ai_prompt: QString::from(if preferences.ai_prompt.is_empty() {
                ai::DEFAULT_PROMPT
            } else {
                &preferences.ai_prompt
            }),
            ai_status: QString::from(if preferences.ai_enabled {
                "Enhancement enabled · provider not yet verified"
            } else {
                "Off · raw transcription stays fully local"
            }),
            write_mode_activation: 0,
            ai_key_configured,
            ai_local_models: QStringList::default(),
            ai_local_endpoint: AiConfig {
                enabled: false,
                provider: provider.id.to_owned(),
                model: ai_model.clone(),
                base_url: ai_base_url.clone(),
                prompt: String::new(),
                api_key: String::new(),
                local_only: false,
                timeout_seconds: 45,
            }
            .is_local(),
            ai_local_only: preferences.ai_local_only,
            ollama_status: QString::from("Run the setup check to inspect Ollama."),
            ollama_installed: false,
            ollama_busy: false,
            ai_profile_names: std::iter::once(QString::from("Default"))
                .chain(
                    ai_profiles
                        .iter()
                        .map(|profile| QString::from(&profile.name)),
                )
                .collect(),
            selected_ai_profile: 0,
            ai_profile_prompt: QString::default(),
            ai_profile_name: QString::default(),
            ai_profile_match: QString::default(),
            auto_profiles_enabled: preferences.auto_profiles_enabled,
            active_application: QString::from("KWin bridge has not reported an application."),
            ai_profiles,
            last_write_instruction: String::new(),
            app_version: QString::from(env!("CARGO_PKG_VERSION")),
            update_status: QString::from("Updates have not been checked."),
        }
    }
}

impl ffi::FluidVoiceController {
    pub fn initialize_desktop_runtime(mut self: Pin<&mut Self>) {
        if self.as_ref().rust().desktop_sender.is_some() {
            return;
        }
        let (desktop_sender, mut desktop_receiver) = mpsc::unbounded_channel();
        self.as_mut().rust_mut().get_mut().desktop_sender = Some(desktop_sender);
        let qt_thread = self.qt_thread();
        let shortcut = selected_shortcut_trigger(self.as_ref().rust());
        std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    eprintln!("Desktop runtime failed: {error}");
                    return;
                }
            };
            runtime.block_on(async move {
                let mut text_input = None;
                let mut requested_shortcut = shortcut;
                let (profile_sender, mut profile_events) = mpsc::channel(16);
                tokio::spawn(async move {
                    if let Err(error) = run_profile_bridge(profile_sender).await {
                        eprintln!("Application profile bridge stopped: {error}");
                    }
                });
                loop {
                    let config = match GlobalShortcutConfig::new(
                        "dictate_hold",
                        "Hold to dictate with FluidVoice Linux",
                        Some(requested_shortcut.clone()),
                    ) {
                        Ok(config) => config,
                        Err(error) => {
                            eprintln!("Shortcut configuration failed: {error}");
                            break;
                        }
                    };
                    let write_config = match GlobalShortcutConfig::new(
                        "write_mode",
                        "Open FluidVoice Write Mode",
                        Some("CTRL+ALT+W"),
                    ) {
                        Ok(config) => config,
                        Err(error) => {
                            eprintln!("Write Mode shortcut configuration failed: {error}");
                            break;
                        }
                    };
                    let binding = match GlobalShortcutBinding::bind_many(&[config, write_config]).await {
                        Ok(binding) => binding,
                        Err(error) => {
                            eprintln!("Global shortcut unavailable: {error}");
                            qt_thread
                                .queue(move |controller| {
                                    controller.set_status_text(QString::from(&format!(
                                        "Global shortcut unavailable: {error}"
                                    )))
                                })
                                .ok();
                            break;
                        }
                    };
                    let (event_sender, mut events) = mpsc::channel(16);
                    let event_task = tokio::spawn(async move {
                        if let Err(error) = binding.forward_events(event_sender).await {
                            eprintln!("Global shortcut stopped: {error}");
                        }
                    });
                    let shortcut_label = requested_shortcut.replace('+', "+");
                    let ready_status = format!("Ready · hold {shortcut_label} to dictate");
                    qt_thread
                        .queue(move |controller| {
                            controller.set_status_text(QString::from(&ready_status))
                        })
                        .ok();

                    let mut rebind = None;
                    loop {
                        tokio::select! {
                            event = events.recv() => match event {
                                Some(GlobalShortcutEvent::Activated { id, .. }) if id == "dictate_hold" => {
                                    qt_thread.queue(|mut controller| {
                                        if !*controller.as_ref().recording()
                                            && !*controller.as_ref().transcribing()
                                        {
                                            controller.as_mut().toggle_recording();
                                        }
                                    }).ok();
                                }
                                Some(GlobalShortcutEvent::Deactivated { id, .. }) if id == "dictate_hold" => {
                                    qt_thread.queue(|mut controller| {
                                        if *controller.as_ref().recording() {
                                            controller.as_mut().toggle_recording();
                                        }
                                    }).ok();
                                }
                                Some(GlobalShortcutEvent::Activated { id, .. }) if id == "write_mode" => {
                                    qt_thread.queue(|mut controller| {
                                        let next = controller.as_ref().write_mode_activation().wrapping_add(1);
                                        controller.as_mut().set_write_mode_activation(next);
                                    }).ok();
                                }
                                Some(_) => {}
                                None => break,
                            },
                            request = desktop_receiver.recv() => match request {
                                Some(DesktopCommand::Paste) => {
                                    let mut outcome = Err("Plasma keyboard permission was not granted".to_owned());
                                    for _ in 0..2 {
                                        if text_input.is_none() {
                                            match TextInputSession::request().await {
                                                Ok(session) => text_input = Some(session),
                                                Err(error) => {
                                                    outcome = Err(error.to_string());
                                                    break;
                                                }
                                            }
                                        }
                                        if let Some(session) = text_input.as_ref() {
                                            match session.paste_clipboard().await {
                                                Ok(()) => {
                                                    outcome = Ok(());
                                                    break;
                                                }
                                                Err(error) => {
                                                    outcome = Err(error.to_string());
                                                    text_input = None;
                                                }
                                            }
                                        }
                                    }
                                    let message = match outcome {
                                        Ok(()) => "Direct paste verified through the Plasma Wayland portal.".to_owned(),
                                        Err(error) => format!("Direct paste failed; the transcript remains on the clipboard. Plasma portal: {error}"),
                                    };
                                    qt_thread.queue(move |mut controller| {
                                        controller.as_mut().set_text_delivery_status(QString::from(&message));
                                    }).ok();
                                }
                                Some(DesktopCommand::CopySelection(reply)) => {
                                    if text_input.is_none() {
                                        text_input = TextInputSession::request().await.ok();
                                    }
                                    let result = if let Some(session) = text_input.as_ref() {
                                        session.copy_selection().await.map_err(|error| error.to_string())
                                    } else {
                                        Err("Wayland keyboard permission is unavailable".to_owned())
                                    };
                                    reply.send(result).ok();
                                }
                                Some(DesktopCommand::Rebind(shortcut)) => {
                                    rebind = Some(shortcut);
                                    break;
                                }
                                Some(DesktopCommand::DiagnoseTextInput(reply)) => {
                                    let portal = match TextInputSession::request().await {
                                        Ok(session) => {
                                            text_input = Some(session);
                                            Ok(())
                                        }
                                        Err(error) => Err(error.to_string()),
                                    };
                                    reply.send(portal).ok();
                                }
                                None => break,
                            },
                            application = profile_events.recv() => match application {
                                Some(application) => {
                                    qt_thread.queue(move |mut controller| {
                                        controller.as_mut().apply_active_application(application);
                                    }).ok();
                                }
                                None => {}
                            }
                        }
                    }
                    event_task.abort();
                    let Some(shortcut) = rebind else { break };
                    requested_shortcut = shortcut;
                }
            });
        });
    }

    pub fn diagnose_text_delivery(mut self: Pin<&mut Self>) {
        let clipboard_ready = ClipboardDelivery::connect().is_ok();
        let Some(sender) = self.as_ref().rust().desktop_sender.clone() else {
            self.as_mut().set_text_delivery_status(QString::from(
                "Desktop integration is not running. Restart FluidVoice and try again.",
            ));
            return;
        };
        self.as_mut().set_text_delivery_status(QString::from(
            "Checking Plasma keyboard permission without typing…",
        ));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let (reply, result) = std::sync::mpsc::channel();
            let portal = if sender
                .send(DesktopCommand::DiagnoseTextInput(reply))
                .is_err()
            {
                Err("desktop integration stopped unexpectedly".to_owned())
            } else {
                result
                    .recv_timeout(Duration::from_secs(30))
                    .map_err(|_| "timed out waiting for the Plasma portal".to_owned())
                    .and_then(|result| result)
            };
            let message = match (clipboard_ready, portal) {
                (true, Ok(())) => "Clipboard and Plasma keyboard portal are ready. The next dictation should paste directly.".to_owned(),
                (true, Err(error)) => format!("Clipboard recovery is ready, but direct paste permission is unavailable: {error}"),
                (false, Ok(())) => "Plasma keyboard permission is ready, but the clipboard service is unavailable. Ensure a Plasma clipboard manager is running.".to_owned(),
                (false, Err(error)) => format!("Clipboard and direct paste are unavailable. Plasma portal: {error}"),
            };
            qt_thread
                .queue(move |mut controller| {
                    controller
                        .as_mut()
                        .set_text_delivery_status(QString::from(&message));
                })
                .ok();
        });
    }

    pub fn initialize_audio(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result = PipeWireCapture::devices();
            qt_thread
                .queue(move |mut controller| match result {
                    Ok(devices) => {
                        let saved_input = Preferences::load().input;
                        let preferred = devices
                            .iter()
                            .find(|device| device.node_name == saved_input)
                            .or_else(|| {
                                devices.iter().find(|device| {
                                    device.description.contains("Input 1")
                                        || device.description.contains("Mic 1")
                                })
                            });
                        let selected = preferred.or_else(|| devices.first()).cloned();
                        if let Some(device) = selected {
                            let selected_index = devices
                                .iter()
                                .position(|candidate| candidate.node_name == device.node_name)
                                .and_then(|index| i32::try_from(index).ok())
                                .unwrap_or(0);
                            let names = devices
                                .iter()
                                .map(|device| QString::from(&device.description))
                                .collect::<QStringList>();
                            let rust = controller.as_mut().rust_mut().get_mut();
                            rust.capture_target = Some(device.node_name.clone());
                            rust.devices = devices;
                            controller.as_mut().set_input_sources(names);
                            controller.as_mut().set_selected_input(selected_index);
                            controller.set_microphone_name(QString::from(&device.description));
                        } else {
                            controller
                                .as_mut()
                                .set_microphone_name(QString::from("No PipeWire input found"));
                            controller.set_status_text(QString::from("No microphone available"));
                        }
                    }
                    Err(error) => {
                        controller
                            .as_mut()
                            .set_microphone_name(QString::from("PipeWire unavailable"));
                        controller.set_status_text(QString::from(&format!(
                            "Input detection failed: {error}"
                        )));
                    }
                })
                .ok();
        });
    }

    pub fn select_input(mut self: Pin<&mut Self>, index: i32) {
        if *self.as_ref().recording() {
            return;
        }
        if *self.as_ref().transcribing() {
            return;
        }
        let Ok(index_usize) = usize::try_from(index) else {
            return;
        };
        let Some(device) = self.as_ref().rust().devices.get(index_usize).cloned() else {
            return;
        };
        self.as_mut().rust_mut().get_mut().capture_target = Some(device.node_name);
        self.as_mut().set_selected_input(index);
        self.as_mut()
            .set_microphone_name(QString::from(&device.description));
        self.as_ref().rust().save_preferences();
        self.set_status_text(QString::from("Input selected · ready to test"));
    }

    pub fn select_language(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, self.as_ref().rust().language_codes.len()) {
            return;
        }
        self.as_mut().set_selected_language(index);
        self.as_ref().rust().save_preferences();
        self.set_status_text(QString::from("Language updated"));
    }

    pub fn select_model(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, self.as_ref().rust().model_paths.len()) {
            return;
        }
        let index_usize = usize::try_from(index).unwrap_or_default();
        if !model_file_valid(
            &self.as_ref().rust().model_paths[index_usize],
            &whisper_model_catalog()[index_usize],
        ) {
            self.set_status_text(QString::from("Model not downloaded · choose Download"));
            return;
        }
        self.as_mut().set_selected_model(index);
        self.as_mut().set_model_name(QString::from(
            whisper_model_catalog()[index_usize].display_name,
        ));
        self.as_ref().rust().save_preferences();
        self.set_status_text(QString::from("Speech model activated"));
    }

    pub fn download_model(mut self: Pin<&mut Self>, index: i32) {
        if *self.as_ref().downloading_model() >= 0
            || !valid_index(index, whisper_model_catalog().len())
        {
            return;
        }
        let index_usize = usize::try_from(index).unwrap_or_default();
        let model = whisper_model_catalog()[index_usize];
        let destination = managed_model_directory().join(model.file_name);
        if model_file_valid(&destination, &model) {
            self.as_mut().refresh_model_catalog();
            self.set_status_text(QString::from("Model is already downloaded"));
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        self.as_mut().rust_mut().get_mut().model_download_cancel = Some(Arc::clone(&cancel));
        self.as_mut().set_downloading_model(index);
        self.as_mut().set_model_download_progress(0.0);
        self.as_mut().set_status_text(QString::from(&format!(
            "Downloading {}…",
            model.display_name
        )));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let progress_thread = qt_thread.clone();
            let result = download_whisper_model(model, &destination, &cancel, move |progress| {
                progress_thread
                    .queue(move |mut controller| {
                        controller.as_mut().set_model_download_progress(progress);
                    })
                    .ok();
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_downloading_model(-1);
                    controller.as_mut().set_model_download_progress(0.0);
                    controller
                        .as_mut()
                        .rust_mut()
                        .get_mut()
                        .model_download_cancel = None;
                    controller.as_mut().refresh_model_catalog();
                    controller.set_status_text(QString::from(match result {
                        Ok(()) => format!("{} downloaded · ready to activate", model.display_name),
                        Err(error) if error == "cancelled" => {
                            format!("{} download cancelled", model.display_name)
                        }
                        Err(error) => format!("Model download failed: {error}"),
                    }));
                })
                .ok();
        });
    }

    pub fn cancel_model_download(self: Pin<&mut Self>) {
        if let Some(cancel) = self.as_ref().rust().model_download_cancel.as_ref() {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    pub fn delete_model(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, whisper_model_catalog().len()) {
            return;
        }
        if index == *self.as_ref().selected_model() {
            self.set_status_text(QString::from(
                "Activate another model before deleting this one",
            ));
            return;
        }
        let model = whisper_model_catalog()[usize::try_from(index).unwrap_or_default()];
        let path = managed_model_directory().join(model.file_name);
        if path.is_file() {
            match fs::remove_file(path) {
                Ok(()) => self
                    .as_mut()
                    .set_status_text(QString::from("Downloaded model deleted")),
                Err(error) => self
                    .as_mut()
                    .set_status_text(QString::from(&format!("Could not delete model: {error}"))),
            }
        }
        self.as_mut().refresh_model_catalog();
    }

    pub fn select_shortcut(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, shortcut_triggers().len()) {
            return;
        }
        self.as_mut().set_selected_shortcut(index);
        self.as_ref().rust().save_preferences();
        if let Some(sender) = self.as_ref().rust().desktop_sender.as_ref() {
            sender
                .send(DesktopCommand::Rebind(selected_shortcut_trigger(
                    self.as_ref().rust(),
                )))
                .ok();
        }
        self.set_status_text(QString::from("Rebinding global shortcut…"));
    }

    pub fn update_gain_db(mut self: Pin<&mut Self>, gain: f32) {
        self.as_mut().set_gain_db(gain.clamp(-24.0, 24.0));
        self.as_ref().rust().save_preferences();
    }

    pub fn update_overlay_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_overlay_enabled(enabled);
        self.as_ref().rust().save_preferences();
    }

    pub fn update_overlay_preferences(
        mut self: Pin<&mut Self>,
        size: i32,
        position: i32,
        show_text: bool,
        opacity: f32,
    ) {
        self.as_mut().set_selected_overlay_size(size.clamp(0, 2));
        self.as_mut()
            .set_selected_overlay_position(position.clamp(0, 2));
        self.as_mut().set_overlay_show_text(show_text);
        self.as_mut().set_overlay_opacity(opacity.clamp(0.55, 1.0));
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_status_text(QString::from("Overlay appearance updated"));
    }

    pub fn copy_last_result(mut self: Pin<&mut Self>, raw: bool) {
        let text = if raw {
            self.as_ref().last_raw_text().to_string()
        } else {
            self.as_ref().transcript_text().to_string()
        };
        if text.trim().is_empty() {
            return;
        }
        let rust = self.as_mut().rust_mut().get_mut();
        if rust.clipboard.is_none() {
            rust.clipboard = ClipboardDelivery::connect().ok();
        }
        let copied = rust
            .clipboard
            .as_mut()
            .is_some_and(|clipboard| clipboard.copy_transcript(&text).is_ok());
        self.as_mut().set_status_text(QString::from(if copied {
            if raw {
                "Raw transcript copied"
            } else {
                "Final transcript copied"
            }
        } else {
            "Clipboard delivery failed"
        }));
    }

    pub fn undo_last_ai(mut self: Pin<&mut Self>) {
        let raw = self.as_ref().last_raw_text().to_string();
        if raw.trim().is_empty() {
            return;
        }
        let rust = self.as_mut().rust_mut().get_mut();
        if rust.clipboard.is_none() {
            rust.clipboard = ClipboardDelivery::connect().ok();
        }
        let copied = rust
            .clipboard
            .as_mut()
            .is_some_and(|clipboard| clipboard.copy_transcript(&raw).is_ok());
        if copied {
            if let Some(sender) = self.as_ref().rust().desktop_sender.as_ref() {
                sender.send(DesktopCommand::Paste).ok();
            }
            self.as_mut().set_transcript_text(QString::from(&raw));
            self.as_mut().set_live_transcript(QString::from(&raw));
            self.as_mut().set_status_text(QString::from(
                "AI cleanup undone · raw text pasted or copied",
            ));
        } else {
            self.as_mut()
                .set_status_text(QString::from("Could not restore raw text to clipboard"));
        }
    }

    pub fn retry_last_ai(mut self: Pin<&mut Self>) {
        if *self.as_ref().transcribing() {
            return;
        }
        let raw = self.as_ref().last_raw_text().to_string();
        if raw.trim().is_empty() {
            self.as_mut()
                .set_status_text(QString::from("No raw transcript is available to retry"));
            return;
        }
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut().set_overlay_visible(true);
        self.as_mut().set_overlay_result_available(false);
        self.as_mut()
            .set_status_text(QString::from("Retrying AI enhancement…"));
        std::thread::spawn(move || {
            let stream_thread = qt_thread.clone();
            let result = ai::enhance_streaming(&config, &raw, move |text| {
                let text = text.to_owned();
                stream_thread
                    .queue(move |mut controller| {
                        controller
                            .as_mut()
                            .set_live_transcript(QString::from(&text));
                    })
                    .ok();
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().set_overlay_result_available(true);
                    match result {
                        Ok(text) => {
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let copied = rust
                                .clipboard
                                .as_mut()
                                .is_some_and(|clipboard| clipboard.copy_transcript(&text).is_ok());
                            if copied {
                                if let Some(sender) =
                                    controller.as_ref().rust().desktop_sender.as_ref()
                                {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                            }
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&text));
                            controller
                                .as_mut()
                                .set_live_transcript(QString::from(&text));
                            controller.set_status_text(QString::from(if copied {
                                "AI enhancement retried · result pasted or copied"
                            } else {
                                "AI retry succeeded · clipboard delivery failed"
                            }));
                        }
                        Err(error) => controller.set_status_text(QString::from(&format!(
                            "AI retry failed · raw text remains available · {error}"
                        ))),
                    }
                })
                .ok();
        });
    }

    pub fn dismiss_overlay(mut self: Pin<&mut Self>) {
        self.as_mut().set_overlay_visible(false);
        self.as_mut().set_overlay_result_available(false);
    }

    pub fn update_command_mode_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_command_mode_enabled(enabled);
        self.as_ref().rust().save_preferences();
    }

    pub fn submit_command(mut self: Pin<&mut Self>, command: &QString) {
        let command = command.to_string().trim().to_owned();
        if command.is_empty() || *self.as_ref().transcribing() {
            return;
        }
        append_command_history(self.as_mut(), "user", &command);
        if let Some(action) = parse_desktop_action(&command) {
            let description = action.description();
            self.as_mut().rust_mut().get_mut().pending_desktop_action = Some(action);
            self.as_mut()
                .set_pending_command(QString::from(description));
            self.as_mut().set_command_output(QString::from(&format!(
                "Confirmation required: {description}"
            )));
            return;
        }
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        config.prompt = "You are FluidVoice Command Mode, a concise KDE Plasma assistant. Answer the user's question or explain how to perform the requested task. Do not claim to have executed anything. Never output shell commands unless explicitly asked, and clearly label them as suggestions.".to_owned();
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_command_output(QString::from("Command Mode is thinking…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, &command);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    match result {
                        Ok(output) => {
                            append_command_history(controller.as_mut(), "assistant", &output);
                            controller
                                .as_mut()
                                .set_command_output(QString::from(&output));
                        }
                        Err(error) => {
                            controller
                                .as_mut()
                                .set_command_output(QString::from(&format!(
                                    "Command Mode failed: {error}"
                                )))
                        }
                    }
                })
                .ok();
        });
    }

    pub fn approve_pending_command(mut self: Pin<&mut Self>) {
        let Some(action) = self
            .as_mut()
            .rust_mut()
            .get_mut()
            .pending_desktop_action
            .take()
        else {
            return;
        };
        self.as_mut().set_pending_command(QString::default());
        let result = action.execute();
        let message = result.map_or_else(
            |error| format!("Desktop action failed: {error}"),
            |()| "Desktop action started".to_owned(),
        );
        append_command_history(self.as_mut(), "system", &message);
        self.as_mut().set_command_output(QString::from(&message));
    }

    pub fn cancel_pending_command(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().get_mut().pending_desktop_action = None;
        self.as_mut().set_pending_command(QString::default());
        self.as_mut()
            .set_command_output(QString::from("Desktop action cancelled"));
    }

    pub fn select_compute_backend(mut self: Pin<&mut Self>, index: i32) {
        if !(0..=2).contains(&index) || *self.as_ref().recording() || *self.as_ref().transcribing()
        {
            return;
        }
        self.as_mut().set_selected_compute_backend(index);
        self.as_ref().rust().save_preferences();
        self.as_mut().set_status_text(QString::from(match index {
            2 => "CPU inference selected",
            1 => "Vulkan GPU preferred; CPU fallback remains available",
            _ => "Automatic Vulkan acceleration selected",
        }));
    }

    pub fn select_theme(mut self: Pin<&mut Self>, index: i32) {
        if !(0..=2).contains(&index) {
            return;
        }
        self.as_mut().set_selected_theme(index);
        self.as_ref().rust().save_preferences();
    }

    pub fn select_accent(mut self: Pin<&mut Self>, index: i32) {
        if !(0..=3).contains(&index) {
            return;
        }
        self.as_mut().set_selected_accent(index);
        self.as_ref().rust().save_preferences();
    }

    pub fn update_ai_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_ai_enabled(enabled);
        self.as_mut().set_ai_status(QString::from(if enabled {
            "Enabled · verify the provider before dictating"
        } else {
            "Off · raw transcription stays fully local"
        }));
        self.as_ref().rust().save_preferences();
    }

    pub fn select_ai_provider(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, ai_provider_catalog().len()) {
            return;
        }
        let provider = ai_provider(index);
        if *self.as_ref().ai_local_only() && !provider.local {
            self.as_mut()
                .set_ai_status(QString::from("Local-only mode blocks network AI providers"));
            return;
        }
        self.as_mut().set_selected_ai_provider(index);
        self.as_mut()
            .set_ai_model(QString::from(provider.default_model));
        self.as_mut()
            .set_ai_base_url(QString::from(provider.default_url));
        self.as_mut()
            .set_ai_key_configured(provider.local || !ai::load_api_key(provider.id).is_empty());
        self.as_mut().set_ai_local_models(QStringList::default());
        self.as_mut().set_ai_local_endpoint(provider.local);
        self.as_mut()
            .set_ai_status(QString::from(if provider.local {
                "Local endpoint · transcript stays on this computer"
            } else {
                "Cloud provider · transcript is sent only when enhancement is enabled"
            }));
        self.as_ref().rust().save_preferences();
    }

    pub fn update_ai_model(mut self: Pin<&mut Self>, value: &QString) {
        self.as_mut()
            .set_ai_model(QString::from(value.to_string().trim()));
        self.as_ref().rust().save_preferences();
    }

    pub fn update_ai_base_url(mut self: Pin<&mut Self>, value: &QString) {
        let value = value.to_string().trim().to_owned();
        let provider = ai_provider(*self.as_ref().selected_ai_provider());
        let is_local = AiConfig {
            enabled: false,
            provider: provider.id.to_owned(),
            model: String::new(),
            base_url: value.clone(),
            prompt: String::new(),
            api_key: String::new(),
            local_only: false,
            timeout_seconds: 45,
        }
        .is_local();
        if provider.local && !is_local {
            self.as_mut().set_ai_status(QString::from(
                "Ollama and LM Studio endpoints must resolve to this computer",
            ));
            return;
        }
        self.as_mut().set_ai_base_url(QString::from(&value));
        self.as_mut().set_ai_local_endpoint(is_local);
        self.as_mut().set_ai_local_models(QStringList::default());
        self.as_ref().rust().save_preferences();
    }

    pub fn update_ai_prompt(mut self: Pin<&mut Self>, value: &QString) {
        self.as_mut()
            .set_ai_prompt(QString::from(value.to_string().trim()));
        self.as_ref().rust().save_preferences();
    }

    pub fn save_ai_api_key(mut self: Pin<&mut Self>, value: &QString) {
        let provider = ai_provider(*self.as_ref().selected_ai_provider());
        match ai::store_api_key(provider.id, &value.to_string()) {
            Ok(()) => {
                self.as_mut().set_ai_key_configured(true);
                self.as_mut().set_ai_status(QString::from(
                    "API key stored securely by KDE Wallet / Secret Service",
                ));
            }
            Err(error) => self.as_mut().set_ai_status(QString::from(&error)),
        }
    }

    pub fn test_ai_provider(mut self: Pin<&mut Self>) {
        if *self.as_ref().transcribing() {
            return;
        }
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_ai_status(QString::from("Verifying provider…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, "hello comma this is a provider test");
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller
                        .as_mut()
                        .set_ai_status(QString::from(match result {
                            Ok(text) => format!("Verified · test result: {text}"),
                            Err(error) => format!("Verification failed · {error}"),
                        }));
                })
                .ok();
        });
    }

    pub fn discover_local_ai_models(mut self: Pin<&mut Self>) {
        if *self.as_ref().transcribing() {
            return;
        }
        let config = self.as_ref().rust().ai_config();
        if !config.is_local() {
            self.as_mut().set_ai_status(QString::from(
                "Local model discovery is available only for this computer",
            ));
            return;
        }
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_ai_status(QString::from("Finding installed local models…"));
        std::thread::spawn(move || {
            let result = ai::discover_local_models(&config);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    match result {
                        Ok(models) => {
                            let count = models.len();
                            controller
                                .as_mut()
                                .set_ai_local_models(models.iter().map(QString::from).collect());
                            controller.as_mut().set_ai_status(QString::from(&format!(
                                "Fully local · found {count} installed model(s)"
                            )));
                        }
                        Err(error) => controller.as_mut().set_ai_status(QString::from(&format!(
                            "Local server unavailable · {error}"
                        ))),
                    }
                })
                .ok();
        });
    }

    pub fn select_local_ai_model(mut self: Pin<&mut Self>, index: i32) {
        let Ok(index) = isize::try_from(index) else {
            return;
        };
        let model = {
            let controller = self.as_ref();
            controller.ai_local_models().get(index).cloned()
        };
        let Some(model) = model else {
            return;
        };
        self.as_mut().set_ai_model(model);
        self.as_mut()
            .set_ai_status(QString::from("Fully local · installed model selected"));
        self.as_ref().rust().save_preferences();
    }

    pub fn diagnose_ollama(mut self: Pin<&mut Self>) {
        if *self.as_ref().ollama_busy() {
            return;
        }
        let config = ollama_config();
        let qt_thread = self.qt_thread();
        self.as_mut().set_ollama_busy(true);
        self.as_mut().set_ollama_status(QString::from(
            "Checking Ollama installation and local server…",
        ));
        std::thread::spawn(move || {
            let installed = Command::new("ollama").arg("--version").output().is_ok();
            let (status, models) = if !installed {
                (
                    "Ollama is not installed. Open the official Linux guide below, then run this check again.".to_owned(),
                    None,
                )
            } else {
                match ai::discover_local_models(&config) {
                    Ok(models) => (
                        format!("Ollama is ready · {} installed model(s).", models.len()),
                        Some(models),
                    ),
                    Err(error) if error.contains("reported no installed models") => (
                        "Ollama is running but has no models. Choose a model below and download it.".to_owned(),
                        Some(Vec::new()),
                    ),
                    Err(_) => (
                        "Ollama is installed but its local server is not responding. Start it below.".to_owned(),
                        None,
                    ),
                }
            };
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_ollama_busy(false);
                    controller.as_mut().set_ollama_installed(installed);
                    controller
                        .as_mut()
                        .set_ollama_status(QString::from(&status));
                    if let Some(models) = models {
                        controller
                            .as_mut()
                            .set_ai_local_models(models.iter().map(QString::from).collect());
                    }
                })
                .ok();
        });
    }

    pub fn start_ollama(mut self: Pin<&mut Self>) {
        if *self.as_ref().ollama_busy() {
            return;
        }
        let qt_thread = self.qt_thread();
        self.as_mut().set_ollama_busy(true);
        self.as_mut()
            .set_ollama_status(QString::from("Starting the local Ollama server…"));
        std::thread::spawn(move || {
            let started = Command::new("ollama")
                .arg("serve")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
            let status = match started {
                Ok(_) => {
                    let mut ready = false;
                    for _ in 0..10 {
                        std::thread::sleep(Duration::from_millis(300));
                        if ollama_server_responds() {
                            ready = true;
                            break;
                        }
                    }
                    if ready {
                        "Ollama is running. Download or select a local model below.".to_owned()
                    } else {
                        "Ollama was launched but did not become ready. Check `ollama serve` in a terminal for details.".to_owned()
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    "Ollama is not installed. Open the official Linux guide below first.".to_owned()
                }
                Err(error) => format!("Ollama could not be started: {error}"),
            };
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_ollama_busy(false);
                    controller
                        .as_mut()
                        .set_ollama_status(QString::from(&status));
                })
                .ok();
        });
    }

    pub fn pull_ollama_model(mut self: Pin<&mut Self>, model: &QString) {
        if *self.as_ref().ollama_busy() {
            return;
        }
        let model = model.to_string().trim().to_owned();
        if !valid_ollama_model_name(&model) {
            self.as_mut().set_ollama_status(QString::from(
                "Enter a valid Ollama model name using letters, numbers, `.`, `_`, `-`, `/`, or `:`.",
            ));
            return;
        }
        let qt_thread = self.qt_thread();
        self.as_mut().set_ollama_busy(true);
        self.as_mut().set_ollama_status(QString::from(&format!(
            "Downloading {model} locally… this can take several minutes."
        )));
        std::thread::spawn(move || {
            let result = Command::new("ollama")
                .args(["pull", &model])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            let (status, models, pulled) = match result {
                Ok(exit_status) if exit_status.success() => {
                    let models = ai::discover_local_models(&ollama_config()).unwrap_or_default();
                    (
                        format!("Downloaded {model} · ready for local enhancement."),
                        models,
                        true,
                    )
                }
                Ok(exit_status) => (
                    format!(
                        "Model download failed with {exit_status}. Run `ollama pull {model}` in a terminal for detailed diagnostics."
                    ),
                    Vec::new(),
                    false,
                ),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => (
                    "Ollama is not installed. Open the official Linux guide below first."
                        .to_owned(),
                    Vec::new(),
                    false,
                ),
                Err(error) => (
                    format!("Model download could not start: {error}"),
                    Vec::new(),
                    false,
                ),
            };
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_ollama_busy(false);
                    controller
                        .as_mut()
                        .set_ollama_status(QString::from(&status));
                    if pulled {
                        controller
                            .as_mut()
                            .set_ai_local_models(models.iter().map(QString::from).collect());
                        controller.as_mut().set_ai_model(QString::from(&model));
                        controller.as_ref().rust().save_preferences();
                    }
                })
                .ok();
        });
    }

    pub fn update_ai_local_only(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_ai_local_only(enabled);
        if enabled && !*self.as_ref().ai_local_endpoint() {
            self.as_mut().select_ai_provider(7);
        }
        self.as_mut().set_ai_status(QString::from(if enabled {
            "Privacy lock active · network AI providers are disabled"
        } else {
            "Privacy lock off · cloud providers may be selected explicitly"
        }));
        self.as_ref().rust().save_preferences();
    }

    pub fn select_ai_profile(mut self: Pin<&mut Self>, index: i32) {
        if index < 0
            || usize::try_from(index)
                .ok()
                .is_none_or(|value| value > self.as_ref().rust().ai_profiles.len())
        {
            return;
        }
        let (name, prompt, application_match) = {
            let controller = self.as_ref();
            let profile = usize::try_from(index - 1)
                .ok()
                .and_then(|value| controller.rust().ai_profiles.get(value));
            profile.map_or_else(
                || (String::new(), String::new(), String::new()),
                |profile| {
                    (
                        profile.name.clone(),
                        profile.prompt.clone(),
                        profile.application_match.clone(),
                    )
                },
            )
        };
        self.as_mut().set_selected_ai_profile(index);
        self.as_mut().set_ai_profile_name(QString::from(&name));
        self.as_mut().set_ai_profile_prompt(QString::from(&prompt));
        self.as_mut()
            .set_ai_profile_match(QString::from(&application_match));
        self.as_mut().set_ai_status(QString::from(if index == 0 {
            "Default cleanup prompt selected"
        } else {
            "Application profile selected · use this profile for the target app"
        }));
    }

    pub fn save_ai_profile(
        mut self: Pin<&mut Self>,
        name: &QString,
        prompt: &QString,
        application_match: &QString,
    ) {
        let name = name.to_string().trim().to_owned();
        let prompt = prompt.to_string().trim().to_owned();
        let application_match = application_match.to_string().trim().to_owned();
        if name.is_empty() || prompt.is_empty() {
            self.as_mut().set_ai_status(QString::from(
                "Profile name and cleanup prompt are required",
            ));
            return;
        }
        let profiles = &mut self.as_mut().rust_mut().get_mut().ai_profiles;
        let index = if let Some(index) = profiles
            .iter()
            .position(|profile| profile.name.eq_ignore_ascii_case(&name))
        {
            profiles[index] = AiProfile {
                name,
                prompt,
                application_match,
            };
            index
        } else {
            profiles.push(AiProfile {
                name,
                prompt,
                application_match,
            });
            profiles.len() - 1
        };
        if let Err(error) = save_ai_profiles(profiles) {
            self.as_mut().set_ai_status(QString::from(&format!(
                "Could not save application profile: {error}"
            )));
            return;
        }
        let names = std::iter::once(QString::from("Default"))
            .chain(profiles.iter().map(|profile| QString::from(&profile.name)))
            .collect();
        let selected = i32::try_from(index + 1).unwrap_or(0);
        let selected_name = profiles[index].name.clone();
        let selected_prompt = profiles[index].prompt.clone();
        let selected_match = profiles[index].application_match.clone();
        self.as_mut().set_ai_profile_names(names);
        self.as_mut().set_selected_ai_profile(selected);
        self.as_mut()
            .set_ai_profile_name(QString::from(&selected_name));
        self.as_mut()
            .set_ai_profile_prompt(QString::from(&selected_prompt));
        self.as_mut()
            .set_ai_profile_match(QString::from(&selected_match));
        self.as_mut()
            .set_ai_status(QString::from("Application profile saved"));
    }

    pub fn delete_ai_profile(mut self: Pin<&mut Self>) {
        let Ok(index) = usize::try_from(*self.as_ref().selected_ai_profile() - 1) else {
            return;
        };
        let profiles = &mut self.as_mut().rust_mut().get_mut().ai_profiles;
        if index >= profiles.len() {
            return;
        }
        profiles.remove(index);
        if save_ai_profiles(profiles).is_err() {
            return;
        }
        let names = std::iter::once(QString::from("Default"))
            .chain(profiles.iter().map(|profile| QString::from(&profile.name)))
            .collect();
        self.as_mut().set_ai_profile_names(names);
        self.as_mut().set_selected_ai_profile(0);
        self.as_mut().set_ai_profile_name(QString::default());
        self.as_mut().set_ai_profile_prompt(QString::default());
        self.as_mut().set_ai_profile_match(QString::default());
        self.as_mut()
            .set_ai_status(QString::from("Application profile deleted"));
    }

    pub fn update_auto_profiles_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        let script = configure_kwin_profile_script(enabled);
        self.as_mut().set_auto_profiles_enabled(enabled);
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_ai_status(QString::from(if let Err(error) = script {
                format!("Could not configure the FluidVoice KWin script: {error}")
            } else if enabled {
                "Automatic profiles enabled · switch applications to test matching".to_owned()
            } else {
                "Automatic profiles disabled · profile selection remains manual".to_owned()
            }));
    }

    fn apply_active_application(mut self: Pin<&mut Self>, application: ActiveApplication) {
        let label = if application.title.is_empty() {
            application.resource_class.clone()
        } else {
            format!("{} · {}", application.resource_class, application.title)
        };
        self.as_mut().set_active_application(QString::from(&label));
        if !*self.as_ref().auto_profiles_enabled() {
            return;
        }
        let haystack =
            format!("{}\n{}", application.resource_class, application.title).to_lowercase();
        let matched = self
            .as_ref()
            .rust()
            .ai_profiles
            .iter()
            .position(|profile| profile_matches_application(profile, &haystack));
        if let Some(index) = matched {
            let selected = i32::try_from(index + 1).unwrap_or(0);
            self.as_mut().select_ai_profile(selected);
            let name = self.as_ref().ai_profile_name().to_string();
            self.as_mut().set_ai_status(QString::from(&format!(
                "Automatically selected {name} for {}",
                application.resource_class
            )));
        }
    }

    pub fn rewrite_selected_text(mut self: Pin<&mut Self>, instruction: &QString) {
        let instruction = instruction.to_string().trim().to_owned();
        if instruction.is_empty() || *self.as_ref().transcribing() {
            return;
        }
        let Some(desktop_sender) = self.as_ref().rust().desktop_sender.clone() else {
            self.as_mut()
                .set_ai_status(QString::from("Desktop integration is not ready"));
            return;
        };
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        config.prompt = "Rewrite the selected text according to the user's instruction. Preserve meaning unless the instruction asks otherwise. Output only the replacement text, with no explanation or markdown fences.".to_owned();
        self.as_mut().rust_mut().get_mut().last_write_instruction = instruction.clone();
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_ai_status(QString::from("Capturing selected text…"));
        std::thread::spawn(move || {
            let (reply, result) = std::sync::mpsc::channel();
            if desktop_sender
                .send(DesktopCommand::CopySelection(reply))
                .is_err()
            {
                return;
            }
            let copied = result
                .recv_timeout(Duration::from_secs(8))
                .map_err(|_| "Timed out waiting for Wayland selection capture".to_owned())
                .and_then(|result| result);
            std::thread::sleep(Duration::from_millis(150));
            let selected = copied.and_then(|()| {
                ClipboardDelivery::connect()
                    .map_err(|error| error.to_string())?
                    .read_text()
                    .map_err(|error| error.to_string())
            });
            let rewritten = selected.and_then(|selected| {
                if selected.trim().is_empty() {
                    return Err("The clipboard contains no selected text".to_owned());
                }
                let text = ai::enhance(
                    &config,
                    &format!("User instruction: {instruction}\n\nSelected text:\n{selected}"),
                )?;
                Ok((selected, text))
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    match rewritten {
                        Ok((selected, text)) => {
                            controller.as_mut().set_last_raw_text(QString::from(&selected));
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let copied = rust
                                .clipboard
                                .as_mut()
                                .is_some_and(|clipboard| clipboard.copy_transcript(&text).is_ok());
                            if copied {
                                if let Some(sender) = controller.as_ref().rust().desktop_sender.as_ref() {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                                controller.as_mut().set_transcript_text(QString::from(&text));
                                controller.as_mut().set_live_transcript(QString::from(&text));
                                controller.as_mut().set_overlay_result_available(true);
                                controller.as_mut().set_overlay_visible(true);
                                controller.as_mut().set_ai_status(QString::from(
                                    "Selected text rewritten · replacement pasted or left on clipboard",
                                ));
                            } else {
                                controller.as_mut().set_ai_status(QString::from(
                                    "Rewrite completed but clipboard delivery failed",
                                ));
                            }
                        }
                        Err(error) => controller.as_mut().set_ai_status(QString::from(&format!(
                            "Rewrite failed · {error}"
                        ))),
                    }
                })
                .ok();
        });
    }

    pub fn write_from_instruction(mut self: Pin<&mut Self>, instruction: &QString) {
        let instruction = instruction.to_string().trim().to_owned();
        if instruction.is_empty() || *self.as_ref().transcribing() {
            return;
        }
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        config.prompt = "Write the requested text. Follow the user's instruction precisely. Output only the finished text, with no explanation or markdown fences.".to_owned();
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut().set_ai_status(QString::from("Writing draft…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, &instruction);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    match result {
                        Ok(text) => {
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let copied = rust
                                .clipboard
                                .as_mut()
                                .is_some_and(|clipboard| clipboard.copy_transcript(&text).is_ok());
                            controller.as_mut().set_last_raw_text(QString::default());
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&text));
                            controller
                                .as_mut()
                                .set_live_transcript(QString::from(&text));
                            controller.as_mut().set_ai_status(QString::from(if copied {
                                "Draft written and copied to the clipboard"
                            } else {
                                "Draft written · use Copy result to recover it"
                            }));
                        }
                        Err(error) => controller
                            .as_mut()
                            .set_ai_status(QString::from(&format!("Write Mode failed · {error}"))),
                    }
                })
                .ok();
        });
    }

    pub fn retry_write_mode(mut self: Pin<&mut Self>) {
        if *self.as_ref().transcribing() {
            return;
        }
        let selected = self.as_ref().last_raw_text().to_string();
        let instruction = self.as_ref().rust().last_write_instruction.clone();
        if selected.trim().is_empty() || instruction.trim().is_empty() {
            self.as_mut()
                .set_ai_status(QString::from("No Write Mode result is available to retry"));
            return;
        }
        let mut config = self.as_ref().rust().ai_config();
        config.enabled = true;
        config.prompt = "Rewrite the selected text according to the user's instruction. Preserve meaning unless the instruction asks otherwise. Output only the replacement text, with no explanation or markdown fences.".to_owned();
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_ai_status(QString::from("Retrying Write Mode…"));
        std::thread::spawn(move || {
            let result = ai::enhance(
                &config,
                &format!("User instruction: {instruction}\n\nSelected text:\n{selected}"),
            );
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    match result {
                        Ok(text) => {
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let copied = rust
                                .clipboard
                                .as_mut()
                                .is_some_and(|clipboard| clipboard.copy_transcript(&text).is_ok());
                            if copied {
                                if let Some(sender) =
                                    controller.as_ref().rust().desktop_sender.as_ref()
                                {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                                controller
                                    .as_mut()
                                    .set_transcript_text(QString::from(&text));
                                controller
                                    .as_mut()
                                    .set_live_transcript(QString::from(&text));
                                controller.as_mut().set_overlay_result_available(true);
                                controller.as_mut().set_overlay_visible(true);
                                controller.as_mut().set_ai_status(QString::from(
                                    "Write Mode retry pasted or left on clipboard",
                                ));
                            } else {
                                controller.as_mut().set_ai_status(QString::from(
                                    "Write Mode retry completed but clipboard delivery failed",
                                ));
                            }
                        }
                        Err(error) => controller.as_mut().set_ai_status(QString::from(&format!(
                            "Write Mode retry failed · {error}"
                        ))),
                    }
                })
                .ok();
        });
    }

    pub fn check_for_updates(mut self: Pin<&mut Self>) {
        if *self.as_ref().transcribing() {
            return;
        }
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_update_status(QString::from("Checking GitHub Releases…"));
        std::thread::spawn(move || {
            let result = check_latest_release(env!("CARGO_PKG_VERSION"));
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller
                        .as_mut()
                        .set_update_status(QString::from(&result));
                })
                .ok();
        });
    }

    pub fn add_dictionary_term(mut self: Pin<&mut Self>, term: &QString) {
        let term = term.to_string().trim().to_owned();
        if term.is_empty() {
            return;
        }
        let mut entries = load_dictionary_entries();
        if entries
            .iter()
            .any(|entry| entry.spoken.eq_ignore_ascii_case(&term))
        {
            self.as_mut()
                .set_status_text(QString::from("Dictionary entry already exists"));
            return;
        }
        entries.push(DictionaryEntry {
            spoken: term.clone(),
            preferred: term,
        });
        if save_dictionary_entries(&entries).is_ok() {
            self.as_mut()
                .set_dictionary_terms(dictionary_ui_list(&entries));
            self.as_mut()
                .set_status_text(QString::from("Dictionary updated"));
        }
    }

    pub fn add_dictionary_replacement(
        mut self: Pin<&mut Self>,
        spoken: &QString,
        preferred: &QString,
    ) {
        let spoken = sanitize_dictionary_field(&spoken.to_string());
        let preferred = sanitize_dictionary_field(&preferred.to_string());
        if spoken.is_empty() || preferred.is_empty() {
            self.as_mut().set_status_text(QString::from(
                "Spoken and preferred forms are both required",
            ));
            return;
        }
        let mut entries = load_dictionary_entries();
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.spoken.eq_ignore_ascii_case(&spoken))
        {
            entry.preferred = preferred;
        } else {
            entries.push(DictionaryEntry { spoken, preferred });
        }
        if save_dictionary_entries(&entries).is_ok() {
            self.as_mut()
                .set_dictionary_terms(dictionary_ui_list(&entries));
            self.as_mut()
                .set_status_text(QString::from("Dictionary replacement saved"));
        }
    }

    pub fn import_dictionary(mut self: Pin<&mut Self>, path: &QString, conflict_mode: i32) {
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let imported = match read_dictionary_import(&path) {
            Ok(entries) if !entries.is_empty() => entries,
            Ok(_) => {
                self.as_mut()
                    .set_status_text(QString::from("Dictionary import contained no entries"));
                return;
            }
            Err(error) => {
                self.as_mut()
                    .set_status_text(QString::from(&format!("Dictionary import failed: {error}")));
                return;
            }
        };
        let mut entries = if conflict_mode == 2 {
            Vec::new()
        } else {
            load_dictionary_entries()
        };
        let mut added = 0;
        let mut updated = 0;
        let mut skipped = 0;
        for candidate in imported {
            if let Some(existing) = entries
                .iter_mut()
                .find(|entry| entry.spoken.eq_ignore_ascii_case(&candidate.spoken))
            {
                if conflict_mode == 1 || conflict_mode == 2 {
                    existing.preferred = candidate.preferred;
                    updated += 1;
                } else {
                    skipped += 1;
                }
            } else {
                entries.push(candidate);
                added += 1;
            }
        }
        if let Err(error) = save_dictionary_entries(&entries) {
            self.as_mut().set_status_text(QString::from(&format!(
                "Dictionary import could not be saved: {error}"
            )));
            return;
        }
        self.as_mut()
            .set_dictionary_terms(dictionary_ui_list(&entries));
        self.as_mut().set_status_text(QString::from(&format!(
            "Dictionary import complete · {added} added · {updated} updated · {skipped} kept"
        )));
    }

    pub fn export_dictionary(mut self: Pin<&mut Self>, path: &QString) {
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        match write_dictionary_csv(&path, &load_dictionary_entries()) {
            Ok(()) => self
                .as_mut()
                .set_status_text(QString::from("Dictionary exported as CSV")),
            Err(error) => self
                .as_mut()
                .set_status_text(QString::from(&format!("Dictionary export failed: {error}"))),
        }
    }

    pub fn remove_dictionary_term(mut self: Pin<&mut Self>, index: i32) {
        let mut terms = load_dictionary_entries();
        let Ok(index) = usize::try_from(index) else {
            return;
        };
        if index >= terms.len() {
            return;
        }
        terms.remove(index);
        if save_dictionary_entries(&terms).is_ok() {
            self.as_mut()
                .set_dictionary_terms(dictionary_ui_list(&terms));
            self.as_mut()
                .set_status_text(QString::from("Dictionary entry removed"));
        }
    }

    pub fn clear_history(mut self: Pin<&mut Self>) {
        save_lines(&history_path(), &[]).ok();
        if audio_history_directory().is_dir() {
            fs::remove_dir_all(audio_history_directory()).ok();
        }
        self.as_mut().set_history_entries(QStringList::default());
        self.as_mut().set_transcript_count(0);
        self.as_mut().set_dictated_word_count(0);
        self.as_mut()
            .set_status_text(QString::from("History cleared"));
        self.as_mut()
            .set_audio_history_status(QString::from(audio_history_summary()));
        self.as_mut().set_history_entries(
            load_lines(&history_path())
                .iter()
                .rev()
                .map(QString::from)
                .collect(),
        );
    }

    pub fn update_stats_preferences(
        mut self: Pin<&mut Self>,
        typing_wpm: i32,
        skip_weekends: bool,
    ) {
        self.as_mut().set_typing_wpm(typing_wpm.clamp(10, 250));
        self.as_mut().set_skip_weekends(skip_weekends);
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_status_text(QString::from("Statistics preferences updated"));
    }

    pub fn update_audio_history(mut self: Pin<&mut Self>, enabled: bool, budget_mb: i32) {
        let budget_mb = budget_mb.clamp(100, 10_000);
        self.as_mut().set_audio_history_enabled(enabled);
        self.as_mut().set_audio_history_budget_mb(budget_mb);
        self.as_ref().rust().save_preferences();
        if let Err(error) = prune_audio_history(u64::try_from(budget_mb).unwrap_or(500) * 1_048_576)
        {
            self.as_mut()
                .set_audio_history_status(QString::from(&format!(
                    "Audio retention updated, but pruning failed: {error}"
                )));
            return;
        }
        self.as_mut()
            .set_audio_history_status(QString::from(audio_history_summary()));
        self.as_mut().set_history_entries(
            load_lines(&history_path())
                .iter()
                .rev()
                .map(QString::from)
                .collect(),
        );
        self.as_mut().set_status_text(QString::from(if enabled {
            "Audio history enabled · recordings stay local"
        } else {
            "Audio history disabled · existing recordings are retained"
        }));
    }

    pub fn delete_history_audio(mut self: Pin<&mut Self>, entry: &QString) {
        let entry = entry.to_string();
        let Some(path) = history_field(&entry, 8).filter(|value| !value.is_empty()) else {
            return;
        };
        let path = PathBuf::from(path);
        let directory = audio_history_directory();
        let safe = path
            .canonicalize()
            .ok()
            .zip(directory.canonicalize().ok())
            .is_some_and(|(path, directory)| path.starts_with(directory));
        if !safe || fs::remove_file(&path).is_err() {
            self.as_mut()
                .set_status_text(QString::from("Recording could not be deleted safely"));
            return;
        }
        let mut history = load_lines(&history_path());
        if let Some(saved) = history.iter_mut().find(|saved| saved.as_str() == entry) {
            let mut fields = saved.split('\t').map(str::to_owned).collect::<Vec<_>>();
            fields.resize(9, String::new());
            fields[8].clear();
            *saved = fields.join("\t");
            save_lines(&history_path(), &history).ok();
            self.as_mut()
                .set_history_entries(history.iter().rev().map(QString::from).collect());
        }
        self.as_mut()
            .set_audio_history_status(QString::from(audio_history_summary()));
        self.as_mut()
            .set_status_text(QString::from("Local recording deleted"));
    }

    pub fn export_audio_history(mut self: Pin<&mut Self>, path: &QString) {
        if *self.as_ref().transcribing() {
            return;
        }
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut()
            .set_status_text(QString::from("Exporting audio history…"));
        std::thread::spawn(move || {
            let result = write_audio_history_zip(&path);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.set_status_text(QString::from(match result {
                        Ok(count) => format!("Exported {count} recording(s) and history metadata"),
                        Err(error) => format!("Audio-history export failed: {error}"),
                    }));
                })
                .ok();
        });
    }

    pub fn export_history(mut self: Pin<&mut Self>, path: &QString, format: &QString) {
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let history = load_lines(&history_path());
        match write_history_export(&path, &format.to_string(), &history) {
            Ok(()) => self
                .as_mut()
                .set_status_text(QString::from("History export complete")),
            Err(error) => self
                .as_mut()
                .set_status_text(QString::from(&format!("History export failed: {error}"))),
        }
    }

    pub fn copy_history_text(mut self: Pin<&mut Self>, entry: &QString, mode: i32) {
        let entry = entry.to_string();
        let (text, label) = history_clipboard_text(&entry, mode);
        let result = self
            .as_mut()
            .rust_mut()
            .get_mut()
            .clipboard
            .as_mut()
            .ok_or_else(|| "Clipboard integration is not ready".to_owned())
            .and_then(|clipboard| {
                clipboard
                    .copy_transcript(&text)
                    .map_err(|error| error.to_string())
            });
        self.as_mut().set_status_text(QString::from(match result {
            Ok(()) => label.to_owned(),
            Err(error) => format!("History copy failed · {error}"),
        }));
    }

    pub fn transcribe_file(mut self: Pin<&mut Self>, path: &QString) {
        if *self.as_ref().transcribing() || *self.as_ref().recording() {
            return;
        }
        let Some(model) = selected_model_path(self.as_ref().rust()) else {
            self.as_mut().set_file_transcription_status(QString::from(
                "Download and activate a Whisper model first.",
            ));
            return;
        };
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let language = selected_language_code(self.as_ref().rust());
        let use_gpu = self.as_ref().rust().selected_compute_backend != 2;
        let ai_config = self.as_ref().rust().ai_config();
        let cancel = Arc::new(AtomicBool::new(false));
        self.as_mut().rust_mut().get_mut().meeting_cancel = Some(cancel.clone());
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        self.as_mut().set_meeting_progress(0.0);
        self.as_mut().set_meeting_segments(QStringList::default());
        self.as_mut()
            .set_file_transcription_status(QString::from("Decoding long recording locally…"));
        std::thread::spawn(move || {
            let progress_thread = qt_thread.clone();
            let result = transcribe_long_audio_file(
                &path,
                &model,
                language,
                use_gpu,
                &cancel,
                move |progress, completed, total| {
                    progress_thread
                        .queue(move |mut controller| {
                            controller.as_mut().set_meeting_progress(progress);
                            controller
                                .as_mut()
                                .set_file_transcription_status(QString::from(&format!(
                                    "Transcribing segment {completed} of {total}…"
                                )));
                        })
                        .ok();
                },
            )
            .map(|meeting| {
                let raw_text = meeting.text.clone();
                let started = Instant::now();
                let (text, ai_error, ai_duration_ms) =
                    if ai_config.enabled && meeting.text.len() <= 20_000 {
                        match ai::enhance(&ai_config, &meeting.text) {
                            Ok(enhanced) => (enhanced, None, started.elapsed().as_millis()),
                            Err(error) => (
                                meeting.text.clone(),
                                Some(error),
                                started.elapsed().as_millis(),
                            ),
                        }
                    } else {
                        (meeting.text.clone(), None, 0)
                    };
                (text, raw_text, meeting.segments, ai_error, ai_duration_ms)
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().rust_mut().get_mut().meeting_cancel = None;
                    match result {
                        Ok((text, raw_text, mut segments, ai_error, ai_duration_ms)) => {
                            let dictionary = load_lines(&dictionary_path());
                            let processed = process_transcript(
                                &text,
                                controller.as_ref().rust().command_mode_enabled,
                                &dictionary,
                            );
                            for segment in &mut segments {
                                segment.text = process_transcript(
                                    &segment.text,
                                    controller.as_ref().rust().command_mode_enabled,
                                    &dictionary,
                                );
                            }
                            let provider = ai_provider_name(&ai_config);
                            let ai_status = if ai_config.enabled {
                                if ai_error.is_some() {
                                    "fallback"
                                } else {
                                    "enhanced"
                                }
                            } else {
                                "disabled"
                            };
                            record_history(
                                controller.as_mut(),
                                &processed,
                                &HistoryContext {
                                    raw_text: &raw_text,
                                    provider,
                                    model: &ai_config.model,
                                    ai_status,
                                    ai_duration_ms,
                                    source: "file",
                                    audio_path: "",
                                },
                            );
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&processed));
                            controller.as_mut().set_meeting_progress(1.0);
                            controller.as_mut().set_meeting_segments(
                                segments.iter().map(meeting_segment_qstring).collect(),
                            );
                            controller.as_mut().rust_mut().get_mut().meeting_results = segments;
                            controller
                                .as_mut()
                                .set_file_transcription_status(QString::from(
                                ai_error.map_or_else(
                                    || "Complete — timestamped transcript added to History and ready to export.".to_owned(),
                                    |error| {
                                        format!(
                                            "AI enhancement failed; raw transcript saved. {error}"
                                        )
                                    },
                                ),
                            ));
                            controller
                                .set_status_text(QString::from("File transcription complete"));
                        }
                        Err(error) => {
                            controller.as_mut().set_meeting_progress(0.0);
                            controller
                                .as_mut()
                                .set_file_transcription_status(QString::from(&error));
                            controller.set_status_text(QString::from("File transcription failed"));
                        }
                    }
                })
                .ok();
        });
    }

    pub fn cancel_meeting_transcription(mut self: Pin<&mut Self>) {
        if let Some(cancel) = self.as_ref().rust().meeting_cancel.as_ref() {
            cancel.store(true, Ordering::Relaxed);
            self.as_mut().set_file_transcription_status(QString::from(
                "Cancelling after the current segment…",
            ));
        }
    }

    pub fn export_meeting(mut self: Pin<&mut Self>, path: &QString, format: &QString) {
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let format = format.to_string();
        match write_meeting_export(&path, &format, &self.as_ref().rust().meeting_results) {
            Ok(()) => self
                .as_mut()
                .set_file_transcription_status(QString::from(&format!(
                    "Meeting transcript exported as {}.",
                    format.to_uppercase()
                ))),
            Err(error) => self
                .as_mut()
                .set_file_transcription_status(QString::from(&format!(
                    "Meeting export failed: {error}"
                ))),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn toggle_recording(mut self: Pin<&mut Self>) {
        if *self.as_ref().recording() {
            if let Some(token) = self.as_ref().rust().stop_token.as_ref() {
                token.stop();
            }
            self.set_status_text(QString::from("Finishing…"));
            return;
        }

        if *self.as_ref().transcribing() {
            return;
        }

        let stop_token = CaptureStopToken::new();
        let capture_target = self.as_ref().rust().capture_target.clone();
        let language = selected_language_code(self.as_ref().rust());
        let use_gpu = self.as_ref().rust().selected_compute_backend != 2;
        let model = selected_model_path(self.as_ref().rust());
        let ai_config = self.as_ref().rust().ai_config();
        let retain_audio = *self.as_ref().audio_history_enabled();
        let audio_budget_bytes =
            u64::try_from(*self.as_ref().audio_history_budget_mb()).unwrap_or(500) * 1_048_576;
        let gain = 10.0_f32.powf(*self.as_ref().gain_db() / 20.0);
        self.as_mut().rust_mut().get_mut().stop_token = Some(stop_token.clone());
        self.as_mut().set_audio_level(0.0);
        self.as_mut().set_input_db(-60.0);
        self.as_mut().set_audio_updates(0);
        self.as_mut().set_transcript_text(QString::default());
        self.as_mut().set_live_transcript(QString::default());
        self.as_mut().set_last_raw_text(QString::default());
        self.as_mut().set_overlay_result_available(false);
        self.as_mut().set_recording(true);
        self.as_mut().set_status_text(QString::from("Listening…"));
        let overlay_enabled = *self.as_ref().overlay_enabled();
        self.as_mut().set_overlay_visible(overlay_enabled);

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let level_thread = qt_thread.clone();
            let preview_thread = qt_thread.clone();
            let mut last_level_report: Option<Instant> = None;
            let (preview_sender, preview_receiver) =
                std::sync::mpsc::sync_channel::<AudioBuffer>(1);
            let preview_model = model.clone();
            let preview_language = language.clone();
            let preview_worker = std::thread::spawn(move || {
                let Some(model) = preview_model else { return };
                let automatic_language = preview_language.is_empty();
                let config = TranscriptionConfig::default()
                    .with_language(Some(preview_language))
                    .with_gpu(use_gpu);
                let Ok(transcriber) = WhisperTranscriber::load(&model, config) else {
                    return;
                };
                while let Ok(audio) = preview_receiver.recv() {
                    let preview_duration = audio.duration();
                    if automatic_language && preview_duration < Duration::from_millis(2_500) {
                        // Language classification on sub-second phonemes is
                        // strongly English-biased in Tiny/Base. Wait for a
                        // meaningful phrase before publishing automatic-mode
                        // preview text.
                        continue;
                    }
                    let mono = audio.to_asr_mono();
                    let preview_audio = mono.amplified(asr_gain(mono.peak(), gain));
                    let Ok(transcript) = transcriber.transcribe(&preview_audio) else {
                        continue;
                    };
                    if transcript.text.is_empty()
                        || suspicious_single_word(&transcript.text, preview_duration)
                    {
                        continue;
                    }
                    preview_thread
                        .queue(move |mut controller| {
                            if *controller.as_ref().recording() {
                                controller
                                    .as_mut()
                                    .set_live_transcript(QString::from(&transcript.text));
                            }
                        })
                        .ok();
                }
            });
            let result = PipeWireCapture::capture_with_preview(
                Duration::from_mins(2),
                capture_target.as_deref(),
                &stop_token,
                move |level| {
                    if last_level_report
                        .is_some_and(|reported| reported.elapsed() < Duration::from_millis(50))
                    {
                        return;
                    }
                    last_level_report = Some(Instant::now());
                    level_thread
                        .queue(move |mut controller| {
                            let adjusted = level * gain;
                            controller.as_mut().set_audio_level(meter_level(adjusted));
                            controller.as_mut().set_input_db(peak_db(adjusted));
                            let updates = controller.as_ref().audio_updates().saturating_add(1);
                            controller.set_audio_updates(updates);
                        })
                        .ok();
                },
                move |audio| {
                    preview_sender.try_send(audio).ok();
                },
            );

            let audio = match result {
                Ok(audio) => audio,
                Err(error) => {
                    qt_thread
                        .queue(move |mut controller| {
                            controller.as_mut().rust_mut().get_mut().stop_token = None;
                            controller.as_mut().set_recording(false);
                            controller.as_mut().set_overlay_visible(false);
                            controller.as_mut().set_audio_level(0.0);
                            controller.set_status_text(QString::from(&format!(
                                "Capture failed: {error}"
                            )));
                        })
                        .ok();
                    return;
                }
            };

            let peak = audio.peak();
            let duration = audio.duration();
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().rust_mut().get_mut().stop_token = None;
                    controller.as_mut().set_recording(false);
                    controller.as_mut().set_transcribing(true);
                    controller
                        .as_mut()
                        .set_audio_level(meter_level(peak * gain));
                    controller.as_mut().set_input_db(peak_db(peak * gain));
                    controller.set_status_text(QString::from("Transcribing locally…"));
                })
                .ok();
            preview_worker.join().ok();

            let mono = audio.to_asr_mono().trim_silence();
            let combined_gain = asr_gain(mono.peak(), gain);
            let asr_audio = mono.amplified(combined_gain);
            let asr_peak = asr_audio.peak();
            let diagnostic_dump = dump_asr_audio(&asr_audio);
            let transcription = model
                .as_deref()
                .ok_or_else(|| "No Whisper model is installed".to_owned())
                .and_then(|model| {
                    // The current preview UI is English-first. Automatic language
                    // detection can classify short utterances correctly yet return
                    // no segments; the fixed language path is reliable for the same
                    // captured buffer and avoids wasting audio on detection.
                    let config = TranscriptionConfig::default()
                        .with_language(Some(language))
                        .with_gpu(use_gpu);
                    let first = WhisperTranscriber::load(model, config.clone())
                        .map_err(|error| error.to_string())?
                        .transcribe(&asr_audio)
                        .map_err(|error| error.to_string())?;

                    // A tiny Whisper model can occasionally return a one-word
                    // hallucination after the live-preview context has been busy.
                    // The captured buffer is still valid, so retry suspicious
                    // multi-second results with a completely fresh context and use
                    // the richer deterministic decode. This happens before either
                    // the UI or clipboard observes the result.
                    if suspicious_single_word(&first.text, duration) {
                        let retry = WhisperTranscriber::load(model, config)
                            .map_err(|error| error.to_string())?
                            .transcribe(&asr_audio)
                            .map_err(|error| error.to_string())?;
                        if retry.text.split_whitespace().count()
                            > first.text.split_whitespace().count()
                        {
                            return Ok(retry);
                        }
                    }
                    Ok(first)
                });
            let enhancement_started = Instant::now();
            let enhancement = transcription.as_ref().ok().and_then(|transcript| {
                if ai_config.enabled {
                    let stream_thread = qt_thread.clone();
                    Some(ai::enhance_streaming(
                        &ai_config,
                        &transcript.text,
                        move |text| {
                            let text = text.to_owned();
                            stream_thread
                                .queue(move |mut controller| {
                                    controller.as_mut().set_overlay_visible(true);
                                    controller
                                        .as_mut()
                                        .set_live_transcript(QString::from(&text));
                                    controller.set_status_text(QString::from(
                                        "Enhancing locally or with selected provider…",
                                    ));
                                })
                                .ok();
                        },
                    ))
                } else {
                    None
                }
            });
            let ai_duration_ms = if enhancement.is_some() {
                enhancement_started.elapsed().as_millis()
            } else {
                0
            };
            let retained_audio = if retain_audio
                && transcription
                    .as_ref()
                    .is_ok_and(|transcript| !transcript.text.is_empty())
            {
                save_audio_history(&asr_audio, audio_budget_bytes).ok()
            } else {
                None
            };
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().set_overlay_visible(false);
                    if let Err(error) = diagnostic_dump {
                        eprintln!("Failed to save FLUIDVOICE_ASR_DUMP: {error}");
                    }
                    match transcription {
                        Ok(transcript) if !transcript.text.is_empty() => {
                            let (enhanced_text, ai_error) = match enhancement {
                                Some(Ok(text)) => (text, None),
                                Some(Err(error)) => (transcript.text.clone(), Some(error)),
                                None => (transcript.text.clone(), None),
                            };
                            let processed = process_transcript(
                                &enhanced_text,
                                controller.as_ref().rust().command_mode_enabled,
                                &load_lines(&dictionary_path()),
                            );
                            let detected_language = transcript
                                .detected_language
                                .as_deref()
                                .and_then(language_display_name)
                                .unwrap_or("Unknown language");
                            controller
                                .as_mut()
                                .set_live_transcript(QString::from(&processed));
                            controller
                                .as_mut()
                                .set_last_raw_text(QString::from(&transcript.text));
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let delivery_result = rust
                                .clipboard
                                .as_mut()
                                .ok_or(())
                                .and_then(|delivery| {
                                    delivery.copy_transcript(&processed).map_err(|_| ())
                                });
                            if delivery_result.is_ok() {
                                if let Some(sender) = controller.as_ref().rust().desktop_sender.as_ref() {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                            }
                            let ai_status = if ai_config.enabled {
                                if ai_error.is_some() { "fallback" } else { "enhanced" }
                            } else {
                                "disabled"
                            };
                            record_history(
                                controller.as_mut(),
                                &processed,
                                &HistoryContext {
                                    raw_text: &transcript.text,
                                    provider: ai_provider_name(&ai_config),
                                    model: &ai_config.model,
                                    ai_status,
                                    ai_duration_ms,
                                    source: "dictation",
                                    audio_path: retained_audio
                                        .as_deref()
                                        .and_then(std::path::Path::to_str)
                                        .unwrap_or(""),
                                },
                            );
                            controller.as_mut().set_audio_history_status(QString::from(
                                audio_history_summary(),
                            ));
                            controller.as_mut().set_transcript_text(QString::from(&processed));
                            controller.as_mut().set_overlay_result_available(true);
                            if *controller.as_ref().overlay_enabled() {
                                controller.as_mut().set_overlay_visible(true);
                            }
                            controller.set_status_text(QString::from(if let Some(error) = ai_error {
                                format!("AI enhancement failed · raw transcript delivered · {error}")
                            } else if delivery_result.is_ok() {
                                format!("Dictated {:.1}s · {detected_language} · pasted or copied", duration.as_secs_f32())
                            } else {
                                format!("Transcribed {:.1}s · {detected_language} · clipboard unavailable", duration.as_secs_f32())
                            }));
                        }
                        Ok(_) => {
                            controller.as_mut().set_transcript_text(QString::from(&format!(
                                "No speech recognized (ASR peak {:.0}%). Increase the Scarlett hardware gain if this persists.",
                                asr_peak * 100.0
                            )));
                            controller.set_status_text(QString::from(&format!(
                                "No speech recognized · ASR peak {:.0}%",
                                asr_peak * 100.0
                            )));
                        }
                        Err(error) => {
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&format!(
                                    "Transcription failed: {error}"
                                )));
                            controller.set_status_text(QString::from("Transcription failed"));
                        }
                    }
                })
                .ok();
        });
    }

    pub fn set_overlay_preview(mut self: Pin<&mut Self>, visible: bool) {
        self.as_mut().set_overlay_visible(visible);
        if !visible && *self.as_ref().recording() {
            if let Some(token) = self.as_ref().rust().stop_token.as_ref() {
                token.stop();
            }
            self.set_status_text(QString::from("Finishing…"));
        }
    }

    fn refresh_model_catalog(mut self: Pin<&mut Self>) {
        let paths = whisper_model_catalog()
            .iter()
            .map(resolve_model_path)
            .collect::<Vec<_>>();
        let (states, details) = model_ui_lists(&paths);
        self.as_mut().rust_mut().get_mut().model_paths = paths;
        self.as_mut().set_model_states(states);
        self.as_mut().set_model_details(details);
    }
}

impl FluidVoiceControllerRust {
    fn save_preferences(&self) {
        let preferences = Preferences {
            language: selected_language_code(self),
            model: selected_model_path(self).unwrap_or_default(),
            shortcut: selected_shortcut_trigger(self),
            input: self.capture_target.clone().unwrap_or_default(),
            gain_db: self.gain_db,
            overlay_enabled: self.overlay_enabled,
            overlay_size: self.selected_overlay_size,
            overlay_position: self.selected_overlay_position,
            overlay_show_text: self.overlay_show_text,
            overlay_opacity: self.overlay_opacity,
            command_mode_enabled: self.command_mode_enabled,
            compute_backend: self.selected_compute_backend,
            theme: self.selected_theme,
            accent: self.selected_accent,
            ai_enabled: self.ai_enabled,
            ai_provider: self.selected_ai_provider,
            ai_model: self.ai_model.to_string(),
            ai_base_url: self.ai_base_url.to_string(),
            ai_prompt: self.ai_prompt.to_string(),
            ai_local_only: self.ai_local_only,
            auto_profiles_enabled: self.auto_profiles_enabled,
            typing_wpm: self.typing_wpm,
            skip_weekends: self.skip_weekends,
            audio_history_enabled: self.audio_history_enabled,
            audio_history_budget_mb: self.audio_history_budget_mb,
        };
        if let Err(error) = preferences.save() {
            eprintln!("Failed to save preferences: {error}");
        }
    }

    fn ai_config(&self) -> AiConfig {
        let provider = ai_provider(self.selected_ai_provider);
        let prompt = usize::try_from(self.selected_ai_profile - 1)
            .ok()
            .and_then(|index| self.ai_profiles.get(index))
            .map_or_else(
                || self.ai_prompt.to_string(),
                |profile| profile.prompt.clone(),
            );
        AiConfig {
            enabled: self.ai_enabled,
            provider: provider.id.to_owned(),
            model: self.ai_model.to_string(),
            base_url: self.ai_base_url.to_string(),
            prompt,
            api_key: ai::load_api_key(provider.id),
            local_only: self.ai_local_only,
            timeout_seconds: 45,
        }
    }
}

struct Preferences {
    language: String,
    model: PathBuf,
    shortcut: String,
    input: String,
    gain_db: f32,
    overlay_enabled: bool,
    overlay_size: i32,
    overlay_position: i32,
    overlay_show_text: bool,
    overlay_opacity: f32,
    command_mode_enabled: bool,
    compute_backend: i32,
    theme: i32,
    accent: i32,
    ai_enabled: bool,
    ai_provider: i32,
    ai_model: String,
    ai_base_url: String,
    ai_prompt: String,
    ai_local_only: bool,
    auto_profiles_enabled: bool,
    typing_wpm: i32,
    skip_weekends: bool,
    audio_history_enabled: bool,
    audio_history_budget_mb: i32,
}

#[derive(Clone)]
struct AiProfile {
    name: String,
    prompt: String,
    application_match: String,
}

fn load_ai_profiles() -> Vec<AiProfile> {
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

fn save_ai_profiles(profiles: &[AiProfile]) -> Result<(), String> {
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
    fs::write(
        path,
        serde_json::to_string_pretty(&values).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

fn profile_matches_application(profile: &AiProfile, lowercase_window_identity: &str) -> bool {
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
        }
    }
}

impl Preferences {
    fn load() -> Self {
        let Ok(contents) = fs::read_to_string(preferences_path()) else {
            return Self::default();
        };
        let mut preferences = Self::default();
        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("language=") {
                preferences.language = value.to_owned();
            } else if let Some(value) = line.strip_prefix("model=") {
                preferences.model = PathBuf::from(value);
            } else if let Some(value) = line.strip_prefix("shortcut=") {
                preferences.shortcut = value.to_owned();
            } else if let Some(value) = line.strip_prefix("input=") {
                preferences.input = value.to_owned();
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
            }
        }
        preferences
    }

    fn save(&self) -> Result<(), String> {
        let path = preferences_path();
        let parent = path
            .parent()
            .ok_or_else(|| "preferences path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        fs::write(
            path,
            format!(
                "language={}\nmodel={}\nshortcut={}\ninput={}\ngain_db={}\noverlay_enabled={}\noverlay_size={}\noverlay_position={}\noverlay_show_text={}\noverlay_opacity={}\ncommand_mode_enabled={}\ncompute_backend={}\ntheme={}\naccent={}\nai_enabled={}\nai_provider={}\nai_model={}\nai_base_url={}\nai_prompt={}\nai_local_only={}\nauto_profiles_enabled={}\ntyping_wpm={}\nskip_weekends={}\naudio_history_enabled={}\naudio_history_budget_mb={}\n",
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
                self.audio_history_budget_mb
            ),
        )
        .map_err(|error| error.to_string())
    }
}

enum DesktopCommand {
    Paste,
    CopySelection(std::sync::mpsc::Sender<Result<(), String>>),
    Rebind(String),
    DiagnoseTextInput(std::sync::mpsc::Sender<Result<(), String>>),
}

enum DesktopAction {
    OpenSystemSettings,
    OpenTerminal,
    OpenFileManager,
    LockScreen,
}

impl DesktopAction {
    fn description(&self) -> &'static str {
        match self {
            Self::OpenSystemSettings => "Open KDE System Settings",
            Self::OpenTerminal => "Open Konsole",
            Self::OpenFileManager => "Open Dolphin file manager",
            Self::LockScreen => "Lock the Plasma session",
        }
    }

    fn execute(&self) -> Result<(), String> {
        let mut command = match self {
            Self::OpenSystemSettings => Command::new("systemsettings"),
            Self::OpenTerminal => Command::new("konsole"),
            Self::OpenFileManager => Command::new("dolphin"),
            Self::LockScreen => {
                let mut command = Command::new("qdbus6");
                command.args([
                    "org.freedesktop.ScreenSaver",
                    "/ScreenSaver",
                    "org.freedesktop.ScreenSaver.Lock",
                ]);
                command
            }
        };
        command
            .spawn()
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
}

fn configure_kwin_profile_script(enabled: bool) -> Result<(), String> {
    let value = if enabled { "true" } else { "false" };
    let status = Command::new("kwriteconfig6")
        .args([
            "--file",
            "kwinrc",
            "--group",
            "Plugins",
            "--key",
            "fluidvoiceprofilesEnabled",
            value,
        ])
        .status()
        .map_err(|error| format!("kwriteconfig6 is unavailable: {error}"))?;
    if !status.success() {
        return Err(format!("kwriteconfig6 exited with {status}"));
    }
    Command::new("qdbus6")
        .args(["org.kde.KWin", "/KWin", "org.kde.KWin.reconfigure"])
        .status()
        .map_err(|error| format!("KWin could not reload its scripts: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "KWin rejected the script reload request".to_owned())
}

fn parse_desktop_action(value: &str) -> Option<DesktopAction> {
    let value = value.trim().to_ascii_lowercase();
    match value.as_str() {
        "open settings" | "open system settings" | "show system settings" => {
            Some(DesktopAction::OpenSystemSettings)
        }
        "open terminal" | "launch terminal" | "open konsole" => Some(DesktopAction::OpenTerminal),
        "open files" | "open file manager" | "open dolphin" => Some(DesktopAction::OpenFileManager),
        "lock screen" | "lock my screen" | "lock computer" => Some(DesktopAction::LockScreen),
        _ => None,
    }
}

fn check_latest_release(current: &str) -> String {
    let response =
        ureq::get("https://api.github.com/repos/davidkodar/fluidvoice-linux/releases/latest")
            .header("user-agent", "FluidVoice-Linux")
            .call();
    let mut response = match response {
        Ok(response) => response,
        Err(ureq::Error::StatusCode(404)) => {
            return "No public release feed is available while the repository is private."
                .to_owned();
        }
        Err(error) => return format!("Update check failed: {error}"),
    };
    let body = match response.body_mut().read_to_string() {
        Ok(body) => body,
        Err(error) => return format!("Release feed could not be read: {error}"),
    };
    let value = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(value) => value,
        Err(error) => return format!("Release feed returned invalid data: {error}"),
    };
    let Some(tag) = value.get("tag_name").and_then(serde_json::Value::as_str) else {
        return "Release feed did not include a version tag.".to_owned();
    };
    let latest = tag.trim_start_matches('v');
    match (version_tuple(current), version_tuple(latest)) {
        (Some(current), Some(latest_tuple)) if latest_tuple > current => {
            format!("Version {latest} is available on GitHub Releases.")
        }
        (Some(_), Some(_)) => format!("FluidVoice Linux {current} is up to date."),
        _ => format!("Latest release tag: {tag}"),
    }
}

fn version_tuple(value: &str) -> Option<(u32, u32, u32)> {
    let mut parts = value.split('.');
    Some((
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.split('-').next()?.parse().ok()?,
    ))
}

#[derive(Clone, Copy)]
struct AiProviderPreset {
    id: &'static str,
    name: &'static str,
    default_url: &'static str,
    default_model: &'static str,
    local: bool,
}

fn ai_provider_catalog() -> &'static [AiProviderPreset] {
    &[
        AiProviderPreset {
            id: "openai",
            name: "OpenAI",
            default_url: "https://api.openai.com/v1",
            default_model: "gpt-4.1",
            local: false,
        },
        AiProviderPreset {
            id: "anthropic",
            name: "Anthropic",
            default_url: "https://api.anthropic.com/v1",
            default_model: "claude-sonnet-4-20250514",
            local: false,
        },
        AiProviderPreset {
            id: "xai",
            name: "xAI",
            default_url: "https://api.x.ai/v1",
            default_model: "grok-3-fast",
            local: false,
        },
        AiProviderPreset {
            id: "groq",
            name: "Groq",
            default_url: "https://api.groq.com/openai/v1",
            default_model: "openai/gpt-oss-120b",
            local: false,
        },
        AiProviderPreset {
            id: "cerebras",
            name: "Cerebras",
            default_url: "https://api.cerebras.ai/v1",
            default_model: "gpt-oss-120b",
            local: false,
        },
        AiProviderPreset {
            id: "google",
            name: "Google Gemini",
            default_url: "https://generativelanguage.googleapis.com/v1beta/openai",
            default_model: "gemini-2.5-flash",
            local: false,
        },
        AiProviderPreset {
            id: "openrouter",
            name: "OpenRouter",
            default_url: "https://openrouter.ai/api/v1",
            default_model: "openai/gpt-oss-20b",
            local: false,
        },
        AiProviderPreset {
            id: "ollama",
            name: "Ollama (local)",
            default_url: "http://localhost:11434/v1",
            default_model: "qwen2.5:7b",
            local: true,
        },
        AiProviderPreset {
            id: "lmstudio",
            name: "LM Studio (local)",
            default_url: "http://localhost:1234/v1",
            default_model: "local-model",
            local: true,
        },
        AiProviderPreset {
            id: "custom",
            name: "Custom OpenAI-compatible",
            default_url: "",
            default_model: "",
            local: false,
        },
    ]
}

fn ai_provider(index: i32) -> AiProviderPreset {
    ai_provider_catalog()
        .get(usize::try_from(index).unwrap_or(7))
        .copied()
        .unwrap_or(ai_provider_catalog()[7])
}

fn ollama_config() -> AiConfig {
    AiConfig {
        enabled: true,
        provider: "ollama".to_owned(),
        model: "qwen2.5:7b".to_owned(),
        base_url: "http://localhost:11434/v1".to_owned(),
        prompt: ai::DEFAULT_PROMPT.to_owned(),
        api_key: String::new(),
        local_only: true,
        timeout_seconds: 8,
    }
}

fn ollama_server_responds() -> bool {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(2)))
        .build()
        .new_agent();
    agent
        .get("http://localhost:11434/api/version")
        .call()
        .is_ok()
}

fn valid_ollama_model_name(model: &str) -> bool {
    !model.is_empty()
        && model.len() <= 128
        && model
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._-/:".contains(character))
}

fn escape_setting(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape_setting(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut escaped = false;
    for character in value.chars() {
        if escaped {
            result.push(match character {
                'n' => '\n',
                'r' => '\r',
                other => other,
            });
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else {
            result.push(character);
        }
    }
    if escaped {
        result.push('\\');
    }
    result
}

fn preferences_path() -> PathBuf {
    if let Some(directory) = std::env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(directory).join("fluidvoice/settings.conf");
    }
    std::env::var_os("HOME").map_or_else(
        || PathBuf::from(".fluidvoice-settings.conf"),
        |home| PathBuf::from(home).join(".config/fluidvoice/settings.conf"),
    )
}

fn data_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(directory).join("fluidvoice");
    }
    std::env::var_os("HOME").map_or_else(
        || PathBuf::from(".fluidvoice"),
        |home| PathBuf::from(home).join(".local/share/fluidvoice"),
    )
}

fn dictionary_path() -> PathBuf {
    data_directory().join("dictionary.txt")
}

fn history_path() -> PathBuf {
    data_directory().join("history.tsv")
}

fn audio_history_directory() -> PathBuf {
    data_directory().join("audio-history")
}

fn audio_history_summary() -> String {
    let Ok(entries) = fs::read_dir(audio_history_directory()) else {
        return "No retained recordings · retention is off by default".to_owned();
    };
    let (count, bytes) = entries
        .flatten()
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold((0_u64, 0_u64), |(count, bytes), metadata| {
            (count + 1, bytes.saturating_add(metadata.len()))
        });
    format!(
        "{count} retained recording(s) · {:.1} MB used",
        bytes as f64 / 1_048_576.0
    )
}

fn save_audio_history(
    audio: &fluidvoice_audio::MonoAudioBuffer,
    budget_bytes: u64,
) -> Result<PathBuf, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let directory = audio_history_directory();
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis());
    let path = directory.join(format!("dictation-{timestamp}.wav"));
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: audio.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).map_err(|error| error.to_string())?;
    for sample in audio.samples() {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        writer
            .write_sample(value)
            .map_err(|error| error.to_string())?;
    }
    writer.finalize().map_err(|error| error.to_string())?;
    prune_audio_history(budget_bytes)?;
    Ok(path)
}

fn prune_audio_history(budget_bytes: u64) -> Result<(), String> {
    let directory = audio_history_directory();
    let Ok(entries) = fs::read_dir(&directory) else {
        return Ok(());
    };
    let mut files = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            metadata
                .is_file()
                .then_some((entry.path(), metadata.len(), metadata.modified().ok()))
        })
        .collect::<Vec<_>>();
    files.sort_by_key(|(_, _, modified)| *modified);
    let mut total = files.iter().map(|(_, size, _)| *size).sum::<u64>();
    for (path, size, _) in files {
        if total <= budget_bytes {
            break;
        }
        fs::remove_file(path).map_err(|error| error.to_string())?;
        total = total.saturating_sub(size);
    }
    clear_missing_audio_history_references()?;
    Ok(())
}

fn clear_missing_audio_history_references() -> Result<(), String> {
    let path = history_path();
    let mut history = load_lines(&path);
    let mut changed = false;
    for entry in &mut history {
        let mut fields = entry.split('\t').map(str::to_owned).collect::<Vec<_>>();
        if fields.get(8).is_some_and(|audio| !audio.is_empty())
            && !fields
                .get(8)
                .is_some_and(|audio| PathBuf::from(audio).is_file())
        {
            fields[8].clear();
            *entry = fields.join("\t");
            changed = true;
        }
    }
    if changed {
        save_lines(&path, &history).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn write_audio_history_zip(output_path: &PathBuf) -> Result<usize, String> {
    let parent = output_path
        .parent()
        .ok_or_else(|| "export path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let file = fs::File::create(output_path).map_err(|error| error.to_string())?;
    let mut archive = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o600);
    archive
        .start_file("history.tsv", options)
        .map_err(|error| error.to_string())?;
    archive
        .write_all(&fs::read(history_path()).unwrap_or_default())
        .map_err(|error| error.to_string())?;
    let directory = audio_history_directory();
    let canonical_directory = directory.canonicalize().ok();
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path == *output_path {
                continue;
            }
            let safe = path
                .canonicalize()
                .ok()
                .zip(canonical_directory.clone())
                .is_some_and(|(path, directory)| path.starts_with(directory));
            if !safe || !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
                continue;
            };
            archive
                .start_file(format!("audio/{name}"), options)
                .map_err(|error| error.to_string())?;
            archive
                .write_all(&fs::read(&path).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            count += 1;
        }
    }
    archive.finish().map_err(|error| error.to_string())?;
    Ok(count)
}

fn ai_profiles_path() -> PathBuf {
    data_directory().join("ai-profiles.json")
}

fn command_history_path() -> PathBuf {
    data_directory().join("command-history.tsv")
}

fn append_command_history(
    mut controller: Pin<&mut ffi::FluidVoiceController>,
    role: &str,
    text: &str,
) {
    let mut history = load_lines(&command_history_path());
    history.push(format!("{}\t{}", history_value(role), history_value(text)));
    if history.len() > 200 {
        history.drain(..history.len() - 200);
    }
    save_lines(&command_history_path(), &history).ok();
    controller
        .as_mut()
        .set_command_history(history.iter().rev().map(QString::from).collect());
}

fn load_lines(path: &PathBuf) -> Vec<String> {
    fs::read_to_string(path)
        .map(|contents| {
            contents
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn save_lines(path: &PathBuf, lines: &[String]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "data path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let mut contents = lines.join("\n");
    if !contents.is_empty() {
        contents.push('\n');
    }
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn process_transcript(text: &str, command_mode: bool, dictionary: &[String]) -> String {
    let mut processed = text.trim().to_owned();
    if command_mode {
        for (spoken, replacement) in [
            ("new paragraph", "\n\n"),
            ("new line", "\n"),
            ("question mark", "?"),
            ("exclamation mark", "!"),
            ("comma", ","),
            ("period", "."),
        ] {
            processed = replace_ascii_case_insensitive(&processed, spoken, replacement);
        }
        processed = processed
            .replace(" ,", ",")
            .replace(" .", ".")
            .replace(" ?", "?")
            .replace(" !", "!")
            .replace(" \n", "\n")
            .replace("\n ", "\n");
    }
    for line in dictionary {
        let entry = DictionaryEntry::from_storage(line);
        processed = replace_ascii_case_insensitive(&processed, &entry.spoken, &entry.preferred);
    }
    processed
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DictionaryEntry {
    spoken: String,
    preferred: String,
}

impl DictionaryEntry {
    fn from_storage(line: &str) -> Self {
        let (spoken, preferred) = line
            .split_once('\t')
            .map_or((line, line), |(spoken, preferred)| (spoken, preferred));
        Self {
            spoken: sanitize_dictionary_field(spoken),
            preferred: sanitize_dictionary_field(preferred),
        }
    }

    fn storage(&self) -> String {
        format!("{}\t{}", self.spoken, self.preferred)
    }
}

fn sanitize_dictionary_field(value: &str) -> String {
    value
        .replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn load_dictionary_entries() -> Vec<DictionaryEntry> {
    load_lines(&dictionary_path())
        .iter()
        .map(|line| DictionaryEntry::from_storage(line))
        .filter(|entry| !entry.spoken.is_empty() && !entry.preferred.is_empty())
        .collect()
}

fn save_dictionary_entries(entries: &[DictionaryEntry]) -> Result<(), String> {
    let mut entries = entries.to_vec();
    entries.sort_by_key(|entry| entry.spoken.to_lowercase());
    entries.dedup_by(|left, right| left.spoken.eq_ignore_ascii_case(&right.spoken));
    save_lines(
        &dictionary_path(),
        &entries
            .iter()
            .map(DictionaryEntry::storage)
            .collect::<Vec<_>>(),
    )
}

fn dictionary_display(line: &str) -> String {
    let entry = DictionaryEntry::from_storage(line);
    if entry.spoken == entry.preferred {
        entry.preferred
    } else {
        format!("{}  →  {}", entry.spoken, entry.preferred)
    }
}

fn dictionary_ui_list(entries: &[DictionaryEntry]) -> QStringList {
    let mut entries = entries.to_vec();
    entries.sort_by_key(|entry| entry.spoken.to_lowercase());
    entries
        .iter()
        .map(|entry| QString::from(&dictionary_display(&entry.storage())))
        .collect()
}

fn read_dictionary_import(path: &PathBuf) -> Result<Vec<DictionaryEntry>, String> {
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut entries = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = if line.contains('\t') {
            line.splitn(2, '\t').map(str::to_owned).collect::<Vec<_>>()
        } else {
            parse_csv_record(line)
        };
        if index == 0
            && fields.first().is_some_and(|field| {
                matches!(field.trim().to_lowercase().as_str(), "spoken" | "source")
            })
        {
            continue;
        }
        let spoken = fields
            .first()
            .map_or_else(String::new, |value| sanitize_dictionary_field(value));
        let preferred = fields
            .get(1)
            .map_or_else(|| spoken.clone(), |value| sanitize_dictionary_field(value));
        if !spoken.is_empty() && !preferred.is_empty() {
            entries.push(DictionaryEntry { spoken, preferred });
        }
    }
    Ok(entries)
}

fn parse_csv_record(line: &str) -> Vec<String> {
    let mut fields = vec![String::new()];
    let mut quoted = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '"' if quoted && characters.peek() == Some(&'"') => {
                fields.last_mut().expect("CSV field").push('"');
                characters.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => fields.push(String::new()),
            value => fields.last_mut().expect("CSV field").push(value),
        }
    }
    fields
}

fn write_dictionary_csv(path: &PathBuf, entries: &[DictionaryEntry]) -> Result<(), String> {
    let mut output = "spoken,preferred\n".to_owned();
    for entry in entries {
        output.push_str(&format!(
            "\"{}\",\"{}\"\n",
            entry.spoken.replace('"', "\"\""),
            entry.preferred.replace('"', "\"\"")
        ));
    }
    fs::write(path, output).map_err(|error| error.to_string())
}

fn replace_ascii_case_insensitive(text: &str, needle: &str, replacement: &str) -> String {
    let lowercase = text.to_ascii_lowercase();
    let needle = needle.to_ascii_lowercase();
    let mut result = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some(offset) = lowercase[cursor..].find(&needle) {
        let start = cursor + offset;
        let end = start + needle.len();
        let boundary_before =
            start == 0 || !lowercase.as_bytes()[start - 1].is_ascii_alphanumeric();
        let boundary_after =
            end == lowercase.len() || !lowercase.as_bytes()[end].is_ascii_alphanumeric();
        if boundary_before && boundary_after {
            result.push_str(&text[cursor..start]);
            result.push_str(replacement);
            cursor = end;
        } else {
            result.push_str(&text[cursor..end]);
            cursor = end;
        }
    }
    result.push_str(&text[cursor..]);
    result
}

struct HistoryContext<'a> {
    raw_text: &'a str,
    provider: &'a str,
    model: &'a str,
    ai_status: &'a str,
    ai_duration_ms: u128,
    source: &'a str,
    audio_path: &'a str,
}

fn record_history(
    mut controller: Pin<&mut ffi::FluidVoiceController>,
    text: &str,
    context: &HistoryContext<'_>,
) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    let mut history = load_lines(&history_path());
    history.push(format!(
        "{timestamp}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        history_value(text),
        history_value(context.raw_text),
        history_value(context.provider),
        history_value(context.model),
        history_value(context.ai_status),
        context.ai_duration_ms,
        history_value(context.source),
        history_value(context.audio_path)
    ));
    if history.len() > 500 {
        history.drain(..history.len() - 500);
    }
    save_lines(&history_path(), &history).ok();
    let word_count = history
        .iter()
        .map(|entry| history_field(entry, 1).unwrap_or(entry))
        .map(|value| value.split_whitespace().count())
        .sum::<usize>();
    controller
        .as_mut()
        .set_history_entries(history.iter().rev().map(QString::from).collect());
    controller
        .as_mut()
        .set_transcript_count(i32::try_from(history.len()).unwrap_or(i32::MAX));
    controller
        .as_mut()
        .set_dictated_word_count(i32::try_from(word_count).unwrap_or(i32::MAX));
}

fn history_value(value: &str) -> String {
    value.replace(['\n', '\r', '\t'], " ")
}

fn history_field(entry: &str, index: usize) -> Option<&str> {
    entry.split('\t').nth(index)
}

fn history_clipboard_text(entry: &str, mode: i32) -> (String, &'static str) {
    let final_text = history_field(entry, 1).unwrap_or(entry);
    let raw_text = history_field(entry, 2).unwrap_or(final_text);
    match mode {
        0 => (
            raw_text.to_owned(),
            "Raw transcript copied · ready to paste",
        ),
        2 => (
            format!("Raw transcript:\n{raw_text}\n\nFinal text:\n{final_text}"),
            "Raw and final text copied · ready to paste",
        ),
        3 => (
            raw_text.to_owned(),
            "AI result undone on clipboard · raw text ready to paste",
        ),
        _ => (
            final_text.to_owned(),
            "Final transcript copied · ready to paste",
        ),
    }
}

fn ai_provider_name(config: &AiConfig) -> &str {
    if config.enabled { &config.provider } else { "" }
}

fn write_history_export(path: &PathBuf, format: &str, history: &[String]) -> Result<(), String> {
    let records = history
        .iter()
        .map(|entry| {
            serde_json::json!({
                "timestamp": history_field(entry, 0).and_then(|value| value.parse::<u64>().ok()).unwrap_or(0),
                "text": history_field(entry, 1).unwrap_or(entry),
                "raw_text": history_field(entry, 2).unwrap_or_else(|| history_field(entry, 1).unwrap_or(entry)),
                "ai_provider": history_field(entry, 3).unwrap_or(""),
                "ai_model": history_field(entry, 4).unwrap_or(""),
                "ai_status": history_field(entry, 5).unwrap_or("not_recorded"),
                "ai_duration_ms": history_field(entry, 6).and_then(|value| value.parse::<u128>().ok()).unwrap_or(0),
                "source": history_field(entry, 7).unwrap_or("dictation"),
                "audio_path": history_field(entry, 8).unwrap_or(""),
            })
        })
        .collect::<Vec<_>>();
    let contents = if format.eq_ignore_ascii_case("csv") {
        let mut output =
            "timestamp,text,raw_text,ai_provider,ai_model,ai_status,ai_duration_ms,source,audio_path\n"
                .to_owned();
        for record in &records {
            let fields = [
                record["timestamp"].to_string(),
                record["text"].as_str().unwrap_or_default().to_owned(),
                record["raw_text"].as_str().unwrap_or_default().to_owned(),
                record["ai_provider"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                record["ai_model"].as_str().unwrap_or_default().to_owned(),
                record["ai_status"].as_str().unwrap_or_default().to_owned(),
                record["ai_duration_ms"].to_string(),
                record["source"].as_str().unwrap_or_default().to_owned(),
                record["audio_path"].as_str().unwrap_or_default().to_owned(),
            ];
            output.push_str(
                &fields
                    .map(|field| format!("\"{}\"", field.replace('"', "\"\"")))
                    .join(","),
            );
            output.push('\n');
        }
        output
    } else {
        serde_json::to_string_pretty(&records).map_err(|error| error.to_string())?
    };
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn decode_file_url(value: &str) -> String {
    let value = value.strip_prefix("file://").unwrap_or(value);
    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(decoded) = u8::from_str_radix(&value[index + 1..index + 3], 16) {
                result.push(decoded);
                index += 3;
                continue;
            }
        }
        result.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MeetingSegment {
    start_milliseconds: u64,
    end_milliseconds: u64,
    speaker: Option<String>,
    text: String,
}

struct MeetingTranscript {
    text: String,
    segments: Vec<MeetingSegment>,
}

fn transcribe_long_audio_file(
    path: &PathBuf,
    model: &PathBuf,
    language: String,
    use_gpu: bool,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32, usize, usize),
) -> Result<MeetingTranscript, String> {
    const CHUNK_SAMPLES: usize = 16_000 * 30;
    let audio = decode_audio_file(path)?;
    let config = TranscriptionConfig::default()
        .with_language(Some(language))
        .with_gpu(use_gpu);
    let transcriber = WhisperTranscriber::load(model, config).map_err(|error| error.to_string())?;
    let total = audio.samples().len().div_ceil(CHUNK_SAMPLES);
    let mut segments = Vec::new();
    for (index, samples) in audio.samples().chunks(CHUNK_SAMPLES).enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err("Meeting transcription cancelled".to_owned());
        }
        let chunk = AudioBuffer::new(samples.to_vec(), 16_000, 1, false)
            .map_err(|error| error.to_string())?
            .to_asr_mono();
        let transcript = transcriber
            .transcribe(&chunk)
            .map_err(|error| error.to_string())?;
        let chunk_offset_ms = u64::try_from(index)
            .unwrap_or(u64::MAX)
            .saturating_mul(30_000);
        for segment in transcript.segments {
            let start = u64::try_from(segment.start_centiseconds.max(0)).unwrap_or_default() * 10;
            let end = u64::try_from(segment.end_centiseconds.max(0)).unwrap_or_default() * 10;
            segments.push(MeetingSegment {
                start_milliseconds: chunk_offset_ms.saturating_add(start),
                end_milliseconds: chunk_offset_ms.saturating_add(end),
                speaker: None,
                text: segment.text,
            });
        }
        let completed = index + 1;
        progress(completed as f32 / total.max(1) as f32, completed, total);
    }
    let text = segments
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    if text.trim().is_empty() {
        return Err("No speech was recognized in this file.".to_owned());
    }
    Ok(MeetingTranscript { text, segments })
}

fn meeting_segment_qstring(segment: &MeetingSegment) -> QString {
    QString::from(&format!(
        "{}\t{}\t{}\t{}",
        segment.start_milliseconds,
        segment.end_milliseconds,
        segment.speaker.as_deref().unwrap_or(""),
        history_value(&segment.text)
    ))
}

fn timestamp_srt(milliseconds: u64, decimal: char) -> String {
    let hours = milliseconds / 3_600_000;
    let minutes = (milliseconds / 60_000) % 60;
    let seconds = (milliseconds / 1_000) % 60;
    let millis = milliseconds % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}{decimal}{millis:03}")
}

fn write_meeting_export(
    path: &PathBuf,
    format: &str,
    segments: &[MeetingSegment],
) -> Result<(), String> {
    if segments.is_empty() {
        return Err("No timestamped meeting transcript is available".to_owned());
    }
    let format = format.to_ascii_lowercase();
    let contents = match format.as_str() {
        "json" => serde_json::to_string_pretty(
            &segments
                .iter()
                .map(|segment| {
                    serde_json::json!({
                        "start_ms": segment.start_milliseconds,
                        "end_ms": segment.end_milliseconds,
                        "speaker": segment.speaker,
                        "text": segment.text,
                    })
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|error| error.to_string())?,
        "srt" => segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                format!(
                    "{}\n{} --> {}\n{}{}\n",
                    index + 1,
                    timestamp_srt(segment.start_milliseconds, ','),
                    timestamp_srt(segment.end_milliseconds, ','),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "vtt" => format!(
            "WEBVTT\n\n{}",
            segments
                .iter()
                .map(|segment| format!(
                    "{} --> {}\n{}{}\n",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    timestamp_srt(segment.end_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        "md" | "markdown" => segments
            .iter()
            .map(|segment| {
                format!(
                    "- **{}**{} {}",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!(" · {speaker}:")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "txt" => segments
            .iter()
            .map(|segment| {
                format!(
                    "[{}] {}{}",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        _ => return Err(format!("Unsupported meeting export format: {format}")),
    };
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn decode_audio_file(path: &PathBuf) -> Result<fluidvoice_audio::MonoAudioBuffer, String> {
    const MAX_DECODED_BYTES: u64 = 16_000 * 4 * 60 * 60 * 2;
    let mut command = Command::new("ffmpeg");
    command
        .args(["-nostdin", "-v", "error", "-i"])
        .arg(path)
        .args(["-vn", "-f", "f32le", "-ac", "1", "-ar", "16000", "pipe:1"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    match command.spawn() {
        Ok(mut child) => {
            let mut bytes = Vec::new();
            child
                .stdout
                .take()
                .ok_or_else(|| "FFmpeg stdout was unavailable".to_owned())?
                .take(MAX_DECODED_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            let output = child
                .wait_with_output()
                .map_err(|error| error.to_string())?;
            if !output.status.success() {
                return Err(format!(
                    "FFmpeg could not decode this audio file: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
            if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_DECODED_BYTES {
                return Err("Decoded audio exceeds the two-hour safety limit".to_owned());
            }
            if bytes.len() % 4 != 0 {
                return Err("FFmpeg returned incomplete audio samples".to_owned());
            }
            let samples = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect::<Vec<_>>();
            return AudioBuffer::new(samples, 16_000, 1, false)
                .map(|audio| audio.to_asr_mono())
                .map_err(|error| error.to_string());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("Could not start FFmpeg: {error}")),
    }

    let mut reader = hound::WavReader::open(path)
        .map_err(|error| format!("FFmpeg is not installed and WAV fallback failed: {error}"))?;
    let specification = reader.spec();
    if specification.sample_format != hound::SampleFormat::Int
        || specification.bits_per_sample != 16
    {
        return Err("FFmpeg is required for audio other than 16-bit PCM WAV.".to_owned());
    }
    let samples = reader
        .samples::<i16>()
        .map(|sample| sample.map(|value| f32::from(value) / f32::from(i16::MAX)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Could not decode WAV file: {error}"))?;
    let native = AudioBuffer::new(
        samples,
        specification.sample_rate,
        u32::from(specification.channels),
        false,
    )
    .map_err(|error| error.to_string())?;
    Ok(native.to_asr_mono())
}

fn supported_languages() -> &'static [(&'static str, &'static str)] {
    &[
        ("Automatic detection", ""),
        ("Afrikaans", "af"),
        ("Amharic", "am"),
        ("Arabic", "ar"),
        ("Assamese", "as"),
        ("Azerbaijani", "az"),
        ("Bashkir", "ba"),
        ("Belarusian", "be"),
        ("Bulgarian", "bg"),
        ("Bengali", "bn"),
        ("Tibetan", "bo"),
        ("Breton", "br"),
        ("Bosnian", "bs"),
        ("Catalan", "ca"),
        ("Czech", "cs"),
        ("Welsh", "cy"),
        ("Danish", "da"),
        ("German", "de"),
        ("Greek", "el"),
        ("English", "en"),
        ("Spanish", "es"),
        ("Estonian", "et"),
        ("Basque", "eu"),
        ("Persian", "fa"),
        ("Finnish", "fi"),
        ("Faroese", "fo"),
        ("French", "fr"),
        ("Galician", "gl"),
        ("Gujarati", "gu"),
        ("Hausa", "ha"),
        ("Hawaiian", "haw"),
        ("Hebrew", "iw"),
        ("Hindi", "hi"),
        ("Croatian", "hr"),
        ("Haitian Creole", "ht"),
        ("Hungarian", "hu"),
        ("Armenian", "hy"),
        ("Indonesian", "id"),
        ("Icelandic", "is"),
        ("Italian", "it"),
        ("Japanese", "ja"),
        ("Javanese", "jw"),
        ("Georgian", "ka"),
        ("Kazakh", "kk"),
        ("Khmer", "km"),
        ("Kannada", "kn"),
        ("Korean", "ko"),
        ("Latin", "la"),
        ("Luxembourgish", "lb"),
        ("Lingala", "ln"),
        ("Lao", "lo"),
        ("Lithuanian", "lt"),
        ("Latvian", "lv"),
        ("Malagasy", "mg"),
        ("Maori", "mi"),
        ("Macedonian", "mk"),
        ("Malayalam", "ml"),
        ("Mongolian", "mn"),
        ("Marathi", "mr"),
        ("Malay", "ms"),
        ("Maltese", "mt"),
        ("Myanmar", "my"),
        ("Nepali", "ne"),
        ("Dutch", "nl"),
        ("Norwegian Nynorsk", "nn"),
        ("Norwegian", "no"),
        ("Occitan", "oc"),
        ("Punjabi", "pa"),
        ("Polish", "pl"),
        ("Pashto", "ps"),
        ("Portuguese", "pt"),
        ("Romanian", "ro"),
        ("Russian", "ru"),
        ("Sanskrit", "sa"),
        ("Sindhi", "sd"),
        ("Sinhala", "si"),
        ("Slovak", "sk"),
        ("Slovenian", "sl"),
        ("Shona", "sn"),
        ("Somali", "so"),
        ("Albanian", "sq"),
        ("Serbian", "sr"),
        ("Sundanese", "su"),
        ("Swedish", "sv"),
        ("Swahili", "sw"),
        ("Tamil", "ta"),
        ("Telugu", "te"),
        ("Tajik", "tg"),
        ("Thai", "th"),
        ("Turkmen", "tk"),
        ("Tagalog", "tl"),
        ("Turkish", "tr"),
        ("Tatar", "tt"),
        ("Ukrainian", "uk"),
        ("Urdu", "ur"),
        ("Uzbek", "uz"),
        ("Vietnamese", "vi"),
        ("Yiddish", "yi"),
        ("Yoruba", "yo"),
        ("Chinese", "zh"),
    ]
}

#[derive(Clone, Copy)]
struct WhisperModel {
    display_name: &'static str,
    file_name: &'static str,
    expected_bytes: u64,
    description: &'static str,
}

fn whisper_model_catalog() -> &'static [WhisperModel] {
    &[
        WhisperModel {
            display_name: "Whisper Tiny",
            file_name: "ggml-tiny.bin",
            expected_bytes: 77_691_713,
            description: "75 MB · fastest · basic accuracy · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Base",
            file_name: "ggml-base.bin",
            expected_bytes: 147_951_465,
            description: "141 MB · fast · balanced for short dictation · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Small",
            file_name: "ggml-small.bin",
            expected_bytes: 487_601_967,
            description: "465 MB · recommended · good accuracy · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Medium",
            file_name: "ggml-medium.bin",
            expected_bytes: 1_533_763_059,
            description: "1.4 GB · slower · high accuracy · 6 GB+ RAM",
        },
        WhisperModel {
            display_name: "Whisper Large Turbo",
            file_name: "ggml-large-v3-turbo.bin",
            expected_bytes: 1_624_555_275,
            description: "1.5 GB · high accuracy · optimized decoding · 8 GB+ RAM",
        },
        WhisperModel {
            display_name: "Whisper Large",
            file_name: "ggml-large-v3.bin",
            expected_bytes: 3_095_033_483,
            description: "2.9 GB · slowest · highest accuracy · 10 GB+ RAM",
        },
    ]
}

fn managed_model_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(directory).join("fluidvoice/models");
    }
    std::env::var_os("HOME").map_or_else(
        || PathBuf::from(".local/share/fluidvoice/models"),
        |home| PathBuf::from(home).join(".local/share/fluidvoice/models"),
    )
}

fn model_search_directories() -> Vec<PathBuf> {
    vec![
        managed_model_directory(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/models"),
    ]
}

fn resolve_model_path(model: &WhisperModel) -> PathBuf {
    model_search_directories()
        .into_iter()
        .map(|directory| directory.join(model.file_name))
        .find(|path| model_file_valid(path, model))
        .unwrap_or_else(|| managed_model_directory().join(model.file_name))
}

fn model_ui_lists(paths: &[PathBuf]) -> (QStringList, QStringList) {
    let states = paths
        .iter()
        .zip(whisper_model_catalog())
        .map(|(path, model)| {
            QString::from(if model_file_valid(path, model) {
                "Downloaded"
            } else {
                "Not downloaded"
            })
        })
        .collect();
    let details = whisper_model_catalog()
        .iter()
        .map(|model| QString::from(model.description))
        .collect();
    (states, details)
}

fn model_file_valid(path: &PathBuf, model: &WhisperModel) -> bool {
    fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == model.expected_bytes)
}

fn download_whisper_model(
    model: WhisperModel,
    destination: &PathBuf,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32),
) -> Result<(), String> {
    let parent = destination
        .parent()
        .ok_or_else(|| "invalid model destination".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let partial = destination.with_extension("bin.part");
    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        model.file_name
    );
    let response = ureq::get(&url).call().map_err(|error| error.to_string())?;
    let mut reader = response.into_body().into_reader();
    let mut output = fs::File::create(&partial).map_err(|error| error.to_string())?;
    let mut buffer = vec![0_u8; 1024 * 256];
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
        downloaded = downloaded.saturating_add(u64::try_from(count).unwrap_or_default());
        progress((downloaded as f64 / model.expected_bytes as f64).clamp(0.0, 1.0) as f32);
    }
    output.sync_all().map_err(|error| error.to_string())?;
    drop(output);
    if downloaded != model.expected_bytes {
        fs::remove_file(&partial).ok();
        return Err(format!(
            "downloaded {downloaded} bytes; expected {}",
            model.expected_bytes
        ));
    }
    fs::rename(&partial, destination).map_err(|error| error.to_string())?;
    Ok(())
}

fn shortcut_triggers() -> &'static [(&'static str, &'static str)] {
    &[
        ("Ctrl  Alt  D", "CTRL+ALT+D"),
        ("Ctrl  Alt  Space", "CTRL+ALT+SPACE"),
        ("Meta  Alt  D", "META+ALT+D"),
        ("Meta  Shift  Space", "META+SHIFT+SPACE"),
    ]
}

fn selected_language_code(controller: &FluidVoiceControllerRust) -> String {
    usize::try_from(controller.selected_language)
        .ok()
        .and_then(|index| controller.language_codes.get(index))
        .cloned()
        .unwrap_or_else(|| "en".to_owned())
}

fn language_display_name(code: &str) -> Option<&'static str> {
    supported_languages()
        .iter()
        .find_map(|(name, candidate)| (*candidate == code).then_some(*name))
}

fn selected_model_path(controller: &FluidVoiceControllerRust) -> Option<PathBuf> {
    let index = usize::try_from(controller.selected_model).ok()?;
    let path = controller.model_paths.get(index)?;
    model_file_valid(path, whisper_model_catalog().get(index)?).then(|| path.clone())
}

fn selected_shortcut_trigger(controller: &FluidVoiceControllerRust) -> String {
    usize::try_from(controller.selected_shortcut)
        .ok()
        .and_then(|index| shortcut_triggers().get(index))
        .map_or("CTRL+ALT+D", |(_, trigger)| trigger)
        .to_owned()
}

fn valid_index(index: i32, length: usize) -> bool {
    usize::try_from(index).is_ok_and(|index| index < length)
}

fn dump_asr_audio(audio: &fluidvoice_audio::MonoAudioBuffer) -> Result<(), String> {
    let Some(path) = std::env::var_os("FLUIDVOICE_ASR_DUMP").map(PathBuf::from) else {
        return Ok(());
    };
    let specification = hound::WavSpec {
        channels: 1,
        sample_rate: audio.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, specification)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    for &sample in audio.samples() {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        writer
            .write_sample(value)
            .map_err(|error| error.to_string())?;
    }
    writer.finalize().map_err(|error| error.to_string())
}

fn meter_level(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return 0.0;
    }
    ((20.0 * peak.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

fn peak_db(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return -60.0;
    }
    (20.0 * peak.log10()).clamp(-60.0, 0.0)
}

fn asr_gain(peak: f32, user_gain: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.000_5 {
        return 1.0;
    }
    // Normalize ordinary speech to a conservative peak, then treat the UI gain
    // as an adjustment. Always retain headroom so a high setting cannot turn
    // the buffer sent to Whisper into a clipped square wave.
    let automatic = (0.35 / peak).clamp(1.0, 64.0);
    let requested = automatic * user_gain.max(0.0);
    let headroom_limit = 0.85 / peak;
    requested.clamp(1.0, headroom_limit.min(64.0))
}

fn suspicious_single_word(text: &str, duration: Duration) -> bool {
    if duration < Duration::from_secs(2) {
        return false;
    }
    let normalized = text
        .trim()
        .trim_matches(|character: char| !character.is_alphanumeric())
        .to_ascii_lowercase();
    matches!(normalized.as_str(), "you" | "thanks" | "thank you")
}

#[cfg(test)]
mod tests {
    use std::{fs, time::Duration};

    use super::{
        AiProfile, DesktopAction, DictionaryEntry, MeetingSegment, asr_gain, decode_audio_file,
        decode_file_url, history_clipboard_text, meter_level, parse_csv_record,
        parse_desktop_action, peak_db, process_transcript, profile_matches_application,
        read_dictionary_import, supported_languages, suspicious_single_word, timestamp_srt,
        valid_ollama_model_name, version_tuple, whisper_model_catalog, write_dictionary_csv,
        write_history_export, write_meeting_export,
    };

    #[test]
    fn maps_audio_peak_to_logarithmic_meter() {
        assert_eq!(meter_level(0.0), 0.0);
        assert!((meter_level(0.01) - 1.0 / 3.0).abs() < 0.001);
        assert_eq!(meter_level(1.0), 1.0);
        assert_eq!(meter_level(f32::NAN), 0.0);
    }

    #[test]
    fn matches_application_profiles_by_class_or_title_fragments() {
        let profile = AiProfile {
            name: "Development".into(),
            prompt: "Keep code identifiers exact".into(),
            application_match: "org.kde.konsole, visual studio code".into(),
        };
        assert!(profile_matches_application(
            &profile,
            "org.kde.konsole\nproject shell"
        ));
        assert!(profile_matches_application(
            &profile,
            "code\nvisual studio code"
        ));
        assert!(!profile_matches_application(
            &profile,
            "org.mozilla.firefox\nnews"
        ));
    }

    #[test]
    fn accepts_only_allowlisted_desktop_actions() {
        assert!(matches!(
            parse_desktop_action("open terminal"),
            Some(DesktopAction::OpenTerminal)
        ));
        assert!(matches!(
            parse_desktop_action("lock my screen"),
            Some(DesktopAction::LockScreen)
        ));
        assert!(parse_desktop_action("rm -rf important files").is_none());
        assert!(parse_desktop_action("run echo hello").is_none());
    }

    #[test]
    fn parses_release_versions_for_update_comparison() {
        assert_eq!(version_tuple("0.3.0"), Some((0, 3, 0)));
        assert_eq!(version_tuple("1.2.3-beta.1"), Some((1, 2, 3)));
        assert_eq!(version_tuple("invalid"), None);
    }

    #[test]
    fn validates_ollama_model_names_before_spawning_the_cli() {
        assert!(valid_ollama_model_name("qwen2.5:7b"));
        assert!(valid_ollama_model_name(
            "registry.example/team/model:latest"
        ));
        assert!(!valid_ollama_model_name(""));
        assert!(!valid_ollama_model_name("model; touch /tmp/nope"));
    }

    #[test]
    fn prepares_raw_final_and_combined_history_clipboard_text() {
        let entry = "100\tFinal sentence.\traw sentence\tollama\tqwen\tenhanced\t42\tdictation";
        assert_eq!(history_clipboard_text(entry, 0).0, "raw sentence");
        assert_eq!(history_clipboard_text(entry, 1).0, "Final sentence.");
        assert_eq!(
            history_clipboard_text(entry, 2).0,
            "Raw transcript:\nraw sentence\n\nFinal text:\nFinal sentence."
        );
        assert_eq!(history_clipboard_text(entry, 3).0, "raw sentence");
    }

    #[test]
    fn decodes_audio_files_to_mono_16khz() {
        let path = std::env::temp_dir().join(format!(
            "fluidvoice-audio-decode-{}.wav",
            std::process::id()
        ));
        let specification = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&path, specification).expect("create test WAV");
        for sample in [0_i16, 1000, -1000, 0] {
            writer.write_sample(sample).expect("write test sample");
        }
        writer.finalize().expect("finalize test WAV");
        let audio = decode_audio_file(&path).expect("decode test audio");
        assert_eq!(audio.sample_rate(), 16_000);
        assert_eq!(audio.samples().len(), 4);
        fs::remove_file(path).expect("remove test WAV");
    }

    #[test]
    fn exports_enriched_and_legacy_history_as_json() {
        let path = std::env::temp_dir().join(format!(
            "fluidvoice-history-export-{}.json",
            std::process::id()
        ));
        let history = [
            "100\tfinal\traw\tollama\tqwen\tenhanced\t42\tdictation".to_owned(),
            "50\tlegacy text".to_owned(),
        ];
        write_history_export(&path, "json", &history).expect("write history export");
        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("read history export"))
                .expect("parse history export");
        assert_eq!(value[0]["raw_text"], "raw");
        assert_eq!(value[0]["ai_provider"], "ollama");
        assert_eq!(value[1]["raw_text"], "legacy text");
        fs::remove_file(path).expect("remove test export");
    }

    #[test]
    fn reports_bounded_decibels() {
        assert_eq!(peak_db(0.0), -60.0);
        assert!((peak_db(0.01) + 40.0).abs() < 0.001);
        assert_eq!(peak_db(1.0), 0.0);
    }

    #[test]
    fn normalizes_quiet_asr_audio_conservatively() {
        assert_eq!(asr_gain(0.0, 1.0), 1.0);
        assert_eq!(asr_gain(0.01, 1.0), 35.0);
        assert_eq!(asr_gain(0.01, 16.0), 64.0);
        assert!((asr_gain(0.1, 1.0) - 3.5).abs() < f32::EPSILON);
        assert_eq!(asr_gain(0.8, 1.0), 1.0);
    }

    #[test]
    fn detects_long_single_word_whisper_hallucinations() {
        assert!(suspicious_single_word(" you ", Duration::from_secs(6)));
        assert!(suspicious_single_word("Thank you.", Duration::from_secs(3)));
        assert!(!suspicious_single_word("you", Duration::from_millis(900)));
        assert!(!suspicious_single_word(
            "you are welcome",
            Duration::from_secs(6)
        ));
    }

    #[test]
    fn exposes_complete_whisper_language_and_model_catalogs() {
        assert_eq!(supported_languages().len(), 100); // Auto + Whisper's 99 languages.
        assert_eq!(whisper_model_catalog().len(), 6);
        assert!(
            whisper_model_catalog()
                .iter()
                .all(|model| model.expected_bytes > 70_000_000)
        );
    }

    #[test]
    fn applies_commands_and_preferred_spellings() {
        let dictionary = vec![
            "FluidVoice".to_owned(),
            "KDE".to_owned(),
            "fluid voice\tFluidVoice Linux".to_owned(),
        ];
        assert_eq!(
            process_transcript(
                "fluidvoice comma new line kde question mark",
                true,
                &dictionary
            ),
            "FluidVoice,\nKDE?"
        );
        assert_eq!(
            process_transcript("fluidvoice comma", false, &dictionary),
            "FluidVoice comma"
        );
        assert_eq!(
            process_transcript("I use fluid voice daily", false, &dictionary),
            "I use FluidVoice Linux daily"
        );
    }

    #[test]
    fn imports_and_exports_quoted_dictionary_csv() {
        assert_eq!(
            parse_csv_record("\"fluid, voice\",\"FluidVoice \"\"Linux\"\"\""),
            ["fluid, voice", "FluidVoice \"Linux\""]
        );
        let input = std::env::temp_dir().join(format!(
            "fluidvoice-dictionary-import-{}.csv",
            std::process::id()
        ));
        let output = input.with_extension("export.csv");
        fs::write(
            &input,
            "spoken,preferred\nfluid voice,FluidVoice Linux\nk d e,KDE\n",
        )
        .expect("write dictionary import");
        let entries = read_dictionary_import(&input).expect("read dictionary import");
        assert_eq!(
            entries,
            [
                DictionaryEntry {
                    spoken: "fluid voice".into(),
                    preferred: "FluidVoice Linux".into()
                },
                DictionaryEntry {
                    spoken: "k d e".into(),
                    preferred: "KDE".into()
                }
            ]
        );
        write_dictionary_csv(&output, &entries).expect("write dictionary export");
        assert!(
            fs::read_to_string(&output)
                .expect("read dictionary export")
                .contains("\"fluid voice\",\"FluidVoice Linux\"")
        );
        fs::remove_file(input).expect("remove import");
        fs::remove_file(output).expect("remove export");
    }

    #[test]
    fn decodes_local_file_urls() {
        assert_eq!(
            decode_file_url("file:///tmp/Voice%20Sample.wav"),
            "/tmp/Voice Sample.wav"
        );
    }

    #[test]
    fn exports_diarization_ready_meeting_formats() {
        let segments = [MeetingSegment {
            start_milliseconds: 1_234,
            end_milliseconds: 65_678,
            speaker: None,
            text: "Hello meeting".into(),
        }];
        assert_eq!(timestamp_srt(3_661_007, ','), "01:01:01,007");
        for format in ["txt", "md", "srt", "vtt", "json"] {
            let path = std::env::temp_dir().join(format!(
                "fluidvoice-meeting-export-{}.{format}",
                std::process::id()
            ));
            write_meeting_export(&path, format, &segments).expect("write meeting export");
            let contents = fs::read_to_string(&path).expect("read meeting export");
            assert!(contents.contains("Hello meeting"));
            if format == "json" {
                let value: serde_json::Value =
                    serde_json::from_str(&contents).expect("parse meeting JSON");
                assert!(value[0]["speaker"].is_null());
                assert_eq!(value[0]["start_ms"], 1_234);
            }
            fs::remove_file(path).expect("remove meeting export");
        }
    }
}
