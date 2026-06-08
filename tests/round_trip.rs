//! Architectural-truth tests for the schema-derived `signal-terminal`
//! contract.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention. The wire form is the schema-rust-next
//! emission on the `signal_frame::StreamingFrame` envelope.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
};
use signal_terminal::{
    AcquireInputGate, ExitCode, Frame, FrameBody, GateAcquired, GateBusy, GateReleased,
    InjectionAck, InjectionRejected, InjectionRejectionReason, Input, InputGateLease,
    InputGateLeaseIdentifier, InputGateReason, ListPromptPatterns, ListSessions, Output,
    OwnerIdentity, PromptPattern, PromptPatternBytes, PromptPatternEntry, PromptPatternIdentifier,
    PromptPatternList, PromptPatternRegistered, PromptPatternUnregistered, PromptState,
    RegisterPromptPattern, ReleaseInputGate, ResolveSession, SessionEntry, SessionList,
    SessionResolved, SocketMode, SubscribeTerminalWorkerLifecycle, SubscriptionRetracted,
    TerminalByteCount, TerminalCapture, TerminalCaptured, TerminalColumns, TerminalConnection,
    TerminalDaemonConfiguration, TerminalDetached, TerminalDetachment, TerminalDetachmentReason,
    TerminalEvent, TerminalExitStatus, TerminalExited, TerminalGeneration, TerminalInput,
    TerminalInputAccepted, TerminalInputBytes, TerminalName, TerminalOperationKind, TerminalReady,
    TerminalRejected, TerminalRejectionReason, TerminalResize, TerminalResized, TerminalRows,
    TerminalSequence, TerminalSignalNumber, TerminalTranscriptBytes, TerminalWorkerKind,
    TerminalWorkerLifecycle, TerminalWorkerLifecycleEvent, TerminalWorkerLifecycleSnapshot,
    TerminalWorkerLifecycleToken, TerminalWorkerStop, TerminalWorkerStopReason, TranscriptDelta,
    UnixUserIdentifier, UnregisterPromptPattern, WirePath, WorkerFailureDetail, WriteInjection,
};

fn terminal() -> TerminalName {
    TerminalName::new("operator".to_owned())
}

fn second_terminal() -> TerminalName {
    TerminalName::new("designer".to_owned())
}

fn data_socket_path(name: &str) -> WirePath {
    WirePath::new(format!("/run/persona/terminal/sessions/{name}/data.sock"))
}

fn prompt_pattern_identifier() -> PromptPatternIdentifier {
    PromptPatternIdentifier::new("codex-ready".to_owned())
}

fn input_gate_lease() -> InputGateLease {
    InputGateLease::new(InputGateLeaseIdentifier::new(42))
}

fn input_bytes() -> TerminalInputBytes {
    TerminalInputBytes::new(b"hello".iter().map(|byte| u64::from(*byte)).collect())
}

