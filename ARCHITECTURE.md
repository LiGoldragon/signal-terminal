# signal-terminal — architecture

*Signal contract for Persona terminal transport control.*

## 0 · TL;DR

`signal-terminal` is the typed communication contract
`harness` (and router delivery adapters) use to ask
`terminal` for terminal work. The raw attached-viewer byte
plane stays outside this contract: PTY bytes, socket bytes, and
viewer-pump bytes live in `terminal-cell` / `terminal`
implementation code, not in Signal frames. Engine lifecycle/readiness
traffic is the separate `signal-engine-management` relation;
do not call the component communication socket a supervision socket.
Meta-only terminal session lifecycle commands live in the separate
terminal meta signal contract. This ordinary surface can
read the session registry; it cannot create or retire sessions.

## Migration history — signal-frame operation heads (2026-06-07)

The public wire no longer carries `SignalVerb` classification words.
Terminal requests travel as contract-local `signal-frame` operation
heads. The worker-lifecycle stream keeps its typed open/event/close
grammar through `operation SubscribeTerminalWorkerLifecycle(...) opens
TerminalWorkerLifecycleStream` and the close operation
`TerminalWorkerLifecycleRetraction`.

This crate owns wire vocabulary and codecs only. Sema classification is
daemon-side projection.

Because this contract still owns NOTA round-trip witnesses, it explicitly
enables `signal-frame/nota-text` through its own default `nota-text`
feature instead of relying on text codecs in the frame kernel's default
build.

There is one `signal_channel!` invocation in `src/lib.rs` declaring
the `Terminal` channel. Terminal-owned introspection records (typed
projections of durable Sema rows for `persona-introspect`) live in
`src/introspection.rs`.

Subscription close on the terminal-worker-lifecycle stream follows
the **Path A** discipline per /181 and
`~/primary/reports/designer-assistant/91-user-decisions-after-designer-184-200-critique.md`
§2: a request-side `Retract TerminalWorkerLifecycleRetraction` carries
the per-stream token; the terminal responds with
`TerminalReply::SubscriptionRetracted` echoing the token.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | Persona components that need terminal transport (today: `harness` and router delivery adapters). |
| Reply / event side | `terminal` |

Two control surfaces share the channel:

- **Harness transport**: `harness` requests connection,
  input, resize, detachment, and capture vectors. `terminal`
  emits readiness, input acceptance, transcript, resize, detachment,
  capture, exit, and rejection events.
- **Terminal control**: `terminal` owns prompt-pattern
  registry, input-gate leases, write-injection acknowledgements, and
  worker-lifecycle observations. It may implement those facts on top
  of `terminal-cell` primitives, but `terminal-cell` is not the
  Persona-facing contract endpoint.

The steady-state flow is pushed by the transport owner. Harnesses
and callers do not poll for transcript or lifecycle state.

## 2 · Wire vocabulary

Records local to this contract (see source for the full list):

- Terminal identity: `TerminalName`, `TerminalGeneration`,
  `TerminalSequence`.
- Byte and geometry types: `TerminalInputBytes`,
  `TerminalTranscriptBytes`, `TerminalRows`, `TerminalColumns`,
  `TerminalByteCount`.
- Prompt-pattern records: `PromptPatternIdentifier`, `PromptPatternBytes`,
  `PromptPattern`, `RegisterPromptPattern`, `UnregisterPromptPattern`,
  `ListPromptPatterns`, `PromptPatternEntry`, `PromptPatternRegistered`,
  `PromptPatternUnregistered`, `PromptPatternList`.
- Input-gate records: `InputGateReason`, `InputGateLeaseIdentifier`,
  `InputGateLease`, `PromptState`, `AcquireInputGate`,
  `ReleaseInputGate`, `WriteInjection`, `GateAcquired`, `GateBusy`,
  `GateReleased`, `InjectionAck`, `InjectionRejected`,
  `InjectionRejectionReason`.
