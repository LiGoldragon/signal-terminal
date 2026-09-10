#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub type TerminalName = String;
pub type TerminalGeneration = i64;
pub type TerminalSequence = i64;
pub type TerminalByteCount = i64;
pub type TerminalRows = i64;
pub type TerminalColumns = i64;
pub type TerminalInputBytes = std::vec::Vec<i64>;
pub type TerminalTranscriptBytes = std::vec::Vec<i64>;
pub type PromptPatternIdentifier = String;
pub type PromptPatternBytes = std::vec::Vec<i64>;
pub type InputGateReason = String;
pub type InputGateLeaseIdentifier = i64;
pub type WirePath = String;
pub type SocketMode = i64;
pub type SystemPrincipal = String;
pub type UnixUserIdentifier = i64;
pub type ExitCode = i64;
pub type TerminalSignalNumber = i64;
pub type WorkerFailureDetail = String;
pub type Terminal = TerminalName;
pub type Rows = TerminalRows;
pub type Columns = TerminalColumns;
pub type Generation = TerminalGeneration;
pub type Sequence = TerminalSequence;
pub type InputBytes = TerminalInputBytes;
pub type TranscriptBytes = TerminalTranscriptBytes;
pub type PatternIdentifier = PromptPatternIdentifier;
pub type Pattern = PromptPattern;
pub type PromptPatternIdentifierSelection = std::option::Option<PromptPatternIdentifier>;
pub type Lease = InputGateLease;
pub type CurrentHolder = InputGateLeaseIdentifier;
pub type CachedHumanBytes = TerminalByteCount;
pub type Observations = std::vec::Vec<TerminalWorkerLifecycle>;
pub type Observation = TerminalWorkerLifecycle;
pub type Token = TerminalWorkerLifecycleToken;
pub type Entries = std::vec::Vec<PromptPatternEntry>;
pub type SessionEntries = std::vec::Vec<SessionEntry>;
pub type Name = TerminalName;
pub type DataSocketPath = WirePath;
pub type TerminalSocketPath = WirePath;
pub type TerminalSocketMode = SocketMode;
pub type MetaTerminalSocketPath = WirePath;
pub type MetaTerminalSocketMode = SocketMode;
pub type SupervisionSocketPath = WirePath;
pub type SupervisionSocketMode = SocketMode;
pub type StorePath = WirePath;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OwnerIdentity {
    UnixUser(UnixUserIdentifier),
    System(SystemPrincipal),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TerminalRejectionReason {
    NotConnected,
    InputRejected,
    ResizeRejected,
    CaptureRejected,
    TransportFailed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum InjectionRejectionReason {
    UnknownTerminal,
    UnknownLease,
    GateNotHeld,
    DirtyPrompt,
    TransportFailed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TerminalExitStatus {
    Exited(ExitCode),
    Signaled(TerminalSignalNumber),
    StatusUnavailable,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum PromptPattern {
    LiteralSuffix(PromptPatternBytes),
    RegexSuffix(PromptPatternBytes),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum PromptState {
    NotChecked,
    Clean,
    Dirty(TerminalByteCount),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct InputGateLease {
    pub input_gate_lease_identifier: InputGateLeaseIdentifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalWorkerStop {
    pub terminal_worker_kind: TerminalWorkerKind,
    pub terminal_worker_stop_reason: TerminalWorkerStopReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TerminalWorkerLifecycle {
    Started(TerminalWorkerKind),
    Stopped(TerminalWorkerStop),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalConnectionRequest {
    pub terminal: Terminal,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalInputRequest {
    pub terminal: Terminal,
    pub input_bytes: InputBytes,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalResizeRequest {
    pub terminal: Terminal,
    pub rows: Rows,
    pub columns: Columns,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalDetachmentRequest {
    pub terminal: Terminal,
    pub terminal_detachment_reason: TerminalDetachmentReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalCaptureRequest {
    pub terminal: Terminal,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RegisterPromptPatternRequest {
    pub terminal: Terminal,
    pub pattern: Pattern,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct UnregisterPromptPatternRequest {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ListPromptPatternsRequest {
    pub terminal: Terminal,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AcquireInputGateRequest {
    pub terminal: Terminal,
    pub input_gate_reason: InputGateReason,
    pub prompt_pattern_identifier_selection: PromptPatternIdentifierSelection,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ReleaseInputGateRequest {
    pub terminal: Terminal,
    pub lease: Lease,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct WriteInjectionRequest {
    pub terminal: Terminal,
    pub lease: Lease,
    pub input_bytes: InputBytes,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubscribeTerminalWorkerLifecycleRequest {
    pub terminal: Terminal,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalWorkerLifecycleToken {
    pub terminal: Terminal,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ListSessionsRequest {}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ResolveSessionRequest {
    pub name: Name,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalReadyReply {
    pub terminal: Terminal,
    pub generation: Generation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalInputAcceptedReply {
    pub terminal: Terminal,
    pub generation: Generation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptDeltaReply {
    pub terminal: Terminal,
    pub sequence: Sequence,
    pub transcript_bytes: TranscriptBytes,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalResizedReply {
    pub terminal: Terminal,
    pub rows: Rows,
    pub columns: Columns,
    pub generation: Generation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalCapturedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub transcript_bytes: TranscriptBytes,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalDetachedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub terminal_detachment_reason: TerminalDetachmentReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalExitedReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub terminal_exit_status: TerminalExitStatus,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalRejectedReply {
    pub terminal: Terminal,
    pub terminal_rejection_reason: TerminalRejectionReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PromptPatternRegisteredReply {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PromptPatternUnregisteredReply {
    pub terminal: Terminal,
    pub pattern_identifier: PatternIdentifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PromptPatternEntry {
    pub pattern_identifier: PatternIdentifier,
    pub pattern: Pattern,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PromptPatternListReply {
    pub terminal: Terminal,
    pub entries: Entries,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GateAcquiredReply {
    pub terminal: Terminal,
    pub lease: Lease,
    pub prompt_state: PromptState,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GateBusyReply {
    pub terminal: Terminal,
    pub current_holder: CurrentHolder,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GateReleasedReply {
    pub terminal: Terminal,
    pub lease: Lease,
    pub cached_human_bytes: CachedHumanBytes,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct InjectionAckReply {
    pub terminal: Terminal,
    pub generation: Generation,
    pub sequence: Sequence,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct InjectionRejectedReply {
    pub terminal: Terminal,
    pub injection_rejection_reason: InjectionRejectionReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalWorkerLifecycleSnapshotReply {
    pub terminal: Terminal,
    pub observations: Observations,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalWorkerLifecycleEventPayload {
    pub terminal: Terminal,
    pub observation: Observation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubscriptionRetractedReply {
    pub token: Token,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionEntry {
    pub name: Name,
    pub data_socket_path: DataSocketPath,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionListReply {
    pub session_entries: SessionEntries,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionResolvedReply {
    pub name: Name,
    pub data_socket_path: DataSocketPath,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TerminalEvent {
    TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEventPayload),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
