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
        #[qproperty(bool, overlay_keep_result, cxx_name = "overlayKeepResult")]
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
        #[qproperty(bool, assistant_busy, cxx_name = "assistantBusy")]
        #[qproperty(bool, update_busy, cxx_name = "updateBusy")]
        #[qproperty(bool, export_busy, cxx_name = "exportBusy")]
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
        #[qproperty(QStringList, meeting_speakers, cxx_name = "meetingSpeakers")]
        #[qproperty(QString, last_meeting_file, cxx_name = "lastMeetingFile")]
        #[qproperty(QStringList, compute_backends, cxx_name = "computeBackends")]
        #[qproperty(i32, selected_compute_backend, cxx_name = "selectedComputeBackend")]
        #[qproperty(QString, compute_status, cxx_name = "computeStatus")]
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
        #[qproperty(bool, write_mode_retry_available, cxx_name = "writeModeRetryAvailable")]
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
        #[qproperty(bool, local_api_enabled, cxx_name = "localApiEnabled")]
        #[qproperty(i32, local_api_port, cxx_name = "localApiPort")]
        #[qproperty(QString, local_api_status, cxx_name = "localApiStatus")]
        #[qproperty(QStringList, speech_engines, cxx_name = "speechEngines")]
        #[qproperty(i32, selected_speech_engine, cxx_name = "selectedSpeechEngine")]
        #[qproperty(QString, local_speech_url, cxx_name = "localSpeechUrl")]
        #[qproperty(QString, parakeet_status, cxx_name = "parakeetStatus")]
        #[qproperty(QString, native_model_detail, cxx_name = "nativeModelDetail")]
        #[qproperty(
            bool,
            parakeet_runtime_installed,
            cxx_name = "parakeetRuntimeInstalled"
        )]
        #[qproperty(bool, parakeet_model_installed, cxx_name = "parakeetModelInstalled")]
        #[qproperty(bool, parakeet_busy, cxx_name = "parakeetBusy")]
        #[qproperty(f32, parakeet_download_progress, cxx_name = "parakeetDownloadProgress")]
        #[qproperty(bool, diarization_enabled, cxx_name = "diarizationEnabled")]
        #[qproperty(bool, sortformer_installed, cxx_name = "sortformerInstalled")]
        #[qproperty(bool, sortformer_busy, cxx_name = "sortformerBusy")]
        #[qproperty(
            f32,
            sortformer_download_progress,
            cxx_name = "sortformerDownloadProgress"
        )]
        #[qproperty(QString, sortformer_status, cxx_name = "sortformerStatus")]
        #[qproperty(bool, onboarding_completed, cxx_name = "onboardingCompleted")]
        type SpevoxController = super::SpevoxControllerRust;

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
        #[cxx_name = "updateOverlayKeepResult"]
        fn update_overlay_keep_result(self: Pin<&mut Self>, enabled: bool);

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
        #[cxx_name = "renameMeetingSpeaker"]
        fn rename_meeting_speaker(self: Pin<&mut Self>, current: &QString, replacement: &QString);

        #[qinvokable]
        #[cxx_name = "retryMeetingTranscription"]
        fn retry_meeting_transcription(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "completeOnboarding"]
        fn complete_onboarding(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "resetOnboarding"]
        fn reset_onboarding(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectComputeBackend"]
        fn select_compute_backend(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "diagnoseComputeBackend"]
        fn diagnose_compute_backend(self: Pin<&mut Self>);

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

        #[qinvokable]
        #[cxx_name = "updateLocalApi"]
        fn update_local_api(self: Pin<&mut Self>, enabled: bool, port: i32);

        #[qinvokable]
        #[cxx_name = "rotateLocalApiToken"]
        fn rotate_local_api_token(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "showLocalApiTokenLocation"]
        fn show_local_api_token_location(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectSpeechEngine"]
        fn select_speech_engine(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "updateLocalSpeechUrl"]
        fn update_local_speech_url(self: Pin<&mut Self>, url: &QString);

        #[qinvokable]
        #[cxx_name = "installParakeetRuntime"]
        fn install_parakeet_runtime(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "downloadParakeetModel"]
        fn download_parakeet_model(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "cancelParakeetDownload"]
        fn cancel_parakeet_download(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "deleteParakeetModel"]
        fn delete_parakeet_model(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "diagnoseParakeet"]
        fn diagnose_parakeet(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "updateDiarizationEnabled"]
        fn update_diarization_enabled(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "setupSortformer"]
        fn setup_sortformer(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "cancelSortformerDownload"]
        fn cancel_sortformer_download(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "deleteSortformer"]
        fn delete_sortformer(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "diagnoseSortformer"]
        fn diagnose_sortformer(self: Pin<&mut Self>);
    }

    impl cxx_qt::Threading for SpevoxController {}
}

