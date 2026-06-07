use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
};
use signal_terminal::{
    AcquireInputGate, GateAcquired, GateBusy, GateReleased, InjectionAck, InjectionRejected,
    InjectionRejectionReason, InputGateLease, InputGateLeaseIdentifier, InputGateReason,
    ListPromptPatterns, ListSessions, PromptPattern, PromptPatternBytes, PromptPatternEntry,
    PromptPatternIdentifier, PromptPatternList, PromptPatternRegistered, PromptPatternUnregistered,
    PromptState, RegisterPromptPattern, ReleaseInputGate, ResolveSession, SessionEntry,
    SessionList, SessionResolved, SubscribeTerminalWorkerLifecycle, SubscriptionRetracted,
    TerminalByteCount, TerminalCapture, TerminalCaptured, TerminalColumns, TerminalConnection,
    TerminalDetached, TerminalDetachment, TerminalDetachmentReason, TerminalEvent,
    TerminalExitStatus, TerminalExited, TerminalFrame as Frame, TerminalFrameBody as FrameBody,
    TerminalGeneration, TerminalInput, TerminalInputAccepted, TerminalInputBytes, TerminalName,
    TerminalOperationKind, TerminalReady, TerminalRejected, TerminalRejectionReason, TerminalReply,
    TerminalRequest, TerminalResize, TerminalResized, TerminalRows, TerminalSequence,
    TerminalStreamKind, TerminalTranscriptBytes, TerminalWorkerKind, TerminalWorkerLifecycle,
    TerminalWorkerLifecycleEvent, TerminalWorkerLifecycleSnapshot, TerminalWorkerLifecycleToken,
    TerminalWorkerStopReason, TranscriptDelta, UnregisterPromptPattern, WriteInjection,
};

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn second_terminal() -> TerminalName {
    TerminalName::new("designer")
}

fn data_socket_path(name: &str) -> signal_persona::WirePath {
    signal_persona::WirePath::new(format!("/run/persona/terminal/sessions/{name}/data.sock"))
}

fn prompt_pattern_identifier() -> PromptPatternIdentifier {
    PromptPatternIdentifier::new("codex-ready")
}

