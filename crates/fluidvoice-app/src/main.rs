use fluidvoice_core::{DictationCoordinator, DictationState};
use fluidvoice_portal::{GlobalShortcutBinding, GlobalShortcutConfig};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    if std::env::args().any(|argument| argument == "--diagnose-shortcut") {
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
