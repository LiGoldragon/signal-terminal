use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{FrameBody, Reply, Request, SemaVerb};
use signal_persona_terminal::{
    AcquireInputGate, Frame, GateAcquired, GateBusy, GateReleased, InjectionAck, InjectionRejected,
    InjectionRejectionReason, InputGateLease, InputGateLeaseId, InputGateReason,
    ListPromptPatterns, PromptPattern, PromptPatternBytes, PromptPatternEntry, PromptPatternId,
    PromptPatternList, PromptPatternRegistered, PromptPatternUnregistered, PromptState,
    RegisterPromptPattern, ReleaseInputGate, SubscribeTerminalWorkerLifecycle, TerminalByteCount,
    TerminalCapture, TerminalCaptured, TerminalColumns, TerminalConnection, TerminalDetached,
    TerminalDetachment, TerminalDetachmentReason, TerminalEvent, TerminalExitStatus,
    TerminalExited, TerminalGeneration, TerminalInput, TerminalInputAccepted, TerminalInputBytes,
    TerminalName, TerminalOperationKind, TerminalReady, TerminalRejected, TerminalRejectionReason,
    TerminalRequest, TerminalResize, TerminalResized, TerminalRows, TerminalSequence,
    TerminalTranscriptBytes, TerminalWorkerKind, TerminalWorkerLifecycle,
    TerminalWorkerLifecycleEvent, TerminalWorkerLifecycleSnapshot, TerminalWorkerStopReason,
    TranscriptDelta, UnregisterPromptPattern, WriteInjection,
};

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn prompt_pattern_id() -> PromptPatternId {
    PromptPatternId::new("codex-ready")
}

fn input_gate_lease() -> InputGateLease {
    InputGateLease {
        id: InputGateLeaseId::new(42),
    }
}

fn round_trip_request(request: TerminalRequest) -> TerminalRequest {
    let expected_verb = request.signal_verb();
    let frame = Frame::new(FrameBody::Request(Request::operation(
        expected_verb,
        request.clone(),
    )));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, expected_verb);
            payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_event(event: TerminalEvent) -> TerminalEvent {
    let frame = Frame::new(FrameBody::Reply(Reply::operation(event.clone())));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply(Reply::Operation(event)) => event,
        other => panic!("expected reply operation, got {other:?}"),
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
        pattern_id: prompt_pattern_id(),
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
        "(RegisterPromptPattern operator (LiteralSuffix [62 32]))",
    );
}

#[test]
fn input_gate_requests_round_trip() {
    let acquire = TerminalRequest::AcquireInputGate(AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("message delivery"),
        prompt_pattern_id: Some(prompt_pattern_id()),
    });
    assert_eq!(round_trip_request(acquire.clone()), acquire);

    let acquire_without_prompt_check = TerminalRequest::AcquireInputGate(AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("raw control"),
        prompt_pattern_id: None,
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
            prompt_pattern_id: Some(prompt_pattern_id()),
        }),
        "(AcquireInputGate operator \"message delivery\" codex-ready)",
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
                pattern_id: prompt_pattern_id(),
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
                prompt_pattern_id: Some(prompt_pattern_id()),
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
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn terminal_request_variants_declare_expected_signal_root_verbs() {
    let cases = [
        (
            TerminalRequest::TerminalConnection(TerminalConnection {
                terminal: terminal(),
            }),
            SemaVerb::Assert,
        ),
        (
            TerminalRequest::TerminalInput(TerminalInput {
                terminal: terminal(),
                bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
            }),
            SemaVerb::Assert,
        ),
        (
            TerminalRequest::TerminalResize(TerminalResize {
                terminal: terminal(),
                rows: TerminalRows::new(32),
                columns: TerminalColumns::new(120),
            }),
            SemaVerb::Mutate,
        ),
        (
            TerminalRequest::TerminalDetachment(TerminalDetachment {
                terminal: terminal(),
                reason: TerminalDetachmentReason::HumanRequested,
            }),
            SemaVerb::Retract,
        ),
        (
            TerminalRequest::TerminalCapture(TerminalCapture {
                terminal: terminal(),
            }),
            SemaVerb::Match,
        ),
        (
            TerminalRequest::RegisterPromptPattern(RegisterPromptPattern {
                terminal: terminal(),
                pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
            }),
            SemaVerb::Assert,
        ),
        (
            TerminalRequest::UnregisterPromptPattern(UnregisterPromptPattern {
                terminal: terminal(),
                pattern_id: prompt_pattern_id(),
            }),
            SemaVerb::Retract,
        ),
        (
            TerminalRequest::ListPromptPatterns(ListPromptPatterns {
                terminal: terminal(),
            }),
            SemaVerb::Match,
        ),
        (
            TerminalRequest::AcquireInputGate(AcquireInputGate {
                terminal: terminal(),
                reason: InputGateReason::new("message delivery"),
                prompt_pattern_id: Some(prompt_pattern_id()),
            }),
            SemaVerb::Assert,
        ),
        (
            TerminalRequest::ReleaseInputGate(ReleaseInputGate {
                terminal: terminal(),
                lease: input_gate_lease(),
            }),
            SemaVerb::Retract,
        ),
        (
            TerminalRequest::WriteInjection(WriteInjection {
                terminal: terminal(),
                lease: input_gate_lease(),
                bytes: TerminalInputBytes::new(b"hello\r".to_vec()),
            }),
            SemaVerb::Assert,
        ),
        (
            TerminalRequest::SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle {
                terminal: terminal(),
            }),
            SemaVerb::Subscribe,
        ),
    ];

    for (request, verb) in cases {
        assert_eq!(request.signal_verb(), verb);
    }
}

