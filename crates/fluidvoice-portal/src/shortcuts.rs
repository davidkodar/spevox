use std::{error::Error, fmt, time::Duration};

use ashpd::desktop::{
    CreateSessionOptions, Session,
    global_shortcuts::{BindShortcutsOptions, GlobalShortcuts, NewShortcut},
};
use futures_util::StreamExt;
use tokio::sync::mpsc;

pub const FLUIDVOICE_APP_ID: &str = "io.github.davidkodar.FluidVoiceLinux";

/// Minimum portal features needed for global hold-to-talk shortcuts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PortalCapabilities {
    pub version: u32,
    pub supports_hold_events: bool,
}

/// User-facing definition of one global shortcut.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalShortcutConfig {
    id: String,
    description: String,
    preferred_trigger: Option<String>,
}

impl GlobalShortcutConfig {
    /// Creates a shortcut configuration.
    ///
    /// # Errors
    ///
    /// Returns [`GlobalShortcutError::InvalidConfiguration`] when the ID or
    /// description is empty, or when the ID contains characters that are not
    /// safe for an application-owned shortcut identifier.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        preferred_trigger: Option<impl Into<String>>,
    ) -> Result<Self, GlobalShortcutError> {
        let id = id.into();
        let description = description.into();

        if id.is_empty()
            || !id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            })
        {
            return Err(GlobalShortcutError::InvalidConfiguration(
                "shortcut ID must contain only ASCII letters, digits, '-' or '_'".into(),
            ));
        }
        if description.trim().is_empty() {
            return Err(GlobalShortcutError::InvalidConfiguration(
                "shortcut description must not be empty".into(),
            ));
        }

        Ok(Self {
            id,
            description,
            preferred_trigger: preferred_trigger.map(Into::into),
        })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Press/release events emitted by the global shortcuts portal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlobalShortcutEvent {
    Activated { id: String, timestamp: Duration },
    Deactivated { id: String, timestamp: Duration },
}

/// A live portal session that owns the registered shortcut.
pub struct GlobalShortcutBinding {
    portal: GlobalShortcuts,
    session: Session<GlobalShortcuts>,
    shortcuts: Vec<BoundShortcut>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundShortcut {
    id: String,
    trigger_description: String,
}

impl GlobalShortcutBinding {
    /// Creates a portal session and asks the desktop to bind one shortcut.
    ///
    /// Plasma may display a consent/configuration dialog during this call.
    ///
    /// # Errors
    ///
    /// Returns [`GlobalShortcutError`] if the portal is unavailable, is too old
    /// for hold events, the user rejects the request, or no shortcut is bound.
    pub async fn bind(config: &GlobalShortcutConfig) -> Result<Self, GlobalShortcutError> {
        Self::bind_many(std::slice::from_ref(config)).await
    }

    /// Creates one portal session containing every application shortcut.
    ///
    /// # Errors
    /// Returns [`GlobalShortcutError`] for invalid configurations, unavailable
    /// portal support, rejected consent, or a failed binding.
    pub async fn bind_many(configs: &[GlobalShortcutConfig]) -> Result<Self, GlobalShortcutError> {
        if configs.is_empty() {
            return Err(GlobalShortcutError::InvalidConfiguration(
                "at least one shortcut must be configured".into(),
            ));
        }
        let app_id = ashpd::AppID::try_from(FLUIDVOICE_APP_ID)?;
        ashpd::register_host_app(app_id).await?;
        let portal = GlobalShortcuts::new().await?;
        let version = portal.version();
        if version < 1 {
            return Err(GlobalShortcutError::UnsupportedPortalVersion(version));
        }

        let session = portal
            .create_session(CreateSessionOptions::default())
            .await?;
        let shortcuts = configs
            .iter()
            .map(|config| {
                NewShortcut::new(&config.id, &config.description)
                    .preferred_trigger(config.preferred_trigger.as_deref())
            })
            .collect::<Vec<_>>();
        let request = portal
            .bind_shortcuts(&session, &shortcuts, None, BindShortcutsOptions::default())
            .await?;
        let response = request.response()?;
        let shortcuts = response
            .shortcuts()
            .iter()
            .map(|shortcut| BoundShortcut {
                id: shortcut.id().to_owned(),
                trigger_description: shortcut.trigger_description().to_owned(),
            })
            .collect::<Vec<_>>();

        if shortcuts.is_empty() {
            return Err(GlobalShortcutError::NoShortcutBound);
        }

        Ok(Self {
            portal,
            session,
            shortcuts,
        })
    }

