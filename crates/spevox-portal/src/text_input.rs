use std::{fs, io::Write, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

use ashpd::desktop::remote_desktop::{
    DeviceType, KeyState, NotifyKeyboardKeycodeOptions, RemoteDesktop, SelectDevicesOptions,
    StartOptions,
};
use ashpd::desktop::{CreateSessionOptions, PersistMode};

/// A consented Wayland keyboard-control session used only to paste text that
/// `Spevox` has already placed on the clipboard.
pub struct TextInputSession {
    portal: RemoteDesktop,
    session: ashpd::desktop::Session<RemoteDesktop>,
}

impl TextInputSession {
    /// Requests keyboard-control permission from the desktop portal.
    ///
    /// # Errors
    /// Returns a portal error when the session cannot be created or consented.
    pub async fn request() -> ashpd::Result<Self> {
        let portal = RemoteDesktop::new().await?;
        let session = portal
            .create_session(CreateSessionOptions::default())
            .await?;
        let restore_token = fs::read_to_string(restore_token_path()).ok();
        portal
            .select_devices(
                &session,
                SelectDevicesOptions::default()
                    .set_devices(Some(DeviceType::Keyboard.into()))
                    .set_persist_mode(PersistMode::ExplicitlyRevoked)
                    .set_restore_token(restore_token.as_deref()),
            )
            .await?
            .response()?;
        let response = portal
            .start(&session, None, StartOptions::default())
            .await?
            .response()?;
        if let Some(token) = response.restore_token() {
            save_restore_token(token).ok();
        }
        Ok(Self { portal, session })
    }

    /// Sends Ctrl+V using Linux input-event key codes.
    ///
    /// # Errors
    /// Returns a portal error if a key event cannot be delivered.
    pub async fn paste_clipboard(&self) -> ashpd::Result<()> {
        const KEY_LEFTCTRL: i32 = 29;
        const KEY_V: i32 = 47;
        self.key(KEY_LEFTCTRL, KeyState::Pressed).await?;
        if let Err(error) = self.key(KEY_V, KeyState::Pressed).await {
            let _ = self.key(KEY_LEFTCTRL, KeyState::Released).await;
            return Err(error);
        }
        self.key(KEY_V, KeyState::Released).await?;
        self.key(KEY_LEFTCTRL, KeyState::Released).await
    }

    /// Sends Ctrl+C so a focused application can place its selected text on
    /// the clipboard. The caller remains responsible for reading it.
    ///
    /// # Errors
    /// Returns a portal error if a key event cannot be delivered.
    pub async fn copy_selection(&self) -> ashpd::Result<()> {
        const KEY_LEFTCTRL: i32 = 29;
        const KEY_C: i32 = 46;
        self.key(KEY_LEFTCTRL, KeyState::Pressed).await?;
        if let Err(error) = self.key(KEY_C, KeyState::Pressed).await {
            let _ = self.key(KEY_LEFTCTRL, KeyState::Released).await;
            return Err(error);
        }
        self.key(KEY_C, KeyState::Released).await?;
        self.key(KEY_LEFTCTRL, KeyState::Released).await
    }

    async fn key(&self, keycode: i32, state: KeyState) -> ashpd::Result<()> {
        self.portal
            .notify_keyboard_keycode(
                &self.session,
                keycode,
                state,
                NotifyKeyboardKeycodeOptions::default(),
            )
            .await
    }
}

fn restore_token_path() -> PathBuf {
    let base = if let Some(directory) = std::env::var_os("XDG_STATE_HOME") {
        PathBuf::from(directory)
    } else {
        PathBuf::from(std::env::var_os("HOME").expect(
            "Spevox requires HOME or XDG_STATE_HOME; refusing shared temporary token storage",
        ))
        .join(".local/state")
    };
    let current = base.join("spevox/portal-restore-token");
    let legacy = base.join("fluidvoice/portal-restore-token");
    if !current.exists() && legacy.exists() {
        legacy
    } else {
        current
    }
}

fn save_restore_token(token: &str) -> std::io::Result<()> {
    let path = restore_token_path();
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("restore token path has no parent"))?;
    fs::create_dir_all(parent)?;
    #[cfg(unix)]
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700))?;
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut options = fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(&temporary)?;
    file.write_all(token.as_bytes())?;
    file.sync_all()?;
    fs::rename(temporary, path)
}
