use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_persona_terminal::{
    TerminalDeliveryAttemptObservation, TerminalDeliveryAttemptState, TerminalEvent,
    TerminalEventObservation, TerminalGeneration, TerminalInputAccepted,
    TerminalIntrospectionSnapshot, TerminalName, TerminalObservationSequence,
    TerminalOperationKind, TerminalSessionArchiveObservation, TerminalSessionArchiveState,
    TerminalSessionHealthObservation, TerminalSessionObservation, TerminalSessionState,
    TerminalViewerAttachmentObservation, TerminalViewerAttachmentState,
};

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn round_trip_archive<T>(value: T) -> T
where
    T: Archive
        + for<'archive> RkyvSerialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'archive>,
                rkyv::rancor::Error,
            >,
        > + PartialEq
        + std::fmt::Debug,
    <T as Archive>::Archived: for<'archive> rkyv::bytecheck::CheckBytes<
            rkyv::api::high::HighValidator<'archive, rkyv::rancor::Error>,
        > + RkyvDeserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
{
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).expect("archive");
    let recovered = rkyv::from_bytes::<T, rkyv::rancor::Error>(&bytes).expect("decode archive");
    assert_eq!(recovered, value);
    recovered
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
fn terminal_session_observation_is_contract_owned_introspection_record() {
    let observation =
        TerminalSessionObservation::ready(terminal(), "/run/persona/engine/terminal.sock");

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(
        observation.socket_path().as_str(),
        "/run/persona/engine/terminal.sock"
    );
    assert_eq!(observation.state(), TerminalSessionState::Ready);
}

#[test]
fn terminal_delivery_attempt_observation_round_trips() {
    let observation = TerminalDeliveryAttemptObservation::started(
        TerminalObservationSequence::new(7),
        terminal(),
        TerminalOperationKind::WriteInjection,
    );

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(observation.sequence().into_u64(), 7);
    assert_eq!(observation.state(), TerminalDeliveryAttemptState::Started);
}

#[test]
fn terminal_event_observation_round_trips() {
    let event = TerminalEvent::from(TerminalInputAccepted {
        terminal: terminal(),
        generation: TerminalGeneration::new(3),
    });
    let observation =
        TerminalEventObservation::new(TerminalObservationSequence::new(11), terminal(), event);

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(observation.sequence().into_u64(), 11);
}

#[test]
fn terminal_viewer_attachment_observation_round_trips() {
    let observation = TerminalViewerAttachmentObservation::new(
        TerminalObservationSequence::new(13),
        terminal(),
        "ghostty-window",
        TerminalViewerAttachmentState::Attached,
    );

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(observation.viewer().as_str(), "ghostty-window");
}

#[test]
fn terminal_session_health_observation_round_trips() {
    let observation = TerminalSessionHealthObservation::new(
        terminal(),
        TerminalSessionState::Ready,
        TerminalGeneration::new(2),
    );

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(observation.generation().into_u64(), 2);
}

#[test]
fn terminal_session_archive_observation_round_trips() {
    let observation = TerminalSessionArchiveObservation::archived(terminal(), "session rotated");

    assert_eq!(round_trip_archive(observation.clone()), observation);
    assert_eq!(observation.reason().as_str(), "session rotated");
    assert_eq!(observation.state(), TerminalSessionArchiveState::Archived);
}

#[test]
fn terminal_introspection_snapshot_round_trips_through_nota_text() {
    round_trip_nota(
        TerminalIntrospectionSnapshot {
            sessions: vec![TerminalSessionObservation::ready(
                terminal(),
                "/run/persona/engine/terminal.sock",
            )],
            delivery_attempts: vec![TerminalDeliveryAttemptObservation::started(
                TerminalObservationSequence::new(7),
                terminal(),
                TerminalOperationKind::WriteInjection,
            )],
            terminal_events: Vec::new(),
            viewer_attachments: Vec::new(),
            session_health: vec![TerminalSessionHealthObservation::new(
                terminal(),
                TerminalSessionState::Ready,
                TerminalGeneration::new(2),
            )],
            session_archive: vec![TerminalSessionArchiveObservation::archived(
                terminal(),
                "session rotated",
            )],
        },
        "(TerminalIntrospectionSnapshot [(TerminalSessionObservation operator \"/run/persona/engine/terminal.sock\" 1 0 Ready)] [(TerminalDeliveryAttemptObservation 7 operator WriteInjection Started)] [] [] [(TerminalSessionHealthObservation operator Ready 2)] [(TerminalSessionArchiveObservation operator \"session rotated\" Archived)])",
    );
}
