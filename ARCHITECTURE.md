# signal-persona-terminal — architecture

*Signal contract for Persona terminal transport control.*

## 0 · TL;DR

`signal-persona-terminal` is the typed control-plane contract
`persona-harness` (and router delivery adapters) use to ask
`persona-terminal` for terminal work. The raw attached-viewer byte
plane stays outside this contract: PTY bytes, socket bytes, and
viewer-pump bytes live in `terminal-cell` / `persona-terminal`
implementation code, not in Signal frames.

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
| Request side | Persona components that need terminal transport (today: `persona-harness` and router delivery adapters). |
| Reply / event side | `persona-terminal` |

Two control surfaces share the channel:

- **Harness transport**: `persona-harness` requests connection,
  input, resize, detachment, and capture vectors. `persona-terminal`
  emits readiness, input acceptance, transcript, resize, detachment,
  capture, exit, and rejection events.
- **Terminal control**: `persona-terminal` owns prompt-pattern
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
- Prompt-pattern records: `PromptPatternId`, `PromptPatternBytes`,
  `PromptPattern`, `RegisterPromptPattern`, `UnregisterPromptPattern`,
  `ListPromptPatterns`, `PromptPatternEntry`, `PromptPatternRegistered`,
  `PromptPatternUnregistered`, `PromptPatternList`.
- Input-gate records: `InputGateReason`, `InputGateLeaseId`,
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
    participant Terminal as persona-terminal

    Caller->>Terminal: SubscribeTerminalWorkerLifecycle(target)
    Terminal-->>Caller: TerminalWorkerLifecycleSnapshot{...}
    Terminal-->>Caller: TerminalWorkerLifecycleEvent{...}
    Caller->>Terminal: TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken)
    Terminal-->>Caller: SubscriptionRetracted{token}
```

The request retract variant is required by the `signal_channel!`
stream-block grammar; the reply ack is the final event consumers
bind their in-flight subscribe to.

### Signal root verbs

```text
TerminalConnection                 -> Assert
TerminalInput                      -> Assert
TerminalResize                     -> Mutate
TerminalDetachment                 -> Retract
TerminalCapture                    -> Match
RegisterPromptPattern              -> Assert
UnregisterPromptPattern            -> Retract
ListPromptPatterns                 -> Match
AcquireInputGate                   -> Assert
ReleaseInputGate                   -> Retract
WriteInjection                     -> Assert
SubscribeTerminalWorkerLifecycle   -> Subscribe   (opens TerminalWorkerLifecycleStream)
TerminalWorkerLifecycleRetraction  -> Retract     (closes TerminalWorkerLifecycleStream)
```

Terminal reads use `Match`; terminal-worker streams use `Subscribe`.
Control operations that append new work or create leases use
`Assert`. State changes to existing terminal geometry use `Mutate`;
detach, unregister, and release requests use `Retract`.

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
`persona-terminal` emits `TerminalRequestUnimplemented` rather than
panicking or producing a generic rejection.

### Injection ordering

`WriteInjection` carries an `injection_sequence: u64` so the
gate-lease holder's writes are sequenced. Out-of-order use returns
`InjectionRejectionReason::InvalidSequence`.

```text
WriteInjection
  | terminal:           TerminalName
  | lease:              InputGateLease
  | injection_sequence: u64
  | bytes:              TerminalInputBytes