fn transcript_bytes() -> TerminalTranscriptBytes {
    TerminalTranscriptBytes::new(b"$ ".iter().map(|byte| u64::from(*byte)).collect())
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn stream_event() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: Input) -> Input {
    let expected = request.clone();
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &expected);
            decoded_request.payloads().head().clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Output) -> Output {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_event(event: TerminalEvent) -> TerminalEvent {
    let frame = Frame::new(FrameBody::SubscriptionEvent {
        event_identifier: stream_event(),
        token: SubscriptionTokenInner::new(1),
        event,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn every_request_round_trips_through_length_prefixed_frame() {
    let requests = [
        Input::TerminalConnection(TerminalConnection::new(terminal())),
        Input::TerminalInput(TerminalInput {
            terminal: terminal(),
            bytes: input_bytes(),
        }),
        Input::TerminalResize(TerminalResize {
            terminal: terminal(),
            rows: TerminalRows::new(24),
            columns: TerminalColumns::new(80),
        }),
        Input::TerminalDetachment(TerminalDetachment {
            terminal: terminal(),
            reason: TerminalDetachmentReason::HarnessStopped,
        }),
        Input::TerminalCapture(TerminalCapture::new(terminal())),
        Input::RegisterPromptPattern(RegisterPromptPattern {
            terminal: terminal(),
            pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(vec![36, 32])),
        }),
        Input::UnregisterPromptPattern(UnregisterPromptPattern {
            terminal: terminal(),
            pattern_id: prompt_pattern_identifier(),
        }),
        Input::ListPromptPatterns(ListPromptPatterns::new(terminal())),
        Input::AcquireInputGate(AcquireInputGate {
            terminal: terminal(),
            reason: InputGateReason::new("inject".to_owned()),
            prompt_pattern_identifier: Some(prompt_pattern_identifier()),
        }),
        Input::AcquireInputGate(AcquireInputGate {
            terminal: terminal(),
            reason: InputGateReason::new("inject".to_owned()),
            prompt_pattern_identifier: None,
        }),
        Input::ReleaseInputGate(ReleaseInputGate {
            terminal: terminal(),
            lease: input_gate_lease(),
        }),
        Input::WriteInjection(WriteInjection {
            terminal: terminal(),
            lease: input_gate_lease(),
            bytes: input_bytes(),
        }),
        Input::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle::new(terminal())),
        Input::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken::new(terminal())),
        Input::ListSessions(ListSessions {}),
        Input::ResolveSession(ResolveSession::new(terminal())),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn every_reply_round_trips_through_length_prefixed_frame() {
    let replies = [
        Output::TerminalReady(TerminalReady {
            terminal: terminal(),
            generation: TerminalGeneration::new(1),
        }),
        Output::TerminalInputAccepted(TerminalInputAccepted {
            terminal: terminal(),
            generation: TerminalGeneration::new(1),
        }),
        Output::TranscriptDelta(TranscriptDelta {
            terminal: terminal(),
            sequence: TerminalSequence::new(5),
            bytes: transcript_bytes(),
        }),
        Output::TerminalResized(TerminalResized {
            terminal: terminal(),
            rows: TerminalRows::new(40),
            columns: TerminalColumns::new(120),
            generation: TerminalGeneration::new(2),
        }),
        Output::TerminalCaptured(TerminalCaptured {
            terminal: terminal(),
            generation: TerminalGeneration::new(2),
            bytes: transcript_bytes(),
        }),
        Output::TerminalDetached(TerminalDetached {
            terminal: terminal(),
            generation: TerminalGeneration::new(2),
            reason: TerminalDetachmentReason::ViewerReplaced,
        }),
        Output::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(3),
            status: TerminalExitStatus::Exited(ExitCode::new(0)),
        }),
        Output::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(3),
            status: TerminalExitStatus::Signaled(TerminalSignalNumber::new(9)),
        }),
        Output::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(3),
            status: TerminalExitStatus::StatusUnavailable,
        }),
        Output::TerminalRejected(TerminalRejected {
            terminal: terminal(),
            reason: TerminalRejectionReason::TransportFailed,
        }),
        Output::PromptPatternRegistered(PromptPatternRegistered {
            terminal: terminal(),
            pattern_id: prompt_pattern_identifier(),
        }),
        Output::PromptPatternUnregistered(PromptPatternUnregistered {
            terminal: terminal(),
            pattern_id: prompt_pattern_identifier(),
        }),
        Output::PromptPatternList(PromptPatternList {
            terminal: terminal(),
            entries: vec![PromptPatternEntry {
                pattern_id: prompt_pattern_identifier(),
                pattern: PromptPattern::RegexSuffix(PromptPatternBytes::new(vec![36])),
            }],
        }),
        Output::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: PromptState::Dirty(TerminalByteCount::new(3)),
        }),
        Output::GateBusy(GateBusy {
            terminal: terminal(),
            current_holder: InputGateLeaseIdentifier::new(41),
        }),
        Output::GateReleased(GateReleased {
            terminal: terminal(),
            lease: input_gate_lease(),
            cached_human_bytes: TerminalByteCount::new(12),
        }),
        Output::InjectionAck(InjectionAck {
            terminal: terminal(),
            generation: TerminalGeneration::new(1),
            sequence: TerminalSequence::new(7),
        }),
        Output::InjectionRejected(InjectionRejected {
            terminal: terminal(),
            reason: InjectionRejectionReason::GateNotHeld,
        }),
        Output::TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot {
            terminal: terminal(),
            observations: vec![
                TerminalWorkerLifecycle::Started(TerminalWorkerKind::InputWriter),
                TerminalWorkerLifecycle::Stopped(TerminalWorkerStop {
                    worker: TerminalWorkerKind::OutputReader,
                    reason: TerminalWorkerStopReason::OutputReadFailed(WorkerFailureDetail::new(
                        "broken pipe".to_owned(),
                    )),
                }),
            ],
        }),
        Output::SubscriptionRetracted(SubscriptionRetracted::new(
            TerminalWorkerLifecycleToken::new(terminal()),
        )),
        Output::SessionList(SessionList::new(vec![
            SessionEntry {
                name: terminal(),
                data_socket_path: data_socket_path("operator"),
            },
            SessionEntry {
                name: second_terminal(),
                data_socket_path: data_socket_path("designer"),
            },
        ])),
        Output::SessionResolved(SessionResolved {
            name: terminal(),
            data_socket_path: data_socket_path("operator"),
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn worker_lifecycle_event_round_trips_through_subscription_frame() {
    let event = TerminalEvent::TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent {
        terminal: terminal(),
        observation: TerminalWorkerLifecycle::Stopped(TerminalWorkerStop {
            worker: TerminalWorkerKind::ChildExitWatcher,
            reason: TerminalWorkerStopReason::ChildExited(WorkerFailureDetail::new(
                "code 1".to_owned(),
            )),
        }),
    });

    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn payload_lift_into_request_uses_generated_from() {
    let payload = TerminalConnection::new(terminal());
    let request: Input = payload.clone().into();
    assert_eq!(request, Input::TerminalConnection(payload));

    // The name/type-mismatched retraction op lifts the token to its variant.
    let token = TerminalWorkerLifecycleToken::new(terminal());
    let retraction: Input = token.clone().into();
    assert_eq!(retraction, Input::TerminalWorkerLifecycleRetraction(token));
}

#[test]
fn payload_lift_into_reply_uses_generated_from() {
    let payload = TerminalReady {
        terminal: terminal(),
        generation: TerminalGeneration::new(4),
    };
    let reply: Output = payload.clone().into();
    assert_eq!(reply, Output::TerminalReady(payload));
}

#[test]
fn event_lifts_into_output_and_terminal_event() {
    let payload = TerminalWorkerLifecycleEvent {
        terminal: terminal(),
        observation: TerminalWorkerLifecycle::Started(TerminalWorkerKind::SocketAcceptLoop),
    };
    let event: TerminalEvent = payload.clone().into();
    assert_eq!(
        event,
        TerminalEvent::TerminalWorkerLifecycleEvent(payload.clone())
    );

    let output: Output = event.clone().into();
    assert_eq!(output, Output::Event(event));
}

#[test]
fn input_exposes_contract_owned_operation_kind() {
    assert_eq!(
        Input::TerminalConnection(TerminalConnection::new(terminal())).operation_kind(),
        TerminalOperationKind::TerminalConnection
    );
    assert_eq!(
        Input::WriteInjection(WriteInjection {
            terminal: terminal(),
            lease: input_gate_lease(),
            bytes: input_bytes(),
        })
        .operation_kind(),
        TerminalOperationKind::WriteInjection
    );
    assert_eq!(
        Input::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken::new(terminal()))
            .operation_kind(),
        TerminalOperationKind::TerminalWorkerLifecycleRetraction
    );
    assert_eq!(
        Input::ListSessions(ListSessions {}).operation_kind(),
        TerminalOperationKind::ListSessions
    );
}

#[test]
fn input_variants_declare_contract_local_operation_heads() {
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &[
            "TerminalConnection",
            "TerminalInput",
            "TerminalResize",
            "TerminalDetachment",
            "TerminalCapture",
            "RegisterPromptPattern",
            "UnregisterPromptPattern",
            "ListPromptPatterns",
            "AcquireInputGate",
            "ReleaseInputGate",
            "WriteInjection",
            "SubscribeTerminalWorkerLifecycle",
            "TerminalWorkerLifecycleRetraction",
            "ListSessions",
            "ResolveSession",
        ]
    );
}