use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    pin::Pin,
    process::{Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use cxx_qt::{CxxQtThread, CxxQtType, Threading};
use cxx_qt_lib::{QString, QStringList};
use sha2::Sha256;
use spevox_audio::{AudioBuffer, AudioDevice, CaptureStopToken, PipeWireCapture};
use spevox_delivery::ClipboardDelivery;
use spevox_portal::{
    ActiveApplication, GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutEvent,
    TextInputSession, run_profile_bridge,
};
use spevox_transcription::{LocalSpeechServer, TranscriptionConfig, WhisperTranscriber};
use tokio::sync::mpsc;

use crate::ai::{self, AiConfig, ProviderId};
use crate::local_api::{self, LocalApiAction};
use crate::parakeet::{self, Backend as ParakeetBackend};
use crate::updates::check_latest_release;

static HISTORY_IO_LOCK: Mutex<()> = Mutex::new(());

// This is the CXX-Qt backing object: boolean fields mirror independent QML
// properties and are intentionally not collapsed into opaque bit flags.
#[allow(clippy::struct_excessive_bools)]
pub struct SpevoxControllerRust {
    status_text: QString,
    text_delivery_status: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
    overlay_enabled: bool,
    overlay_keep_result: bool,
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
    assistant_busy: bool,
    update_busy: bool,
    export_busy: bool,
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
    meeting_speakers: QStringList,
    last_meeting_file: QString,
    last_meeting_source: QString,
    meeting_results: Vec<MeetingSegment>,
    meeting_cancel: Option<Arc<AtomicBool>>,
    compute_backends: QStringList,
    selected_compute_backend: i32,
    compute_status: QString,
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
    write_mode_retry_available: bool,
    ai_key_configured: bool,
    ai_api_key: String,
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
    last_write_job: Option<WriteModeJob>,
    app_version: QString,
    update_status: QString,
    local_api_enabled: bool,
    local_api_port: i32,
    local_api_status: QString,
    speech_engines: QStringList,
    selected_speech_engine: i32,
    local_speech_url: QString,
    parakeet_status: QString,
    native_model_detail: QString,
    parakeet_runtime_installed: bool,
    parakeet_model_installed: bool,
    parakeet_busy: bool,
    parakeet_download_progress: f32,
    parakeet_download_cancel: Option<Arc<AtomicBool>>,
    parakeet_supervisor: Arc<Mutex<parakeet::Supervisor>>,
    diarization_enabled: bool,
    sortformer_installed: bool,
    sortformer_busy: bool,
    sortformer_download_progress: f32,
    sortformer_status: QString,
    sortformer_download_cancel: Option<Arc<AtomicBool>>,
    onboarding_completed: bool,
}

struct StartupSnapshot {
    dictionary: Vec<String>,
    history: Vec<String>,
    command_history: Vec<String>,
    lifetime_stats: LifetimeStats,
    ai_profiles: Vec<AiProfile>,
    audio_history_status: String,
}

fn load_startup_snapshot() -> StartupSnapshot {
    clear_missing_audio_history_references().ok();
    let history = load_lines(&history_path());
    StartupSnapshot {
        dictionary: load_lines(&dictionary_path()),
        command_history: load_lines(&command_history_path()),
        lifetime_stats: LifetimeStats::load_or_migrate(&history),
        ai_profiles: load_ai_profiles(),
        audio_history_status: audio_history_summary(),
        history,
    }
}

fn apply_startup_snapshot(
    mut controller: Pin<&mut ffi::SpevoxController>,
    snapshot: StartupSnapshot,
) {
    controller.as_mut().set_dictionary_terms(
        snapshot
            .dictionary
            .iter()
            .map(|line| QString::from(&dictionary_display(line)))
            .collect(),
    );
    controller.as_mut().set_history_entries(
        snapshot
            .history
            .iter()
            .rev()
            .take(HISTORY_VISIBLE_LIMIT)
            .map(QString::from)
            .collect(),
    );
    controller.as_mut().set_command_history(
        snapshot
            .command_history
            .iter()
            .rev()
            .map(QString::from)
            .collect(),
    );
    controller
        .as_mut()
        .set_transcript_count(snapshot.lifetime_stats.transcript_count_i32());
    controller
        .as_mut()
        .set_dictated_word_count(snapshot.lifetime_stats.dictated_word_count_i32());
    controller
        .as_mut()
        .set_audio_history_status(QString::from(&snapshot.audio_history_status));
    controller.as_mut().set_ai_profile_names(
        std::iter::once(QString::from("Default"))
            .chain(
                snapshot
                    .ai_profiles
                    .iter()
                    .map(|profile| QString::from(&profile.name)),
            )
            .collect(),
    );
    controller.as_mut().rust_mut().get_mut().ai_profiles = snapshot.ai_profiles;
}

impl Default for SpevoxControllerRust {
    // Constructs the complete Q_PROPERTY snapshot atomically for QML.
    #[allow(clippy::too_many_lines)]
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
        let model_states = model_paths
            .iter()
            .map(|_| QString::from("Checking…"))
            .collect();
        let model_details = model_paths
            .iter()
            .map(|_| QString::from("Inspecting local model…"))
            .collect();
        let selected_ai_provider = if preferences.ai_local_only {
            let saved = preferences.ai_provider.clamp(0, 9);
            if ai_provider(saved).id.is_local() {
                saved
            } else {
                ProviderId::Ollama.preference_index()
            }
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
        if provider.id.is_local()
            && !AiConfig::new(provider.id, "", &ai_base_url)
                .with_enabled(false)
                .is_local()
        {
            provider.default_url.clone_into(&mut ai_base_url);
        }
        // Keyring access may display an unlock prompt, so defer it until after
        // QML construction instead of blocking the first frame.
        let ai_api_key = String::new();
        let ai_key_configured = provider.id.is_local();
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
            overlay_keep_result: preferences.overlay_keep_result,
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
            assistant_busy: false,
            update_busy: false,
            export_busy: false,
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
            dictionary_terms: QStringList::default(),
            history_entries: QStringList::default(),
            audio_history_enabled: preferences.audio_history_enabled,
            audio_history_budget_mb: preferences.audio_history_budget_mb,
            audio_history_status: QString::from("Inspecting retained recordings…"),
            transcript_count: 0,
            dictated_word_count: 0,
            typing_wpm: preferences.typing_wpm,
            skip_weekends: preferences.skip_weekends,
            command_mode_enabled: preferences.command_mode_enabled,
            command_output: QString::from(
                "Ask a question or request an allowlisted desktop action.",
            ),
            pending_command: QString::default(),
            pending_desktop_action: None,
            command_history: QStringList::default(),
            file_transcription_status: QString::from("Choose a WAV file to transcribe locally."),
            meeting_progress: 0.0,
            meeting_segments: QStringList::default(),
            meeting_speakers: QStringList::default(),
            last_meeting_file: QString::default(),
            last_meeting_source: QString::default(),
            meeting_results: Vec::new(),
            meeting_cancel: None,
            compute_backends: ["Automatic (Vulkan → CPU)", "Vulkan only", "CPU only"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_compute_backend: preferences.compute_backend.clamp(0, 2),
            compute_status: QString::from(compute_backend_summary(preferences.compute_backend)),
            theme_options: ["System", "Spevox Dark", "Spevox Light"]
                .into_iter()
                .map(QString::from)
                .collect(),
            selected_theme: preferences.theme.clamp(0, 2),
            accent_options: ["KDE system accent", "Spevox Cyan", "Green", "Purple"]
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
            write_mode_retry_available: false,
            ai_key_configured,
            ai_api_key,
            ai_local_models: QStringList::default(),
            ai_local_endpoint: AiConfig::new(provider.id, ai_model.clone(), ai_base_url.clone())
                .with_enabled(false)
                .is_local(),
            ai_local_only: preferences.ai_local_only,
            ollama_status: QString::from("Run the setup check to inspect Ollama."),
            ollama_installed: false,
            ollama_busy: false,
            ai_profile_names: [QString::from("Default")].into_iter().collect(),
            selected_ai_profile: 0,
            ai_profile_prompt: QString::default(),
            ai_profile_name: QString::default(),
            ai_profile_match: QString::default(),
            auto_profiles_enabled: preferences.auto_profiles_enabled,
            active_application: QString::from("KWin bridge has not reported an application."),
            ai_profiles: Vec::new(),
            last_write_job: None,
            app_version: QString::from(env!("CARGO_PKG_VERSION")),
            update_status: QString::from("Updates have not been checked."),
            local_api_enabled: preferences.local_api_enabled,
            local_api_port: preferences.local_api_port,
            local_api_status: QString::from(if preferences.local_api_enabled {
                format!("Starting on 127.0.0.1:{}…", preferences.local_api_port)
            } else {
                "Off · no local port is open".to_owned()
            }),
            speech_engines: [
                "Built-in Whisper",
                "Parakeet TDT v3 (beta)",
                "Nemotron 3.5 Multilingual (beta)",
                "Nemotron Streaming English (beta)",
                "Parakeet CTC 1.1B (beta)",
                "Custom local speech server",
            ]
            .into_iter()
            .map(QString::from)
            .collect(),
            selected_speech_engine: preferences.speech_engine.clamp(0, 5),
            local_speech_url: QString::from(&preferences.local_speech_url),
            parakeet_status: QString::from(
                "Run setup to install the native Parakeet runtime and model.",
            ),
            native_model_detail: QString::from(
                native_model_for_engine(preferences.speech_engine)
                    .map_or("Select a native speech engine.", |model| model.detail),
            ),
            parakeet_runtime_installed: parakeet_runtime_available(preferences.compute_backend),
            parakeet_model_installed: native_model_for_engine(preferences.speech_engine)
                .is_some_and(parakeet::model_installed),
            parakeet_busy: false,
            parakeet_download_progress: 0.0,
            parakeet_download_cancel: None,
            parakeet_supervisor: Arc::new(Mutex::new(parakeet::Supervisor::new())),
            diarization_enabled: preferences.diarization_enabled,
            sortformer_installed: parakeet::model_installed(parakeet::SORTFORMER_V2),
            sortformer_busy: false,
            sortformer_download_progress: 0.0,
            sortformer_status: QString::from(
                if parakeet::model_installed(parakeet::SORTFORMER_V2) {
                    "Downloaded experimental Sortformer model is ready."
                } else {
                    "Optional experimental setup for speaker-labelled meeting transcripts."
                },
            ),
            sortformer_download_cancel: None,
            onboarding_completed: preferences.onboarding_completed,
        }
    }
}

impl ffi::SpevoxController {
    pub fn complete_onboarding(mut self: Pin<&mut Self>) {
        let mut preferences = Preferences::load();
        preferences.onboarding_completed = true;
        if preferences.save().is_ok() {
            self.as_mut().set_onboarding_completed(true);
            self.as_mut()
                .set_status_text(QString::from("Setup complete · Spevox is ready"));
        }
    }

    pub fn reset_onboarding(mut self: Pin<&mut Self>) {
        let mut preferences = Preferences::load();
        preferences.onboarding_completed = false;
        if preferences.save().is_ok() {
            self.as_mut().set_onboarding_completed(false);
        }
    }

    // Owns one Tokio select loop whose branches must share the same portal
    // bindings and channels; splitting it would obscure their shutdown rules.
    #[allow(clippy::too_many_lines)]
    pub fn initialize_desktop_runtime(mut self: Pin<&mut Self>) {
        if self.as_ref().rust().desktop_sender.is_some() {
            return;
        }
        let (desktop_sender, mut desktop_receiver) = mpsc::unbounded_channel();
        self.as_mut().rust_mut().get_mut().desktop_sender = Some(desktop_sender);
        let qt_thread = self.qt_thread();
        let model_thread = qt_thread.clone();
        std::thread::spawn(move || {
            verify_unmarked_whisper_models();
            let snapshot = load_startup_snapshot();
            model_thread
                .queue(move |mut controller| {
                    controller.as_mut().refresh_model_catalog();
                    apply_startup_snapshot(controller.as_mut(), snapshot);
                })
                .ok();
        });
        let key_provider = ai_provider(*self.as_ref().selected_ai_provider());
        if !key_provider.id.is_local() {
            let key_thread = qt_thread.clone();
            std::thread::spawn(move || {
                let api_key = ai::load_api_key(key_provider.id.as_str());
                key_thread
                    .queue(move |mut controller| {
                        let configured = !api_key.is_empty();
                        controller.as_mut().rust_mut().get_mut().ai_api_key = api_key;
                        controller.as_mut().set_ai_key_configured(configured);
                    })
                    .ok();
            });
        }
        let api_preferences = Preferences::load();
        if api_preferences.local_api_enabled {
            let (api_actions, api_events) = std::sync::mpsc::channel();
            match u16::try_from(api_preferences.local_api_port)
                .map_err(|_| "port is outside the valid range".to_owned())
                .and_then(|port| local_api::start(port, api_actions))
            {
                Ok(()) => {
                    self.as_mut().set_local_api_status(QString::from(&format!(
                        "Listening securely on 127.0.0.1:{}",
                        api_preferences.local_api_port
                    )));
                    let api_qt_thread = self.qt_thread();
                    std::thread::spawn(move || {
                        while let Ok(action) = api_events.recv() {
                            match action {
                                LocalApiAction::ToggleDictation => {
                                    api_qt_thread
                                        .queue(|mut controller| {
                                            controller.as_mut().toggle_recording();
                                        })
                                        .ok();
                                }
                            }
                        }
                    });
                }
                Err(error) => self.as_mut().set_local_api_status(QString::from(&format!(
                    "Local API unavailable: {error}"
                ))),
            }
        }
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
                let mut profile_events_open = true;
                loop {
                    let config = match GlobalShortcutConfig::new(
                        "dictate_hold",
                        "Hold to dictate with Spevox",
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
                        "Open Spevox Write Mode",
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
                                    )));
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
                    let ready_status = format!("Ready · hold {requested_shortcut} to dictate");
                    qt_thread
                        .queue(move |controller| {
                            controller.set_status_text(QString::from(&ready_status));
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
                            application = profile_events.recv(), if profile_events_open => match application {
                                Some(application) => {
                                    qt_thread.queue(move |mut controller| {
                                        controller.as_mut().apply_active_application(&application);
                                    }).ok();
                                }
                                None => profile_events_open = false,
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
                "Desktop integration is not running. Restart Spevox and try again.",
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
            let mut last_progress = Instant::now()
                .checked_sub(Duration::from_secs(1))
                .unwrap_or_else(Instant::now);
            let result = download_whisper_model(model, &destination, &cancel, move |progress| {
                if progress < 1.0 && last_progress.elapsed() < Duration::from_millis(50) {
                    return;
                }
                last_progress = Instant::now();
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
            match fs::remove_file(&path) {
                Ok(()) => {
                    fs::remove_file(path.with_extension("bin.sha256")).ok();
                    self.as_mut()
                        .set_status_text(QString::from("Downloaded model deleted"));
                }
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

    pub fn update_overlay_keep_result(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_overlay_keep_result(enabled);
        if !enabled && *self.as_ref().overlay_result_available() {
            self.as_mut().set_overlay_visible(false);
        }
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
        let copied = copy_to_clipboard(&mut rust.clipboard, &text).is_ok();
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
        let copied = copy_to_clipboard(&mut rust.clipboard, &raw).is_ok();
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
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_status_text(QString::from(
                "AI enhancement is off · enable it before retrying",
            ));
            return;
        }
        let raw = self.as_ref().last_raw_text().to_string();
        if raw.trim().is_empty() {
            self.as_mut()
                .set_status_text(QString::from("No raw transcript is available to retry"));
            return;
        }
        let config = self.as_ref().rust().ai_config();
        let qt_thread = self.qt_thread();
        self.as_mut().set_transcribing(true);
        if *self.as_ref().overlay_enabled() {
            self.as_mut().set_overlay_visible(true);
        }
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
                            let copied = copy_to_clipboard(&mut rust.clipboard, &text).is_ok();
                            if copied
                                && let Some(sender) =
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
                            controller
                                .as_mut()
                                .set_status_text(QString::from(if copied {
                                    "AI enhancement retried · result pasted or copied"
                                } else {
                                    "AI retry succeeded · clipboard delivery failed"
                                }));
                        }
                        Err(error) => controller.as_mut().set_status_text(QString::from(&format!(
                            "AI retry failed · raw text remains available · {error}"
                        ))),
                    }
                    if !*controller.as_ref().overlay_keep_result() {
                        controller.as_mut().set_overlay_visible(false);
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
        if command.is_empty() || *self.as_ref().assistant_busy() {
            return;
        }
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_command_output(QString::from(
                "AI enhancement is off · enable it before using Command Mode",
            ));
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
        "You are Spevox Command Mode, a concise KDE Plasma assistant. Answer the user's question or explain how to perform the requested task. Do not claim to have executed anything. Never output shell commands unless explicitly asked, and clearly label them as suggestions."
            .clone_into(&mut config.prompt);
        let qt_thread = self.qt_thread();
        self.as_mut().set_assistant_busy(true);
        self.as_mut()
            .set_command_output(QString::from("Command Mode is thinking…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, &command);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
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
                                )));
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
        self.as_mut()
            .set_parakeet_runtime_installed(parakeet_runtime_available(index));
        self.as_mut()
            .set_compute_status(QString::from(compute_backend_summary(index)));
        self.as_ref().rust().save_preferences();
        self.as_mut().set_status_text(QString::from(match index {
            2 => "CPU inference selected",
            1 => "Vulkan-only inference selected; its development packages are required",
            _ => "Automatic inference selected; Vulkan is preferred with managed CPU fallback",
        }));
    }

    pub fn diagnose_compute_backend(mut self: Pin<&mut Self>) {
        let selected = *self.as_ref().selected_compute_backend();
        let summary = compute_backend_summary(selected);
        self.as_mut().set_compute_status(QString::from(&summary));
        self.as_mut().set_status_text(QString::from(&summary));
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
        if *self.as_ref().ai_local_only() && !provider.id.is_local() {
            self.as_mut()
                .set_ai_status(QString::from("Local-only mode blocks network AI providers"));
            return;
        }
        self.as_mut().set_selected_ai_provider(index);
        self.as_mut()
            .set_ai_model(QString::from(provider.default_model));
        self.as_mut()
            .set_ai_base_url(QString::from(provider.default_url));
        self.as_mut().rust_mut().get_mut().ai_api_key.clear();
        self.as_mut().set_ai_key_configured(provider.id.is_local());
        self.as_mut().set_ai_local_models(QStringList::default());
        self.as_mut().set_ai_local_endpoint(provider.id.is_local());
        self.as_mut()
            .set_ai_status(QString::from(if provider.id.is_local() {
                "Local endpoint · transcript stays on this computer"
            } else {
                "Cloud provider · transcript is sent only when enhancement is enabled"
            }));
        self.as_ref().rust().save_preferences();
        if !provider.id.is_local() {
            let qt_thread = self.qt_thread();
            std::thread::spawn(move || {
                let api_key = ai::load_api_key(provider.id.as_str());
                qt_thread
                    .queue(move |mut controller| {
                        // Ignore a late keyring result if the user selected a
                        // different provider while the worker was running.
                        if *controller.as_ref().selected_ai_provider() == index {
                            let configured = !api_key.is_empty();
                            controller.as_mut().rust_mut().get_mut().ai_api_key = api_key;
                            controller.as_mut().set_ai_key_configured(configured);
                        }
                    })
                    .ok();
            });
        }
    }

    pub fn update_ai_model(mut self: Pin<&mut Self>, value: &QString) {
        self.as_mut()
            .set_ai_model(QString::from(value.to_string().trim()));
        self.as_ref().rust().save_preferences();
    }

    pub fn update_ai_base_url(mut self: Pin<&mut Self>, value: &QString) {
        let value = value.to_string().trim().to_owned();
        let provider = ai_provider(*self.as_ref().selected_ai_provider());
        let is_local = AiConfig::new(provider.id, "", &value)
            .with_enabled(false)
            .is_local();
        if provider.id.is_local() && !is_local {
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
        let provider_index = *self.as_ref().selected_ai_provider();
        let provider = ai_provider(*self.as_ref().selected_ai_provider());
        let provider_id = provider.id;
        let value = value.to_string();
        self.as_mut()
            .set_ai_status(QString::from("Storing API key securely…"));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result = ai::store_api_key(provider_id.as_str(), &value);
            qt_thread
                .queue(move |mut controller| {
                    if *controller.as_ref().selected_ai_provider() != provider_index {
                        return;
                    }
                    match result {
                        Ok(()) => {
                            controller.as_mut().rust_mut().get_mut().ai_api_key = value;
                            controller.as_mut().set_ai_key_configured(true);
                            controller.as_mut().set_ai_status(QString::from(
                                "API key stored securely by KDE Wallet / Secret Service",
                            ));
                        }
                        Err(error) => controller.as_mut().set_ai_status(QString::from(&error)),
                    }
                })
                .ok();
        });
    }

    pub fn test_ai_provider(mut self: Pin<&mut Self>) {
        if *self.as_ref().assistant_busy() {
            return;
        }
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_ai_status(QString::from(
                "AI enhancement is off · enable it before testing a provider",
            ));
            return;
        }
        let config = self.as_ref().rust().ai_config();
        let qt_thread = self.qt_thread();
        self.as_mut().set_assistant_busy(true);
        self.as_mut()
            .set_ai_status(QString::from("Verifying provider…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, "hello comma this is a provider test");
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
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
        if *self.as_ref().assistant_busy() {
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
        self.as_mut().set_assistant_busy(true);
        self.as_mut()
            .set_ai_status(QString::from("Finding installed local models…"));
        std::thread::spawn(move || {
            let result = ai::discover_local_models(&config);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
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
            let (status, models) = if installed {
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
            } else {
                (
                    "Ollama is not installed. Open the official Linux guide below, then run this check again.".to_owned(),
                    None,
                )
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
            let mut command = Command::new("ollama");
            command
                .arg("serve")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            let started = spawn_reaped(&mut command);
            let status = match started {
                Ok(()) => {
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
            self.as_mut()
                .select_ai_provider(ProviderId::Ollama.preference_index());
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
        self.as_mut().set_auto_profiles_enabled(enabled);
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_ai_status(QString::from("Configuring the Spevox KWin script…"));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let script = configure_kwin_profile_script(enabled);
            qt_thread
                .queue(move |mut controller| {
                    if *controller.as_ref().auto_profiles_enabled() != enabled {
                        return;
                    }
                    controller
                        .as_mut()
                        .set_ai_status(QString::from(if let Err(error) = script {
                            format!("Could not configure the Spevox KWin script: {error}")
                        } else if enabled {
                            "Automatic profiles enabled · switch applications to test matching"
                                .to_owned()
                        } else {
                            "Automatic profiles disabled · profile selection remains manual"
                                .to_owned()
                        }));
                })
                .ok();
        });
    }

    fn apply_active_application(mut self: Pin<&mut Self>, application: &ActiveApplication) {
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
        } else if self.as_ref().rust().selected_ai_profile != 0 {
            self.as_mut().select_ai_profile(0);
            self.as_mut().set_ai_status(QString::from(
                "No application profile matched · reverted to Default",
            ));
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn rewrite_selected_text(mut self: Pin<&mut Self>, instruction: &QString) {
        let instruction = instruction.to_string().trim().to_owned();
        if instruction.is_empty() || *self.as_ref().assistant_busy() {
            return;
        }
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_ai_status(QString::from(
                "AI enhancement is off · enable it before using Write Mode",
            ));
            return;
        }
        let Some(desktop_sender) = self.as_ref().rust().desktop_sender.clone() else {
            self.as_mut()
                .set_ai_status(QString::from("Desktop integration is not ready"));
            return;
        };
        let mut config = self.as_ref().rust().ai_config();
        let qt_thread = self.qt_thread();
        self.as_mut().set_assistant_busy(true);
        self.as_mut()
            .set_ai_status(QString::from("Capturing selected text…"));
        std::thread::spawn(move || {
            let (reply, result) = std::sync::mpsc::channel();
            if desktop_sender
                .send(DesktopCommand::CopySelection(reply))
                .is_err()
            {
                qt_thread
                    .queue(|mut controller| {
                        controller.as_mut().set_assistant_busy(false);
                        controller.as_mut().set_ai_status(QString::from(
                            "Rewrite failed · desktop integration stopped",
                        ));
                    })
                    .ok();
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
                let job = WriteModeJob::rewrite(instruction.clone(), selected.clone());
                job.prompt().clone_into(&mut config.prompt);
                let text = ai::enhance(&config, &job.input())?;
                Ok((job, selected, text))
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
                    match rewritten {
                Ok((job, selected, text)) => {
                            controller.as_mut().rust_mut().get_mut().last_write_job = Some(job);
                            controller.as_mut().set_write_mode_retry_available(true);
                            controller.as_mut().set_last_raw_text(QString::from(&selected));
                            let rust = controller.as_mut().rust_mut().get_mut();
                            let copied = copy_to_clipboard(&mut rust.clipboard, &text).is_ok();
                            if copied {
                                if let Some(sender) = controller.as_ref().rust().desktop_sender.as_ref() {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                                controller.as_mut().set_transcript_text(QString::from(&text));
                                controller.as_mut().set_live_transcript(QString::from(&text));
                                controller.as_mut().set_overlay_result_available(true);
                                if *controller.as_ref().overlay_enabled()
                                    && *controller.as_ref().overlay_keep_result()
                                {
                                    controller.as_mut().set_overlay_visible(true);
                                }
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
        if instruction.is_empty() || *self.as_ref().assistant_busy() {
            return;
        }
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_ai_status(QString::from(
                "AI enhancement is off · enable it before using Write Mode",
            ));
            return;
        }
        let job = WriteModeJob::draft(instruction);
        let mut config = self.as_ref().rust().ai_config();
        job.prompt().clone_into(&mut config.prompt);
        let input = job.input();
        self.as_mut().rust_mut().get_mut().last_write_job = Some(job);
        self.as_mut().set_write_mode_retry_available(true);
        let qt_thread = self.qt_thread();
        self.as_mut().set_assistant_busy(true);
        self.as_mut().set_ai_status(QString::from("Writing draft…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, &input);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
                    match result {
                        Ok(text) => {
                            let rust = controller.as_mut().rust_mut().get_mut();
                            let copied = copy_to_clipboard(&mut rust.clipboard, &text).is_ok();
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
        if *self.as_ref().assistant_busy() {
            return;
        }
        if !*self.as_ref().ai_enabled() {
            self.as_mut().set_ai_status(QString::from(
                "AI enhancement is off · enable it before retrying Write Mode",
            ));
            return;
        }
        let Some(job) = self.as_ref().rust().last_write_job.clone() else {
            self.as_mut()
                .set_ai_status(QString::from("No Write Mode result is available to retry"));
            return;
        };
        let mut config = self.as_ref().rust().ai_config();
        job.prompt().clone_into(&mut config.prompt);
        let input = job.input();
        let paste_result = job.paste_result();
        let success_status = job.retry_success_status();
        let qt_thread = self.qt_thread();
        self.as_mut().set_assistant_busy(true);
        self.as_mut()
            .set_ai_status(QString::from("Retrying Write Mode…"));
        std::thread::spawn(move || {
            let result = ai::enhance(&config, &input);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_assistant_busy(false);
                    match result {
                        Ok(text) => {
                            let rust = controller.as_mut().rust_mut().get_mut();
                            let copied = copy_to_clipboard(&mut rust.clipboard, &text).is_ok();
                            if copied {
                                if paste_result
                                    && let Some(sender) =
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
                                if *controller.as_ref().overlay_enabled()
                                    && *controller.as_ref().overlay_keep_result()
                                {
                                    controller.as_mut().set_overlay_visible(true);
                                }
                                controller
                                    .as_mut()
                                    .set_ai_status(QString::from(success_status));
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
        if *self.as_ref().update_busy() {
            return;
        }
        let qt_thread = self.qt_thread();
        self.as_mut().set_update_busy(true);
        self.as_mut()
            .set_update_status(QString::from("Checking GitHub Releases…"));
        std::thread::spawn(move || {
            let result = check_latest_release(env!("CARGO_PKG_VERSION"));
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_update_busy(false);
                    controller
                        .as_mut()
                        .set_update_status(QString::from(&result));
                })
                .ok();
        });
    }

    pub fn update_local_api(mut self: Pin<&mut Self>, enabled: bool, port: i32) {
        let port = port.clamp(1024, 65_535);
        self.as_mut().set_local_api_enabled(enabled);
        self.as_mut().set_local_api_port(port);
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_local_api_status(QString::from(if enabled {
                format!("Enabled for 127.0.0.1:{port} · restart Spevox to apply")
            } else {
                "Disabled · restart Spevox to close the current listener".to_owned()
            }));
    }

    pub fn rotate_local_api_token(mut self: Pin<&mut Self>) {
        let message = match local_api::rotate_token(&local_api::token_path()) {
            Ok(_) => "Local API token rotated · existing clients are now revoked".to_owned(),
            Err(error) => format!("Could not rotate local API token: {error}"),
        };
        self.as_mut().set_local_api_status(QString::from(&message));
    }

    pub fn show_local_api_token_location(mut self: Pin<&mut Self>) {
        let path = local_api::token_path();
        let message = match local_api::ensure_token(&path) {
            Ok(_) => format!(
                "Bearer token is stored privately at {} · read it directly when configuring a client",
                path.display()
            ),
            Err(error) => format!("Could not prepare local API token: {error}"),
        };
        self.as_mut().set_local_api_status(QString::from(&message));
    }

    pub fn select_speech_engine(mut self: Pin<&mut Self>, index: i32) {
        let index = index.clamp(0, 5);
        self.as_mut().set_selected_speech_engine(index);
        if let Some(model) = native_model_for_engine(index) {
            let installed = parakeet::model_installed(model);
            self.as_mut().set_parakeet_model_installed(installed);
            self.as_mut()
                .set_parakeet_status(QString::from(if installed {
                    format!("{} is downloaded and ready.", model.name)
                } else {
                    format!("Download {} to use this native engine.", model.name)
                }));
            self.as_mut()
                .set_native_model_detail(QString::from(model.detail));
        }
        self.as_ref().rust().save_preferences();
        let status = match index {
            1 => "Parakeet TDT v3 beta selected · native local inference with Whisper fallback",
            2 => "Nemotron 3.5 multilingual selected · native local inference",
            3 => "Nemotron streaming English selected · native local inference",
            4 => "Parakeet CTC 1.1B selected · high-throughput English inference",
            5 => "Custom local speech server selected · audio stays on loopback",
            _ => "Built-in Whisper selected",
        };
        self.as_mut().set_status_text(QString::from(status));
    }

    pub fn install_parakeet_runtime(mut self: Pin<&mut Self>) {
        if *self.as_ref().parakeet_busy() {
            return;
        }
        let compute_backend = *self.as_ref().selected_compute_backend();
        self.as_mut().set_parakeet_busy(true);
        self.as_mut().set_parakeet_status(QString::from(
            "Building the pinned native runtime… This can take several minutes.",
        ));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result = install_native_runtime(compute_backend);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_parakeet_busy(false);
                    controller.as_mut().set_parakeet_download_progress(0.0);
                    controller
                        .as_mut()
                        .set_parakeet_runtime_installed(parakeet_runtime_available(compute_backend));
                    controller
                        .as_mut()
                        .set_parakeet_status(QString::from(match result {
                            Ok(runtime) => if runtime.fell_back {
                                "Vulkan development files were unavailable, so the pinned CPU runtime was installed automatically. Parakeet is ready to use.".to_owned()
                            } else {
                                format!("Pinned {} Parakeet runtime installed.", runtime.backend.id().to_uppercase())
                            },
                            Err(error) => format!("Runtime installation failed: {error}"),
                        }));
                })
                .ok();
        });
    }

    pub fn download_parakeet_model(mut self: Pin<&mut Self>) {
        if *self.as_ref().parakeet_busy() {
            return;
        }
        let Some(model) = native_model_for_engine(*self.as_ref().selected_speech_engine()) else {
            return;
        };
        let compute_backend = *self.as_ref().selected_compute_backend();
        let cancel = Arc::new(AtomicBool::new(false));
        self.as_mut().rust_mut().get_mut().parakeet_download_cancel = Some(cancel.clone());
        self.as_mut().set_parakeet_busy(true);
        self.as_mut().set_parakeet_download_progress(0.0);
        self.as_mut().set_parakeet_status(QString::from(&format!(
            "Preparing the shared runtime and {}…",
            model.name
        )));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let progress_thread = qt_thread.clone();
            let result =
                prepare_native_model(compute_backend, model, &cancel, true, move |progress| {
                    progress_thread
                        .queue(move |mut controller| {
                            controller.as_mut().set_parakeet_download_progress(progress);
                        })
                        .ok();
                });
            qt_thread
                .queue(move |mut controller| {
                    controller
                        .as_mut()
                        .rust_mut()
                        .get_mut()
                        .parakeet_download_cancel = None;
                    controller.as_mut().set_parakeet_busy(false);
                    controller.as_mut().set_parakeet_download_progress(0.0);
                    controller
                        .as_mut()
                        .set_parakeet_runtime_installed(parakeet_runtime_available(
                            compute_backend,
                        ));
                    controller
                        .as_mut()
                        .set_parakeet_model_installed(parakeet::model_installed(model));
                    controller
                        .as_mut()
                        .set_parakeet_status(QString::from(match result {
                            Ok(_) => format!("{} and its native runtime are ready.", model.name),
                            Err(error) => format!("One-click setup failed: {error}"),
                        }));
                })
                .ok();
        });
    }

    pub fn cancel_parakeet_download(self: Pin<&mut Self>) {
        if let Some(cancel) = self.as_ref().rust().parakeet_download_cancel.as_ref() {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    pub fn delete_parakeet_model(mut self: Pin<&mut Self>) {
        let Some(model) = native_model_for_engine(*self.as_ref().selected_speech_engine()) else {
            return;
        };
        if let Ok(mut supervisor) = self.as_ref().rust().parakeet_supervisor.lock() {
            supervisor.stop();
        }
        let result = parakeet::delete_model(model);
        self.as_mut()
            .set_parakeet_model_installed(parakeet::model_installed(model));
        self.as_mut()
            .set_parakeet_status(QString::from(match result {
                Ok(()) => format!("{} removed; Whisper remains available.", model.name),
                Err(error) => format!("Could not remove model: {error}"),
            }));
    }

    pub fn diagnose_parakeet(mut self: Pin<&mut Self>) {
        let Some(native_model) = native_model_for_engine(*self.as_ref().selected_speech_engine())
        else {
            return;
        };
        let compute_backend = *self.as_ref().selected_compute_backend();
        let runtime = parakeet_runtime_available(compute_backend);
        let model = parakeet::model_installed(native_model);
        self.as_mut().set_parakeet_runtime_installed(runtime);
        self.as_mut().set_parakeet_model_installed(model);
        self.as_mut()
            .set_parakeet_status(QString::from(match (runtime, model) {
                (true, true) => {
                    "Runtime and downloaded model are ready; the server starts on first dictation."
                }
                (false, true) => "The model is ready; install the shared native runtime.",
                (true, false) => "The runtime is ready; download this model.",
                (false, false) => "Install the shared native runtime and download this model.",
            }));
    }

    pub fn update_diarization_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        self.as_mut().set_diarization_enabled(enabled);
        self.as_ref().rust().save_preferences();
        self.as_mut()
            .set_sortformer_status(QString::from(if enabled {
                if parakeet::model_installed(parakeet::SORTFORMER_V2) {
                    "Speaker diarization enabled for file transcription."
                } else {
                    "Speaker diarization enabled, but one-click setup is still required."
                }
            } else {
                "Speaker diarization is off; file transcription uses Whisper only."
            }));
    }

    pub fn setup_sortformer(mut self: Pin<&mut Self>) {
        if *self.as_ref().sortformer_busy() {
            return;
        }
        let compute_backend = *self.as_ref().selected_compute_backend();
        let cancel = Arc::new(AtomicBool::new(false));
        self.as_mut()
            .rust_mut()
            .get_mut()
            .sortformer_download_cancel = Some(cancel.clone());
        self.as_mut().set_sortformer_busy(true);
        self.as_mut().set_sortformer_download_progress(0.0);
        self.as_mut().set_sortformer_status(QString::from(
            "Preparing the shared native runtime and Sortformer model…",
        ));
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let progress_thread = qt_thread.clone();
            let result = prepare_native_model(
                compute_backend,
                parakeet::SORTFORMER_V2,
                &cancel,
                false,
                move |progress| {
                    progress_thread
                        .queue(move |mut controller| {
                            controller
                                .as_mut()
                                .set_sortformer_download_progress(progress);
                        })
                        .ok();
                },
            );
            qt_thread
                .queue(move |mut controller| {
                    controller
                        .as_mut()
                        .rust_mut()
                        .get_mut()
                        .sortformer_download_cancel = None;
                    controller.as_mut().set_sortformer_busy(false);
                    controller.as_mut().set_sortformer_download_progress(0.0);
                    controller
                        .as_mut()
                        .set_sortformer_installed(parakeet::model_installed(
                            parakeet::SORTFORMER_V2,
                        ));
                    controller
                        .as_mut()
                        .set_parakeet_runtime_installed(parakeet_runtime_available(
                            compute_backend,
                        ));
                    controller.as_mut().set_sortformer_status(QString::from(match result {
                        Ok(runtime) => format!(
                            "Sortformer is ready for experimental speaker diarization using {}.",
                            runtime.backend.id().to_uppercase()
                        ),
                        Err(error) => format!("Sortformer setup failed: {error}"),
                    }));
                })
                .ok();
        });
    }

    pub fn cancel_sortformer_download(self: Pin<&mut Self>) {
        if let Some(cancel) = self.as_ref().rust().sortformer_download_cancel.as_ref() {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    pub fn delete_sortformer(mut self: Pin<&mut Self>) {
        let result = parakeet::delete_model(parakeet::SORTFORMER_V2);
        self.as_mut()
            .set_sortformer_installed(parakeet::model_installed(parakeet::SORTFORMER_V2));
        self.as_mut()
            .set_sortformer_status(QString::from(match result {
                Ok(()) => {
                    "Sortformer model removed; ordinary transcription is unchanged.".to_owned()
                }
                Err(error) => format!("Could not remove Sortformer: {error}"),
            }));
    }

    pub fn diagnose_sortformer(mut self: Pin<&mut Self>) {
        let runtime = parakeet_runtime_available(*self.as_ref().selected_compute_backend());
        let model = parakeet::model_installed(parakeet::SORTFORMER_V2);
        self.as_mut().set_sortformer_installed(model);
        self.as_mut()
            .set_sortformer_status(QString::from(match (runtime, model) {
                (true, true) => "Shared native runtime and downloaded Sortformer model are ready.",
                (false, true) => "Sortformer is downloaded; rebuild the shared native runtime.",
                (true, false) => "The native runtime is ready; download the Sortformer model.",
                (false, false) => {
                    "Run one-click setup to build the runtime and download Sortformer."
                }
            }));
    }

    pub fn update_local_speech_url(mut self: Pin<&mut Self>, url: &QString) {
        let url = url.to_string();
        match LocalSpeechServer::new(&url) {
            Ok(_) => {
                self.as_mut().set_local_speech_url(QString::from(&url));
                self.as_ref().rust().save_preferences();
                self.as_mut().set_status_text(QString::from(
                    "Local speech endpoint saved · microphone audio cannot leave loopback",
                ));
            }
            Err(error) => self
                .as_mut()
                .set_status_text(QString::from(&format!("Speech endpoint rejected: {error}"))),
        }
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
        let source = path.to_string();
        let path = PathBuf::from(decode_file_url(&source));
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
        let _history_guard = HISTORY_IO_LOCK.lock().ok();
        save_lines(&history_path(), &[]).ok();
        if audio_history_directory().is_dir() {
            fs::remove_dir_all(audio_history_directory()).ok();
        }
        self.as_mut().set_history_entries(QStringList::default());
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
        let _history_guard = HISTORY_IO_LOCK.lock().ok();
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
        if *self.as_ref().export_busy() {
            return;
        }
        let path = PathBuf::from(decode_file_url(&path.to_string()));
        let qt_thread = self.qt_thread();
        self.as_mut().set_export_busy(true);
        self.as_mut()
            .set_status_text(QString::from("Exporting audio history…"));
        std::thread::spawn(move || {
            let result = write_audio_history_zip(&path);
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_export_busy(false);
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
        let result = copy_to_clipboard(&mut self.as_mut().rust_mut().get_mut().clipboard, &text);
        self.as_mut().set_status_text(QString::from(match result {
            Ok(()) => label.to_owned(),
            Err(error) => format!("History copy failed · {error}"),
        }));
    }

    // Qt-facing orchestration remains here; decoding, meeting segmentation,
    // history, and ASR live in their extracted pure-Rust boundaries.
    #[allow(clippy::too_many_lines)]
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
        let source = path.to_string();
        let path = PathBuf::from(decode_file_url(&source));
        self.as_mut().rust_mut().get_mut().last_meeting_source = QString::from(&source);
        self.as_mut()
            .set_last_meeting_file(QString::from(path.to_string_lossy().as_ref()));
        let language = selected_language_code(self.as_ref().rust());
        let use_gpu = self.as_ref().rust().selected_compute_backend != 2;
        let diarization_enabled = self.as_ref().rust().diarization_enabled;
        let diarization_backend =
            effective_parakeet_backend(self.as_ref().rust().selected_compute_backend);
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
                diarization_enabled,
                diarization_backend,
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
                (
                    text,
                    raw_text,
                    meeting.segments,
                    meeting.diarization_warning,
                    ai_error,
                    ai_duration_ms,
                )
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().rust_mut().get_mut().meeting_cancel = None;
                    match result {
                        Ok((
                            text,
                            raw_text,
                            mut segments,
                            diarization_warning,
                            ai_error,
                            ai_duration_ms,
                        )) => {
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
                            let history_text = if segments.iter().any(|segment| segment.speaker.is_some()) {
                                segments
                                    .iter()
                                    .map(|segment| format!(
                                        "{}: {}",
                                        segment.speaker.as_deref().unwrap_or("Speaker unassigned"),
                                        segment.text
                                    ))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            } else {
                                processed.clone()
                            };
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
                            let history_update = record_history(
                                &history_text,
                                &HistoryContext {
                                    raw_text: &raw_text,
                                    provider,
                                    model: &ai_config.model,
                                    ai_status,
                                    ai_duration_ms,
                                    source: "file",
                                    audio_path: "",
                                    cleanup_mode: if ai_config.enabled {
                                        "conservative-v1"
                                    } else {
                                        "deterministic"
                                    },
                                    language: &ai_config.language,
                                },
                            );
                            apply_history_update(controller.as_mut(), history_update);
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&processed));
                            controller.as_mut().set_meeting_progress(1.0);
                            controller.as_mut().set_meeting_segments(
                                segments.iter().map(meeting_segment_qstring).collect(),
                            );
                            controller.as_mut().set_meeting_speakers(
                                meeting_speaker_names(&segments)
                                    .iter()
                                    .map(QString::from)
                                    .collect(),
                            );
                            controller.as_mut().rust_mut().get_mut().meeting_results = segments;
                            controller
                                .as_mut()
                                .set_file_transcription_status(QString::from(
                                ai_error.map_or_else(
                                    || diarization_warning.map_or_else(
                                        || "Complete — timestamped speaker transcript added to History and ready to export.".to_owned(),
                                        |warning| format!("Complete without speaker labels — {warning}"),
                                    ),
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

    pub fn rename_meeting_speaker(
        mut self: Pin<&mut Self>,
        current: &QString,
        replacement: &QString,
    ) {
        let current = current.to_string();
        let replacement = replacement.to_string().trim().to_owned();
        if current.is_empty()
            || replacement.is_empty()
            || replacement.len() > 40
            || replacement.chars().any(char::is_control)
        {
            self.as_mut().set_file_transcription_status(QString::from(
                "Speaker names must contain 1–40 printable characters.",
            ));
            return;
        }
        let rust = self.as_mut().rust_mut().get_mut();
        let mut changed = false;
        for segment in &mut rust.meeting_results {
            if segment.speaker.as_deref() == Some(&current) {
                segment.speaker = Some(replacement.clone());
                changed = true;
            }
        }
        if !changed {
            self.as_mut().set_file_transcription_status(QString::from(
                "The selected speaker no longer exists in this transcript.",
            ));
            return;
        }
        let segments = self.as_ref().rust().meeting_results.clone();
        self.as_mut()
            .set_meeting_segments(segments.iter().map(meeting_segment_qstring).collect());
        self.as_mut().set_meeting_speakers(
            meeting_speaker_names(&segments)
                .iter()
                .map(QString::from)
                .collect(),
        );
        let history_result = rename_latest_file_history_speaker(&current, &replacement);
        if history_result.is_ok() {
            let history = load_lines(&history_path());
            self.as_mut()
                .set_history_entries(history.iter().rev().map(QString::from).collect());
        }
        self.as_mut()
            .set_file_transcription_status(QString::from(history_result.map_or_else(
                |error| format!("Speaker renamed in this result; History update failed: {error}"),
                |()| format!("Renamed {current} to {replacement} in this result and History."),
            )));
    }

    pub fn retry_meeting_transcription(mut self: Pin<&mut Self>) {
        let path = self.as_ref().rust().last_meeting_source.clone();
        if path.is_empty() {
            self.as_mut().set_file_transcription_status(QString::from(
                "Choose an audio or video file before retrying.",
            ));
            return;
        }
        self.as_mut().transcribe_file(&path);
    }

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
        let speech_engine = *self.as_ref().selected_speech_engine();
        let job = DictationJob {
            stop_token: stop_token.clone(),
            capture_target: self.as_ref().rust().capture_target.clone(),
            language: selected_language_code(self.as_ref().rust()),
            use_gpu: self.as_ref().rust().selected_compute_backend != 2,
            model: selected_model_path(self.as_ref().rust()),
            speech_engine,
            native_model: native_model_for_engine(speech_engine),
            local_speech_url: self.as_ref().local_speech_url().to_string(),
            native_supervisor: Arc::clone(&self.as_ref().rust().parakeet_supervisor),
            native_backend: effective_parakeet_backend(*self.as_ref().selected_compute_backend()),
            ai_config: self.as_ref().rust().ai_config(),
            retain_audio: *self.as_ref().audio_history_enabled(),
            audio_budget_bytes: u64::try_from(*self.as_ref().audio_history_budget_mb())
                .unwrap_or(500)
                * 1_048_576,
            gain: 10.0_f32.powf(*self.as_ref().gain_db() / 20.0),
            command_mode_enabled: self.as_ref().rust().command_mode_enabled,
        };
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

        start_dictation_worker(job, self.qt_thread());
    }

    pub fn set_overlay_preview(mut self: Pin<&mut Self>, visible: bool) {
        if visible {
            self.as_mut().set_overlay_result_available(false);
            self.as_mut().set_live_transcript(QString::from(
                "Live transcription appears here while you speak.",
            ));
        }
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

impl SpevoxControllerRust {
    fn save_preferences(&self) {
        let preferences = Preferences {
            onboarding_completed: self.onboarding_completed,
            language: selected_language_code(self),
            model: selected_model_path(self).unwrap_or_default(),
            shortcut: selected_shortcut_trigger(self),
            input: self.capture_target.clone().unwrap_or_default(),
            gain_db: self.gain_db,
            overlay_enabled: self.overlay_enabled,
            overlay_keep_result: self.overlay_keep_result,
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
            local_api_enabled: self.local_api_enabled,
            local_api_port: self.local_api_port,
            speech_engine: self.selected_speech_engine,
            local_speech_url: self.local_speech_url.to_string(),
            diarization_enabled: self.diarization_enabled,
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
        let language_code = selected_language_code(self);
        let language_name = supported_languages()
            .iter()
            .find_map(|(name, code)| (*code == language_code).then_some(*name))
            .unwrap_or("Automatic detection");
        AiConfig::new(
            provider.id,
            self.ai_model.to_string(),
            self.ai_base_url.to_string(),
        )
        .with_enabled(self.ai_enabled)
        .with_prompt(prompt)
        .with_language(language_code)
        .with_language_name(language_name)
        .with_api_key(self.ai_api_key.clone())
        .with_local_only(self.ai_local_only)
    }
}

#[path = "settings.rs"]
mod settings;
use settings::{
    AiProfile, Preferences, WriteModeJob, load_ai_profiles, profile_matches_application,
    save_ai_profiles,
};
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
        spawn_reaped(&mut command).map_err(|error| error.to_string())
    }
}

fn spawn_reaped(command: &mut Command) -> std::io::Result<()> {
    let mut child = command.spawn()?;
    std::thread::spawn(move || {
        child.wait().ok();
    });
    Ok(())
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
            "spevoxprofilesEnabled",
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

#[derive(Clone, Copy)]
struct AiProviderPreset {
    id: ProviderId,
    name: &'static str,
    default_url: &'static str,
    default_model: &'static str,
}

fn ai_provider_catalog() -> &'static [AiProviderPreset] {
    &[
        AiProviderPreset {
            id: ProviderId::OpenAi,
            name: "OpenAI",
            default_url: "https://api.openai.com/v1",
            default_model: "gpt-4.1",
        },
        AiProviderPreset {
            id: ProviderId::Anthropic,
            name: "Anthropic",
            default_url: "https://api.anthropic.com/v1",
            default_model: "claude-sonnet-4-20250514",
        },
        AiProviderPreset {
            id: ProviderId::Xai,
            name: "xAI",
            default_url: "https://api.x.ai/v1",
            default_model: "grok-3-fast",
        },
        AiProviderPreset {
            id: ProviderId::Groq,
            name: "Groq",
            default_url: "https://api.groq.com/openai/v1",
            default_model: "openai/gpt-oss-120b",
        },
        AiProviderPreset {
            id: ProviderId::Cerebras,
            name: "Cerebras",
            default_url: "https://api.cerebras.ai/v1",
            default_model: "gpt-oss-120b",
        },
        AiProviderPreset {
            id: ProviderId::Google,
            name: "Google Gemini",
            default_url: "https://generativelanguage.googleapis.com/v1beta/openai",
            default_model: "gemini-2.5-flash",
        },
        AiProviderPreset {
            id: ProviderId::OpenRouter,
            name: "OpenRouter",
            default_url: "https://openrouter.ai/api/v1",
            default_model: "openai/gpt-oss-20b",
        },
        AiProviderPreset {
            id: ProviderId::Ollama,
            name: "Ollama (local)",
            default_url: "http://localhost:11434/v1",
            default_model: "qwen2.5:7b",
        },
        AiProviderPreset {
            id: ProviderId::LmStudio,
            name: "LM Studio (local)",
            default_url: "http://localhost:1234/v1",
            default_model: "local-model",
        },
        AiProviderPreset {
            id: ProviderId::Custom,
            name: "Custom OpenAI-compatible",
            default_url: "",
            default_model: "",
        },
    ]
}

fn ai_provider(index: i32) -> AiProviderPreset {
    let provider_id = ProviderId::from_preference_index(index);
    ai_provider_catalog()
        .iter()
        .find(|provider| provider.id == provider_id)
        .copied()
        .expect("the provider catalog covers every ProviderId")
}

fn ollama_config() -> AiConfig {
    AiConfig::new(
        ProviderId::Ollama,
        "qwen2.5:7b",
        "http://localhost:11434/v1",
    )
    .with_local_only(true)
    .with_timeout(8)
}

fn ollama_server_responds() -> bool {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(2)))
        .proxy(None)
        .max_redirects(0)
        .build()
        .new_agent();
    agent
        .get("http://localhost:11434/api/version")
        .call()
        .is_ok()
}

fn valid_ollama_model_name(model: &str) -> bool {
    !model.is_empty()
        && !model.starts_with('-')
        && model.len() <= 128
        && model
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._-/:".contains(character))
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
    let base =
        if let Some(directory) = std::env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(directory)
        } else {
            PathBuf::from(std::env::var_os("HOME").expect(
                "Spevox requires HOME or XDG_CONFIG_HOME; refusing shared temporary storage",
            ))
            .join(".config")
        };
    let current = base.join("spevox/settings.conf");
    let legacy = base.join("fluidvoice/settings.conf");
    if !current.exists() && legacy.exists() {
        legacy
    } else {
        current
    }
}

#[path = "storage.rs"]
mod storage;
use storage::{
    LifetimeStats, ai_profiles_path, append_command_history, append_private_line,
    atomic_write_private, audio_history_directory, audio_history_summary,
    clear_missing_audio_history_references, command_history_path, data_directory, dictionary_path,
    history_path, load_lines, prune_audio_history, save_audio_history, save_lines,
    write_audio_history_zip,
};
#[path = "dictionary.rs"]
mod dictionary;
use dictionary::{
    DictionaryEntry, dictionary_display, dictionary_ui_list, load_dictionary_entries,
    preprocess_for_cleanup, process_transcript, read_dictionary_import, sanitize_dictionary_field,
    save_dictionary_entries, spreadsheet_safe, write_dictionary_csv,
};
#[path = "history.rs"]
mod history;
use history::{
    HISTORY_VISIBLE_LIMIT, HistoryContext, HistoryUpdate, ai_provider_name, decode_file_url,
    history_clipboard_text, history_field, history_value, record_history, write_history_export,
};
#[path = "meeting.rs"]
mod meeting;
use meeting::{
    MeetingSegment, meeting_segment_qstring, meeting_speaker_names,
    rename_latest_file_history_speaker, transcribe_long_audio_file, write_meeting_export,
};
#[path = "models.rs"]
mod models;
use models::{
    download_whisper_model, managed_model_directory, model_file_valid, model_ui_lists,
    resolve_model_path, supported_languages, verify_unmarked_whisper_models, whisper_model_catalog,
};
#[path = "dictation.rs"]
mod dictation;
#[path = "speech_runtime.rs"]
mod speech_runtime;
use dictation::{
    EnhancementResult, FinalAsrRequest, FinalAsrResult, PersistedDictationResult, PreviewConfig,
    PreviewSession, capture_audio, copy_to_clipboard, deliver_transcript, enhance_transcript,
    persist_dictation_result, resolve_final_text, transcribe_final,
};
#[cfg(test)]
use dictionary::parse_csv_record;
#[cfg(test)]
use meeting::{
    assign_speakers, decode_audio_file, rename_latest_file_history_speaker_entries, timestamp_srt,
};
pub(crate) use speech_runtime::progress_ratio;
use speech_runtime::{
    asr_gain, compute_backend_summary, display_ratio, dump_asr_audio, effective_parakeet_backend,
    install_native_runtime, language_display_name, meter_level, native_language_for_model,
    native_model_for_engine, native_model_supports_language, parakeet_runtime_available, peak_db,
    prepare_native_model, selected_language_code, selected_model_path, selected_shortcut_trigger,
    shortcut_triggers, suspicious_single_word, valid_index,
};

fn apply_history_update(mut controller: Pin<&mut ffi::SpevoxController>, update: HistoryUpdate) {
    let HistoryUpdate {
        entries,
        transcript_count,
        dictated_word_count,
    } = update;
    controller.as_mut().set_history_entries(
        entries
            .into_iter()
            .map(|entry| QString::from(entry.as_str()))
            .collect(),
    );
    controller.as_mut().set_transcript_count(transcript_count);
    controller
        .as_mut()
        .set_dictated_word_count(dictated_word_count);
}

struct DictationUiContext {
    result: PersistedDictationResult,
    duration: Duration,
    asr_peak: f32,
    native_fallback_error: Option<String>,
    diagnostic_dump: Result<(), String>,
}

type ControllerThread = CxxQtThread<ffi::SpevoxController>;

struct DictationJob {
    stop_token: CaptureStopToken,
    capture_target: Option<String>,
    language: String,
    use_gpu: bool,
    model: Option<PathBuf>,
    speech_engine: i32,
    native_model: Option<parakeet::Model>,
    local_speech_url: String,
    native_supervisor: Arc<Mutex<parakeet::Supervisor>>,
    native_backend: ParakeetBackend,
    ai_config: AiConfig,
    retain_audio: bool,
    audio_budget_bytes: u64,
    gain: f32,
    command_mode_enabled: bool,
}

fn apply_dictation_completion(
    mut controller: Pin<&mut ffi::SpevoxController>,
    context: DictationUiContext,
) {
    controller.as_mut().set_transcribing(false);
    controller.as_mut().set_overlay_visible(false);
    if let Err(error) = context.diagnostic_dump {
        eprintln!("Failed to save SPEVOX_ASR_DUMP: {error}");
    }
    match context.result {
        PersistedDictationResult::Complete(persisted) => {
            let completed = persisted.completed;
            controller
                .as_mut()
                .set_live_transcript(QString::from(&completed.processed_text));
            controller
                .as_mut()
                .set_last_raw_text(QString::from(&completed.raw_text));
            if let Some(error) = context.native_fallback_error.as_deref() {
                controller
                    .as_mut()
                    .set_parakeet_status(QString::from(&format!(
                        "Parakeet unavailable; Whisper fallback succeeded: {error}"
                    )));
            }
            let rust = controller.as_mut().rust_mut().get_mut();
            let delivery_succeeded = deliver_transcript(
                &mut rust.clipboard,
                rust.desktop_sender.as_ref(),
                &completed.processed_text,
            );
            apply_history_update(controller.as_mut(), persisted.history_update);
            controller
                .as_mut()
                .set_audio_history_status(QString::from(&persisted.audio_history_status));
            controller
                .as_mut()
                .set_transcript_text(QString::from(&completed.processed_text));
            controller.as_mut().set_overlay_result_available(true);
            if *controller.as_ref().overlay_enabled() && *controller.as_ref().overlay_keep_result()
            {
                controller.as_mut().set_overlay_visible(true);
            }
            controller.set_status_text(QString::from(if let Some(error) = completed.ai_error {
                format!("AI enhancement failed · raw transcript delivered · {error}")
            } else if let Some(error) = context.native_fallback_error.as_deref() {
                format!("Native speech engine unavailable · Whisper fallback delivered · {error}")
            } else if delivery_succeeded {
                format!(
                    "Dictated {:.1}s · {} · pasted or copied",
                    context.duration.as_secs_f32(),
                    completed.detected_language
                )
            } else {
                format!(
                    "Transcribed {:.1}s · {} · clipboard unavailable",
                    context.duration.as_secs_f32(),
                    completed.detected_language
                )
            }));
        }
        PersistedDictationResult::Empty => {
            controller
                .as_mut()
                .set_transcript_text(QString::from(&format!(
                    "No speech recognized (ASR peak {:.0}%). Increase the microphone or interface hardware gain if this persists.",
                    context.asr_peak * 100.0
                )));
            controller.set_status_text(QString::from(&format!(
                "No speech recognized · ASR peak {:.0}%",
                context.asr_peak * 100.0
            )));
        }
        PersistedDictationResult::Failed(error) => {
            controller
                .as_mut()
                .set_transcript_text(QString::from(&format!("Transcription failed: {error}")));
            controller.set_status_text(QString::from("Transcription failed"));
        }
    }
}

fn capture_dictation(
    job: &DictationJob,
    qt_thread: &ControllerThread,
) -> Result<AudioBuffer, String> {
    let level_thread = qt_thread.clone();
    let preview_thread = qt_thread.clone();
    let preview_session = PreviewSession::start(
        PreviewConfig {
            speech_engine: job.speech_engine,
            whisper_model: job.model.clone(),
            language: job.language.clone(),
            use_gpu: job.use_gpu,
            gain: job.gain,
            native_model: job.native_model,
            native_backend: job.native_backend,
            native_supervisor: Arc::clone(&job.native_supervisor),
        },
        move |text| {
            preview_thread
                .queue(move |mut controller| {
                    if *controller.as_ref().recording() {
                        controller
                            .as_mut()
                            .set_live_transcript(QString::from(&text));
                    }
                })
                .ok();
        },
    );
    let preview_sender = preview_session.preview_sender();
    let stream_sender = preview_session.stream_sender();
    let gain = job.gain;
    let result = capture_audio(
        job.capture_target.as_deref(),
        &job.stop_token,
        move |level| {
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
        move |audio| {
            if let Some(stream_sender) = &stream_sender {
                stream_sender.send(audio).ok();
            }
        },
    );
    preview_session.stop_and_join();
    result
}

fn process_dictation_audio(
    job: &DictationJob,
    audio: &AudioBuffer,
    qt_thread: &ControllerThread,
) -> DictationUiContext {
    let duration = audio.duration();
    let mono = audio.to_asr_mono().trim_silence();
    let combined_gain = asr_gain(mono.peak(), job.gain);
    let asr_audio = mono.amplified(combined_gain);
    let asr_peak = asr_audio.peak();
    let diagnostic_dump = dump_asr_audio(&asr_audio);
    let fallback_thread = qt_thread.clone();
    let FinalAsrResult {
        transcription,
        native_fallback_error,
    } = transcribe_final(
        &FinalAsrRequest {
            audio: &asr_audio,
            language: &job.language,
            speech_engine: job.speech_engine,
            whisper_model: job.model.as_deref(),
            use_gpu: job.use_gpu,
            local_speech_url: &job.local_speech_url,
            native_model: job.native_model,
            native_backend: job.native_backend,
            native_supervisor: &job.native_supervisor,
        },
        move || {
            fallback_thread
                .queue(|mut controller| {
                    controller.as_mut().set_status_text(QString::from(
                        "Native engine failed · running Whisper fallback…",
                    ));
                    controller.as_mut().set_live_transcript(QString::from(
                        "Native engine failed; recovering with Whisper…",
                    ));
                })
                .ok();
        },
    );
    let EnhancementResult {
        result: enhancement,
        duration_ms: ai_duration_ms,
    } = transcription.as_ref().ok().map_or(
        EnhancementResult {
            result: None,
            duration_ms: 0,
        },
        |transcript| {
            let input = preprocess_for_cleanup(&transcript.text, job.command_mode_enabled);
            enhance_for_dictation(&job.ai_config, &input, qt_thread)
        },
    );
    let retained_audio = if job.retain_audio
        && transcription
            .as_ref()
            .is_ok_and(|transcript| !transcript.text.is_empty())
    {
        save_audio_history(&asr_audio, job.audio_budget_bytes).ok()
    } else {
        None
    };
    let dictionary = load_lines(&dictionary_path());
    let text_result = resolve_final_text(
        transcription,
        enhancement,
        job.command_mode_enabled,
        &dictionary,
    );
    let audio_path = retained_audio
        .as_deref()
        .and_then(std::path::Path::to_str)
        .unwrap_or("");
    DictationUiContext {
        result: persist_dictation_result(text_result, &job.ai_config, ai_duration_ms, audio_path),
        duration,
        asr_peak,
        native_fallback_error,
        diagnostic_dump,
    }
}

fn enhance_for_dictation(
    config: &AiConfig,
    transcript: &str,
    qt_thread: &ControllerThread,
) -> EnhancementResult {
    let stream_thread = qt_thread.clone();
    enhance_transcript(config, transcript, move |text| {
        let text = text.to_owned();
        stream_thread
            .queue(move |mut controller| {
                if *controller.as_ref().overlay_enabled() {
                    controller.as_mut().set_overlay_visible(true);
                }
                controller
                    .as_mut()
                    .set_live_transcript(QString::from(&text));
                controller.set_status_text(QString::from(
                    "Enhancing locally or with selected provider…",
                ));
            })
            .ok();
    })
}

fn start_dictation_worker(job: DictationJob, qt_thread: ControllerThread) {
    std::thread::spawn(move || {
        let audio = match capture_dictation(&job, &qt_thread) {
            Ok(audio) => audio,
            Err(error) => {
                qt_thread
                    .queue(move |mut controller| {
                        controller.as_mut().rust_mut().get_mut().stop_token = None;
                        controller.as_mut().set_recording(false);
                        controller.as_mut().set_overlay_visible(false);
                        controller.as_mut().set_audio_level(0.0);
                        controller
                            .set_status_text(QString::from(&format!("Capture failed: {error}")));
                    })
                    .ok();
                return;
            }
        };
        let peak = audio.peak();
        let gain = job.gain;
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
        let result = process_dictation_audio(&job, &audio, &qt_thread);
        qt_thread
            .queue(move |mut controller| {
                apply_dictation_completion(controller.as_mut(), result);
            })
            .ok();
    });
}
#[cfg(test)]
mod tests {
    use std::{fs, time::Duration};

    use super::{
        AiProfile, DesktopAction, DictionaryEntry, MeetingSegment, WriteModeJob,
        ai_provider_catalog, asr_gain, assign_speakers, decode_audio_file, decode_file_url,
        history_clipboard_text, meeting_speaker_names, meter_level, native_language_for_model,
        native_model_for_engine, native_model_supports_language, parse_csv_record,
        parse_desktop_action, peak_db, preprocess_for_cleanup, process_transcript,
        profile_matches_application, read_dictionary_import,
        rename_latest_file_history_speaker_entries, supported_languages, suspicious_single_word,
        timestamp_srt, valid_ollama_model_name, whisper_model_catalog, write_dictionary_csv,
        write_history_export, write_meeting_export,
    };
    use crate::parakeet;

    #[test]
    fn write_mode_job_owns_request_and_delivery_policy() {
        let rewrite = WriteModeJob::rewrite("make concise".into(), "selected text".into());
        assert!(rewrite.prompt().starts_with("Rewrite the selected text"));
        assert_eq!(
            rewrite.input(),
            "User instruction: make concise\n\nSelected text:\nselected text"
        );
        assert!(rewrite.paste_result());

        let draft = WriteModeJob::draft("write a note".into());
        assert!(draft.prompt().starts_with("Write the requested text"));
        assert_eq!(draft.input(), "write a note");
        assert!(!draft.paste_result());
    }

    #[test]
    fn provider_catalog_order_matches_the_stable_qml_index_mapping() {
        for (index, provider) in ai_provider_catalog().iter().enumerate() {
            assert_eq!(
                usize::try_from(provider.id.preference_index()).unwrap(),
                index
            );
        }
    }

    #[test]
    fn deterministic_cleanup_is_conservative_across_languages() {
        assert_eq!(
            preprocess_for_cleanup("  hello   world  ", false),
            "hello world"
        );
        assert_eq!(
            preprocess_for_cleanup("hej   världen", false),
            "hej världen"
        );
        assert_eq!(
            preprocess_for_cleanup("Rust API och svenska ord", false),
            "Rust API och svenska ord"
        );
        assert_eq!(
            preprocess_for_cleanup("is this safe question mark", true),
            "is this safe?"
        );
    }

    #[test]
    fn maps_native_engines_and_their_language_contracts() {
        assert_eq!(native_model_for_engine(1), Some(parakeet::PARAKEET_V3));
        assert_eq!(native_model_for_engine(2), Some(parakeet::NEMOTRON_35));
        assert_eq!(native_model_for_engine(3), Some(parakeet::NEMOTRON_EN));
        assert_eq!(native_model_for_engine(4), Some(parakeet::PARAKEET_CTC));
        assert_eq!(native_model_for_engine(5), None);
        assert_eq!(
            native_language_for_model(parakeet::NEMOTRON_35, "sv"),
            "sv-SE"
        );
        assert_eq!(
            native_language_for_model(parakeet::NEMOTRON_EN, "sv"),
            "en-US"
        );
        assert_eq!(native_language_for_model(parakeet::PARAKEET_V3, "sv"), "sv");
        assert!(!native_model_supports_language(parakeet::NEMOTRON_EN, "sv"));
        assert!(!native_model_supports_language(
            parakeet::PARAKEET_CTC,
            "de"
        ));
        assert!(native_model_supports_language(parakeet::NEMOTRON_EN, "en"));
        assert!(native_model_supports_language(parakeet::NEMOTRON_EN, ""));
    }

    #[test]
    fn assigns_each_transcript_segment_to_the_longest_overlapping_speaker() {
        let mut transcript = vec![
            MeetingSegment {
                start_milliseconds: 0,
                end_milliseconds: 2_000,
                speaker: None,
                text: "First".into(),
            },
            MeetingSegment {
                start_milliseconds: 2_000,
                end_milliseconds: 4_000,
                speaker: None,
                text: "Second".into(),
            },
        ];
        let diarization = vec![
            parakeet::DiarizationSegment {
                start_seconds: 0.0,
                end_seconds: 1.8,
                speaker: 1,
            },
            parakeet::DiarizationSegment {
                start_seconds: 1.7,
                end_seconds: 4.0,
                speaker: 2,
            },
        ];
        assign_speakers(&mut transcript, &diarization);
        assert_eq!(transcript[0].speaker.as_deref(), Some("Speaker 1"));
        assert_eq!(transcript[1].speaker.as_deref(), Some("Speaker 2"));
        assert_eq!(
            meeting_speaker_names(&transcript),
            ["Speaker 1", "Speaker 2"]
        );
    }

    #[test]
    fn renames_speakers_only_in_the_latest_file_history_entry() {
        let mut history = vec![
            "1\tSpeaker 1: older\traw\t\t\tdisabled\t0\tfile\t".to_owned(),
            "2\tdictation\traw\t\t\tdisabled\t0\tdictation\t".to_owned(),
            "3\tSpeaker 1: hello Speaker 2: hi\traw\t\t\tdisabled\t0\tfile\t".to_owned(),
        ];
        rename_latest_file_history_speaker_entries(&mut history, "Speaker 1", "David")
            .expect("rename latest file speaker");
        assert!(history[0].contains("Speaker 1: older"));
        assert!(history[2].contains("David: hello"));
        assert!(!history[2].contains("Speaker 1: hello"));
    }

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
    fn validates_ollama_model_names_before_spawning_the_cli() {
        assert!(valid_ollama_model_name("qwen2.5:7b"));
        assert!(valid_ollama_model_name(
            "registry.example/team/model:latest"
        ));
        assert!(!valid_ollama_model_name(""));
        assert!(!valid_ollama_model_name("--insecure"));
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
        let path =
            std::env::temp_dir().join(format!("spevox-audio-decode-{}.wav", std::process::id()));
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
        let path =
            std::env::temp_dir().join(format!("spevox-history-export-{}.json", std::process::id()));
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
        assert_eq!(asr_gain(0.95, 1.0), 1.0);
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
            "Spevox".to_owned(),
            "KDE".to_owned(),
            "fluid voice\tSpevox".to_owned(),
        ];
        assert_eq!(
            process_transcript("spevox comma new line kde question mark", true, &dictionary),
            "Spevox,\nKDE?"
        );
        assert_eq!(
            process_transcript("spevox comma", false, &dictionary),
            "Spevox comma"
        );
        assert_eq!(
            process_transcript("I use fluid voice daily", false, &dictionary),
            "I use Spevox daily"
        );
    }

    #[test]
    fn imports_and_exports_quoted_dictionary_csv() {
        assert_eq!(
            parse_csv_record("\"fluid, voice\",\"Spevox \"\"Linux\"\"\""),
            ["fluid, voice", "Spevox \"Linux\""]
        );
        let input = std::env::temp_dir().join(format!(
            "spevox-dictionary-import-{}.csv",
            std::process::id()
        ));
        let output = input.with_extension("export.csv");
        fs::write(&input, "spoken,preferred\nfluid voice,Spevox\nk d e,KDE\n")
            .expect("write dictionary import");
        let entries = read_dictionary_import(&input).expect("read dictionary import");
        assert_eq!(
            entries,
            [
                DictionaryEntry {
                    spoken: "fluid voice".into(),
                    preferred: "Spevox".into()
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
                .contains("\"fluid voice\",\"Spevox\"")
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
        assert_eq!(decode_file_url("file:///tmp/100%aö.wav"), "/tmp/100%aö.wav");
        assert_eq!(
            decode_file_url("file:///tmp/literal%25.wav"),
            "/tmp/literal%.wav"
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
                "spevox-meeting-export-{}.{format}",
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
