//! XDG desktop portal integration for `Spevox` Linux.

mod shortcuts;
mod text_input;
mod window_profiles;

pub use shortcuts::{
    GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutError, GlobalShortcutEvent,
    PortalCapabilities, SPEVOX_APP_ID,
};
pub use text_input::TextInputSession;
pub use window_profiles::{ActiveApplication, run_profile_bridge};