- Worker-lifecycle subscription records:
  `SubscribeTerminalWorkerLifecycle`, `TerminalWorkerLifecycleToken`,
  `SubscriptionRetracted`, `TerminalWorkerKind`,
  `TerminalWorkerStopReason`, `TerminalWorkerLifecycle`,
  `TerminalWorkerLifecycleSnapshot`, `TerminalWorkerLifecycleEvent`.
- Connection / transport: `TerminalConnection`, `TerminalInput`,
  `TerminalResize`, `TerminalDetachment`, `TerminalCapture`,
  `TerminalReady`, `TerminalInputAccepted`, `TranscriptDelta`,
  `TerminalResized`, `TerminalCaptured`, `TerminalDetached`,
  `TerminalExited`, `TerminalRejected`.
- Session registry reads: `ListSessions`, `ResolveSession`,
  `SessionEntry`, `SessionList`, `SessionResolved`.
- Introspection projections (in `src/introspection.rs`):
  `TerminalObservationSequence`, `TerminalSocketPath`,
  `TerminalViewerName`, `TerminalArchiveReason`,
  `TerminalSessionState`, `TerminalSessionObservation`,
  `TerminalDeliveryAttemptState`, `TerminalDeliveryAttemptObservation`,
  `TerminalEventObservation`, `TerminalViewerAttachmentState`,
  `TerminalViewerAttachmentObservation`,
  `TerminalSessionHealthObservation`, `TerminalSessionArchiveState`,
  `TerminalSessionArchiveObservation`, `TerminalIntrospectionSnapshot`.

The records are terminal-transport vocabulary. They are not router,
message, auth, or terminal raw-data records.

## 3 · Messages

```text
TerminalRequest                          TerminalReply
├─ TerminalConnection                    ├─ TerminalReady
├─ TerminalInput                         ├─ TerminalInputAccepted
├─ TerminalResize                        ├─ TerminalResized
├─ TerminalDetachment                    ├─ TerminalCaptured
├─ TerminalCapture                       ├─ TranscriptDelta
├─ ListSessions                          ├─ SessionList
├─ ResolveSession                        ├─ SessionResolved
├─ RegisterPromptPattern                 ├─ TerminalDetached
├─ UnregisterPromptPattern               ├─ TerminalExited
├─ ListPromptPatterns                    ├─ TerminalRejected
├─ AcquireInputGate                      ├─ PromptPatternRegistered
├─ ReleaseInputGate                      ├─ PromptPatternUnregistered
├─ WriteInjection                        ├─ PromptPatternList
├─ SubscribeTerminalWorker...            ├─ GateAcquired
└─ TerminalWorkerLifecycleRetraction     ├─ GateBusy
                                         ├─ GateReleased
                                         ├─ InjectionAck
                                         ├─ InjectionRejected
                                         ├─ TerminalRequestUnimplemented
                                         ├─ TerminalWorkerLifecycleSnapshot
                                         ├─ TerminalWorkerLifecycleEvent
                                         └─ SubscriptionRetracted

(TerminalWorkerLifecycleEvent flows on TerminalWorkerLifecycleStream.)
```

Closed enums; typed rejection reasons; no string-tagged event kinds.

### Path A lifecycle on the worker-lifecycle stream

```mermaid
sequenceDiagram
    participant Caller as caller
    participant Terminal as terminal

    Caller->>Terminal: SubscribeTerminalWorkerLifecycle(target)
    Terminal-->>Caller: TerminalWorkerLifecycleSnapshot{...}
    Terminal-->>Caller: TerminalWorkerLifecycleEvent{...}
    Caller->>Terminal: TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken)
    Terminal-->>Caller: SubscriptionRetracted{token}
```

The request retract variant is required by the `signal_channel!`
stream-block grammar; the reply ack is the final event consumers
bind their in-flight subscribe to.

### Sema-class projections (Layer 3)

