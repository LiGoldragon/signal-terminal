#![cfg(feature = "nota-text")]

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! through its NOTA codec and asserting the re-encoded text equals
//! the canonical form. The selection covers the Path A lifecycle
//! and one example per request/reply/event family. Exhaustive per-variant
//! round-trip witnesses already live in `tests/round_trip.rs`.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_terminal::{
    AcquireInputGate, GateAcquired, GateBusy, InjectionAck, InjectionRejected,
    InjectionRejectionReason, Input, InputGateLease, InputGateLeaseIdentifier, InputGateReason,
    Output, PromptState, ReleaseInputGate, ResolveSession, SessionResolved,
    SubscribeTerminalWorkerLifecycle, SubscriptionRetracted, TerminalCapture, TerminalColumns,
    TerminalConnection, TerminalDetachment, TerminalDetachmentReason, TerminalEvent,
    TerminalGeneration, TerminalInput, TerminalInputAccepted, TerminalInputBytes, TerminalName,
    TerminalReady, TerminalResize, TerminalRows, TerminalSequence, TerminalWorkerKind,
    TerminalWorkerLifecycle, TerminalWorkerLifecycleEvent, TerminalWorkerLifecycleToken, WirePath,
    WriteInjection,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn operator() -> TerminalName {
    TerminalName::new("operator".to_owned())
}

fn token() -> TerminalWorkerLifecycleToken {
    TerminalWorkerLifecycleToken::new(operator())
}

fn lease() -> InputGateLease {
    InputGateLease::new(InputGateLeaseIdentifier::new(42))
}

fn hello_bytes() -> TerminalInputBytes {
    TerminalInputBytes::new(b"hello".iter().map(|byte| u64::from(*byte)).collect())
}

fn data_socket_path() -> WirePath {
    WirePath::new("/run/persona/terminal/sessions/operator/data.sock".to_owned())
}

fn round_trip<T>(value: T, canonical_text: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, canonical_text, "encode for {value:?}");

    let decoded = NotaSource::new(canonical_text)
        .parse::<T>()
        .expect("decode");
    assert_eq!(decoded, value, "decode for {canonical_text}");

    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.nota missing line: {canonical_text}",
    );
}

#[test]
fn canonical_request_examples_round_trip() {
    round_trip(
        Input::TerminalConnection(TerminalConnection::new(operator())),
        "(TerminalConnection operator)",
    );
    round_trip(
        Input::TerminalInput(TerminalInput {
            terminal: operator(),
            bytes: hello_bytes(),
        }),
        "(TerminalInput (operator [104 101 108 108 111]))",
    );
    round_trip(
        Input::TerminalResize(TerminalResize {
            terminal: operator(),
            rows: TerminalRows::new(24),
            columns: TerminalColumns::new(80),
        }),
        "(TerminalResize (operator 24 80))",
    );
    round_trip(
        Input::TerminalDetachment(TerminalDetachment {
            terminal: operator(),
            reason: TerminalDetachmentReason::HumanRequested,
        }),
        "(TerminalDetachment (operator HumanRequested))",
    );
    round_trip(
        Input::TerminalCapture(TerminalCapture::new(operator())),
        "(TerminalCapture operator)",
    );
    round_trip(
        Input::ResolveSession(ResolveSession::new(operator())),
        "(ResolveSession operator)",
    );
    round_trip(
        Input::AcquireInputGate(AcquireInputGate {
            terminal: operator(),
            reason: InputGateReason::new("send router-delivered command".to_owned()),
            prompt_pattern_identifier: None,
        }),
        "(AcquireInputGate (operator [send router-delivered command] None))",
    );
    round_trip(
        Input::ReleaseInputGate(ReleaseInputGate {
            terminal: operator(),
            lease: lease(),
        }),
        "(ReleaseInputGate (operator 42))",
    );
    round_trip(
        Input::WriteInjection(WriteInjection {
            terminal: operator(),
            lease: lease(),
            bytes: hello_bytes(),
        }),
        "(WriteInjection (operator 42 [104 101 108 108 111]))",
    );
    round_trip(
        Input::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle::new(operator())),
        "(SubscribeTerminalWorkerLifecycle operator)",
    );
    round_trip(
        Input::TerminalWorkerLifecycleRetraction(token()),
        "(TerminalWorkerLifecycleRetraction operator)",
    );
}

#[test]
fn canonical_reply_examples_round_trip() {
    round_trip(
        Output::TerminalReady(TerminalReady {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
        }),
        "(TerminalReady (operator 1))",
    );
    round_trip(
        Output::TerminalInputAccepted(TerminalInputAccepted {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
        }),
        "(TerminalInputAccepted (operator 1))",
    );
    round_trip(
        Output::GateAcquired(GateAcquired {
            terminal: operator(),
            lease: lease(),
            prompt_state: PromptState::Clean,
        }),
        "(GateAcquired (operator 42 Clean))",
    );
    round_trip(
        Output::GateBusy(GateBusy {
            terminal: operator(),
            current_holder: InputGateLeaseIdentifier::new(41),
        }),
        "(GateBusy (operator 41))",
    );
    round_trip(
        Output::InjectionAck(InjectionAck {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
            sequence: TerminalSequence::new(7),
        }),
        "(InjectionAck (operator 1 7))",
    );
    round_trip(
        Output::InjectionRejected(InjectionRejected {
            terminal: operator(),
            reason: InjectionRejectionReason::UnknownTerminal,
        }),
        "(InjectionRejected (operator UnknownTerminal))",
    );
    round_trip(
        Output::SubscriptionRetracted(SubscriptionRetracted::new(token())),
        "(SubscriptionRetracted operator)",
    );
    round_trip(
        Output::SessionResolved(SessionResolved {
            name: operator(),
            data_socket_path: data_socket_path(),
        }),
        "(SessionResolved (operator /run/persona/terminal/sessions/operator/data.sock))",
    );
}

#[test]
fn canonical_event_example_round_trips() {
    round_trip(
        Output::Event(TerminalEvent::TerminalWorkerLifecycleEvent(
            TerminalWorkerLifecycleEvent {
                terminal: operator(),
                observation: TerminalWorkerLifecycle::Started(TerminalWorkerKind::ViewerFanout),
            },
        )),
        "(Event (TerminalWorkerLifecycleEvent (operator (Started ViewerFanout))))",
    );
}
