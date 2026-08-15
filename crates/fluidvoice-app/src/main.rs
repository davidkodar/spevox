use fluidvoice_core::{DictationCoordinator, DictationState};

fn main() {
    let coordinator = DictationCoordinator::default();
    debug_assert_eq!(coordinator.state(), &DictationState::Idle);

    println!(
        "FluidVoice Linux foundation ready: Rust core initialized; Qt/QML shell and Linux adapters are next."
    );
}
