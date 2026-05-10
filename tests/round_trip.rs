use signal_core::{FrameBody, Reply, Request, SemaVerb};
use signal_persona_terminal::{
    Frame, TerminalCapture, TerminalCaptured, TerminalColumns, TerminalConnection,
    TerminalDetached, TerminalDetachment, TerminalDetachmentReason, TerminalEvent,
    TerminalExitStatus, TerminalExited, TerminalGeneration, TerminalInput, TerminalInputAccepted,
    TerminalInputBytes, TerminalName, TerminalReady, TerminalRejected, TerminalRejectionReason,
    TerminalRequest, TerminalResize, TerminalResized, TerminalRows, TerminalSequence,
    TerminalTranscriptBytes, TranscriptDelta,
};

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn round_trip_request(request: TerminalRequest) -> TerminalRequest {
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Assert);
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
