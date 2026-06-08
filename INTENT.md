# INTENT — signal-terminal

*The ordinary peer-callable wire contract for Persona terminal transport
control. Defines the typed channel `harness` and router delivery
adapters use to ask `terminal` for terminal work — connection, input,
resize, capture, the prompt-pattern registry, input-gate leases, write
injection, and worker-lifecycle observations. Companion to
`ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-terminal`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in
`terminal/INTENT.md`. Meta-only session lifecycle intent stays in
the terminal meta signal contract.

## Why this repo exists

`signal-terminal` is the **ordinary peer-callable wire contract** for
the `terminal` daemon. It carries the terminal transport control
vocabulary that persona components needing terminal work (today
`harness` and router delivery adapters) exchange with `terminal`. The
raw attached-viewer byte plane stays OUTSIDE this contract — PTY bytes,
socket bytes, and viewer-pump bytes live in `terminal-cell`/`terminal`
implementation code, on a separate data socket, never in Signal frames.
This ordinary surface can read the session registry; it cannot create or
retire sessions — those meta-only commands live in the terminal meta
signal contract.

## The channel shape

The terminal channel carries four concern groups plus the standardized
observer hook:

- **Transport:** `Connect`, `Input`, `Resize`, `Detach`, `Capture` with
  their replies (`TerminalReady`, `TerminalInputAccepted`,
  `TranscriptDelta`, `TerminalResized`, `TerminalCaptured`,
  `TerminalDetached`, `TerminalExited`, `TerminalRejected`).
- **Session discovery:** `Query` over `ListSessions`/`ResolveSession`,
  read-only over the session registry.
- **Prompt-pattern registry:** `Register`, `Unregister`, `Query` — the
  terminal-ready shape that makes write injection safe to attempt.
- **Input-gate / injection:** `Acquire`, `Release`, `Inject` — the
  exclusive write lease and terminal-minted injection acknowledgement.
- **Worker-lifecycle subscription:** `Watch`/`Unwatch` over
  `TerminalWorkerLifecycleStream`, plus the mandatory `Tap`/`Untap`
  observer hook.

The wire vocabulary is contract-local — the daemon lowers these public
operations into component-local commands; Sema classification happens at
observation publish time, not on the wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; the only
  `Unknown*`-named records (`InjectionRejectionReason::UnknownTerminal`,
  `UnknownLease`) are positive "the id you sent isn't in our state"
  rejections, not polling-shape placeholders.
- Subscription close uses **Path A**: a request-side `Unwatch` carrying
  the per-stream token, plus a reply-side `SubscriptionRetracted` ack
  echoing it.
- Write injection is lease-scoped; `terminal` mints the resulting
  generation and sequence in `InjectionAck`.
- Request payloads do not mint terminal generations, leases, or
  sequences the daemon owns; `terminal` mints those.
- No stringly-typed dispatch. Kind, reason, and state fields are typed
  closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form (`Connect`, `Input`,
  `Resize`, `Detach`, `Capture`, `Register`, `Acquire`, `Inject`,
  `Watch`), not Sema class words. The six Sema classification words must
  not appear as request roots on this wire.
- Reply success variants name the concrete outcome the daemon produced.
- Payload record names drop redundant `Terminal*` prefixes where the
  crate namespace already supplies them.
- A valid request whose runtime behavior is not built yet returns
  `TerminalRequestUnimplemented` with a typed reason, never free-text
  error or a panic.

## Three-layer model

Layer 1 (this crate): contract operations on the wire.
Layer 2 (daemon): component-local `TerminalCommand` records
(`AssertConnection`, `DeliverInput`, `MutateGeometry`, `AcquireInputGate`,
`RecordInjection`, `ReadSessionList`, `OpenWorkerLifecycleStream`).
Layer 3 (observation): payloadless Sema class labels (`Assert`,
`Mutate`, `Retract`, `Match`, `Subscribe`) computed daemon-side for
cross-component introspection.

The contract names the public action; the daemon decides internal work
and Sema class. Sema classification never appears on the wire.

## Introspection records

Terminal durable Sema rows that need to be inspectable outside
`terminal` carry typed record shapes in this contract (`src/introspection.rs`).
The contract owns the *vocabulary* of inspectable terminal state; the
component still owns its sema-engine store, reducers, consistency model, and
redaction policy. `introspect` asks the running component for these
records; it never opens `terminal`'s database directly.

## Constraints

- This crate carries only typed wire vocabulary, explicit NOTA text
  codecs for CLI/tooling projection, and round-trip witnesses.
- The daemon configuration record may carry ordinary, meta, and
  supervision socket locations for the generated terminal process; that
  launch record is binary configuration, not a public working operation
  and not authority to mutate sessions through this ordinary contract.
- No runtime code: no actors, no tokio, no socket binding, no storage, no
  terminal-cell transport logic.
- Contract types derive NOTA in this crate. Clients do not carry shadow
  types that re-derive the text surface.
- Every request, reply, and event variant round-trips through both rkyv
  frames and NOTA text; the full subscribe/event/retract/ack lifecycle
  is witnessed.
- This contract carries no raw PTY / viewer byte data plane.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Non-ownership

This crate does not own:

- `terminal` daemon runtime, the harness actor, or component lifecycle;
- `terminal.sema` or any storage tables, reducers, or transcript state;
- the `terminal-cell` daemon behind `terminal`;
- meta-only session lifecycle commands;
- router delivery policy, OS focus policy, or prompt interpretation;
- the raw byte data plane, transport loop, reconnect policy, or socket
  path.

## See also

- `ARCHITECTURE.md` — detailed channel shape, the four concern groups,
  Path A lifecycle, closed-enum discipline, and the three-layer
  migration.
- `../terminal/INTENT.md` — daemon-side intent (terminal sessions,
  schema-driven planes, state).
- terminal meta signal contract — meta-only session lifecycle contract.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure, wire
  layers, and the high-bandwidth-data-plane carve-out.