```

### `TerminalName` namespace scope

`TerminalName` identifies a supervised terminal session. For the
prototype, the canonical scope is "one role per name" —
`TerminalName::new("operator")`, `TerminalName::new("designer")`,
etc. Future cases where multiple harnesses share a role get a
richer namespace; until then, the name space matches the role-name
vocabulary in `signal-persona-mind::RoleName`.

## 4 · Terminal-Cell Control

Prompt-pattern records let a caller register the terminal-ready
shape that makes write injection safe to attempt. Input-gate records
make the exclusive write lease explicit and include prompt state in
the acquisition reply. Write-injection records acknowledge the
terminal generation and sequence produced by a successful write.
Worker-lifecycle records expose transport task start/stop
observations as typed events.

This contract does not decide whether a write should happen. It only
carries the transport control facts needed by `persona-terminal` and
its consumers.

## 5 · Introspection records

Terminal durable Sema rows that need to be inspectable outside
`persona-terminal` have typed record shapes in this contract. The
component still owns its redb file, table declarations, reducers,
consistency model, and redaction policy. `persona-introspect` asks
the running component for these records; it does not open
`persona-terminal`'s database directly.

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
| Every `TerminalRequest` variant declares a Signal root verb. | `signal-core` generates `TerminalRequest::signal_verb()`; round-trip tests assert each variant's expected root. |
| Subscription close uses **Path A**: request-side `Retract TerminalWorkerLifecycleRetraction` carrying the token, plus reply-side `SubscriptionRetracted` ack echoing the token. | The `signal_channel!` declaration names `Retract TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken)` and a `stream TerminalWorkerLifecycleStream { close TerminalWorkerLifecycleRetraction; … }` block. The kernel grammar (`signal-core::macros::validate`) rejects a `stream` block whose `close` is not a request-side `Retract` variant. Wire witnesses cover the retract request and the reply ack. |
| Wire enums contain no `Unknown` variant. | Source scan: only `InjectionRejectionReason::{UnknownTerminal,UnknownLease}` carry the word "Unknown" and those are positive domain rejections (see next row). |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | `InjectionRejectionReason::UnknownTerminal` and `UnknownLease` name "the terminal/lease id you sent isn't in our state" — closed, positively-defined failure modes, not lifecycle uncertainty placeholders. |
| Skeleton honesty uses typed reasons, not free text. | `TerminalRequestUnimplemented.operation` is the closed `TerminalOperationKind`; `reason` is the closed `TerminalUnimplementedReason`. |
| Injection ordering is enforced by sequence number, not retry. | `WriteInjection.injection_sequence`; out-of-order use returns `InjectionRejectionReason::InvalidSequence`. |
| Every `signal_channel!` request variant has a typed `signal_verb()` mapping. | Generated by the macro; round-trip witness asserts each variant. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` covers every request, reply, and event variant. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply/event variant; round-trip tests parse and re-emit each. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All kind / reason / state fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-core` is declared `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or redb code. |

## 7 · NOTA codec quirk on `signal_channel!` payload heads

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `TerminalRequest::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken { .. })`
encodes as `(TerminalWorkerLifecycleToken (...))`, not
`(TerminalWorkerLifecycleRetraction ...)`. Canonical examples and
round-trip tests carry the payload heads.

## 8 · Versioning

`signal_core::Frame` carries the protocol version. Schema-level
changes are breaking; coordinate `persona-harness`,
`persona-terminal`, and terminal-cell transport on the upgrade.

This crate depends on `signal-core` via a named-branch reference, not
a raw revision pin. The destination is a stable `signal-core` API
branch/bookmark once that lane is declared.

## 9 · Non-ownership

- No terminal daemon. That is `persona-terminal`.
- No harness actor. That is `persona-harness`.
- No router delivery policy. That is `persona-router`.
- No OS focus policy. That is `persona-system`.
- No terminal-cell daemon. That is `terminal-cell`, behind
  `persona-terminal`.
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

- `signal-core/src/channel.rs` — the macro and stream-block grammar
  that enforces the request-side retract variant.
- `signal-persona-harness/ARCHITECTURE.md` — sibling contract using
  the same Path A subscription discipline.
- `signal-persona-system/ARCHITECTURE.md` and
  `signal-criome/ARCHITECTURE.md` — sibling contracts using the same
  Path A subscription discipline.
- `persona-harness/ARCHITECTURE.md`
- `persona-terminal/ARCHITECTURE.md`
- `persona-router/ARCHITECTURE.md`
- `terminal-cell/ARCHITECTURE.md`