fn input_gate_lease() -> InputGateLease {
    InputGateLease {
        id: InputGateLeaseIdentifier::new(42),
    }
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

fn round_trip_request(request: TerminalRequest) -> TerminalRequest {
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

fn round_trip_reply(reply: TerminalReply) -> TerminalReply {
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
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn terminal_connection_round_trips() {
    let request = TerminalRequest::TerminalConnection(TerminalConnection {
        terminal: terminal(),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn terminal_input_round_trips() {
    let request = TerminalRequest::TerminalInput(TerminalInput {
        terminal: terminal(),
        bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn terminal_resize_round_trips() {
    let request = TerminalRequest::TerminalResize(TerminalResize {
        terminal: terminal(),
        rows: TerminalRows::new(32),
        columns: TerminalColumns::new(120),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn terminal_detachment_round_trips_for_each_reason() {
    for reason in [
        TerminalDetachmentReason::HumanRequested,
        TerminalDetachmentReason::HarnessStopped,
        TerminalDetachmentReason::ViewerReplaced,
    ] {
        let request = TerminalRequest::TerminalDetachment(TerminalDetachment {
            terminal: terminal(),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn terminal_capture_round_trips() {
    let request = TerminalRequest::TerminalCapture(TerminalCapture {
        terminal: terminal(),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn prompt_pattern_requests_round_trip() {
    let literal = TerminalRequest::RegisterPromptPattern(RegisterPromptPattern {
        terminal: terminal(),
        pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
    });
    assert_eq!(round_trip_request(literal.clone()), literal);

    let regex = TerminalRequest::RegisterPromptPattern(RegisterPromptPattern {
        terminal: terminal(),
        pattern: PromptPattern::RegexSuffix {
            pattern: PromptPatternBytes::new(br"(?m)^assistant> $".to_vec()),
        },
    });
    assert_eq!(round_trip_request(regex.clone()), regex);

    let unregister = TerminalRequest::UnregisterPromptPattern(UnregisterPromptPattern {
        terminal: terminal(),
        pattern_id: prompt_pattern_identifier(),
    });
    assert_eq!(round_trip_request(unregister.clone()), unregister);

    let list = TerminalRequest::ListPromptPatterns(ListPromptPatterns {
        terminal: terminal(),
    });
    assert_eq!(round_trip_request(list.clone()), list);
}

#[test]
fn prompt_pattern_registration_request_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalRequest::RegisterPromptPattern(RegisterPromptPattern {
            terminal: terminal(),
            pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
        }),
        "(RegisterPromptPattern (operator (LiteralSuffix [62 32])))",
    );
}

#[test]
fn input_gate_requests_round_trip() {
    let acquire = TerminalRequest::AcquireInputGate(AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("message delivery"),
        prompt_pattern_identifier: Some(prompt_pattern_identifier()),
    });
    assert_eq!(round_trip_request(acquire.clone()), acquire);

    let acquire_without_prompt_check = TerminalRequest::AcquireInputGate(AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("raw control"),
        prompt_pattern_identifier: None,
    });
    assert_eq!(
        round_trip_request(acquire_without_prompt_check.clone()),
        acquire_without_prompt_check
    );

    let release = TerminalRequest::ReleaseInputGate(ReleaseInputGate {
        terminal: terminal(),
        lease: input_gate_lease(),
    });
    assert_eq!(round_trip_request(release.clone()), release);
}

#[test]
fn acquire_input_gate_request_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalRequest::AcquireInputGate(AcquireInputGate {
            terminal: terminal(),
            reason: InputGateReason::new("message delivery"),
            prompt_pattern_identifier: Some(prompt_pattern_identifier()),
        }),
        "(AcquireInputGate (operator [message delivery] (Some codex-ready)))",
    );
}

#[test]
fn injection_and_worker_requests_round_trip() {
    let injection = TerminalRequest::WriteInjection(WriteInjection {
        terminal: terminal(),
        lease: input_gate_lease(),
        bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
    });
    assert_eq!(round_trip_request(injection.clone()), injection);

    let subscription =
        TerminalRequest::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle {
            terminal: terminal(),
        });
    assert_eq!(round_trip_request(subscription.clone()), subscription);
}

#[test]
fn session_registry_requests_round_trip() {
    let list = TerminalRequest::ListSessions(ListSessions {});
    assert_eq!(round_trip_request(list.clone()), list);

    let resolve = TerminalRequest::ResolveSession(ResolveSession { name: terminal() });
    assert_eq!(round_trip_request(resolve.clone()), resolve);
}

#[test]
fn terminal_request_exposes_contract_owned_operation_kind() {
    let cases = [
        (
            TerminalRequest::TerminalConnection(TerminalConnection {
                terminal: terminal(),
            }),
            TerminalOperationKind::TerminalConnection,
        ),
        (
            TerminalRequest::TerminalInput(TerminalInput {
                terminal: terminal(),
                bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
            }),
            TerminalOperationKind::TerminalInput,
        ),
        (
            TerminalRequest::TerminalResize(TerminalResize {
                terminal: terminal(),
                rows: TerminalRows::new(32),
                columns: TerminalColumns::new(120),
            }),
            TerminalOperationKind::TerminalResize,
        ),
        (
            TerminalRequest::TerminalDetachment(TerminalDetachment {
                terminal: terminal(),
                reason: TerminalDetachmentReason::HumanRequested,
            }),
            TerminalOperationKind::TerminalDetachment,
        ),
        (
            TerminalRequest::TerminalCapture(TerminalCapture {
                terminal: terminal(),
            }),
            TerminalOperationKind::TerminalCapture,
        ),
        (
            TerminalRequest::RegisterPromptPattern(RegisterPromptPattern {
                terminal: terminal(),
                pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
            }),
            TerminalOperationKind::RegisterPromptPattern,
        ),
        (
            TerminalRequest::UnregisterPromptPattern(UnregisterPromptPattern {
                terminal: terminal(),
                pattern_id: prompt_pattern_identifier(),
            }),
            TerminalOperationKind::UnregisterPromptPattern,
        ),
        (
            TerminalRequest::ListPromptPatterns(ListPromptPatterns {
                terminal: terminal(),
            }),
            TerminalOperationKind::ListPromptPatterns,
        ),
        (
            TerminalRequest::AcquireInputGate(AcquireInputGate {
                terminal: terminal(),
                reason: InputGateReason::new("message delivery"),
                prompt_pattern_identifier: Some(prompt_pattern_identifier()),
            }),
            TerminalOperationKind::AcquireInputGate,
        ),
        (
            TerminalRequest::ReleaseInputGate(ReleaseInputGate {
                terminal: terminal(),
                lease: input_gate_lease(),
            }),
            TerminalOperationKind::ReleaseInputGate,
        ),
        (
            TerminalRequest::WriteInjection(WriteInjection {
                terminal: terminal(),
                lease: input_gate_lease(),
                bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
            }),
            TerminalOperationKind::WriteInjection,
        ),
        (
            TerminalRequest::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle {
                terminal: terminal(),
            }),
            TerminalOperationKind::SubscribeTerminalWorkerLifecycle,
        ),
        (
            TerminalRequest::ListSessions(ListSessions {}),
            TerminalOperationKind::ListSessions,
        ),
        (
            TerminalRequest::ResolveSession(ResolveSession { name: terminal() }),
            TerminalOperationKind::ResolveSession,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn terminal_request_heads_are_contract_local_operations() {
    assert_eq!(
        <TerminalRequest as SignalOperationHeads>::HEADS,
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
fn terminal_worker_lifecycle_operations_name_their_stream() {
    let watch =
        TerminalRequest::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle {
            terminal: terminal(),
        });
    let unwatch =
        TerminalRequest::TerminalWorkerLifecycleRetraction(TerminalWorkerLifecycleToken {
            terminal: terminal(),
        });

    assert_eq!(
        watch.opened_stream(),
        Some(TerminalStreamKind::TerminalWorkerLifecycleStream)
    );
    assert_eq!(
        unwatch.closed_stream(),
        Some(TerminalStreamKind::TerminalWorkerLifecycleStream)
    );
}

#[test]
fn terminal_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(TerminalOperationKind::AcquireInputGate, "AcquireInputGate");
}

#[test]
fn terminal_ready_round_trips() {
    let reply = TerminalReply::TerminalReady(TerminalReady {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn terminal_input_accepted_round_trips() {
    let reply = TerminalReply::TerminalInputAccepted(TerminalInputAccepted {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn transcript_delta_round_trips() {
    let reply = TerminalReply::TranscriptDelta(TranscriptDelta {
        terminal: terminal(),
        sequence: TerminalSequence::new(7),
        bytes: TerminalTranscriptBytes::new(b"hello\r\n".to_vec()),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn terminal_resized_round_trips() {
    let reply = TerminalReply::TerminalResized(TerminalResized {
        terminal: terminal(),
        rows: TerminalRows::new(40),
        columns: TerminalColumns::new(100),
        generation: TerminalGeneration::new(2),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn terminal_captured_round_trips() {
    let reply = TerminalReply::TerminalCaptured(TerminalCaptured {
        terminal: terminal(),
        generation: TerminalGeneration::new(3),
        bytes: TerminalTranscriptBytes::new(b"screen".to_vec()),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn terminal_detached_round_trips() {
    let reply = TerminalReply::TerminalDetached(TerminalDetached {
        terminal: terminal(),
        generation: TerminalGeneration::new(4),
        reason: TerminalDetachmentReason::HarnessStopped,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn terminal_exited_round_trips_for_each_status() {
    for status in [
        TerminalExitStatus::Exited { code: 0 },
        TerminalExitStatus::Signaled { signal: 15 },
        TerminalExitStatus::StatusUnavailable,
    ] {
        let reply = TerminalReply::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(5),
            status: status.clone(),
        });
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn terminal_rejected_round_trips_for_each_reason() {
    for reason in [
        TerminalRejectionReason::NotConnected,
        TerminalRejectionReason::InputRejected,
        TerminalRejectionReason::ResizeRejected,
        TerminalRejectionReason::CaptureRejected,
        TerminalRejectionReason::TransportFailed,
    ] {
        let reply = TerminalReply::TerminalRejected(TerminalRejected {
            terminal: terminal(),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn prompt_pattern_events_round_trip() {
    let registered = TerminalReply::PromptPatternRegistered(PromptPatternRegistered {
        terminal: terminal(),
        pattern_id: prompt_pattern_identifier(),
    });
    assert_eq!(round_trip_reply(registered.clone()), registered);

    let unregistered = TerminalReply::PromptPatternUnregistered(PromptPatternUnregistered {
        terminal: terminal(),
        pattern_id: prompt_pattern_identifier(),
    });
    assert_eq!(round_trip_reply(unregistered.clone()), unregistered);

    let list = TerminalReply::PromptPatternList(PromptPatternList {
        terminal: terminal(),
        entries: vec![PromptPatternEntry {
            pattern_id: prompt_pattern_identifier(),
            pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
        }],
    });
    assert_eq!(round_trip_reply(list.clone()), list);
}

#[test]
fn input_gate_events_round_trip() {
    for prompt_state in [
        PromptState::NotChecked,
        PromptState::Clean,
        PromptState::Dirty {
            trailing_count: TerminalByteCount::new(3),
        },
    ] {
        let acquired = TerminalReply::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: prompt_state.clone(),
        });
        assert_eq!(round_trip_reply(acquired.clone()), acquired);
    }

    let busy = TerminalReply::GateBusy(GateBusy {
        terminal: terminal(),
        current_holder: InputGateLeaseIdentifier::new(7),
    });
    assert_eq!(round_trip_reply(busy.clone()), busy);

    let released = TerminalReply::GateReleased(GateReleased {
        terminal: terminal(),
        lease: input_gate_lease(),
        cached_human_bytes: TerminalByteCount::new(12),
    });
    assert_eq!(round_trip_reply(released.clone()), released);
}

#[test]
fn gate_acquired_event_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalReply::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: PromptState::Clean,
        }),
        "(GateAcquired (operator (42) (Clean)))",
    );
}

#[test]
fn injection_events_round_trip() {
    let ack = TerminalReply::InjectionAck(InjectionAck {
        terminal: terminal(),
        generation: TerminalGeneration::new(5),
        sequence: TerminalSequence::new(9),
    });
    assert_eq!(round_trip_reply(ack.clone()), ack);

    for reason in [
        InjectionRejectionReason::UnknownTerminal,
        InjectionRejectionReason::UnknownLease,
        InjectionRejectionReason::GateNotHeld,
        InjectionRejectionReason::DirtyPrompt,
        InjectionRejectionReason::TransportFailed,
    ] {
        let rejected = TerminalReply::InjectionRejected(InjectionRejected {
            terminal: terminal(),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_reply(rejected.clone()), rejected);
    }
}

#[test]
fn session_registry_replies_round_trip() {
    let list = TerminalReply::SessionList(SessionList {
        entries: vec![
            SessionEntry {
                name: terminal(),
                data_socket_path: data_socket_path("operator"),
            },
            SessionEntry {
                name: second_terminal(),
                data_socket_path: data_socket_path("designer"),
            },
        ],
    });
    assert_eq!(round_trip_reply(list.clone()), list);

    let resolved = TerminalReply::SessionResolved(SessionResolved {
        name: terminal(),
        data_socket_path: data_socket_path("operator"),
    });
    assert_eq!(round_trip_reply(resolved.clone()), resolved);
}

#[test]
fn worker_lifecycle_events_round_trip() {
    let observations = vec![
        TerminalWorkerLifecycle::Started(TerminalWorkerKind::InputWriter),
        TerminalWorkerLifecycle::Stopped {
            worker: TerminalWorkerKind::OutputReader,
            reason: TerminalWorkerStopReason::OutputReaderFinished,
        },
        TerminalWorkerLifecycle::Stopped {
            worker: TerminalWorkerKind::AttachConnectionPump,
            reason: TerminalWorkerStopReason::AttachConnectionFailed("closed".to_string()),
        },
    ];

    let snapshot =
        TerminalReply::TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot {
            terminal: terminal(),
            observations: observations.clone(),
        });
    assert_eq!(round_trip_reply(snapshot.clone()), snapshot);

    let event = TerminalEvent::TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent {
        terminal: terminal(),
        observation: observations[0].clone(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn worker_lifecycle_snapshot_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalReply::TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot {
            terminal: terminal(),
            observations: vec![
                TerminalWorkerLifecycle::Started(TerminalWorkerKind::InputWriter),
                TerminalWorkerLifecycle::Stopped {
                    worker: TerminalWorkerKind::OutputReader,
                    reason: TerminalWorkerStopReason::OutputReaderFinished,
                },
            ],
        }),
        "(TerminalWorkerLifecycleSnapshot (operator [(Started InputWriter) (Stopped OutputReader (OutputReaderFinished))]))",
    );
}

#[test]
fn from_impl_lifts_terminal_input_into_request() {
    let payload = TerminalInput {
        terminal: terminal(),
        bytes: TerminalInputBytes::new(b"via from".to_vec()),
    };
    let request: TerminalRequest = payload.clone().into();
    assert_eq!(request, TerminalRequest::TerminalInput(payload));
}

#[test]
fn from_impl_lifts_transcript_delta_into_reply() {
    let payload = TranscriptDelta {
        terminal: terminal(),
        sequence: TerminalSequence::new(9),
        bytes: TerminalTranscriptBytes::new(b"via from".to_vec()),
    };
    let reply: TerminalReply = payload.clone().into();
    assert_eq!(reply, TerminalReply::TranscriptDelta(payload));
}

#[test]
fn from_impl_lifts_gate_acquisition_into_request() {
    let payload = AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("delivery"),
        prompt_pattern_identifier: Some(prompt_pattern_identifier()),
    };
    let request: TerminalRequest = payload.clone().into();
    assert_eq!(request, TerminalRequest::AcquireInputGate(payload));
}

#[test]
fn subscription_retracted_reply_round_trips_through_length_prefixed_frame() {
    let token = TerminalWorkerLifecycleToken {
        terminal: terminal(),
    };
    let reply = TerminalReply::SubscriptionRetracted(SubscriptionRetracted {
        token: token.clone(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);

    let lifted: TerminalReply = SubscriptionRetracted { token }.into();
    assert!(matches!(lifted, TerminalReply::SubscriptionRetracted(_)));
}

#[test]
fn from_impls_lift_session_registry_records() {
    let resolved = SessionResolved {
        name: terminal(),
        data_socket_path: data_socket_path("operator"),
    };
    let reply: TerminalReply = resolved.clone().into();
    assert_eq!(reply, TerminalReply::SessionResolved(resolved));
}

#[test]
fn from_impl_lifts_injection_ack_into_reply() {
    let payload = InjectionAck {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
        sequence: TerminalSequence::new(1),
    };
    let reply: TerminalReply = payload.clone().into();
    assert_eq!(reply, TerminalReply::InjectionAck(payload));
}

#[test]
fn terminal_contract_names_terminal_as_the_production_endpoint() {
    let source = include_str!("../src/lib.rs");

    assert!(source.contains("Terminal owns prompt-pattern registration"));
    assert!(source.contains("terminal-cell"));
    assert!(!source.contains("terminal-cell's control plane"));
    assert!(!source.contains("terminal-cell integration callers"));
}

#[test]
fn terminal_daemon_configuration_round_trips_through_nota_text() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};
    use signal_terminal::TerminalDaemonConfiguration;

    let configuration = TerminalDaemonConfiguration {
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        terminal_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/terminal-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/terminal.sema"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let mut encoder = Encoder::new();
    configuration
        .encode(&mut encoder)
        .expect("encode configuration");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered =
        TerminalDaemonConfiguration::decode(&mut decoder).expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn terminal_daemon_configuration_round_trips_through_rkyv() {
    use nota_config::ConfigurationRecord;
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};
    use signal_terminal::TerminalDaemonConfiguration;

    let configuration = TerminalDaemonConfiguration {
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        terminal_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/terminal-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/terminal.sema"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
    let recovered = TerminalDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}
