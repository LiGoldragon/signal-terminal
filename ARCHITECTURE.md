# ARCHITECTURE - signal-persona-terminal

Signal contract for Persona terminal transport control.

`persona-terminal` is the transport owner. This crate defines the closed
control-plane records that other Persona components use to ask the transport
owner for terminal work. The raw attached-viewer byte plane is intentionally
outside this contract: PTY bytes, socket bytes, and viewer pump bytes stay in
`terminal-cell` / `persona-terminal` implementation code and are not wrapped in
Signal frames.

The control channel is one `signal_channel!` invocation in `src/lib.rs`.
Terminal-owned introspection records live in `src/introspection.rs`; they are
record vocabulary, not a second runtime owner.

## Channel

| Side | Component |
|---|---|
| Request side | Persona components that need terminal transport, currently `persona-harness` and router delivery adapters |
| Event side | `persona-terminal` |

There are two control surfaces:

- Harness transport: `persona-harness` requests connection, input, resize,
  detachment, and capture vectors. `persona-terminal` emits readiness, input
  acceptance, transcript, resize, detachment, capture, exit, and rejection
  events.
- Terminal control: `persona-terminal` owns prompt-pattern registry, input gate
  leases, write injection acknowledgements, and worker lifecycle observations.
  It may implement those facts on top of `terminal-cell` primitives, but
  `terminal-cell` is not the Persona-facing contract endpoint.

The steady-state flow is pushed by the transport owner. Harnesses and callers do
not poll for transcript or lifecycle state.

## Record Source

Records local to this contract:

- `TerminalName`
- `TerminalGeneration`
- `TerminalSequence`
- `TerminalInputBytes`
- `TerminalTranscriptBytes`
- `TerminalRows`
- `TerminalColumns`
- `TerminalByteCount`
- `PromptPatternId`
- `PromptPatternBytes`
- `PromptPattern`
- `RegisterPromptPattern`
- `UnregisterPromptPattern`
- `ListPromptPatterns`
- `PromptPatternEntry`
- `PromptPatternRegistered`
- `PromptPatternUnregistered`
- `PromptPatternList`
- `InputGateReason`
- `InputGateLeaseId`
- `InputGateLease`
- `PromptState`
- `AcquireInputGate`
- `ReleaseInputGate`
- `WriteInjection`
- `GateAcquired`
- `GateBusy`
- `GateReleased`
- `InjectionAck`
- `InjectionRejected`
- `InjectionRejectionReason`
- `SubscribeTerminalWorkerLifecycle`
- `TerminalWorkerKind`
- `TerminalWorkerStopReason`
- `TerminalWorkerLifecycle`
- `TerminalWorkerLifecycleSnapshot`
- `TerminalWorkerLifecycleEvent`
- `TerminalConnection`
- `TerminalInput`
- `TerminalResize`
- `TerminalDetachment`
- `TerminalCapture`
- `TerminalReady`
- `TerminalInputAccepted`
- `TranscriptDelta`
- `TerminalResized`
- `TerminalCaptured`
- `TerminalDetached`
- `TerminalExited`
- `TerminalRejected`
- `TerminalObservationSequence`
- `TerminalSocketPath`
- `TerminalViewerName`
- `TerminalArchiveReason`
- `TerminalSessionState`
- `TerminalSessionObservation`
- `TerminalDeliveryAttemptState`
- `TerminalDeliveryAttemptObservation`
- `TerminalEventObservation`
- `TerminalViewerAttachmentState`
- `TerminalViewerAttachmentObservation`
- `TerminalSessionHealthObservation`
- `TerminalSessionArchiveState`
- `TerminalSessionArchiveObservation`
- `TerminalIntrospectionSnapshot`

The records are terminal-transport vocabulary. They are not router, message,
auth, or terminal raw-data records.

## Messages

```text
TerminalRequest                 TerminalEvent
├─ TerminalConnection           ├─ TerminalReady
├─ TerminalInput                ├─ TerminalInputAccepted
├─ TerminalResize               ├─ TerminalResized
├─ TerminalDetachment           ├─ TerminalCaptured
├─ TerminalCapture              ├─ TranscriptDelta
├─ RegisterPromptPattern        ├─ TerminalDetached
├─ UnregisterPromptPattern      ├─ TerminalExited
├─ ListPromptPatterns           ├─ TerminalRejected
├─ AcquireInputGate             ├─ PromptPatternRegistered
├─ ReleaseInputGate             ├─ PromptPatternUnregistered
├─ WriteInjection               ├─ PromptPatternList
└─ SubscribeTerminalWorker...   ├─ GateAcquired
                                ├─ GateBusy
                                ├─ GateReleased
                                ├─ InjectionAck
                                ├─ InjectionRejected
                                ├─ TerminalRequestUnimplemented
                                ├─ TerminalWorkerLifecycleSnapshot
                                └─ TerminalWorkerLifecycleEvent
```

Closed enums; typed rejection reasons; no string-tagged event kinds.

### Skeleton honesty (Unimplemented event)

Per
`~/primary/reports/designer/143-prototype-readiness-gap-audit.md` §4.3:

```text
TerminalUnimplementedReason
  | NotInPrototypeScope
  | DependencyMissing(DependencyKind)
  | ResourceUnavailable(ResourceKind)

TerminalRequestUnimplemented
  | terminal:    TerminalName
  | operation:   TerminalOperationKind          (closed enum mirroring TerminalRequest variants)
  | reason:      TerminalUnimplementedReason
```