Each contract-local operation's daemon-side Component Command
projects to a payloadless Sema class label for observation:

```text
Connect                            -> Assert
Input                              -> Assert
Resize                             -> Mutate
Detach                             -> Retract
Capture                            -> Match
Query (ListSessions)               -> Match
Query (ResolveSession)             -> Match
Register (PromptPattern)           -> Assert
Unregister (PromptPattern)         -> Retract
Query (ListPromptPatterns)         -> Match
Acquire (InputGate)                -> Assert
Release (InputGate)                -> Retract
Inject (WriteInjection)            -> Assert
Watch (WorkerLifecycle)            -> Subscribe   (opens TerminalWorkerLifecycleStream)
Unwatch (WorkerLifecycle)          -> Retract     (closes TerminalWorkerLifecycleStream)
Tap (mandatory observability)      -> Subscribe
Untap (mandatory observability)    -> Retract
```

The wire form carries the contract-local verb only; the Sema class
label is computed at observation publish time inside the daemon.
Session lifecycle mutation is intentionally absent here; it belongs
to the terminal meta signal contract.

### Skeleton honesty (Unimplemented event)

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

When a `TerminalRequest` variant has no built behavior yet,
`terminal` emits `TerminalRequestUnimplemented` rather than
panicking or producing a generic rejection.

### Injection ordering

`WriteInjection` is lease-scoped. The caller supplies the terminal,
input-gate lease, and bytes; `terminal` mints the resulting terminal
generation and sequence in `InjectionAck`.

```text
WriteInjection
  | terminal:           TerminalName
  | lease:              InputGateLease
  | bytes:              TerminalInputBytes
```

### `TerminalName` namespace scope

`TerminalName` identifies a supervised terminal session. For the
prototype, the canonical scope is "one role per name" —
`TerminalName::new("operator")`, `TerminalName::new("designer")`,
etc. Future cases where multiple harnesses share a role get a
richer namespace; until then, the name space matches the role-name
vocabulary in `signal-mind::RoleName`.

## 4 · Terminal-Cell Control

Prompt-pattern records let a caller register the terminal-ready
shape that makes write injection safe to attempt. Input-gate records
make the exclusive write lease explicit and include prompt state in
the acquisition reply. Write-injection records acknowledge the
terminal generation and sequence produced by a successful write.
Worker-lifecycle records expose transport task start/stop
observations as typed events.

This contract does not decide whether a write should happen. It only
carries the transport control facts needed by `terminal` and
its consumers.

## 5 · Introspection records

Terminal durable Sema rows that need to be inspectable outside
`terminal` have typed record shapes in this contract. The
component still owns its Sema store, table declarations, reducers,
consistency model, and redaction policy. `persona-introspect` asks
the running component for these records; it does not open
`terminal`'s database directly.

`TerminalIntrospectionSnapshot` is the prototype projection bundle
over: terminal session observations; delivery attempt observations;
terminal event observations; viewer attachment observations; session
health observations; session archive observations.

These records are not router, harness, message, or terminal-cell
records. They name terminal-owned inspectable state at the Persona
terminal boundary.

## 6 · Constraints

