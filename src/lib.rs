//! Signal contract - Persona terminal transport control plane.
//!
//! Read this file as the public interface of the terminal control plane. The
//! harness requests terminal connection, input, resize, detachment, and
//! capture. Terminal owns prompt-pattern registration, input-gate
//! leases, programmatic injection, and worker lifecycle observation at the
//! Persona boundary, even when it implements those facts with terminal-cell
//! primitives underneath.
//!
//! Raw attached-viewer bytes are not Signal frames. They stay on the
//! terminal-cell data plane.
//!
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub mod introspection;
pub use introspection::*;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    NotaEncode,
    NotaDecode,
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
    NotaEncode,
    NotaDecode,
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

impl NotaDecode for TerminalInputBytes {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let values = Vec::<u64>::from_nota_block(block)?;
        let bytes = values
            .into_iter()
            .map(|value| {
                u8::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
                    value: value.to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(bytes))
    }
}

impl NotaEncode for TerminalInputBytes {
    fn to_nota(&self) -> String {
        Delimiter::SquareBracket.wrap(self.0.iter().map(u8::to_string))
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

impl NotaDecode for TerminalTranscriptBytes {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let values = Vec::<u64>::from_nota_block(block)?;
        let bytes = values
            .into_iter()
            .map(|value| {
                u8::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
                    value: value.to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(bytes))
    }
}

impl NotaEncode for TerminalTranscriptBytes {
    fn to_nota(&self) -> String {
        Delimiter::SquareBracket.wrap(self.0.iter().map(u8::to_string))
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

impl NotaDecode for TerminalRows {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let rows = u16::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(rows))
    }
}

impl NotaEncode for TerminalRows {
    fn to_nota(&self) -> String {
        self.0.to_string()
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

impl NotaDecode for TerminalColumns {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let columns = u16::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(columns))
    }
}

impl NotaEncode for TerminalColumns {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
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
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ListSessions {}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ResolveSession {
    pub name: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionEntry {
    pub name: TerminalName,
    pub data_socket_path: signal_engine_management::WirePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionList {
    pub entries: Vec<SessionEntry>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionResolved {
    pub name: TerminalName,
    pub data_socket_path: signal_engine_management::WirePath,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PromptPatternIdentifier(String);

impl PromptPatternIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

impl NotaDecode for PromptPatternBytes {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let values = Vec::<u64>::from_nota_block(block)?;
        let bytes = values
            .into_iter()
            .map(|value| {
                u8::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
                    value: value.to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(bytes))
    }
}

impl NotaEncode for PromptPatternBytes {
    fn to_nota(&self) -> String {
        Delimiter::SquareBracket.wrap(self.0.iter().map(u8::to_string))
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptPattern {
    LiteralSuffix(PromptPatternBytes),
    RegexSuffix { pattern: PromptPatternBytes },
}

impl NotaEncode for PromptPattern {
    fn to_nota(&self) -> String {
        match self {
            Self::LiteralSuffix(pattern) => {
                Delimiter::Parenthesis.wrap(["LiteralSuffix".to_owned(), pattern.to_nota()])
            }
            Self::RegexSuffix { pattern } => {
                Delimiter::Parenthesis.wrap(["RegexSuffix".to_owned(), pattern.to_nota()])
            }
        }
    }
}

impl NotaDecode for PromptPattern {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "PromptPattern", 2)?;
        let head = fields[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "PromptPattern head",
            })?;
        match head {
            "LiteralSuffix" => {
                let pattern = PromptPatternBytes::from_nota_block(&fields[1])?;
                Ok(Self::LiteralSuffix(pattern))
            }
            "RegexSuffix" => {
                let pattern = PromptPatternBytes::from_nota_block(&fields[1])?;
                Ok(Self::RegexSuffix { pattern })
            }
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "PromptPattern",
                variant: other.to_string(),
            }),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RegisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern: PromptPattern,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct UnregisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ListPromptPatterns {
    pub terminal: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PromptPatternEntry {
    pub pattern_id: PromptPatternIdentifier,
    pub pattern: PromptPattern,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PromptPatternRegistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PromptPatternUnregistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PromptPatternList {
    pub terminal: TerminalName,
    pub entries: Vec<PromptPatternEntry>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InputGateLeaseIdentifier(u64);

impl InputGateLeaseIdentifier {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InputGateLease {
    pub id: InputGateLeaseIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptState {
    NotChecked,
    Clean,
    Dirty { trailing_count: TerminalByteCount },
}

impl NotaEncode for PromptState {
    fn to_nota(&self) -> String {
        match self {
            Self::NotChecked => Delimiter::Parenthesis.wrap(["NotChecked".to_owned()]),
            Self::Clean => Delimiter::Parenthesis.wrap(["Clean".to_owned()]),
            Self::Dirty { trailing_count } => {
                Delimiter::Parenthesis.wrap(["Dirty".to_owned(), trailing_count.to_nota()])
            }
        }
    }
}

impl NotaDecode for PromptState {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let body = NotaBlock::new(block).expect_body(Delimiter::Parenthesis, "PromptState")?;
        let fields = body.root_objects();
        let head = fields.first().and_then(Block::demote_to_string).ok_or(
            NotaDecodeError::ExpectedAtom {
                type_name: "PromptState head",
            },
        )?;
        match head {
            "NotChecked" if fields.len() == 1 => Ok(Self::NotChecked),
            "Clean" if fields.len() == 1 => Ok(Self::Clean),
            "Dirty" => {
                if fields.len() != 2 {
                    return Err(NotaDecodeError::ExpectedRootCount {
                        type_name: "Dirty",
                        expected: 2,
                        found: fields.len(),
                    });
                }
                let trailing_count = TerminalByteCount::from_nota_block(&fields[1])?;
                Ok(Self::Dirty { trailing_count })
            }
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "PromptState",
                variant: other.to_string(),
            }),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AcquireInputGate {
    pub terminal: TerminalName,
    pub reason: InputGateReason,
    pub prompt_pattern_identifier: Option<PromptPatternIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReleaseInputGate {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WriteInjection {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub bytes: TerminalInputBytes,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct GateAcquired {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub prompt_state: PromptState,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct GateBusy {
    pub terminal: TerminalName,
    pub current_holder: InputGateLeaseIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct GateReleased {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub cached_human_bytes: TerminalByteCount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InjectionAck {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub sequence: TerminalSequence,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InjectionRejected {
    pub terminal: TerminalName,
    pub reason: InjectionRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum InjectionRejectionReason {
    UnknownTerminal,
    UnknownLease,
    GateNotHeld,
    DirtyPrompt,
    TransportFailed,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscribeTerminalWorkerLifecycle {
    pub terminal: TerminalName,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
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
    TerminalWorkerLifecycleRetraction,
    ListSessions,
    ResolveSession,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TerminalWorkerKind {
    InputWriter,
    ViewerFanout,
    TranscriptScriber,
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
    TranscriptNoticeChannelClosed,
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
    fn to_nota(&self) -> String {
        match self {
            Self::InputCommandChannelClosed => {
                Delimiter::Parenthesis.wrap(["InputCommandChannelClosed".to_owned()])
            }
            Self::InputWriteFailed(reason) => {
                Delimiter::Parenthesis.wrap(["InputWriteFailed".to_owned(), reason.to_nota()])
            }
            Self::OutputCommandChannelClosed => {
                Delimiter::Parenthesis.wrap(["OutputCommandChannelClosed".to_owned()])
            }
            Self::TranscriptNoticeChannelClosed => {
                Delimiter::Parenthesis.wrap(["TranscriptNoticeChannelClosed".to_owned()])
            }
            Self::OutputReaderFinished => {
                Delimiter::Parenthesis.wrap(["OutputReaderFinished".to_owned()])
            }
            Self::OutputReadFailed(reason) => {
                Delimiter::Parenthesis.wrap(["OutputReadFailed".to_owned(), reason.to_nota()])
            }
            Self::OutputPortClosed => Delimiter::Parenthesis.wrap(["OutputPortClosed".to_owned()]),
            Self::ChildExited(reason) => {
                Delimiter::Parenthesis.wrap(["ChildExited".to_owned(), reason.to_nota()])
            }
            Self::ChildWaitFailed(reason) => {
                Delimiter::Parenthesis.wrap(["ChildWaitFailed".to_owned(), reason.to_nota()])
            }
            Self::SocketAcceptFailed(reason) => {
                Delimiter::Parenthesis.wrap(["SocketAcceptFailed".to_owned(), reason.to_nota()])
            }
            Self::AttachConnectionClosed => {
                Delimiter::Parenthesis.wrap(["AttachConnectionClosed".to_owned()])
            }
            Self::AttachConnectionFailed(reason) => {
                Delimiter::Parenthesis.wrap(["AttachConnectionFailed".to_owned(), reason.to_nota()])
            }
        }
    }
}

impl NotaDecode for TerminalWorkerStopReason {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let body = NotaBlock::new(block)
            .expect_body(Delimiter::Parenthesis, "TerminalWorkerStopReason")?;
        let fields = body.root_objects();
        let head = fields.first().and_then(Block::demote_to_string).ok_or(
            NotaDecodeError::ExpectedAtom {
                type_name: "TerminalWorkerStopReason head",
            },
        )?;
        match head {
            "InputCommandChannelClosed" if fields.len() == 1 => Ok(Self::InputCommandChannelClosed),
            "InputWriteFailed" => {
                let reason = Self::decode_single_string_payload("InputWriteFailed", fields)?;
                Ok(Self::InputWriteFailed(reason))
            }
            "OutputCommandChannelClosed" if fields.len() == 1 => {
                Ok(Self::OutputCommandChannelClosed)
            }
            "TranscriptNoticeChannelClosed" => {
                Self::expect_unit("TranscriptNoticeChannelClosed", fields)?;
                Ok(Self::TranscriptNoticeChannelClosed)
            }
            "OutputReaderFinished" if fields.len() == 1 => Ok(Self::OutputReaderFinished),
            "OutputReadFailed" => {
                let reason = Self::decode_single_string_payload("OutputReadFailed", fields)?;
                Ok(Self::OutputReadFailed(reason))
            }
            "OutputPortClosed" if fields.len() == 1 => Ok(Self::OutputPortClosed),
            "ChildExited" => {
                let reason = Self::decode_single_string_payload("ChildExited", fields)?;
                Ok(Self::ChildExited(reason))
            }
            "ChildWaitFailed" => {
                let reason = Self::decode_single_string_payload("ChildWaitFailed", fields)?;
                Ok(Self::ChildWaitFailed(reason))
            }
            "SocketAcceptFailed" => {
                let reason = Self::decode_single_string_payload("SocketAcceptFailed", fields)?;
                Ok(Self::SocketAcceptFailed(reason))
            }
            "AttachConnectionClosed" if fields.len() == 1 => Ok(Self::AttachConnectionClosed),
            "AttachConnectionFailed" => {
                let reason = Self::decode_single_string_payload("AttachConnectionFailed", fields)?;
                Ok(Self::AttachConnectionFailed(reason))
            }
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "TerminalWorkerStopReason",
                variant: other.to_string(),
            }),
        }
    }
}

impl TerminalWorkerStopReason {
    fn expect_unit(type_name: &'static str, fields: &[Block]) -> Result<(), NotaDecodeError> {
        if fields.len() == 1 {
            return Ok(());
        }
        Err(NotaDecodeError::ExpectedRootCount {
            type_name,
            expected: 1,
            found: fields.len(),
        })
    }

    fn decode_single_string_payload(
        type_name: &'static str,
        fields: &[Block],
    ) -> Result<String, NotaDecodeError> {
        if fields.len() != 2 {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name,
                expected: 2,
                found: fields.len(),
            });
        }
        String::from_nota_block(&fields[1])
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
    fn to_nota(&self) -> String {
        match self {
            Self::Started(worker) => {
                Delimiter::Parenthesis.wrap(["Started".to_owned(), worker.to_nota()])
            }
            Self::Stopped { worker, reason } => Delimiter::Parenthesis.wrap([
                "Stopped".to_owned(),
                worker.to_nota(),
                reason.to_nota(),
            ]),
        }
    }
}

impl NotaDecode for TerminalWorkerLifecycle {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let body =
            NotaBlock::new(block).expect_body(Delimiter::Parenthesis, "TerminalWorkerLifecycle")?;
        let fields = body.root_objects();
        let head = fields.first().and_then(Block::demote_to_string).ok_or(
            NotaDecodeError::ExpectedAtom {
                type_name: "TerminalWorkerLifecycle head",
            },
        )?;
        match head {
            "Started" => {
                if fields.len() != 2 {
                    return Err(NotaDecodeError::ExpectedRootCount {
                        type_name: "Started",
                        expected: 2,
                        found: fields.len(),
                    });
                }
                let worker = TerminalWorkerKind::from_nota_block(&fields[1])?;
                Ok(Self::Started(worker))
            }
            "Stopped" => {
                if fields.len() != 3 {
                    return Err(NotaDecodeError::ExpectedRootCount {
                        type_name: "Stopped",
                        expected: 3,
                        found: fields.len(),
                    });
                }
                let worker = TerminalWorkerKind::from_nota_block(&fields[1])?;
                let reason = TerminalWorkerStopReason::from_nota_block(&fields[2])?;
                Ok(Self::Stopped { worker, reason })
            }
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "TerminalWorkerLifecycle",
                variant: other.to_string(),
            }),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalWorkerLifecycleSnapshot {
    pub terminal: TerminalName,
    pub observations: Vec<TerminalWorkerLifecycle>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalWorkerLifecycleEvent {
    pub terminal: TerminalName,
    pub observation: TerminalWorkerLifecycle,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalConnection {
    pub terminal: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalInput {
    pub terminal: TerminalName,
    pub bytes: TerminalInputBytes,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalResize {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalDetachment {
    pub terminal: TerminalName,
    pub reason: TerminalDetachmentReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalCapture {
    pub terminal: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalReady {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalInputAccepted {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptDelta {
    pub terminal: TerminalName,
    pub sequence: TerminalSequence,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalResized {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
    pub generation: TerminalGeneration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalCaptured {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalDetached {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub reason: TerminalDetachmentReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
    fn to_nota(&self) -> String {
        match self {
            Self::Exited { code } => {
                Delimiter::Parenthesis.wrap(["Exited".to_owned(), code.to_string()])
            }
            Self::Signaled { signal } => {
                Delimiter::Parenthesis.wrap(["Signaled".to_owned(), signal.to_string()])
            }
            Self::StatusUnavailable => {
                Delimiter::Parenthesis.wrap(["StatusUnavailable".to_owned()])
            }
        }
    }
}

impl NotaDecode for TerminalExitStatus {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let body =
            NotaBlock::new(block).expect_body(Delimiter::Parenthesis, "TerminalExitStatus")?;
        let fields = body.root_objects();
        let head = fields.first().and_then(Block::demote_to_string).ok_or(
            NotaDecodeError::ExpectedAtom {
                type_name: "TerminalExitStatus head",
            },
        )?;
        match head {
            "Exited" => {
                let code = Self::decode_signed_payload("Exited", fields)?;
                Ok(Self::Exited { code })
            }
            "Signaled" => {
                let signal = Self::decode_signed_payload("Signaled", fields)?;
                Ok(Self::Signaled { signal })
            }
            "StatusUnavailable" if fields.len() == 1 => Ok(Self::StatusUnavailable),
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "TerminalExitStatus",
                variant: other.to_string(),
            }),
        }
    }
}

impl TerminalExitStatus {
    fn decode_signed_payload(
        type_name: &'static str,
        fields: &[Block],
    ) -> Result<i32, NotaDecodeError> {
        if fields.len() != 2 {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name,
                expected: 2,
                found: fields.len(),
            });
        }
        let value = fields[1]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom { type_name })?;
        value
            .parse::<i32>()
            .map_err(|_| NotaDecodeError::InvalidInteger {
                value: value.to_string(),
            })
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalRejected {
    pub terminal: TerminalName,
    pub reason: TerminalRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TerminalRejectionReason {
    NotConnected,
    InputRejected,
    ResizeRejected,
    CaptureRejected,
    TransportFailed,
}

/// Per-subscription identity for the terminal worker lifecycle stream.
/// Matches the structural shape of `<Channel>SubscriptionToken` newtypes
/// per /176 §1 stream-block grammar.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalWorkerLifecycleToken {
    pub terminal: TerminalName,
}

/// Typed acknowledgement that a worker-lifecycle subscription has been
/// retracted. Returned in reply to `TerminalWorkerLifecycleRetraction`.
/// Carries the retracted token so callers can match the ack to the
/// request they sent.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscriptionRetracted {
    pub token: TerminalWorkerLifecycleToken,
}

signal_channel! {
    channel Terminal {
        operation TerminalConnection(TerminalConnection),
        operation TerminalInput(TerminalInput),
        operation TerminalResize(TerminalResize),
        operation TerminalDetachment(TerminalDetachment),
        operation TerminalCapture(TerminalCapture),
        operation RegisterPromptPattern(RegisterPromptPattern),
        operation UnregisterPromptPattern(UnregisterPromptPattern),
        operation ListPromptPatterns(ListPromptPatterns),
        operation AcquireInputGate(AcquireInputGate),
        operation ReleaseInputGate(ReleaseInputGate),
        operation WriteInjection(WriteInjection),
        operation SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle) opens TerminalWorkerLifecycleStream,
        operation TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken),
        operation ListSessions(ListSessions),
        operation ResolveSession(ResolveSession),
    }
    reply TerminalReply {
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
        SubscriptionRetracted(SubscriptionRetracted),
        SessionList(SessionList),
        SessionResolved(SessionResolved),
    }
    event TerminalEvent {
        TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent) belongs TerminalWorkerLifecycleStream,
    }
    stream TerminalWorkerLifecycleStream {
        token TerminalWorkerLifecycleToken;
        opened TerminalWorkerLifecycleSnapshot;
        event TerminalWorkerLifecycleEvent;
        close TerminalWorkerLifecycleRetraction;
    }
}

pub type TerminalRequest = Operation;
pub type TerminalFrame = Frame;
pub type TerminalFrameBody = FrameBody;
pub type TerminalRequestBuilder = RequestBuilder;
pub type TerminalStreamKind = StreamKind;

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
            Self::TerminalWorkerLifecycleRetraction(_) => {
                TerminalOperationKind::TerminalWorkerLifecycleRetraction
            }
            Self::ListSessions(_) => TerminalOperationKind::ListSessions,
            Self::ResolveSession(_) => TerminalOperationKind::ResolveSession,
        }
    }
}

// And one for the event enum, used by daemon code that constructs
// events before handing them off to the streaming-event emit path.
impl From<TerminalWorkerLifecycleEvent> for TerminalEvent {
    fn from(payload: TerminalWorkerLifecycleEvent) -> Self {
        Self::TerminalWorkerLifecycleEvent(payload)
    }
}

// Hand-written From<Payload> for TerminalRequest. Same pattern — every
// request variant carries a unique payload type per /176 §3.
impl From<TerminalConnection> for TerminalRequest {
    fn from(p: TerminalConnection) -> Self {
        Self::TerminalConnection(p)
    }
}
impl From<TerminalInput> for TerminalRequest {
    fn from(p: TerminalInput) -> Self {
        Self::TerminalInput(p)
    }
}
impl From<TerminalResize> for TerminalRequest {
    fn from(p: TerminalResize) -> Self {
        Self::TerminalResize(p)
    }
}
impl From<TerminalDetachment> for TerminalRequest {
    fn from(p: TerminalDetachment) -> Self {
        Self::TerminalDetachment(p)
    }
}
impl From<TerminalCapture> for TerminalRequest {
    fn from(p: TerminalCapture) -> Self {
        Self::TerminalCapture(p)
    }
}
impl From<RegisterPromptPattern> for TerminalRequest {
    fn from(p: RegisterPromptPattern) -> Self {
        Self::RegisterPromptPattern(p)
    }
}
impl From<UnregisterPromptPattern> for TerminalRequest {
    fn from(p: UnregisterPromptPattern) -> Self {
        Self::UnregisterPromptPattern(p)
    }
}
impl From<ListPromptPatterns> for TerminalRequest {
    fn from(p: ListPromptPatterns) -> Self {
        Self::ListPromptPatterns(p)
    }
}
impl From<AcquireInputGate> for TerminalRequest {
    fn from(p: AcquireInputGate) -> Self {
        Self::AcquireInputGate(p)
    }
}
impl From<ReleaseInputGate> for TerminalRequest {
    fn from(p: ReleaseInputGate) -> Self {
        Self::ReleaseInputGate(p)
    }
}
impl From<WriteInjection> for TerminalRequest {
    fn from(p: WriteInjection) -> Self {
        Self::WriteInjection(p)
    }
}
impl From<SubscribeTerminalWorkerLifecycle> for TerminalRequest {
    fn from(p: SubscribeTerminalWorkerLifecycle) -> Self {
        Self::SubscribeTerminalWorkerLifecycle(p)
    }
}
impl From<TerminalWorkerLifecycleToken> for TerminalRequest {
    fn from(p: TerminalWorkerLifecycleToken) -> Self {
        Self::TerminalWorkerLifecycleRetraction(p)
    }
}
impl From<ListSessions> for TerminalRequest {
    fn from(payload: ListSessions) -> Self {
        Self::ListSessions(payload)
    }
}
impl From<ResolveSession> for TerminalRequest {
    fn from(payload: ResolveSession) -> Self {
        Self::ResolveSession(payload)
    }
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `terminal-supervisor`.
// Deploy/test tooling may render this as NOTA, but a daemon receives it
// as a signal-encoded/rkyv file and never parses NOTA at startup.

/// Startup configuration for `terminal-supervisor`.
///
/// The generated process needs one binary configuration carrying its
/// ordinary, meta, and supervision socket locations before it can bind any
/// listener. This launch record is not a public terminal operation.
///
/// Replaces the previous `--socket`, `--store`,
/// `PERSONA_SOCKET_PATH`, `TERMINAL_STORE`,
/// `PERSONA_STATE_PATH`, `PERSONA_SOCKET_MODE`,
/// `PERSONA_SUPERVISION_SOCKET_PATH`, and
/// `PERSONA_SUPERVISION_SOCKET_MODE` argv/environment-variable
/// surface.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalDaemonConfiguration {
    /// Where the supervisor binds its terminal Unix socket.
    pub terminal_socket_path: signal_engine_management::WirePath,
    /// chmod applied to the terminal socket after bind.
    pub terminal_socket_mode: signal_engine_management::SocketMode,
    /// Where the supervisor binds its privileged meta terminal Unix socket.
    pub meta_terminal_socket_path: signal_engine_management::WirePath,
    /// chmod applied to the meta terminal socket after bind.
    pub meta_terminal_socket_mode: signal_engine_management::SocketMode,
    /// Where the supervisor binds its supervision Unix socket.
    pub supervision_socket_path: signal_engine_management::WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: signal_engine_management::SocketMode,
    /// Path to the terminal supervisor's sema-engine store file.
    pub store_path: signal_engine_management::WirePath,
    /// The engine owner identity passed to the terminal supervisor.
    pub owner_identity: signal_persona_origin::OwnerIdentity,
}

impl TerminalDaemonConfiguration {
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, TerminalDaemonConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| TerminalDaemonConfigurationArchiveError::Decode)
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, TerminalDaemonConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| TerminalDaemonConfigurationArchiveError::Encode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TerminalDaemonConfigurationArchiveError {
    #[error("failed to encode terminal daemon configuration archive")]
    Encode,

    #[error("failed to decode terminal daemon configuration archive")]
    Decode,
}
