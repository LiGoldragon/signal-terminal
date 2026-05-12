//! Signal contract - Persona terminal transport control plane.
//!
//! Read this file as the public interface of the terminal control plane. The
//! harness requests terminal connection, input, resize, detachment, and
//! capture. Persona-terminal owns prompt-pattern registration, input-gate
//! leases, programmatic injection, and worker lifecycle observation at the
//! Persona boundary, even when it implements those facts with terminal-cell
//! primitives underneath.
//!
//! Raw attached-viewer bytes are not Signal frames. They stay on the
//! terminal-cell data plane.
//!
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalName(String);

impl TerminalName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalGeneration(u64);

impl TerminalGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalSequence(u64);

impl TerminalSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
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

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalRows(u16);

impl TerminalRows {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalColumns(u16);

impl TerminalColumns {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalByteCount(u64);

impl TerminalByteCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct PromptPatternId(String);

impl PromptPatternId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct PromptPatternBytes(Vec<u8>);

impl PromptPatternBytes {
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptPattern {
    LiteralSuffix(PromptPatternBytes),
    RegexSuffix { pattern: PromptPatternBytes },
}

impl NotaEncode for PromptPattern {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::LiteralSuffix(pattern) => {
                encoder.start_record("LiteralSuffix")?;
                pattern.encode(encoder)?;
                encoder.end_record()
            }
            Self::RegexSuffix { pattern } => {
                encoder.start_record("RegexSuffix")?;
                pattern.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for PromptPattern {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "LiteralSuffix" => {
                decoder.expect_record_head("LiteralSuffix")?;
                let pattern = PromptPatternBytes::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::LiteralSuffix(pattern))
            }
            "RegexSuffix" => {
                decoder.expect_record_head("RegexSuffix")?;
                let pattern = PromptPatternBytes::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::RegexSuffix { pattern })
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "PromptPattern",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RegisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern: PromptPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct UnregisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ListPromptPatterns {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternEntry {
    pub pattern_id: PromptPatternId,
    pub pattern: PromptPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternRegistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternUnregistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternList {
    pub terminal: TerminalName,
    pub entries: Vec<PromptPatternEntry>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct InputGateReason(String);

impl InputGateReason {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InputGateLeaseId(u64);

impl InputGateLeaseId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InputGateLease {
    pub id: InputGateLeaseId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptState {
    NotChecked,
    Clean,
    Dirty { trailing_count: TerminalByteCount },
}

impl NotaEncode for PromptState {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::NotChecked => {
                encoder.start_record("NotChecked")?;
                encoder.end_record()
            }
            Self::Clean => {
                encoder.start_record("Clean")?;
                encoder.end_record()
            }
            Self::Dirty { trailing_count } => {
                encoder.start_record("Dirty")?;
                trailing_count.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for PromptState {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "NotChecked" => {
                decoder.expect_record_head("NotChecked")?;
                decoder.expect_record_end()?;
                Ok(Self::NotChecked)
            }
            "Clean" => {
                decoder.expect_record_head("Clean")?;
                decoder.expect_record_end()?;
                Ok(Self::Clean)
            }
            "Dirty" => {
                decoder.expect_record_head("Dirty")?;
                let trailing_count = TerminalByteCount::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Dirty { trailing_count })
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "PromptState",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AcquireInputGate {
    pub terminal: TerminalName,
    pub reason: InputGateReason,
    pub prompt_pattern_id: Option<PromptPatternId>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ReleaseInputGate {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct WriteInjection {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub bytes: TerminalInputBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GateAcquired {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub prompt_state: PromptState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GateBusy {
    pub terminal: TerminalName,
    pub current_holder: InputGateLeaseId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GateReleased {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub cached_human_bytes: TerminalByteCount,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InjectionAck {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub sequence: TerminalSequence,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InjectionRejected {
    pub terminal: TerminalName,
    pub reason: InjectionRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum InjectionRejectionReason {
    UnknownTerminal,
    UnknownLease,
    GateNotHeld,
    DirtyPrompt,
    TransportFailed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SubscribeTerminalWorkerLifecycle {
    pub terminal: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum TerminalOperationKind {
    TerminalConnection,
    TerminalInput,
    TerminalResize,
    TerminalDetachment,
    TerminalCapture,
    RegisterPromptPattern,
    UnregisterPromptPattern,
    ListPromptPatterns,
    AcquireInputGate,
    ReleaseInputGate,
    WriteInjection,
    SubscribeTerminalWorkerLifecycle,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum TerminalWorkerKind {
    InputWriter,
    OutputFanout,
    OutputReader,
    ChildExitWatcher,
    SocketAcceptLoop,
    AttachConnectionPump,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalWorkerStopReason {
    InputCommandChannelClosed,
    InputWriteFailed(String),
    OutputCommandChannelClosed,
    OutputReaderFinished,
    OutputReadFailed(String),
    OutputPortClosed,
    ChildExited(String),
    ChildWaitFailed(String),
    SocketAcceptFailed(String),
    AttachConnectionClosed,
    AttachConnectionFailed(String),
}

impl NotaEncode for TerminalWorkerStopReason {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::InputCommandChannelClosed => {
                encoder.start_record("InputCommandChannelClosed")?;
                encoder.end_record()
            }
            Self::InputWriteFailed(reason) => {
                encoder.start_record("InputWriteFailed")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
            Self::OutputCommandChannelClosed => {
                encoder.start_record("OutputCommandChannelClosed")?;
                encoder.end_record()
            }
            Self::OutputReaderFinished => {
                encoder.start_record("OutputReaderFinished")?;
                encoder.end_record()
            }
            Self::OutputReadFailed(reason) => {
                encoder.start_record("OutputReadFailed")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
            Self::OutputPortClosed => {
                encoder.start_record("OutputPortClosed")?;
                encoder.end_record()
            }
            Self::ChildExited(reason) => {
                encoder.start_record("ChildExited")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
            Self::ChildWaitFailed(reason) => {
                encoder.start_record("ChildWaitFailed")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
            Self::SocketAcceptFailed(reason) => {
                encoder.start_record("SocketAcceptFailed")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
            Self::AttachConnectionClosed => {
                encoder.start_record("AttachConnectionClosed")?;
                encoder.end_record()
            }
            Self::AttachConnectionFailed(reason) => {
                encoder.start_record("AttachConnectionFailed")?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for TerminalWorkerStopReason {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "InputCommandChannelClosed" => {
                decoder.expect_record_head("InputCommandChannelClosed")?;
                decoder.expect_record_end()?;
                Ok(Self::InputCommandChannelClosed)
            }
            "InputWriteFailed" => {
                decoder.expect_record_head("InputWriteFailed")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::InputWriteFailed(reason))
            }
            "OutputCommandChannelClosed" => {
                decoder.expect_record_head("OutputCommandChannelClosed")?;
                decoder.expect_record_end()?;
                Ok(Self::OutputCommandChannelClosed)
            }
            "OutputReaderFinished" => {
                decoder.expect_record_head("OutputReaderFinished")?;
                decoder.expect_record_end()?;
                Ok(Self::OutputReaderFinished)
            }
            "OutputReadFailed" => {
                decoder.expect_record_head("OutputReadFailed")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::OutputReadFailed(reason))
            }
            "OutputPortClosed" => {
                decoder.expect_record_head("OutputPortClosed")?;
                decoder.expect_record_end()?;
                Ok(Self::OutputPortClosed)
            }
            "ChildExited" => {
                decoder.expect_record_head("ChildExited")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::ChildExited(reason))
            }
            "ChildWaitFailed" => {
                decoder.expect_record_head("ChildWaitFailed")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::ChildWaitFailed(reason))
            }
            "SocketAcceptFailed" => {
                decoder.expect_record_head("SocketAcceptFailed")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::SocketAcceptFailed(reason))
            }
            "AttachConnectionClosed" => {
                decoder.expect_record_head("AttachConnectionClosed")?;
                decoder.expect_record_end()?;
                Ok(Self::AttachConnectionClosed)
            }
            "AttachConnectionFailed" => {
                decoder.expect_record_head("AttachConnectionFailed")?;
                let reason = String::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::AttachConnectionFailed(reason))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "TerminalWorkerStopReason",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalWorkerLifecycle {
    Started(TerminalWorkerKind),
    Stopped {
        worker: TerminalWorkerKind,
        reason: TerminalWorkerStopReason,
    },
}

impl NotaEncode for TerminalWorkerLifecycle {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Started(worker) => {
                encoder.start_record("Started")?;
                worker.encode(encoder)?;
                encoder.end_record()
            }
            Self::Stopped { worker, reason } => {
                encoder.start_record("Stopped")?;
                worker.encode(encoder)?;
                reason.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for TerminalWorkerLifecycle {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Started" => {
                decoder.expect_record_head("Started")?;
                let worker = TerminalWorkerKind::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Started(worker))
            }
            "Stopped" => {
                decoder.expect_record_head("Stopped")?;
                let worker = TerminalWorkerKind::decode(decoder)?;
                let reason = TerminalWorkerStopReason::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Stopped { worker, reason })
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "TerminalWorkerLifecycle",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalWorkerLifecycleSnapshot {
    pub terminal: TerminalName,
    pub observations: Vec<TerminalWorkerLifecycle>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalWorkerLifecycleEvent {
    pub terminal: TerminalName,
    pub observation: TerminalWorkerLifecycle,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalConnection {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalInput {
    pub terminal: TerminalName,
    pub bytes: TerminalInputBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResize {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetachment {
    pub terminal: TerminalName,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCapture {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalReady {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalInputAccepted {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TranscriptDelta {
    pub terminal: TerminalName,
    pub sequence: TerminalSequence,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResized {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCaptured {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetached {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
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

impl NotaEncode for TerminalExitStatus {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Exited { code } => {
                encoder.start_record("Exited")?;
                code.encode(encoder)?;
                encoder.end_record()
            }
            Self::Signaled { signal } => {
                encoder.start_record("Signaled")?;
                signal.encode(encoder)?;
                encoder.end_record()
            }
            Self::StatusUnavailable => {
                encoder.start_record("StatusUnavailable")?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for TerminalExitStatus {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Exited" => {
                decoder.expect_record_head("Exited")?;
                let code = i32::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Exited { code })
            }
            "Signaled" => {
                decoder.expect_record_head("Signaled")?;
                let signal = i32::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Signaled { signal })
            }
            "StatusUnavailable" => {
                decoder.expect_record_head("StatusUnavailable")?;
                decoder.expect_record_end()?;
                Ok(Self::StatusUnavailable)
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "TerminalExitStatus",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalRejected {
    pub terminal: TerminalName,
    pub reason: TerminalRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
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
        RegisterPromptPattern(RegisterPromptPattern),
        UnregisterPromptPattern(UnregisterPromptPattern),
        ListPromptPatterns(ListPromptPatterns),
        AcquireInputGate(AcquireInputGate),
        ReleaseInputGate(ReleaseInputGate),
        WriteInjection(WriteInjection),
        SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle),
    }
    reply TerminalEvent {
        TerminalReady(TerminalReady),
        TerminalInputAccepted(TerminalInputAccepted),
        TranscriptDelta(TranscriptDelta),
        TerminalResized(TerminalResized),
        TerminalCaptured(TerminalCaptured),
        TerminalDetached(TerminalDetached),
        TerminalExited(TerminalExited),
        TerminalRejected(TerminalRejected),
        PromptPatternRegistered(PromptPatternRegistered),
        PromptPatternUnregistered(PromptPatternUnregistered),
        PromptPatternList(PromptPatternList),
        GateAcquired(GateAcquired),
        GateBusy(GateBusy),
        GateReleased(GateReleased),
        InjectionAck(InjectionAck),
        InjectionRejected(InjectionRejected),
        TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot),
        TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent),
    }
}

impl TerminalRequest {
    pub fn operation_kind(&self) -> TerminalOperationKind {
        match self {
            Self::TerminalConnection(_) => TerminalOperationKind::TerminalConnection,
            Self::TerminalInput(_) => TerminalOperationKind::TerminalInput,
            Self::TerminalResize(_) => TerminalOperationKind::TerminalResize,
            Self::TerminalDetachment(_) => TerminalOperationKind::TerminalDetachment,
            Self::TerminalCapture(_) => TerminalOperationKind::TerminalCapture,
            Self::RegisterPromptPattern(_) => TerminalOperationKind::RegisterPromptPattern,
            Self::UnregisterPromptPattern(_) => TerminalOperationKind::UnregisterPromptPattern,
            Self::ListPromptPatterns(_) => TerminalOperationKind::ListPromptPatterns,
            Self::AcquireInputGate(_) => TerminalOperationKind::AcquireInputGate,
            Self::ReleaseInputGate(_) => TerminalOperationKind::ReleaseInputGate,
            Self::WriteInjection(_) => TerminalOperationKind::WriteInjection,
            Self::SubscribeTerminalWorkerLifecycle(_) => {
                TerminalOperationKind::SubscribeTerminalWorkerLifecycle
            }
        }
    }
}
