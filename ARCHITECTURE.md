# ARCHITECTURE - signal-persona-terminal

The Signal contract between `persona-harness` and terminal transport.
It relates one harness-owned terminal session to the backend that owns PTY
bytes and disposable viewers. The whole channel is one `signal_channel!`
invocation in `src/lib.rs`.

`persona-terminal` is the transport owner. It consumes `terminal-cell` as the
low-level one-cell PTY/transcript primitive.

## Channel

| Side | Component |
|---|---|
| Request side | `persona-harness` |
| Event side | `persona-terminal` |

The harness requests connection, input, resize, detachment, and capture
vectors. The terminal transport emits readiness, input acceptance, transcript, resize,
detachment, capture, exit, and rejection events. The steady-state flow is
transport to harness:
transcript deltas and lifecycle events are pushed; the harness does not poll.

## Record Source

Records local to this contract:

- `TerminalName`
- `TerminalGeneration`
- `TerminalSequence`
- `TerminalInputBytes`
- `TerminalTranscriptBytes`
- `TerminalRows`
- `TerminalColumns`
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

The records are terminal-transport vocabulary, not router or message records.

## Messages

```text
TerminalRequest                 TerminalEvent
├─ TerminalConnection           ├─ TerminalReady
├─ TerminalInput                ├─ TerminalInputAccepted
├─ TerminalResize               ├─ TerminalResized
├─ TerminalDetachment           ├─ TerminalCaptured
└─ TerminalCapture              ├─ TranscriptDelta
                                ├─ TerminalDetached
                                ├─ TerminalExited
                                └─ TerminalRejected
```

Closed enums; typed rejection reasons; no string-tagged event kinds.

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
```

## Round Trips

Round-trip tests in `tests/round_trip.rs` cover every request variant, every
event variant, typed rejection reasons, and representative `From` impl
witnesses.

## Non-ownership

- No terminal daemon. That is `persona-terminal`.
- No harness actor. That is `persona-harness`.
- No router delivery policy. That is `persona-router`.
- No OS focus policy. That is `persona-system`.
- No transport loop, reconnect policy, or socket path.

## Code Map

```text
src/
└── lib.rs    - payloads + signal_channel! invocation
tests/
└── round_trip.rs - per-variant wire-form round trips
```

## See Also

- `persona-harness/ARCHITECTURE.md`
- `persona-terminal/ARCHITECTURE.md`
- `persona-router/ARCHITECTURE.md`
- `~/primary/reports/designer/97-persona-system-vision-and-architecture-development.md`
- `signal-core/src/channel.rs`
