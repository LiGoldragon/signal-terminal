//! Signal contract - `persona-harness` to terminal transport.
//!
//! Read this file as the public interface of the terminal transport channel.
//! The harness requests terminal connection, input, resize, detachment, and
//! capture. The terminal transport pushes readiness, transcript, resize,
//! detachment, capture, exit, and rejection events back to the harness.
//!
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalName(String);

impl TerminalName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalGeneration(u64);

impl TerminalGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalSequence(u64);

impl TerminalSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalInputBytes(Vec<u8>);

impl TerminalInputBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalTranscriptBytes(Vec<u8>);

impl TerminalTranscriptBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalRows(u16);

impl TerminalRows {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalColumns(u16);

impl TerminalColumns {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalConnection {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalInput {
    pub terminal: TerminalName,
    pub bytes: TerminalInputBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResize {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetachment {
    pub terminal: TerminalName,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCapture {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalReady {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TranscriptDelta {
    pub terminal: TerminalName,
    pub sequence: TerminalSequence,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResized {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCaptured {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetached {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalExited {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub status: TerminalExitStatus,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalExitStatus {
    Exited { code: i32 },
    Signaled { signal: i32 },
    StatusUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalRejected {
    pub terminal: TerminalName,
    pub reason: TerminalRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalRejectionReason {
    NotConnected,
    InputRejected,
    ResizeRejected,
    CaptureRejected,
    TransportFailed,
}

signal_channel! {
    request TerminalRequest {
        TerminalConnection(TerminalConnection),
        TerminalInput(TerminalInput),
        TerminalResize(TerminalResize),
        TerminalDetachment(TerminalDetachment),
        TerminalCapture(TerminalCapture),
    }
    reply TerminalEvent {
        TerminalReady(TerminalReady),
        TranscriptDelta(TranscriptDelta),
        TerminalResized(TerminalResized),
        TerminalCaptured(TerminalCaptured),
        TerminalDetached(TerminalDetached),
        TerminalExited(TerminalExited),
        TerminalRejected(TerminalRejected),
    }
}
