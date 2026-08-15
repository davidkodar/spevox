use std::time::Duration;

use fluidvoice_audio::{AudioBuffer, PipeWireCapture};
use fluidvoice_core::{DictationCoordinator, DictationState};
use fluidvoice_portal::{GlobalShortcutBinding, GlobalShortcutConfig};
use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments
        .first()
        .is_some_and(|argument| argument == "--diagnose-transcription-file")
    {
        let (Some(model_path), Some(wav_path)) = (arguments.get(1), arguments.get(2)) else {
            eprintln!("Usage: --diagnose-transcription-file MODEL WAV");
            std::process::exit(2);
        };
        if let Err(error) = diagnose_transcription_file(model_path.clone(), wav_path.clone()).await
        {
            eprintln!("File transcription diagnostic failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "--diagnose-transcription")
    {
        let Some(model_path) = arguments.get(1).cloned() else {
            eprintln!("Usage: --diagnose-transcription MODEL [SECONDS] [PIPEWIRE_NODE]");
            std::process::exit(2);
        };
        let seconds = arguments
            .get(2)
            .map_or(Ok(5), |value| value.parse::<u64>())
            .unwrap_or_else(|error| {
                eprintln!("Invalid duration: {error}");
                std::process::exit(2);
            });
        let target = arguments.get(3).cloned();
        if let Err(error) = diagnose_transcription(model_path, seconds, target).await {
            eprintln!("Transcription diagnostic failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "--diagnose-audio")
    {
        let seconds = arguments
            .get(1)
            .map_or(Ok(3), |value| value.parse::<u64>())
            .unwrap_or_else(|error| {
                eprintln!("Invalid duration: {error}");
                std::process::exit(2);
            });
        let target = arguments.get(2).cloned();
        if let Err(error) = diagnose_audio(seconds, target).await {
            eprintln!("Audio diagnostic failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    if arguments
        .iter()
        .any(|argument| argument == "--diagnose-shortcut")
    {
        if let Err(error) = diagnose_shortcut().await {
            eprintln!("Shortcut diagnostic failed: {error}");
            std::process::exit(1);
        }
        return;
    }

    let coordinator = DictationCoordinator::default();
    debug_assert_eq!(coordinator.state(), &DictationState::Idle);

    println!(
        "FluidVoice Linux foundation ready: Rust core initialized; Qt/QML shell and Linux adapters are next."
    );
}

async fn diagnose_transcription_file(
    model_path: String,
    wav_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transcript = tokio::task::spawn_blocking(move || {
        let mut reader = hound::WavReader::open(wav_path)?;
        let specification = reader.spec();
        if specification.sample_format != hound::SampleFormat::Int
            || specification.bits_per_sample != 16
        {
            return Err("diagnostic WAV must contain 16-bit PCM samples".into());
        }
        let samples = reader
            .samples::<i16>()
            .map(|sample| sample.map(|value| f32::from(value) / f32::from(i16::MAX)))
            .collect::<Result<Vec<_>, _>>()?;
        let native = AudioBuffer::new(
            samples,
            specification.sample_rate,
            u32::from(specification.channels),
            false,
        )?;
        let audio = native.to_asr_mono();
        let transcriber = WhisperTranscriber::load(
            std::path::Path::new(&model_path),
            TranscriptionConfig::default().with_language(Some("en".to_owned())),
        )?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(transcriber.transcribe(&audio)?)
    })
    .await??;
    println!("Transcript: {}", transcript.text);
    Ok(())
}

async fn diagnose_transcription(
    model_path: String,
    seconds: u64,
    target: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Recording {seconds} seconds. Speak now…");
    let audio = tokio::task::spawn_blocking(move || {
        PipeWireCapture::capture_for(Duration::from_secs(seconds), target.as_deref())
            .map(|capture| capture.to_asr_mono())
    })
    .await??;
    println!("Transcribing {} samples locally…", audio.samples().len());
    let transcript = tokio::task::spawn_blocking(move || {
        let transcriber = WhisperTranscriber::load(
            std::path::Path::new(&model_path),
            TranscriptionConfig::default(),
        )?;
        transcriber.transcribe(&audio)
    })
    .await??;
    println!("Transcript: {}", transcript.text);
    for segment in transcript.segments {
        println!(
            "  [{}–{} cs, no-speech {:.3}] {}",
            segment.start_centiseconds,
            segment.end_centiseconds,
            segment.no_speech_probability,
            segment.text
        );
    }
    Ok(())
}

async fn diagnose_audio(
    seconds: u64,
    target: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let devices = tokio::task::spawn_blocking(PipeWireCapture::devices).await??;
    println!("PipeWire microphone sources:");
    for device in devices {
        println!(
            "  {} — {} ({})",
            device.id, device.description, device.node_name
        );
    }
    println!(
        "Recording for {seconds} seconds from {}. Speak now…",
        target.as_deref().unwrap_or("the default source")
    );
    let capture = tokio::task::spawn_blocking(move || {
        PipeWireCapture::capture_for(Duration::from_secs(seconds), target.as_deref())
    })
    .await??;
    let asr = capture.to_asr_mono();
    println!(
        "Captured: {} Hz, {} channel(s), {} frames, {:.2?}, peak {:.4}, RMS {:.4}, truncated {}",
        capture.sample_rate(),
        capture.channels(),
        capture.frame_count(),
        capture.duration(),
        capture.peak(),
        capture.rms(),
        capture.truncated()
    );
    println!(
        "ASR boundary: {} Hz mono, {} samples, {:.2?}, peak {:.4}",
        asr.sample_rate(),
        asr.samples().len(),
        asr.duration(),
        asr.peak()
    );
    Ok(())
}

async fn diagnose_shortcut() -> Result<(), Box<dyn std::error::Error>> {
    let config = GlobalShortcutConfig::new(
        "dictate_hold",
        "Hold to dictate with FluidVoice Linux",
        Some("CTRL+ALT+D"),
    )?;
    println!("Requesting a KDE global shortcut binding…");
    let binding = GlobalShortcutBinding::bind(&config).await?;
    let capabilities = binding.capabilities();
    println!(
        "Portal version {} (hold events: {})",
        capabilities.version, capabilities.supports_hold_events
    );
    for (id, trigger) in binding.shortcuts() {
        println!("Bound {id} to {trigger}");
    }
    println!("Press and release the shortcut to inspect events; press Ctrl+C to stop.");

    let (sender, mut receiver) = mpsc::channel(16);
    let event_task = tokio::spawn(binding.forward_events(sender));

    loop {
        tokio::select! {
            event = receiver.recv() => {
                let Some(event) = event else {
                    break;
                };
                println!("{event:?}");
            }
            signal = tokio::signal::ctrl_c() => {
                signal?;
                break;
            }
        }
    }

    event_task.abort();
    Ok(())
}