| Constraint | Witness |
|---|---|
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Every `TerminalRequest` variant is a contract-local verb in verb form. | Round-trip tests assert each variant's NOTA head. Sema classification is daemon-side projection only. |
| Session lifecycle mutation is meta-only, not part of the ordinary terminal contract. | Source scan: ordinary `TerminalRequest` has no `CreateSession` or `RetireSession`; those records live in the terminal meta signal contract. |
| Session lookup is a read; its Component Command projects to Sema `Match`. | `ListSessions` and `ResolveSession` return typed session rows or typed rejection from the daemon. |
| Subscription close uses **Path A**: request-side `TerminalWorkerLifecycleRetraction` carrying the token, plus reply-side `SubscriptionRetracted` ack echoing the token. | The `signal_channel!` declaration names `operation TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken)` and a `stream TerminalWorkerLifecycleStream { close TerminalWorkerLifecycleRetraction; … }` block. The kernel grammar (`signal-frame::macros::validate`) rejects a stream block without a typed close operation. Wire witnesses cover the retract request and the reply ack. |
| Wire enums contain no `Unknown` variant. | Source scan: only `InjectionRejectionReason::{UnknownTerminal,UnknownLease}` carry the word "Unknown" and those are positive domain rejections (see next row). |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | `InjectionRejectionReason::UnknownTerminal` and `UnknownLease` name "the terminal/lease id you sent isn't in our state" — closed, positively-defined failure modes, not lifecycle uncertainty placeholders. |
| Skeleton honesty uses typed reasons, not free text. | `TerminalRequestUnimplemented.operation` is the closed `TerminalOperationKind`; `reason` is the closed `TerminalUnimplementedReason`. |
| Write injection is lease-scoped; terminal mints the resulting sequence. | `WriteInjection` carries `InputGateLease`; `InjectionAck` carries the generated `TerminalGeneration` and `TerminalSequence`. |
| Each variant's NOTA head matches the contract-local verb declared in `signal_channel!`. | Generated by the macro; round-trip witness asserts each variant's head. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` covers every request, reply, and event variant. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply/event variant; round-trip tests parse and re-emit each. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All kind / reason / state fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-frame` is declared `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or storage code. |

## 7 · NOTA codec shape on `signal_channel!` operation heads

The `signal_channel!` macro emits each request's NOTA head from the
contract-local operation name. For example,
`TerminalRequest::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken { .. })`
encodes as `(TerminalWorkerLifecycleRetraction (...))`.
Canonical examples and round-trip tests carry the operation heads.

## 8 · Versioning

`signal_frame::Frame` carries the protocol version. Schema-level
changes are breaking; coordinate `harness`,
`terminal`, and terminal-cell transport on the upgrade.

This crate depends on `signal-frame` via a named-branch reference, not
a raw revision pin. The destination is a stable `signal-frame` API
branch/bookmark once that lane is declared.

## 9 · Non-ownership

- No terminal daemon. That is `terminal`.
- No harness actor. That is `harness`.
- No router delivery policy. That is `router`.
- No OS focus policy. That is `system`.
- No terminal-cell daemon. That is `terminal-cell`, behind
  `terminal`.
- No meta-only terminal session lifecycle commands. Those are in the
  terminal meta signal contract.
- No prompt interpretation or delivery policy. That belongs in the
  caller and transport owner, not this contract.
- No raw PTY / viewer byte data plane.
- No transport loop, reconnect policy, or socket path.

## 10 · Code map

```text
src/
├── lib.rs                — control payloads + signal_channel! invocation
└── introspection.rs      — terminal-owned inspectable-state record shapes
examples/
└── canonical.nota         — one canonical example per request/reply/event variant
tests/
├── round_trip.rs          — per-variant frame round trips + NOTA witnesses
│                            + closed-enum + verb-mapping witnesses
│                            + canonical examples parser
│                            + full subscribe/event/retract/ack lifecycle witness
└── introspection.rs       — rkyv + NOTA witnesses for inspection records
```

## See also

- `signal-frame/macros/src/validate.rs` — the macro and stream-block
  grammar that enforces the request-side retract variant.
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
- `signal-harness/ARCHITECTURE.md` — sibling contract using
  the same Path A subscription discipline.
- The terminal meta signal contract — meta-only terminal session
  lifecycle mutation contract.
- `signal-system/ARCHITECTURE.md` and
  `signal-criome/ARCHITECTURE.md` — sibling contracts using the same
  Path A subscription discipline.
- `harness/ARCHITECTURE.md`
- `terminal/ARCHITECTURE.md`
- `router/ARCHITECTURE.md`
- `terminal-cell/ARCHITECTURE.md`
