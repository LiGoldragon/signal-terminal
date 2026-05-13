# skills - signal-persona-terminal

*Per-repo agent guide.*

## Checkpoint - read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/push-not-pull.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`persona-harness/`, `persona-terminal/`, `terminal-cell/`)

## What this repo owns

- The closed `TerminalRequest` enum.
- The closed `TerminalEvent` enum.
- Typed terminal identity, generation, sequence, size, input bytes,
  transcript bytes, capture, and rejection reason records.
- Prompt pattern registration records used to identify terminal-ready shapes.
- Input gate lease records used to serialize controlled terminal writes.
- Write injection acknowledgement and rejection records.
- Worker lifecycle subscription and observation records.
- Terminal-owned introspection records for session state, delivery attempts,
  terminal events, viewer attachments, session health, and session archive
  projections.
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
- Runtime database access, reducers, or consistency policy for introspection.
  `persona-terminal` owns those; this contract owns only the typed observation
  vocabulary.

`terminal-cell` is the low-level PTY primitive behind `persona-terminal`; do
not describe it as an independent production Signal endpoint.
