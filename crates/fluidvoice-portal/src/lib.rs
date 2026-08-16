//! XDG desktop portal integration for `FluidVoice` Linux.

mod shortcuts;
mod text_input;

pub use shortcuts::{
    FLUIDVOICE_APP_ID, GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutError,
    GlobalShortcutEvent, PortalCapabilities,
};
pub use text_input::TextInputSession;
