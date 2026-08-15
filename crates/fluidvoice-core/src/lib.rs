//! Platform-independent domain types and state transitions for `FluidVoice` Linux.
//!
//! Linux integration crates will translate `PipeWire`, portal, transcription, and
//! text-delivery callbacks into these events. Keeping the coordinator free of UI
//! and platform dependencies makes its behavior deterministic and testable.

/// The observable lifecycle of one dictation operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum DictationState {
    /// Ready to begin a new dictation.
    #[default]
    Idle,
    /// Microphone capture is active.
    Recording,
    /// Captured audio is being converted into text.
    Transcribing,
    /// A transcript is being delivered to the user's target application.
    Delivering { transcript: String },
    /// The transcript reached the target through a verified delivery path.
    Delivered { transcript: String },
    /// Automatic delivery was unavailable and the transcript was preserved on the clipboard.
    CopiedForManualPaste { transcript: String },
    /// The operation was cancelled without producing output.
    Cancelled,
    /// The operation failed. Any recoverable transcript remains attached.
    Failed {
        message: String,
        transcript: Option<String>,
    },
}

/// Events accepted by the dictation coordinator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DictationEvent {
    StartRequested,
    StopRequested,
    CancelRequested,
    TranscriptionCompleted(String),
    DeliveryVerified,
    DeliveryRequiresManualPaste,
    OperationFailed(String),
    Reset,
}

/// Rejects an event that is invalid for the current state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidTransition {
    pub state: DictationState,
    pub event: DictationEvent,
}

/// Owns the dictation lifecycle without performing platform side effects.
#[derive(Debug, Default)]
pub struct DictationCoordinator {
    state: DictationState,
}

impl DictationCoordinator {
    #[must_use]
    pub const fn state(&self) -> &DictationState {
        &self.state
    }

    /// Applies one event and returns the resulting state.
    ///
    /// Platform adapters are responsible for initiating side effects after a
    /// successful transition—for example, starting `PipeWire` capture after
    /// `StartRequested` moves the coordinator into `Recording`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTransition`] when the event is not valid for the current state.
    pub fn apply(
        &mut self,
        event: DictationEvent,
    ) -> Result<&DictationState, Box<InvalidTransition>> {
        let next = match (&self.state, &event) {
            (DictationState::Idle, DictationEvent::StartRequested) => DictationState::Recording,
            (DictationState::Recording, DictationEvent::StopRequested) => {
                DictationState::Transcribing
            }
            (DictationState::Recording, DictationEvent::CancelRequested) => {
                DictationState::Cancelled
            }
            (DictationState::Transcribing, DictationEvent::TranscriptionCompleted(text))
                if !text.trim().is_empty() =>
            {
                DictationState::Delivering {
                    transcript: text.clone(),
                }
            }
            (DictationState::Delivering { transcript }, DictationEvent::DeliveryVerified) => {
                DictationState::Delivered {
                    transcript: transcript.clone(),
                }
            }
            (
                DictationState::Delivering { transcript },
                DictationEvent::DeliveryRequiresManualPaste,
            ) => DictationState::CopiedForManualPaste {
                transcript: transcript.clone(),
            },
            (state, DictationEvent::OperationFailed(message)) if !state.is_terminal() => {
                DictationState::Failed {
                    message: message.clone(),
                    transcript: state.transcript().map(ToOwned::to_owned),
                }
            }
            (state, DictationEvent::Reset) if state.is_terminal() => DictationState::Idle,
            _ => {
                return Err(Box::new(InvalidTransition {
                    state: self.state.clone(),
                    event,
                }));
            }
        };

        self.state = next;
        Ok(&self.state)
    }
}

impl DictationState {
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Delivered { .. }
                | Self::CopiedForManualPaste { .. }
                | Self::Cancelled
                | Self::Failed { .. }
        )
    }

    #[must_use]
    pub fn transcript(&self) -> Option<&str> {
        match self {
            Self::Delivering { transcript }
            | Self::Delivered { transcript }
            | Self::CopiedForManualPaste { transcript } => Some(transcript),
            Self::Failed { transcript, .. } => transcript.as_deref(),
            Self::Idle | Self::Recording | Self::Transcribing | Self::Cancelled => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DictationCoordinator, DictationEvent, DictationState};

    #[test]
    fn completes_verified_delivery_flow() {
        let mut coordinator = DictationCoordinator::default();

        coordinator.apply(DictationEvent::StartRequested).unwrap();
        coordinator.apply(DictationEvent::StopRequested).unwrap();
        coordinator
            .apply(DictationEvent::TranscriptionCompleted(
                "Hello from KDE".into(),
            ))
            .unwrap();
        coordinator.apply(DictationEvent::DeliveryVerified).unwrap();

        assert_eq!(
            coordinator.state(),
            &DictationState::Delivered {
                transcript: "Hello from KDE".into(),
            }
        );
    }

    #[test]
    fn preserves_transcript_for_manual_paste() {
        let mut coordinator = DictationCoordinator::default();

        coordinator.apply(DictationEvent::StartRequested).unwrap();
        coordinator.apply(DictationEvent::StopRequested).unwrap();
        coordinator
            .apply(DictationEvent::TranscriptionCompleted(
                "Clipboard fallback".into(),
            ))
            .unwrap();
        coordinator
            .apply(DictationEvent::DeliveryRequiresManualPaste)
            .unwrap();

        assert_eq!(coordinator.state().transcript(), Some("Clipboard fallback"));
        assert!(coordinator.state().is_terminal());
    }

    #[test]
    fn rejects_overlapping_start_requests() {
        let mut coordinator = DictationCoordinator::default();
        coordinator.apply(DictationEvent::StartRequested).unwrap();

        let error = coordinator
            .apply(DictationEvent::StartRequested)
            .unwrap_err();

        assert_eq!(error.state, DictationState::Recording);
    }

    #[test]
    fn can_reset_after_cancellation() {
        let mut coordinator = DictationCoordinator::default();
        coordinator.apply(DictationEvent::StartRequested).unwrap();
        coordinator.apply(DictationEvent::CancelRequested).unwrap();
        coordinator.apply(DictationEvent::Reset).unwrap();

        assert_eq!(coordinator.state(), &DictationState::Idle);
    }
}
