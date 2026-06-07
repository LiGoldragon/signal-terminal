//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! through its NOTA codec and asserting the re-encoded text equals
//! the canonical form. The selection covers the Path A lifecycle
//! and one example per request/reply family. Exhaustive per-variant
//! round-trip witnesses already live in `tests/round_trip.rs`.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_terminal::{
    AcquireInputGate, GateAcquired, GateBusy, InjectionAck, InjectionRejected,
    InjectionRejectionReason, InputGateLease, InputGateLeaseIdentifier, InputGateReason,
    PromptState, ReleaseInputGate, ResolveSession, SessionResolved,
    SubscribeTerminalWorkerLifecycle, SubscriptionRetracted, TerminalCapture, TerminalColumns,
    TerminalConnection, TerminalDetachment, TerminalDetachmentReason, TerminalGeneration,
    TerminalInput, TerminalInputAccepted, TerminalInputBytes, TerminalName, TerminalReady,
    TerminalReply, TerminalRequest, TerminalResize, TerminalRows, TerminalSequence,
    TerminalWorkerLifecycleToken, WriteInjection,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn operator() -> TerminalName {
    TerminalName::new("operator")
}

fn token() -> TerminalWorkerLifecycleToken {
    TerminalWorkerLifecycleToken {
        terminal: operator(),
    }
}

fn lease() -> InputGateLease {
    InputGateLease {
        id: InputGateLeaseIdentifier::new(42),
    }
}

fn hello_bytes() -> TerminalInputBytes {
    TerminalInputBytes::new(b"hello".to_vec())
}

fn data_socket_path() -> signal_engine_management::WirePath {
    signal_engine_management::WirePath::new("/run/persona/terminal/sessions/operator/data.sock")
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
        TerminalRequest::TerminalConnection(TerminalConnection {
            terminal: operator(),
        }),
        "(TerminalConnection ([operator]))",
    );
    round_trip(
        TerminalRequest::TerminalInput(TerminalInput {
            terminal: operator(),
            bytes: hello_bytes(),
        }),
        "(TerminalInput ([operator] [104 101 108 108 111]))",
    );
    round_trip(
        TerminalRequest::TerminalResize(TerminalResize {
            terminal: operator(),
            rows: TerminalRows::new(24),
            columns: TerminalColumns::new(80),
        }),
        "(TerminalResize ([operator] 24 80))",
    );
    round_trip(
        TerminalRequest::TerminalDetachment(TerminalDetachment {
            terminal: operator(),
            reason: TerminalDetachmentReason::HumanRequested,
        }),
        "(TerminalDetachment ([operator] HumanRequested))",
    );
    round_trip(
        TerminalRequest::TerminalCapture(TerminalCapture {
            terminal: operator(),
        }),
        "(TerminalCapture ([operator]))",
    );
    round_trip(
        TerminalRequest::ResolveSession(ResolveSession { name: operator() }),
        "(ResolveSession ([operator]))",
    );
    round_trip(
        TerminalRequest::AcquireInputGate(AcquireInputGate {
            terminal: operator(),
            reason: InputGateReason::new("send router-delivered command"),
            prompt_pattern_identifier: None,
        }),
        "(AcquireInputGate ([operator] [send router-delivered command] None))",
    );
    round_trip(
        TerminalRequest::ReleaseInputGate(ReleaseInputGate {
            terminal: operator(),
            lease: lease(),
        }),
        "(ReleaseInputGate ([operator] (42)))",
    );
    round_trip(
        TerminalRequest::WriteInjection(WriteInjection {
            terminal: operator(),
            lease: lease(),
            bytes: hello_bytes(),
        }),
        "(WriteInjection ([operator] (42) [104 101 108 108 111]))",
    );
    round_trip(
        TerminalRequest::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle {
            terminal: operator(),
        }),
        "(SubscribeTerminalWorkerLifecycle ([operator]))",
    );
    round_trip(
        TerminalRequest::TerminalWorkerLifecycleRetraction(token()),
        "(TerminalWorkerLifecycleRetraction ([operator]))",
    );
}

#[test]
fn canonical_reply_examples_round_trip() {
    round_trip(
        TerminalReply::TerminalReady(TerminalReady {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
        }),
        "(TerminalReady ([operator] 1))",
    );
    round_trip(
        TerminalReply::TerminalInputAccepted(TerminalInputAccepted {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
        }),
        "(TerminalInputAccepted ([operator] 1))",
    );
    round_trip(
        TerminalReply::GateAcquired(GateAcquired {
            terminal: operator(),
            lease: lease(),
            prompt_state: PromptState::Clean,
        }),
        "(GateAcquired ([operator] (42) (Clean)))",
    );
    round_trip(
        TerminalReply::GateBusy(GateBusy {
            terminal: operator(),
            current_holder: InputGateLeaseIdentifier::new(41),
        }),
        "(GateBusy ([operator] 41))",
    );
    round_trip(
        TerminalReply::InjectionAck(InjectionAck {
            terminal: operator(),
            generation: TerminalGeneration::new(1),
            sequence: TerminalSequence::new(7),
        }),
        "(InjectionAck ([operator] 1 7))",
    );
    round_trip(
        TerminalReply::InjectionRejected(InjectionRejected {
            terminal: operator(),
            reason: InjectionRejectionReason::UnknownTerminal,
        }),
        "(InjectionRejected ([operator] UnknownTerminal))",
    );
    round_trip(
        TerminalReply::SubscriptionRetracted(SubscriptionRetracted { token: token() }),
        "(SubscriptionRetracted (([operator])))",
    );
    round_trip(
        TerminalReply::SessionResolved(SessionResolved {
            name: operator(),
            data_socket_path: data_socket_path(),
        }),
        "(SessionResolved ([operator] [/run/persona/terminal/sessions/operator/data.sock]))",
    );
}
