//! Terminal-owned introspection record vocabulary.
//!
//! These records describe inspectable terminal state. `terminal`
//! remains the database and reducer owner; this module owns only the typed
//! shape that `persona-introspect` and other observers can name.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{Output, TerminalGeneration, TerminalName, TerminalOperationKind, TerminalSequence};

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalObservationSequence(u64);

impl TerminalObservationSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalControlSocketPath(String);

impl TerminalControlSocketPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalDataSocketPath(String);

impl TerminalDataSocketPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalViewerName(String);

impl TerminalViewerName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalArchiveReason(String);

impl TerminalArchiveReason {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalSessionState {
    Ready,
    Exited,
}

impl TerminalSessionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Exited => "exited",
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalSessionObservation {
    pub terminal: TerminalName,
    pub control_socket_path: TerminalControlSocketPath,
    pub data_socket_path: TerminalDataSocketPath,
    pub generation: TerminalGeneration,
    pub transcript_sequence: TerminalSequence,
    pub state: TerminalSessionState,
}

impl TerminalSessionObservation {
    pub fn ready(
        terminal: TerminalName,
        control_socket_path: impl Into<String>,
        data_socket_path: impl Into<String>,
    ) -> Self {
        Self {
            terminal,
            control_socket_path: TerminalControlSocketPath::new(control_socket_path),
            data_socket_path: TerminalDataSocketPath::new(data_socket_path),
            generation: TerminalGeneration::new(1),
            transcript_sequence: TerminalSequence::new(0),
            state: TerminalSessionState::Ready,
        }
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub fn control_socket_path(&self) -> &TerminalControlSocketPath {
        &self.control_socket_path
    }

    pub fn data_socket_path(&self) -> &TerminalDataSocketPath {
        &self.data_socket_path
    }

    pub fn generation(&self) -> &TerminalGeneration {
        &self.generation
    }

    pub fn transcript_sequence(&self) -> &TerminalSequence {
        &self.transcript_sequence
    }

    pub const fn state(&self) -> TerminalSessionState {
        self.state
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalDeliveryAttemptState {
    Started,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDeliveryAttemptObservation {
    pub sequence: TerminalObservationSequence,
    pub terminal: TerminalName,
    pub operation: TerminalOperationKind,
    pub state: TerminalDeliveryAttemptState,
}

impl TerminalDeliveryAttemptObservation {
    pub fn started(
        sequence: TerminalObservationSequence,
        terminal: TerminalName,
        operation: TerminalOperationKind,
    ) -> Self {
        Self {
            sequence,
            terminal,
            operation,
            state: TerminalDeliveryAttemptState::Started,
        }
    }

    pub const fn sequence(&self) -> TerminalObservationSequence {
        self.sequence
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub const fn operation(&self) -> TerminalOperationKind {
        self.operation
    }

    pub const fn state(&self) -> TerminalDeliveryAttemptState {
        self.state
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalEventObservation {
    pub sequence: TerminalObservationSequence,
    pub terminal: TerminalName,
    pub event: Output,
}

impl TerminalEventObservation {
    pub fn new(
        sequence: TerminalObservationSequence,
        terminal: TerminalName,
        event: Output,
    ) -> Self {
        Self {
            sequence,
            terminal,
            event,
        }
    }

    pub const fn sequence(&self) -> TerminalObservationSequence {
        self.sequence
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub fn event(&self) -> &Output {
        &self.event
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalViewerAttachmentState {
    Attached,
    Detached,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalViewerAttachmentObservation {
    pub sequence: TerminalObservationSequence,
    pub terminal: TerminalName,
    pub viewer: TerminalViewerName,
    pub state: TerminalViewerAttachmentState,
}

impl TerminalViewerAttachmentObservation {
    pub fn new(
        sequence: TerminalObservationSequence,
        terminal: TerminalName,
        viewer: impl Into<String>,
        state: TerminalViewerAttachmentState,
    ) -> Self {
        Self {
            sequence,
            terminal,
            viewer: TerminalViewerName::new(viewer),
            state,
        }
    }

    pub const fn sequence(&self) -> TerminalObservationSequence {
        self.sequence
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub fn viewer(&self) -> &TerminalViewerName {
        &self.viewer
    }

    pub const fn state(&self) -> TerminalViewerAttachmentState {
        self.state
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalSessionHealthObservation {
    pub terminal: TerminalName,
    pub state: TerminalSessionState,
    pub generation: TerminalGeneration,
}

impl TerminalSessionHealthObservation {
    pub fn new(
        terminal: TerminalName,
        state: TerminalSessionState,
        generation: TerminalGeneration,
    ) -> Self {
        Self {
            terminal,
            state,
            generation,
        }
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub const fn state(&self) -> TerminalSessionState {
        self.state
    }

    pub fn generation(&self) -> &TerminalGeneration {
        &self.generation
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalSessionArchiveState {
    Archived,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalSessionArchiveObservation {
    pub terminal: TerminalName,
    pub reason: TerminalArchiveReason,
    pub state: TerminalSessionArchiveState,
}

impl TerminalSessionArchiveObservation {
    pub fn archived(terminal: TerminalName, reason: impl Into<String>) -> Self {
        Self {
            terminal,
            reason: TerminalArchiveReason::new(reason),
            state: TerminalSessionArchiveState::Archived,
        }
    }

    pub fn terminal(&self) -> &TerminalName {
        &self.terminal
    }

    pub fn reason(&self) -> &TerminalArchiveReason {
        &self.reason
    }

    pub const fn state(&self) -> TerminalSessionArchiveState {
        self.state
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaDecode, nota_next::NotaEncode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalIntrospectionSnapshot {
    pub sessions: Vec<TerminalSessionObservation>,
    pub delivery_attempts: Vec<TerminalDeliveryAttemptObservation>,
    pub terminal_events: Vec<TerminalEventObservation>,
    pub viewer_attachments: Vec<TerminalViewerAttachmentObservation>,
    pub session_health: Vec<TerminalSessionHealthObservation>,
    pub session_archive: Vec<TerminalSessionArchiveObservation>,
}

impl TerminalIntrospectionSnapshot {
    pub fn empty() -> Self {
        Self {
            sessions: Vec::new(),
            delivery_attempts: Vec::new(),
            terminal_events: Vec::new(),
            viewer_attachments: Vec::new(),
            session_health: Vec::new(),
            session_archive: Vec::new(),
        }
    }
}
