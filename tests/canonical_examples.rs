#![cfg(feature = "dotos-text")]

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.dotos` end-to-end, decoding each record
//! through its DOTOS codec and asserting the re-encoded text equals
//! the canonical form. The selection covers the Path A lifecycle
//! and one example per request/reply/event family. Exhaustive per-variant
//! round-trip witnesses already live in `tests/round_trip.rs`.

use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_terminal::{
    AcquireInputGateRequest, GateAcquiredReply, GateBusyReply, InjectionAckReply,
    InjectionRejectedReply, InjectionRejectionReason, Input, InputGateLease,
    InputGateLeaseIdentifier, InputGateReason, Output, PromptState, ReleaseInputGateRequest,
    ResolveSessionRequest, SessionResolvedReply, SubscribeTerminalWorkerLifecycleRequest,
    SubscriptionRetractedReply, TerminalCaptureRequest, TerminalColumns, TerminalConnectionRequest,
    TerminalDetachmentReason, TerminalDetachmentRequest, TerminalEvent, TerminalGeneration,
    TerminalInputAcceptedReply, TerminalInputBytes, TerminalInputRequest, TerminalName,
    TerminalReadyReply, TerminalResizeRequest, TerminalRows, TerminalSequence, TerminalWorkerKind,
    TerminalWorkerLifecycle, TerminalWorkerLifecycleEventPayload, TerminalWorkerLifecycleToken,
    WirePath, WriteInjectionRequest,
};

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

fn operator() -> TerminalName {
    TerminalName::new("operator".to_owned())
}

fn token() -> TerminalWorkerLifecycleToken {
    TerminalWorkerLifecycleToken::new(operator().into())
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
    T: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    assert_eq!(text, canonical_text, "encode for {value:?}");
    let decoded = DotosSource::new(canonical_text)
        .parse::<T>()
        .expect("decode");
    assert_eq!(decoded, value, "decode for {canonical_text}");
    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.dotos missing line: {canonical_text}",
    );
}

#[test]
fn canonical_request_examples_round_trip() {
    round_trip(
        Input::TerminalConnection(TerminalConnectionRequest::new(operator().into())),
        "TerminalConnection.operator",
    );
    round_trip(
        Input::TerminalInput(TerminalInputRequest {
            terminal: operator().into(),
            input_bytes: hello_bytes().into(),
        }),
        "TerminalInput.{operator [104 101 108 108 111]}",
    );
    round_trip(
        Input::TerminalResize(TerminalResizeRequest {
            terminal: operator().into(),
            rows: TerminalRows::new(24).into(),
            columns: TerminalColumns::new(80).into(),
        }),
        "TerminalResize.{operator 24 80}",
    );
    round_trip(
        Input::TerminalDetachment(TerminalDetachmentRequest {
            terminal: operator().into(),
            terminal_detachment_reason: TerminalDetachmentReason::HumanRequested,
        }),
        "TerminalDetachment.{operator HumanRequested}",
    );
    round_trip(
        Input::TerminalCapture(TerminalCaptureRequest::new(operator().into())),
        "TerminalCapture.operator",
    );
    round_trip(
        Input::ResolveSession(ResolveSessionRequest::new(operator().into())),
        "ResolveSession.operator",
    );
    round_trip(
        Input::AcquireInputGate(AcquireInputGateRequest {
            terminal: operator().into(),
            input_gate_reason: InputGateReason::new("send router-delivered command".to_owned()),
            prompt_pattern_identifier_selection: None.into(),
        }),
        "AcquireInputGate.{operator (send router-delivered command) None}",
    );
    round_trip(
        Input::ReleaseInputGate(ReleaseInputGateRequest {
            terminal: operator().into(),
            lease: lease().into(),
        }),
        "ReleaseInputGate.{operator 42}",
    );
    round_trip(
        Input::WriteInjection(WriteInjectionRequest {
            terminal: operator().into(),
            lease: lease().into(),
            input_bytes: hello_bytes().into(),
        }),
        "WriteInjection.{operator 42 [104 101 108 108 111]}",
    );
    round_trip(
        Input::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycleRequest::new(
            operator().into(),
        )),
        "SubscribeTerminalWorkerLifecycle.operator",
    );
    round_trip(
        Input::TerminalWorkerLifecycleRetraction(token()),
        "TerminalWorkerLifecycleRetraction.operator",
    );
}

#[test]
fn canonical_reply_examples_round_trip() {
    round_trip(
        Output::TerminalReady(TerminalReadyReply {
            terminal: operator().into(),
            generation: TerminalGeneration::new(1).into(),
        }),
        "TerminalReady.{operator 1}",
    );
    round_trip(
        Output::TerminalInputAccepted(TerminalInputAcceptedReply {
            terminal: operator().into(),
            generation: TerminalGeneration::new(1).into(),
        }),
        "TerminalInputAccepted.{operator 1}",
    );
    round_trip(
        Output::GateAcquired(GateAcquiredReply {
            terminal: operator().into(),
            lease: lease().into(),
            prompt_state: PromptState::Clean,
        }),
        "GateAcquired.{operator 42 Clean}",
    );
    round_trip(
        Output::GateBusy(GateBusyReply {
            terminal: operator().into(),
            current_holder: InputGateLeaseIdentifier::new(41).into(),
        }),
        "GateBusy.{operator 41}",
    );
    round_trip(
        Output::InjectionAck(InjectionAckReply {
            terminal: operator().into(),
            generation: TerminalGeneration::new(1).into(),
            sequence: TerminalSequence::new(7).into(),
        }),
        "InjectionAck.{operator 1 7}",
    );
    round_trip(
        Output::InjectionRejected(InjectionRejectedReply {
            terminal: operator().into(),
            injection_rejection_reason: InjectionRejectionReason::UnknownTerminal,
        }),
        "InjectionRejected.{operator UnknownTerminal}",
    );
    round_trip(
        Output::SubscriptionRetracted(SubscriptionRetractedReply::new(token().into())),
        "SubscriptionRetracted.operator",
    );
    round_trip(
        Output::SessionResolved(SessionResolvedReply {
            name: operator().into(),
            data_socket_path: data_socket_path().into(),
        }),
        "SessionResolved.{operator /run/persona/terminal/sessions/operator/data.sock}",
    );
}

#[test]
fn canonical_event_example_round_trips() {
    round_trip(
        Output::Event(TerminalEvent::TerminalWorkerLifecycleEvent(
            TerminalWorkerLifecycleEventPayload {
                terminal: operator().into(),
                observation: TerminalWorkerLifecycle::Started(TerminalWorkerKind::ViewerFanout)
                    .into(),
            },
        )),
        "Event.TerminalWorkerLifecycleEvent.{operator Started.ViewerFanout}",
    );
}