#[test]
fn terminal_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(TerminalOperationKind::AcquireInputGate, "AcquireInputGate");
}

#[test]
fn terminal_ready_round_trips() {
    let event = TerminalEvent::TerminalReady(TerminalReady {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn terminal_input_accepted_round_trips() {
    let event = TerminalEvent::TerminalInputAccepted(TerminalInputAccepted {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn transcript_delta_round_trips() {
    let event = TerminalEvent::TranscriptDelta(TranscriptDelta {
        terminal: terminal(),
        sequence: TerminalSequence::new(7),
        bytes: TerminalTranscriptBytes::new(b"hello\r\n".to_vec()),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn terminal_resized_round_trips() {
    let event = TerminalEvent::TerminalResized(TerminalResized {
        terminal: terminal(),
        rows: TerminalRows::new(40),
        columns: TerminalColumns::new(100),
        generation: TerminalGeneration::new(2),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn terminal_captured_round_trips() {
    let event = TerminalEvent::TerminalCaptured(TerminalCaptured {
        terminal: terminal(),
        generation: TerminalGeneration::new(3),
        bytes: TerminalTranscriptBytes::new(b"screen".to_vec()),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn terminal_detached_round_trips() {
    let event = TerminalEvent::TerminalDetached(TerminalDetached {
        terminal: terminal(),
        generation: TerminalGeneration::new(4),
        reason: TerminalDetachmentReason::HarnessStopped,
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn terminal_exited_round_trips_for_each_status() {
    for status in [
        TerminalExitStatus::Exited { code: 0 },
        TerminalExitStatus::Signaled { signal: 15 },
        TerminalExitStatus::StatusUnavailable,
    ] {
        let event = TerminalEvent::TerminalExited(TerminalExited {
            terminal: terminal(),
            generation: TerminalGeneration::new(5),
            status: status.clone(),
        });
        assert_eq!(round_trip_event(event.clone()), event);
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
        let event = TerminalEvent::TerminalRejected(TerminalRejected {
            terminal: terminal(),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

#[test]
fn prompt_pattern_events_round_trip() {
    let registered = TerminalEvent::PromptPatternRegistered(PromptPatternRegistered {
        terminal: terminal(),
        pattern_id: prompt_pattern_id(),
    });
    assert_eq!(round_trip_event(registered.clone()), registered);

    let unregistered = TerminalEvent::PromptPatternUnregistered(PromptPatternUnregistered {
        terminal: terminal(),
        pattern_id: prompt_pattern_id(),
    });
    assert_eq!(round_trip_event(unregistered.clone()), unregistered);

    let list = TerminalEvent::PromptPatternList(PromptPatternList {
        terminal: terminal(),
        entries: vec![PromptPatternEntry {
            pattern_id: prompt_pattern_id(),
            pattern: PromptPattern::LiteralSuffix(PromptPatternBytes::new(b"> ".to_vec())),
        }],
    });
    assert_eq!(round_trip_event(list.clone()), list);
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
        let acquired = TerminalEvent::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: prompt_state.clone(),
        });
        assert_eq!(round_trip_event(acquired.clone()), acquired);
    }

    let busy = TerminalEvent::GateBusy(GateBusy {
        terminal: terminal(),
        current_holder: InputGateLeaseId::new(7),
    });
    assert_eq!(round_trip_event(busy.clone()), busy);

    let released = TerminalEvent::GateReleased(GateReleased {
        terminal: terminal(),
        lease: input_gate_lease(),
        cached_human_bytes: TerminalByteCount::new(12),
    });
    assert_eq!(round_trip_event(released.clone()), released);
}

#[test]
fn gate_acquired_event_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalEvent::GateAcquired(GateAcquired {
            terminal: terminal(),
            lease: input_gate_lease(),
            prompt_state: PromptState::Clean,
        }),
        "(GateAcquired operator (InputGateLease 42) (Clean))",
    );
}

#[test]
fn injection_events_round_trip() {
    let ack = TerminalEvent::InjectionAck(InjectionAck {
        terminal: terminal(),
        generation: TerminalGeneration::new(5),
        sequence: TerminalSequence::new(9),
    });
    assert_eq!(round_trip_event(ack.clone()), ack);

    for reason in [
        InjectionRejectionReason::UnknownTerminal,
        InjectionRejectionReason::UnknownLease,
        InjectionRejectionReason::GateNotHeld,
        InjectionRejectionReason::DirtyPrompt,
        InjectionRejectionReason::TransportFailed,
    ] {
        let rejected = TerminalEvent::InjectionRejected(InjectionRejected {
            terminal: terminal(),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_event(rejected.clone()), rejected);
    }
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
        TerminalEvent::TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot {
            terminal: terminal(),
            observations: observations.clone(),
        });
    assert_eq!(round_trip_event(snapshot.clone()), snapshot);

    let event = TerminalEvent::TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent {
        terminal: terminal(),
        observation: observations[0].clone(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn worker_lifecycle_snapshot_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalEvent::TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot {
            terminal: terminal(),
            observations: vec![
                TerminalWorkerLifecycle::Started(TerminalWorkerKind::InputWriter),
                TerminalWorkerLifecycle::Stopped {
                    worker: TerminalWorkerKind::OutputReader,
                    reason: TerminalWorkerStopReason::OutputReaderFinished,
                },
            ],
        }),
        "(TerminalWorkerLifecycleSnapshot operator [(Started InputWriter) (Stopped OutputReader (OutputReaderFinished))])",
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
fn from_impl_lifts_transcript_delta_into_event() {
    let payload = TranscriptDelta {
        terminal: terminal(),
        sequence: TerminalSequence::new(9),
        bytes: TerminalTranscriptBytes::new(b"via from".to_vec()),
    };
    let event: TerminalEvent = payload.clone().into();
    assert_eq!(event, TerminalEvent::TranscriptDelta(payload));
}

#[test]
fn from_impl_lifts_gate_acquisition_into_request() {
    let payload = AcquireInputGate {
        terminal: terminal(),
        reason: InputGateReason::new("delivery"),
        prompt_pattern_id: Some(prompt_pattern_id()),
    };
    let request: TerminalRequest = payload.clone().into();
    assert_eq!(request, TerminalRequest::AcquireInputGate(payload));
}

#[test]
fn from_impl_lifts_injection_ack_into_event() {
    let payload = InjectionAck {
        terminal: terminal(),
        generation: TerminalGeneration::new(1),
        sequence: TerminalSequence::new(1),
    };
    let event: TerminalEvent = payload.clone().into();
    assert_eq!(event, TerminalEvent::InjectionAck(payload));
}

#[test]
fn terminal_contract_names_persona_terminal_as_the_production_endpoint() {
    let source = include_str!("../src/lib.rs");

    assert!(source.contains("Persona-terminal owns prompt-pattern registration"));
    assert!(source.contains("terminal-cell"));
    assert!(!source.contains("terminal-cell's control plane"));
    assert!(!source.contains("terminal-cell integration callers"));
}
