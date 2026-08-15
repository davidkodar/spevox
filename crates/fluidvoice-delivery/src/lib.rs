//! Clipboard-backed transcript recovery for Linux desktops.

use std::{error::Error, fmt};

use arboard::Clipboard;

/// Owns the Linux clipboard selection for the application's lifetime.
///
/// Wayland and X11 clipboards are served by the process that sets them, so the
/// owner must not be dropped immediately after copying a transcript.
pub struct ClipboardDelivery {
    clipboard: Clipboard,
    last_transcript: Option<String>,
}

impl ClipboardDelivery {
    /// Connects to the current desktop clipboard.
    ///
    /// # Errors
    /// Returns an error when no supported Wayland or X11 clipboard is available.
    pub fn connect() -> Result<Self, ClipboardDeliveryError> {
        Ok(Self {
            clipboard: Clipboard::new().map_err(ClipboardDeliveryError::backend)?,
            last_transcript: None,
        })
    }

    /// Copies a non-empty transcript and verifies that it can be read back.
    ///
    /// # Errors
    /// Returns an error for empty text, backend failures, or failed verification.
    pub fn copy_transcript(&mut self, transcript: &str) -> Result<(), ClipboardDeliveryError> {
        let transcript = validate_transcript(transcript)?;
        self.clipboard
            .set_text(transcript.to_owned())
            .map_err(ClipboardDeliveryError::backend)?;
        let recovered = self
            .clipboard
            .get_text()
            .map_err(ClipboardDeliveryError::backend)?;
        if recovered != transcript {
            return Err(ClipboardDeliveryError::new(
                "clipboard verification returned different text",
            ));
        }
        self.last_transcript = Some(transcript.to_owned());
        Ok(())
    }

    #[must_use]
    pub fn last_transcript(&self) -> Option<&str> {
        self.last_transcript.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardDeliveryError(String);

impl ClipboardDeliveryError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    fn backend(error: impl fmt::Display) -> Self {
        Self::new(format!("clipboard error: {error}"))
    }
}

impl fmt::Display for ClipboardDeliveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for ClipboardDeliveryError {}

fn validate_transcript(transcript: &str) -> Result<&str, ClipboardDeliveryError> {
    if transcript.trim().is_empty() {
        return Err(ClipboardDeliveryError::new(
            "refusing to replace the clipboard with an empty transcript",
        ));
    }
    Ok(transcript)
}

#[cfg(test)]
mod tests {
    use super::validate_transcript;

    #[test]
    fn accepts_non_empty_transcript_without_changing_whitespace() {
        assert_eq!(validate_transcript(" hello ").unwrap(), " hello ");
    }

    #[test]
    fn rejects_blank_transcript() {
        assert!(validate_transcript(" \n\t").is_err());
    }
}
