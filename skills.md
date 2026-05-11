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
  (`persona-harness/`, `persona-terminal/`)

## What this repo owns

- The closed `TerminalRequest` enum.
- The closed `TerminalEvent` enum.
- Typed terminal identity, generation, sequence, size, input bytes,
  transcript bytes, capture, and rejection reason records.
- The `Frame` type alias.
- Wire-form round-trip tests.

## What this repo does not own

- PTY lifecycle.
- Viewer lifecycle.
- Harness lifecycle.
- Router delivery policy.
- Socket paths, reconnects, or process supervision.
