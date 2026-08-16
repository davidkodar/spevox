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
        #[qproperty(i32, selected_model, cxx_name = "selectedModel")]
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
        #[cxx_name = "selectShortcut"]
        fn select_shortcut(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);
    }

    impl cxx_qt::Threading for FluidVoiceController {}
}

use std::{
    fs,
    path::PathBuf,
    pin::Pin,
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
    paste_sender: Option<mpsc::UnboundedSender<()>>,
    languages: QStringList,
    selected_language: i32,
    language_codes: Vec<String>,
    models: QStringList,
    selected_model: i32,
    model_paths: Vec<PathBuf>,
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
        let model_paths = discover_whisper_models();
        let models = model_paths
            .iter()
            .map(|path| QString::from(&model_display_name(path)))
            .collect::<QStringList>();
        let selected_model = model_paths
            .iter()
            .position(|path| path == &preferences.model)
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
                |path| QString::from(&model_display_name(path)),
            );
        Self {
            status_text: QString::from("Ready"),
            microphone_name: QString::from("Detecting PipeWire inputs…"),
            model_name,
            recording: false,
            overlay_visible: false,
            audio_level: 0.0,
            input_db: -60.0,
            audio_updates: 0,
            input_sources: QStringList::default(),
            selected_input: -1,
            gain_db: 0.0,
            transcribing: false,
            transcript_text: QString::default(),
            live_transcript: QString::default(),
            stop_token: None,
            capture_target: None,
            devices: Vec::new(),
            clipboard: None,
            paste_sender: None,
            languages,
            selected_language,
            language_codes,
            models,
            selected_model,
            model_paths,
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
        if self.as_ref().rust().paste_sender.is_some() {
            return;
        }
        let (paste_sender, mut paste_receiver) = mpsc::unbounded_channel();
        self.as_mut().rust_mut().get_mut().paste_sender = Some(paste_sender);
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
                let config = match GlobalShortcutConfig::new(
                    "dictate_hold",
                    "Hold to dictate with FluidVoice Linux",
                    Some(shortcut),
                ) {
                    Ok(config) => config,
                    Err(error) => {
                        eprintln!("Shortcut configuration failed: {error}");
                        return;
                    }
                };
                let binding = match GlobalShortcutBinding::bind(&config).await {
                    Ok(binding) => binding,
                    Err(error) => {
                        eprintln!("Global shortcut unavailable: {error}");
                        return;
                    }
                };
                let (event_sender, mut events) = mpsc::channel(16);
                tokio::spawn(async move {
                    if let Err(error) = binding.forward_events(event_sender).await {
                        eprintln!("Global shortcut stopped: {error}");
                    }
                });
                let text_input = TextInputSession::request().await.ok();
                let automatic_paste = text_input.is_some();
                qt_thread
                    .queue(move |controller| {
                        controller.set_status_text(QString::from(if automatic_paste {
                            "Ready · hold Ctrl+Alt+D to dictate"
                        } else {
                            "Ready · shortcut active · clipboard fallback"
                        }));
                    })
                    .ok();

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
                        request = paste_receiver.recv() => match request {
                            Some(()) => {
                                if let Some(session) = text_input.as_ref() {
                                    if let Err(error) = session.paste_clipboard().await {
                                        eprintln!("Automatic paste failed: {error}");
                                    }
                                }
                            }
                            None => break,
                        }
                    }
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
                        let preferred = devices.iter().find(|device| {
                            device.description.contains("Input 1")
                                || device.description.contains("Mic 1")
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
        self.as_mut().set_selected_model(index);
        let model_name = self
            .as_ref()
            .rust()
            .model_paths
            .get(usize::try_from(index).unwrap_or_default())
            .map(|path| model_display_name(path));
        if let Some(model_name) = model_name {
            self.as_mut().set_model_name(QString::from(&model_name));
        }
        self.as_ref().rust().save_preferences();
        self.set_status_text(QString::from("Speech model updated"));
    }

    pub fn select_shortcut(mut self: Pin<&mut Self>, index: i32) {
        if !valid_index(index, shortcut_triggers().len()) {
            return;
        }
        self.as_mut().set_selected_shortcut(index);
        self.as_ref().rust().save_preferences();
        self.set_status_text(QString::from(
            "Shortcut saved · restart FluidVoice to rebind",
        ));
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
        self.as_mut().set_overlay_visible(true);

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
                    let mono = audio.to_asr_mono();
                    let preview_audio = mono.amplified(asr_gain(mono.peak(), gain));
                    let Ok(transcript) = transcriber.transcribe(&preview_audio) else {
                        continue;
                    };
                    if transcript.text.is_empty() {
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

            let mono = audio.to_asr_mono();
            let combined_gain = asr_gain(mono.peak(), gain);
            let asr_audio = mono.amplified(combined_gain);
            let asr_peak = asr_audio.peak();
            let diagnostic_dump = dump_asr_audio(&asr_audio);
            let transcription = model
                .ok_or_else(|| "No Whisper model is installed".to_owned())
                .and_then(|model| {
                    // The current preview UI is English-first. Automatic language
                    // detection can classify short utterances correctly yet return
                    // no segments; the fixed language path is reliable for the same
                    // captured buffer and avoids wasting audio on detection.
                    let config = TranscriptionConfig::default().with_language(Some(language));
                    WhisperTranscriber::load(&model, config)
                        .map_err(|error| error.to_string())?
                        .transcribe(&asr_audio)
                        .map_err(|error| error.to_string())
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
                                if let Some(sender) = controller.as_ref().rust().paste_sender.as_ref() {
                                    sender.send(()).ok();
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
}

impl FluidVoiceControllerRust {
    fn save_preferences(&self) {
        let preferences = Preferences {
            language: selected_language_code(self),
            model: selected_model_path(self).unwrap_or_default(),
            shortcut: selected_shortcut_trigger(self),
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
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            language: "en".to_owned(),
            model: PathBuf::new(),
            shortcut: "CTRL+ALT+D".to_owned(),
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
                "language={}\nmodel={}\nshortcut={}\n",
                self.language,
                self.model.display(),
                self.shortcut
            ),
        )
        .map_err(|error| error.to_string())
    }
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
        ("English", "en"),
        ("Swedish", "sv"),
        ("Danish", "da"),
        ("Norwegian", "no"),
        ("Finnish", "fi"),
        ("German", "de"),
        ("French", "fr"),
        ("Spanish", "es"),
        ("Italian", "it"),
        ("Portuguese", "pt"),
        ("Dutch", "nl"),
        ("Polish", "pl"),
        ("Ukrainian", "uk"),
        ("Russian", "ru"),
        ("Japanese", "ja"),
        ("Korean", "ko"),
        ("Chinese", "zh"),
    ]
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
    usize::try_from(controller.selected_model)
        .ok()
        .and_then(|index| controller.model_paths.get(index))
        .cloned()
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

fn discover_whisper_models() -> Vec<PathBuf> {
    let mut directories = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/models")];
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        directories.push(PathBuf::from(data_home).join("fluidvoice/models"));
    } else if let Some(home) = std::env::var_os("HOME") {
        directories.push(PathBuf::from(home).join(".local/share/fluidvoice/models"));
    }
    let mut models = directories
        .into_iter()
        .filter_map(|directory| fs::read_dir(directory).ok())
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "bin"))
        .collect::<Vec<_>>();
    if let Some(configured) = std::env::var_os("FLUIDVOICE_WHISPER_MODEL").map(PathBuf::from) {
        if configured.is_file() {
            models.push(configured);
        }
    }
    models.sort();
    models.dedup();
    models
}

fn model_display_name(path: &std::path::Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Whisper model")
        .trim_start_matches("ggml-")
        .replace(['-', '_'], " ");
    let mut characters = stem.chars();
    let name = characters.next().map_or_else(
        || "Whisper".to_owned(),
        |first| format!("{}{}", first.to_uppercase(), characters.as_str()),
    );
    let size = fs::metadata(path)
        .map(|metadata| format!(" · {:.0} MB", metadata.len() as f64 / 1_048_576.0))
        .unwrap_or_default();
    format!("Whisper {name}{size}")
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

#[cfg(test)]
mod tests {
    use super::{asr_gain, meter_level, peak_db};

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
}
