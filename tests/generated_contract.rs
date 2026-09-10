use signal_terminal::{
    ByteViewable, ListSessionsRequest, Query, Response, Restorable, SessionListReply, Signal,
    Signalizable,
};
#[test]
fn list_sessions_query_and_reply_round_trip_through_received_bytes() {
    let query = Query::ListSessions(ListSessionsRequest {});
    let received = Signal::<Query>::from(query.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), query);
    let response = Response::SessionList(SessionListReply {
        session_entries: vec![],
    });
    let received =
        Signal::<Response>::from(response.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), response);
}
#[cfg(feature = "datom")]
#[test]
fn list_sessions_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::ListSessions(ListSessionsRequest {});
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(text);
    let decoded = pending
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("actualize");
    assert_eq!(decoded, query);
}
