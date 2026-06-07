# skills — signal-terminal

*Per-repo agent guide for the terminal transport control contract.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/push-not-pull.md`
- `~/primary/skills/subscription-lifecycle.md` (the canonical
  subscription FSM the worker-lifecycle stream implements)
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`harness/`, `terminal/`, `terminal-cell/`).

## What this repo is for

`signal-terminal` is the typed control-plane contract
`harness` and router delivery adapters use to ask
`terminal` for terminal work. The raw attached-viewer byte
plane stays outside this contract: PTY bytes, socket bytes, and
viewer-pump bytes live in `terminal-cell` / `terminal`
implementation code, not in Signal frames.

This is the ordinary terminal communication surface. It can query the
session registry, but it cannot create or retire sessions. Meta-only
session lifecycle mutation is declared in the terminal meta signal
contract.

The terminal-worker-lifecycle subscription follows the canonical
lifecycle in `~/primary/skills/subscription-lifecycle.md`: open with
a typed `Subscribe`, push typed `TerminalWorkerLifecycleEvent`
events, close with a typed request-side `Retract` carrying the
per-stream token, end with a typed reply-side `SubscriptionRetracted`
ack echoing the token.

## What this repo owns

- The closed `TerminalRequest` enum (including request-side
  subscription retraction).
- The closed `TerminalReply` enum (including reply-side
  `SubscriptionRetracted` ack).
- Typed terminal identity, generation, sequence, size, input bytes,
  transcript bytes, capture, and rejection reason records.
- Prompt-pattern registration records used to identify terminal-ready
  shapes.
- Input-gate lease records used to serialize controlled terminal
  writes.
- Write-injection acknowledgement and rejection records.
- Worker-lifecycle subscription, retraction, and observation records.
- Terminal-owned introspection records for session state, delivery
  attempts, terminal events, viewer attachments, session health, and
  session archive projections.
- The `Frame` type alias.
- Wire-form round-trip tests.

## What this repo does not own

- PTY lifecycle.
- Viewer lifecycle.
- Harness lifecycle.
- Terminal-cell implementation.
- Raw PTY, socket, or viewer byte streams.
- Prompt interpretation, routing authority, or policy decisions.
- Router delivery policy.
- Socket paths, reconnects, or process supervision.
- Runtime database access, reducers, or consistency policy for
  introspection. `terminal` owns those; this contract owns
  only the typed observation vocabulary.
- Meta-only session lifecycle commands (`CreateSession`,
  `RetireSession`, and their replies). Those belong to the terminal
  meta signal contract.

`terminal-cell` is the low-level PTY primitive behind
`terminal`; do not describe it as an independent production
Signal endpoint.

## Load-bearing invariants

- **Subscription close uses both sides.** The kernel grammar in
  `signal-frame/macros/src/validate.rs` requires the `stream` block
  to name a request-side `Retract` variant; the reply-side
  `SubscriptionRetracted` ack is the final event consumers bind to.
  Both are present in `src/lib.rs`. Do not remove either.
- **Wire enums are closed.** No `Unknown` variant on the wire.
  `InjectionRejectionReason::UnknownTerminal` and `UnknownLease`
  name **positive** "the entity you sent isn't in our state"
  rejections, not lifecycle uncertainty placeholders. They are
  closed positive failure modes.
- **Skeleton honesty uses typed reasons.** A request that reaches
  the daemon and is not built yet returns
  `TerminalRequestUnimplemented` carrying typed
  `TerminalOperationKind` and `TerminalUnimplementedReason`, not a
  text error or a hang.
- **Write injection is lease-scoped.** `WriteInjection` carries
  `TerminalName`, `InputGateLease`, and bytes; `terminal` returns the
  generated `TerminalGeneration` and `TerminalSequence` in
  `InjectionAck`. Do not add a retry policy "for ordering."
- **Every request variant declares a contract-local operation head.**
  The `signal_channel!` declaration is the source of truth;
  round-trip tests assert every generated operation head.
- **Session lifecycle mutation is meta-only.** Do not add
  `CreateSession`, `RetireSession`, or equivalent lifecycle mutation
  variants to the ordinary `TerminalRequest`; use the terminal meta
  signal contract.
- **No runtime code.** No Kameo, Tokio, socket, storage, or daemon
  glue in this crate.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/round_trip.rs`; canonical NOTA examples in
  `examples/canonical.nota` with a parser test. Introspection
  records are exercised in `tests/introspection.rs`.
- **Pin upstream contracts via a named API reference.** Cargo deps
  declare `git = "..."` with a named branch/bookmark, never raw
  `rev = "..."`.

## Editing patterns

### Adding a new injection rejection reason

1. Add the variant to `InjectionRejectionReason`. If the variant
   names a positive "entity not in our state" rejection
   (`UnknownX` shape), call out that it is a closed positive
   rejection, not a polling-shape placeholder.
2. Add round-trip witnesses through rkyv and NOTA.
3. Update consumers' rejection handling.

### Adding a new subscription kind

1. Read `~/primary/skills/subscription-lifecycle.md` end-to-end.
2. Add the typed subscribe payload, token, snapshot, and event
   records.
3. Add the new `stream` block in `signal_channel!`, with the
   subscribe request, the request-side retract variant, the
   reply-side ack, and the typed event variant. The kernel grammar
   enforces the close-is-Retract shape.
4. Witness the full subscribe → event → retract → ack → end
   lifecycle.

## NOTA codec shape

The `signal_channel!` macro emits each request's NOTA head from the
contract-local operation name. For example,
`TerminalRequest::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken { .. })`
encodes as `(TerminalWorkerLifecycleRetraction (...))`. Canonical
examples and round-trip tests use the operation heads.

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/subscription-lifecycle.md`.
- this workspace's `skills/push-not-pull.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- `signal-harness`'s `skills.md`,
  `signal-system`'s `skills.md`, and `signal-criome`'s
  `skills.md` — sibling contracts using the same Path A subscription
  discipline.
