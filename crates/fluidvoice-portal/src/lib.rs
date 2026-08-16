//! XDG desktop portal integration for `FluidVoice` Linux.

mod shortcuts;
mod text_input;
mod window_profiles;

pub use shortcuts::{
    FLUIDVOICE_APP_ID, GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutError,
    GlobalShortcutEvent, PortalCapabilities,
};
pub use text_input::TextInputSession;
pub use window_profiles::{ActiveApplication, run_profile_bridge};
