#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type TerminalName = String;
#[rustfmt::skip]
pub type TerminalGeneration = i64;
#[rustfmt::skip]
pub type TerminalSequence = i64;
#[rustfmt::skip]
pub type TerminalByteCount = i64;
#[rustfmt::skip]
pub type TerminalRows = i64;
#[rustfmt::skip]
pub type TerminalColumns = i64;
#[rustfmt::skip]
pub type TerminalInputBytes = std::vec::Vec<i64>;
#[rustfmt::skip]
pub type TerminalTranscriptBytes = std::vec::Vec<i64>;
#[rustfmt::skip]
pub type PromptPatternIdentifier = String;
#[rustfmt::skip]
pub type PromptPatternBytes = std::vec::Vec<i64>;
#[rustfmt::skip]
pub type InputGateReason = String;
#[rustfmt::skip]
pub type InputGateLeaseIdentifier = i64;
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
pub type SystemPrincipal = String;
#[rustfmt::skip]
pub type UnixUserIdentifier = i64;
#[rustfmt::skip]
pub type ExitCode = i64;
#[rustfmt::skip]
pub type TerminalSignalNumber = i64;
#[rustfmt::skip]
pub type WorkerFailureDetail = String;
#[rustfmt::skip]
pub type Terminal = TerminalName;
#[rustfmt::skip]
pub type Rows = TerminalRows;
#[rustfmt::skip]
pub type Columns = TerminalColumns;
#[rustfmt::skip]
pub type Generation = TerminalGeneration;
#[rustfmt::skip]
pub type Sequence = TerminalSequence;
#[rustfmt::skip]
pub type InputBytes = TerminalInputBytes;
#[rustfmt::skip]
pub type TranscriptBytes = TerminalTranscriptBytes;
#[rustfmt::skip]
pub type PatternIdentifier = PromptPatternIdentifier;
#[rustfmt::skip]
pub type Pattern = PromptPattern;
#[rustfmt::skip]
pub type PromptPatternIdentifierSelection = std::option::Option<PromptPatternIdentifier>;
#[rustfmt::skip]
pub type Lease = InputGateLease;
#[rustfmt::skip]
pub type CurrentHolder = InputGateLeaseIdentifier;
#[rustfmt::skip]
pub type CachedHumanBytes = TerminalByteCount;
#[rustfmt::skip]
pub type Observations = std::vec::Vec<TerminalWorkerLifecycle>;
#[rustfmt::skip]
pub type Observation = TerminalWorkerLifecycle;
#[rustfmt::skip]
pub type Token = TerminalWorkerLifecycleToken;
#[rustfmt::skip]
pub type Entries = std::vec::Vec<PromptPatternEntry>;
#[rustfmt::skip]
pub type SessionEntries = std::vec::Vec<SessionEntry>;
#[rustfmt::skip]
pub type Name = TerminalName;
#[rustfmt::skip]
pub type DataSocketPath = WirePath;
#[rustfmt::skip]
pub type TerminalSocketPath = WirePath;
#[rustfmt::skip]
pub type TerminalSocketMode = SocketMode;
#[rustfmt::skip]
pub type MetaTerminalSocketPath = WirePath;
#[rustfmt::skip]
pub type MetaTerminalSocketMode = SocketMode;
#[rustfmt::skip]
pub type SupervisionSocketPath = WirePath;
#[rustfmt::skip]
pub type SupervisionSocketMode = SocketMode;
#[rustfmt::skip]
pub type StorePath = WirePath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OwnerIdentity {
    UnixUser(UnixUserIdentifier),
    System(SystemPrincipal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalRejectionReason {
    NotConnected,
    InputRejected,
    ResizeRejected,
    CaptureRejected,
    TransportFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InjectionRejectionReason {
    UnknownTerminal,
    UnknownLease,
    GateNotHeld,
    DirtyPrompt,
    TransportFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalExitStatus {
    Exited(ExitCode),
    Signaled(TerminalSignalNumber),
    StatusUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptPattern {
    LiteralSuffix(PromptPatternBytes),
    RegexSuffix(PromptPatternBytes),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptState {
    NotChecked,
    Clean,
    Dirty(TerminalByteCount),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InputGateLease {
    pub input_gate_lease_identifier: InputGateLeaseIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalWorkerKind {
    InputWriter,
    ViewerFanout,
    TranscriptScriber,
    OutputReader,
    ChildExitWatcher,
    SocketAcceptLoop,
    AttachConnectionPump,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalWorkerStopReason {
    InputCommandChannelClosed,
    InputWriteFailed(WorkerFailureDetail),
    OutputCommandChannelClosed,
    TranscriptNoticeChannelClosed,
    OutputReaderFinished,
    OutputReadFailed(WorkerFailureDetail),
    OutputPortClosed,
    ChildExited(WorkerFailureDetail),
    ChildWaitFailed(WorkerFailureDetail),
    SocketAcceptFailed(WorkerFailureDetail),
    AttachConnectionClosed,
    AttachConnectionFailed(WorkerFailureDetail),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalWorkerStop {
    pub terminal_worker_kind: TerminalWorkerKind,
    pub terminal_worker_stop_reason: TerminalWorkerStopReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalWorkerLifecycle {
    Started(TerminalWorkerKind),
    Stopped(TerminalWorkerStop),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalConnectionRequest {
    pub terminal: Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalInputRequest {
    pub terminal: Terminal,
    pub input_bytes: InputBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalResizeRequest {
    pub terminal: Terminal,
    pub rows: Rows,
    pub columns: Columns,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalDetachmentRequest {
    pub terminal: Terminal,
    pub terminal_detachment_reason: TerminalDetachmentReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalCaptureRequest {
    pub terminal: Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RegisterPromptPatternRequest {
    pub terminal: Terminal,
    pub pattern: Pattern,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UnregisterPromptPatternRequest {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ListPromptPatternsRequest {
    pub terminal: Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AcquireInputGateRequest {
    pub terminal: Terminal,
    pub input_gate_reason: InputGateReason,
    pub prompt_pattern_identifier_selection: PromptPatternIdentifierSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReleaseInputGateRequest {
    pub terminal: Terminal,
    pub lease: Lease,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WriteInjectionRequest {
    pub terminal: Terminal,
    pub lease: Lease,
    pub input_bytes: InputBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscribeTerminalWorkerLifecycleRequest {
    pub terminal: Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalWorkerLifecycleToken {
    pub terminal: Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ListSessionsRequest {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ResolveSessionRequest {
    pub name: Name,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalReadyReply {
    pub terminal: Terminal,
    pub generation: Generation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalInputAcceptedReply {
    pub terminal: Terminal,
    pub generation: Generation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptDeltaReply {
    pub terminal: Terminal,
    pub sequence: Sequence,
    pub transcript_bytes: TranscriptBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalResizedReply {
    pub terminal: Terminal,
    pub rows: Rows,
    pub columns: Columns,
    pub generation: Generation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalCapturedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub transcript_bytes: TranscriptBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalDetachedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub terminal_detachment_reason: TerminalDetachmentReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalExitedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub terminal_exit_status: TerminalExitStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalRejectedReply {
    pub terminal: Terminal,
    pub terminal_rejection_reason: TerminalRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptPatternRegisteredReply {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptPatternUnregisteredReply {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptPatternEntry {
    pub pattern_identifier: PatternIdentifier,
    pub pattern: Pattern,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptPatternListReply {
    pub terminal: Terminal,
    pub entries: Entries,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GateAcquiredReply {
    pub terminal: Terminal,
    pub lease: Lease,
    pub prompt_state: PromptState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GateBusyReply {
    pub terminal: Terminal,
    pub current_holder: CurrentHolder,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GateReleasedReply {
    pub terminal: Terminal,
    pub lease: Lease,
    pub cached_human_bytes: CachedHumanBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InjectionAckReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub sequence: Sequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InjectionRejectedReply {
    pub terminal: Terminal,
    pub injection_rejection_reason: InjectionRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalWorkerLifecycleSnapshotReply {
    pub terminal: Terminal,
    pub observations: Observations,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalWorkerLifecycleEventPayload {
    pub terminal: Terminal,
    pub observation: Observation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionRetractedReply {
    pub token: Token,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionEntry {
    pub name: Name,
    pub data_socket_path: DataSocketPath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionListReply {
    pub session_entries: SessionEntries,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionResolvedReply {
    pub name: Name,
    pub data_socket_path: DataSocketPath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TerminalEvent {
    TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEventPayload),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalDaemonConfiguration {
    pub terminal_socket_path: TerminalSocketPath,
    pub terminal_socket_mode: TerminalSocketMode,
    pub meta_terminal_socket_path: MetaTerminalSocketPath,
    pub meta_terminal_socket_mode: MetaTerminalSocketMode,
    pub supervision_socket_path: SupervisionSocketPath,
    pub supervision_socket_mode: SupervisionSocketMode,
    pub store_path: StorePath,
    pub owner_identity: OwnerIdentity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    TerminalConnection(TerminalConnectionRequest),
    TerminalInput(TerminalInputRequest),
    TerminalResize(TerminalResizeRequest),
    TerminalDetachment(TerminalDetachmentRequest),
    TerminalCapture(TerminalCaptureRequest),
    RegisterPromptPattern(RegisterPromptPatternRequest),
    UnregisterPromptPattern(UnregisterPromptPatternRequest),
    ListPromptPatterns(ListPromptPatternsRequest),
    AcquireInputGate(AcquireInputGateRequest),
    ReleaseInputGate(ReleaseInputGateRequest),
    WriteInjection(WriteInjectionRequest),
    SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycleRequest),
    TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken),
    ListSessions(ListSessionsRequest),
    ResolveSession(ResolveSessionRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    TerminalReady(TerminalReadyReply),
    TerminalInputAccepted(TerminalInputAcceptedReply),
    TranscriptDelta(TranscriptDeltaReply),
    TerminalResized(TerminalResizedReply),
    TerminalCaptured(TerminalCapturedReply),
    TerminalDetached(TerminalDetachedReply),
    TerminalExited(TerminalExitedReply),
    TerminalRejected(TerminalRejectedReply),
    PromptPatternRegistered(PromptPatternRegisteredReply),
    PromptPatternUnregistered(PromptPatternUnregisteredReply),
    PromptPatternList(PromptPatternListReply),
    GateAcquired(GateAcquiredReply),
    GateBusy(GateBusyReply),
    GateReleased(GateReleasedReply),
    InjectionAck(InjectionAckReply),
    InjectionRejected(InjectionRejectedReply),
    TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshotReply),
    SubscriptionRetracted(SubscriptionRetractedReply),
    SessionList(SessionListReply),
    SessionResolved(SessionResolvedReply),
    Event(TerminalEvent),
}
