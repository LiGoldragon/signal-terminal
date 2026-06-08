//! Schema-derived Signal contract for Persona terminal transport control.
//!
//! Read this crate as the public interface of the terminal control plane. The
//! harness requests terminal connection, input, resize, detachment, and
//! capture. Terminal owns prompt-pattern registration, input-gate leases,
//! programmatic injection, and worker lifecycle observation at the Persona
//! boundary, even when it implements those facts with terminal-cell primitives
//! underneath.
//!
//! Raw attached-viewer bytes are not Signal frames. They stay on the
//! terminal-cell data plane.
//!
//! `schema/lib.schema` is the source of truth. The checked-in
//! `src/schema/lib.rs` is a freshness-checked schema-rust-next artifact, not
//! handwritten vocabulary. See `ARCHITECTURE.md` for the channel's role and
//! boundaries.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub mod introspection;
pub use introspection::*;

impl TerminalName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl TerminalGeneration {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl TerminalSequence {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl TerminalByteCount {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl TerminalRows {
    pub fn as_u16(&self) -> u16 {
        *self.payload() as u16
    }

    pub fn into_u16(self) -> u16 {
        self.into_payload() as u16
    }
}

impl TerminalColumns {
    pub fn as_u16(&self) -> u16 {
        *self.payload() as u16
    }

    pub fn into_u16(self) -> u16 {
        self.into_payload() as u16
    }
}

impl TerminalInputBytes {
    pub fn as_slice(&self) -> &[u64] {
        self.payload().as_slice()
    }

    pub fn into_vec(self) -> Vec<u64> {
        self.into_payload()
    }
}

impl TerminalTranscriptBytes {
    pub fn as_slice(&self) -> &[u64] {
        self.payload().as_slice()
    }

    pub fn into_vec(self) -> Vec<u64> {
        self.into_payload()
    }
}

impl PromptPatternIdentifier {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl PromptPatternBytes {
    pub fn as_slice(&self) -> &[u64] {
        self.payload().as_slice()
    }

    pub fn into_vec(self) -> Vec<u64> {
        self.into_payload()
    }
}

impl InputGateReason {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl InputGateLeaseIdentifier {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl WirePath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl SocketMode {
    pub fn into_u32(self) -> u32 {
        self.into_payload() as u32
    }
}

impl SystemPrincipal {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl UnixUserIdentifier {
    pub fn as_u32(&self) -> u32 {
        *self.payload() as u32
    }
}

impl ExitCode {
    pub fn into_i32(self) -> i32 {
        self.into_payload() as i32
    }
}

impl TerminalSignalNumber {
    pub fn into_i32(self) -> i32 {
        self.into_payload() as i32
    }
}

impl WorkerFailureDetail {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl Input {
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
