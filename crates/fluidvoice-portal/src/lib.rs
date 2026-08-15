//! XDG desktop portal integration for `FluidVoice` Linux.

mod shortcuts;

pub use shortcuts::{
    FLUIDVOICE_APP_ID, GlobalShortcutBinding, GlobalShortcutConfig, GlobalShortcutError,
    GlobalShortcutEvent, PortalCapabilities,
};