#[test]
fn remodeled_enum_variants_round_trip_through_nota_text() {
    round_trip_nota(
        Output::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(2),
            status: TerminalExitStatus::Exited(ExitCode::new(0)),
        }),
        "(TerminalExited ([operator] 2 (Exited 0)))",
    );
    round_trip_nota(
        Output::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(2),
            status: TerminalExitStatus::Signaled(TerminalSignalNumber::new(9)),
        }),
        "(TerminalExited ([operator] 2 (Signaled 9)))",
    );
    round_trip_nota(
        Output::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: PromptState::Dirty(TerminalByteCount::new(3)),
        }),
        "(GateAcquired ([operator] 42 (Dirty 3)))",
    );
    round_trip_nota(
        TerminalWorkerLifecycle::Stopped(TerminalWorkerStop {
            worker: TerminalWorkerKind::OutputReader,
            reason: TerminalWorkerStopReason::OutputReadFailed(WorkerFailureDetail::new(
                "broken pipe".to_owned(),
            )),
        }),
        "(Stopped (OutputReader (OutputReadFailed [broken pipe])))",
    );
}

#[test]
fn byte_fields_carry_one_integer_per_byte_on_the_wire() {
    round_trip_nota(
        Input::TerminalInput(TerminalInput {
            terminal: terminal(),
            bytes: input_bytes(),
        }),
        "(TerminalInput ([operator] [104 101 108 108 111]))",
    );
}

#[test]
fn operation_kind_round_trips_through_nota_text() {
    round_trip_nota(TerminalOperationKind::WriteInjection, "WriteInjection");
}

#[test]
fn terminal_daemon_configuration_round_trips_through_nota_text() {
    let configuration = TerminalDaemonConfiguration {
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock".to_owned()),
        terminal_socket_mode: SocketMode::new(0o600),
        meta_terminal_socket_path: WirePath::new("/run/persona/X/meta-terminal.sock".to_owned()),
        meta_terminal_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new(
            "/run/persona/X/terminal-supervision.sock".to_owned(),
        ),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/terminal.sema".to_owned()),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<TerminalDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("[/run/persona/X/terminal.sock]"));
    assert!(text.contains("(UnixUser 1000)"));
}

#[test]
fn terminal_daemon_configuration_round_trips_through_rkyv() {
    let configuration = TerminalDaemonConfiguration {
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock".to_owned()),
        terminal_socket_mode: SocketMode::new(0o600),
        meta_terminal_socket_path: WirePath::new("/run/persona/X/meta-terminal.sock".to_owned()),
        meta_terminal_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new(
            "/run/persona/X/terminal-supervision.sock".to_owned(),
        ),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/terminal.sema".to_owned()),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = TerminalDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}
