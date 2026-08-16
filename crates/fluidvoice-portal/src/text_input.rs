use ashpd::desktop::remote_desktop::{DeviceType, KeyState, RemoteDesktop, SelectDevicesOptions};

/// A consented Wayland keyboard-control session used only to paste text that
/// FluidVoice has already placed on the clipboard.
pub struct TextInputSession {
    portal: RemoteDesktop,
    session: ashpd::desktop::Session<RemoteDesktop>,
}

impl TextInputSession {
    /// Requests keyboard-control permission from the desktop portal.
    pub async fn request() -> ashpd::Result<Self> {
        let portal = RemoteDesktop::new().await?;
        let session = portal.create_session(Default::default()).await?;
        portal
            .select_devices(
                &session,
                SelectDevicesOptions::default().set_devices(Some(DeviceType::Keyboard.into())),
            )
            .await?
            .response()?;
        portal
            .start(&session, None, Default::default())
            .await?
            .response()?;
        Ok(Self { portal, session })
    }

    /// Sends Ctrl+V using Linux input-event key codes.
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
            .notify_keyboard_keycode(&self.session, keycode, state, Default::default())
            .await
    }
}
