use std::time::Duration;

use fluidvoice_audio::PipeWireCapture;
use fluidvoice_core::{DictationCoordinator, DictationState};
use fluidvoice_portal::{GlobalShortcutBinding, GlobalShortcutConfig};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
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