When a `TerminalRequest` variant has no built behavior yet, `persona-terminal`
emits `TerminalRequestUnimplemented` rather than panicking or producing a
generic rejection.

### Injection ordering

Per
`~/primary/reports/designer/143-prototype-readiness-gap-audit.md` §3 (Agent C
terminal §7): `WriteInjection` carries an `injection_sequence: u64` so the
gate-lease holder's writes are sequenced. Out-of-order use returns
`InjectionRejectionReason::InvalidSequence`.

```text
WriteInjection
  | terminal:          TerminalName
  | lease:             InputGateLease
  | injection_sequence: u64
  | bytes:             TerminalInputBytes
```

### `TerminalName` namespace scope

`TerminalName` identifies a supervised terminal session. For the prototype,
the canonical scope is "one role per name" — `TerminalName::new("operator")`,
`TerminalName::new("designer")`, etc. Future cases where multiple harnesses
share a role get a richer namespace; until then, the name space matches the
role-name vocabulary in `signal-persona-mind::RoleName`.

## Terminal-Cell Control

Prompt pattern records let a caller register the terminal-ready shape that makes
write injection safe to attempt. Input gate records make the exclusive write
lease explicit and include prompt state in the acquisition reply. Write
injection records acknowledge the terminal generation and sequence produced by a
successful write. Worker lifecycle records expose transport task start/stop
observations as typed events.

This contract does not decide whether a write should happen. It only carries the
transport control facts needed by `persona-terminal` and its consumers.

## Introspection Records

Terminal durable Sema rows that need to be inspectable outside
`persona-terminal` have typed record shapes in this contract. The component
still owns its redb file, table declarations, reducers, consistency model, and
redaction policy. `persona-introspect` asks the running component for these
records; it does not open `persona-terminal`'s database directly.

`TerminalIntrospectionSnapshot` is the prototype projection bundle over:

- terminal session observations;
- delivery attempt observations;
- terminal event observations;
- viewer attachment observations;
- session health observations;
- session archive observations.

These records are not router, harness, message, or terminal-cell records. They
name terminal-owned inspectable state at the Persona terminal boundary.

## Versioning

`signal_core::Frame` carries the protocol version. Schema-level changes are
breaking; coordinate `persona-harness` and terminal transport upgrades.

## Examples

```text
;; harness -> terminal: connect a named terminal session
TerminalRequest::TerminalConnection(TerminalConnection {
    terminal: TerminalName::new("operator"),
})

;; harness -> terminal: write bytes to the session PTY
TerminalRequest::TerminalInput(TerminalInput {
    terminal: TerminalName::new("operator"),
    bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
})

;; terminal -> harness: initial current-state event
TerminalEvent::TerminalReady(TerminalReady {
    terminal: TerminalName::new("operator"),
    generation: TerminalGeneration::new(1),
})

;; terminal -> harness: pushed output bytes
TerminalEvent::TranscriptDelta(TranscriptDelta {
    terminal: TerminalName::new("operator"),
    sequence: TerminalSequence::new(7),
    bytes: TerminalTranscriptBytes::new(b"hello\r\n".to_vec()),
})

;; caller -> terminal: acquire exclusive write access after prompt check
TerminalRequest::AcquireInputGate(AcquireInputGate {
    terminal: TerminalName::new("operator"),
    reason: InputGateReason::new("send router-delivered command"),
    prompt_pattern_id: Some(PromptPatternId::new("shell-ready")),
})

;; terminal -> caller: gate acquired and prompt was clean
TerminalEvent::GateAcquired(GateAcquired {
    terminal: TerminalName::new("operator"),
    lease: InputGateLease::new(42),
    prompt_state: PromptState::Clean,
})
```

## Round Trips

Round-trip tests in `tests/round_trip.rs` cover every request variant, every
event variant, typed rejection reasons, and representative `From` impl
witnesses. Representative NOTA text witnesses cover prompt pattern
registration, input gate acquisition, gate acquisition events, and worker
lifecycle snapshots. Manual codec enums are exercised through those witnesses
plus per-variant frame round trips.

## Non-ownership

- No terminal daemon. That is `persona-terminal`.
- No harness actor. That is `persona-harness`.
- No router delivery policy. That is `persona-router`.
- No OS focus policy. That is `persona-system`.
- No terminal-cell daemon. That is `terminal-cell`, behind `persona-terminal`.
- No prompt interpretation or delivery policy. That belongs in the caller and
  transport owner, not this contract.
- No raw PTY/viewer byte data plane.
- No transport loop, reconnect policy, or socket path.

## Code Map

```text
src/
├── lib.rs           - control payloads + signal_channel! invocation
└── introspection.rs - terminal-owned inspectable-state record shapes
tests/
├── round_trip.rs    - per-variant frame round trips + NOTA text witnesses
└── introspection.rs - rkyv + NOTA witnesses for inspection records
```

## See Also

- `persona-harness/ARCHITECTURE.md`
- `persona-terminal/ARCHITECTURE.md`
- `persona-router/ARCHITECTURE.md`
- `terminal-cell/ARCHITECTURE.md`
- `~/primary/reports/designer/127-decisions-resolved-2026-05-11.md`
- `~/primary/reports/designer-assistant/18-current-persona-handoff-after-editorial-pass.md`
- `signal-core/src/channel.rs`