    #[must_use]
    pub fn capabilities(&self) -> PortalCapabilities {
        PortalCapabilities {
            version: self.portal.version(),
            supports_hold_events: true,
        }
    }

    pub fn shortcuts(&self) -> impl Iterator<Item = (&str, &str)> {
        self.shortcuts
            .iter()
            .map(|shortcut| (shortcut.id.as_str(), shortcut.trigger_description.as_str()))
    }

    /// Forwards portal activation and deactivation signals until the receiver
    /// closes or the portal session ends.
    ///
    /// # Errors
    ///
    /// Returns [`GlobalShortcutError`] if signal subscriptions fail or the
    /// portal session cannot be closed cleanly.
    pub async fn forward_events(
        self,
        sender: mpsc::Sender<GlobalShortcutEvent>,
    ) -> Result<(), GlobalShortcutError> {
        let mut activated = self.portal.receive_activated().await?;
        let mut deactivated = self.portal.receive_deactivated().await?;

        loop {
            let event = tokio::select! {
                event = activated.next() => event.map(|event| GlobalShortcutEvent::Activated {
                    id: event.shortcut_id().to_owned(),
                    timestamp: event.timestamp(),
                }),
                event = deactivated.next() => event.map(|event| GlobalShortcutEvent::Deactivated {
                    id: event.shortcut_id().to_owned(),
                    timestamp: event.timestamp(),
                }),
            };

            let Some(event) = event else {
                break;
            };
            if sender.send(event).await.is_err() {
                break;
            }
        }

        self.session.close().await?;
        Ok(())
    }
}

/// Errors produced while registering or monitoring portal shortcuts.
#[derive(Debug)]
pub enum GlobalShortcutError {
    Portal(ashpd::Error),
    InvalidConfiguration(String),
    UnsupportedPortalVersion(u32),
    NoShortcutBound,
}

impl fmt::Display for GlobalShortcutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Portal(error) => write!(formatter, "global shortcuts portal error: {error}"),
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid shortcut configuration: {message}")
            }
            Self::UnsupportedPortalVersion(version) => {
                write!(
                    formatter,
                    "unsupported global shortcuts portal version {version}"
                )
            }
            Self::NoShortcutBound => formatter.write_str("the desktop did not bind a shortcut"),
        }
    }
}

impl Error for GlobalShortcutError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Portal(error) => Some(error),
            Self::InvalidConfiguration(_)
            | Self::UnsupportedPortalVersion(_)
            | Self::NoShortcutBound => None,
        }
    }
}

impl From<ashpd::Error> for GlobalShortcutError {
    fn from(error: ashpd::Error) -> Self {
        Self::Portal(error)
    }
}

#[cfg(test)]
mod tests {
    use super::GlobalShortcutConfig;

    #[test]
    fn accepts_stable_application_owned_id() {
        let config =
            GlobalShortcutConfig::new("dictate_hold", "Hold to dictate", Some("CTRL+ALT+D"))
                .unwrap();

        assert_eq!(config.id(), "dictate_hold");
    }

    #[test]
    fn rejects_unsafe_id() {
        let error = GlobalShortcutConfig::new("dictate hold", "Hold to dictate", None::<String>)
            .unwrap_err();

        assert!(error.to_string().contains("shortcut ID"));
    }

    #[test]
    fn rejects_blank_description() {
        let error = GlobalShortcutConfig::new("dictate", "  ", None::<String>).unwrap_err();

        assert!(error.to_string().contains("description"));
    }
}
