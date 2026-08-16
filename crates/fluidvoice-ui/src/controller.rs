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
        #[qproperty(QString, microphone_name, cxx_name = "microphoneName")]
        #[qproperty(QString, model_name, cxx_name = "modelName")]
        #[qproperty(bool, recording)]
        #[qproperty(bool, overlay_visible, cxx_name = "overlayVisible")]
        #[qproperty(bool, overlay_enabled, cxx_name = "overlayEnabled")]
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
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);
    }

    impl cxx_qt::Threading for FluidVoiceController {}
}

use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    pin::Pin,
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
    GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutEvent, TextInputSession,
};
use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};
use tokio::sync::mpsc;

pub struct FluidVoiceControllerRust {
    status_text: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
    overlay_enabled: bool,
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
        Self {
            status_text: QString::from("Ready"),
            microphone_name: QString::from("Detecting PipeWire inputs…"),
            model_name,
            recording: false,
            overlay_visible: false,
            overlay_enabled: preferences.overlay_enabled,
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
                    let binding = match GlobalShortcutBinding::bind(&config).await {
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
                                Some(GlobalShortcutEvent::Activated { .. }) => {
                                    qt_thread.queue(|mut controller| {
                                        if !*controller.as_ref().recording()
                                            && !*controller.as_ref().transcribing()
                                        {
                                            controller.as_mut().toggle_recording();
                                        }
                                    }).ok();
                                }
                                Some(GlobalShortcutEvent::Deactivated { .. }) => {
                                    qt_thread.queue(|mut controller| {
                                        if *controller.as_ref().recording() {
                                            controller.as_mut().toggle_recording();
                                        }
                                    }).ok();
                                }
                                None => break,
                            },
                            request = desktop_receiver.recv() => match request {
                                Some(DesktopCommand::Paste) => {
                                    // Request optional text injection only after
                                    // capture and transcription have succeeded,
                                    // so it can never delay the shortcut loop.
                                    if text_input.is_none() {
                                        text_input = TextInputSession::request().await.ok();
                                    }
                                    if let Some(session) = text_input.as_ref() {
                                        if let Err(error) = session.paste_clipboard().await {
                                            eprintln!("Automatic paste failed: {error}");
                                        }
                                    }
                                }
                                Some(DesktopCommand::Rebind(shortcut)) => {
                                    rebind = Some(shortcut);
                                    break;
                                }
                                None => break,
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
        let model = selected_model_path(self.as_ref().rust());
        let gain = 10.0_f32.powf(*self.as_ref().gain_db() / 20.0);
        self.as_mut().rust_mut().get_mut().stop_token = Some(stop_token.clone());
        self.as_mut().set_audio_level(0.0);
        self.as_mut().set_input_db(-60.0);
        self.as_mut().set_audio_updates(0);
        self.as_mut().set_transcript_text(QString::default());
        self.as_mut().set_live_transcript(QString::default());
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
                let config = TranscriptionConfig::default().with_language(Some(preview_language));
                let Ok(transcriber) = WhisperTranscriber::load(&model, config) else {
                    return;
                };
                while let Ok(audio) = preview_receiver.recv() {
                    let preview_duration = audio.duration();
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
                    let config = TranscriptionConfig::default().with_language(Some(language));
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
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().set_overlay_visible(false);
                    if let Err(error) = diagnostic_dump {
                        eprintln!("Failed to save FLUIDVOICE_ASR_DUMP: {error}");
                    }
                    match transcription {
                        Ok(transcript) if !transcript.text.is_empty() => {
                            controller
                                .as_mut()
                                .set_live_transcript(QString::from(&transcript.text));
                            let rust = controller.as_mut().rust_mut().get_mut();
                            if rust.clipboard.is_none() {
                                rust.clipboard = ClipboardDelivery::connect().ok();
                            }
                            let delivery_result = rust
                                .clipboard
                                .as_mut()
                                .ok_or(())
                                .and_then(|delivery| {
                                    delivery.copy_transcript(&transcript.text).map_err(|_| ())
                                });
                            if delivery_result.is_ok() {
                                if let Some(sender) = controller.as_ref().rust().desktop_sender.as_ref() {
                                    sender.send(DesktopCommand::Paste).ok();
                                }
                            }
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&transcript.text));
                            controller.set_status_text(QString::from(if delivery_result.is_ok() {
                                format!("Dictated {:.1}s · pasted or copied", duration.as_secs_f32())
                            } else {
                                format!("Transcribed {:.1}s · clipboard unavailable", duration.as_secs_f32())
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
        };
        if let Err(error) = preferences.save() {
            eprintln!("Failed to save preferences: {error}");
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
                "language={}\nmodel={}\nshortcut={}\ninput={}\ngain_db={}\noverlay_enabled={}\n",
                self.language,
                self.model.display(),
                self.shortcut,
                self.input,
                self.gain_db,
                self.overlay_enabled
            ),
        )
        .map_err(|error| error.to_string())
    }
}

enum DesktopCommand {
    Paste,
    Rebind(String),
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
    use std::time::Duration;

    use super::{
        asr_gain, meter_level, peak_db, supported_languages, suspicious_single_word,
        whisper_model_catalog,
    };

    #[test]
    fn maps_audio_peak_to_logarithmic_meter() {
        assert_eq!(meter_level(0.0), 0.0);
        assert!((meter_level(0.01) - 1.0 / 3.0).abs() < 0.001);
        assert_eq!(meter_level(1.0), 1.0);
        assert_eq!(meter_level(f32::NAN), 0.0);
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
}
