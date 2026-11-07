//! The 4.2.0 newtypes keep the wire and the text of the type aliases they replace.
//!
//! `support/previous_410.rs` is `src/generated/signal.rs` at 4.1.0 (fe5d668) byte for byte, so the
//! old shapes, field order and variant order are the reviewed ones. Each sample is built once in
//! the old family and once in the new, from the same data, and the archived bytes (and, with
//! `datom`, the text) must be equal. The samples are representative values, not a proof for every
//! value, and they cover a subset of the variants listed in the test names.

#[path = "support/previous_410.rs"]
#[allow(clippy::all, dead_code)]
mod previous;

use rkyv::rancor::Error;
use signal_terminal as current;

macro_rules! old_w {
    ($t:ident, $e:expr) => {
        $e
    };
}
macro_rules! new_w {
    ($t:ident, $e:expr) => {
        $t($e)
    };
}

/// The same field values, built as the aliases (old) or as the newtypes (new).
macro_rules! family {
    ($name:ident, $m:ident, $w:ident) => {
        mod $name {
            #![allow(unused_imports)]
            use super::$m::*;
            pub fn terminal(s: &str) -> Terminal {
                $w!(Terminal, $w!(TerminalName, s.to_string()))
            }
            pub fn rows(n: i64) -> Rows {
                $w!(Rows, $w!(TerminalRows, n))
            }
            pub fn columns(n: i64) -> Columns {
                $w!(Columns, $w!(TerminalColumns, n))
            }
            pub fn generation(n: i64) -> Generation {
                $w!(Generation, $w!(TerminalGeneration, n))
            }
            pub fn sequence(n: i64) -> Sequence {
                $w!(Sequence, $w!(TerminalSequence, n))
            }
            pub fn input_bytes(v: Vec<i64>) -> InputBytes {
                $w!(InputBytes, $w!(TerminalInputBytes, v))
            }
            pub fn transcript_bytes(v: Vec<i64>) -> TranscriptBytes {
                $w!(TranscriptBytes, $w!(TerminalTranscriptBytes, v))
            }
            pub fn pattern(v: Vec<i64>) -> Pattern {
                $w!(
                    Pattern,
                    PromptPattern::LiteralSuffix($w!(PromptPatternBytes, v))
                )
            }
            pub fn regex(v: Vec<i64>) -> Pattern {
                $w!(
                    Pattern,
                    PromptPattern::RegexSuffix($w!(PromptPatternBytes, v))
                )
            }
            pub fn identifier(s: &str) -> PatternIdentifier {
                $w!(
                    PatternIdentifier,
                    $w!(PromptPatternIdentifier, s.to_string())
                )
            }
            pub fn selection(s: Option<&str>) -> PromptPatternIdentifierSelection {
                $w!(
                    PromptPatternIdentifierSelection,
                    s.map(|s| $w!(PromptPatternIdentifier, s.to_string()))
                )
            }
            pub fn reason(s: &str) -> InputGateReason {
                $w!(InputGateReason, s.to_string())
            }
            pub fn lease(n: i64) -> Lease {
                $w!(
                    Lease,
                    InputGateLease {
                        input_gate_lease_identifier: $w!(InputGateLeaseIdentifier, n)
                    }
                )
            }
            pub fn name(s: &str) -> Name {
                $w!(Name, $w!(TerminalName, s.to_string()))
            }
            pub fn data_socket(s: &str) -> DataSocketPath {
                $w!(DataSocketPath, $w!(WirePath, s.to_string()))
            }
            pub fn entry(id: &str, bytes: Vec<i64>) -> PromptPatternEntry {
                PromptPatternEntry {
                    pattern_identifier: identifier(id),
                    pattern: pattern(bytes),
                }
            }
            pub fn session(name_: &str, socket: &str) -> SessionEntry {
                SessionEntry {
                    name: name(name_),
                    data_socket_path: data_socket(socket),
                }
            }
            pub fn entries(v: Vec<PromptPatternEntry>) -> Entries {
                $w!(Entries, v)
            }
            pub fn sessions(v: Vec<SessionEntry>) -> SessionEntries {
                $w!(SessionEntries, v)
            }
            pub fn observation() -> Observation {
                $w!(
                    Observation,
                    TerminalWorkerLifecycle::Started(TerminalWorkerKind::TranscriptScriber)
                )
            }
            pub fn exited(code: i64) -> TerminalExitStatus {
                TerminalExitStatus::Exited($w!(ExitCode, code))
            }
            pub fn signaled(n: i64) -> TerminalExitStatus {
                TerminalExitStatus::Signaled($w!(TerminalSignalNumber, n))
            }
            pub fn dirty(n: i64) -> PromptState {
                PromptState::Dirty($w!(TerminalByteCount, n))
            }
            pub fn user(n: i64) -> OwnerIdentity {
                OwnerIdentity::UnixUser($w!(UnixUserIdentifier, n))
            }
            pub fn principal(s: &str) -> OwnerIdentity {
                OwnerIdentity::System($w!(SystemPrincipal, s.to_string()))
            }
            pub fn configuration(owner: OwnerIdentity) -> TerminalDaemonConfiguration {
                TerminalDaemonConfiguration {
                    terminal_socket_path: $w!(
                        TerminalSocketPath,
                        $w!(WirePath, "/run/terminal.sock".to_string())
                    ),
                    terminal_socket_mode: $w!(TerminalSocketMode, $w!(SocketMode, 0o660)),
                    meta_terminal_socket_path: $w!(
                        MetaTerminalSocketPath,
                        $w!(WirePath, "/run/terminal-meta.sock".to_string())
                    ),
                    meta_terminal_socket_mode: $w!(MetaTerminalSocketMode, $w!(SocketMode, 0o600)),
                    supervision_socket_path: $w!(
                        SupervisionSocketPath,
                        $w!(WirePath, "/run/terminal-supervision.sock".to_string())
                    ),
                    supervision_socket_mode: $w!(SupervisionSocketMode, $w!(SocketMode, 0o600)),
                    store_path: $w!(StorePath, $w!(WirePath, "/var/lib/terminal".to_string())),
                    owner_identity: owner,
                }
            }
        }
    };
}

family!(oldk, previous, old_w);
family!(newk, current, new_w);

/// Every sample, once for a family, from the helpers `$k` and the types of `$m`.
macro_rules! samples {
    ($m:ident, $k:ident) => {{
        use $m::*;
        Samples {
            queries: vec![
                Query::TerminalConnection(TerminalConnectionRequest {
                    terminal: $k::terminal("main"),
                }),
                Query::TerminalInput(TerminalInputRequest {
                    terminal: $k::terminal("main"),
                    input_bytes: $k::input_bytes(vec![104, 105, 10]),
                }),
                Query::TerminalResize(TerminalResizeRequest {
                    terminal: $k::terminal("main"),
                    rows: $k::rows(24),
                    columns: $k::columns(80),
                }),
                Query::RegisterPromptPattern(RegisterPromptPatternRequest {
                    terminal: $k::terminal("main"),
                    pattern: $k::pattern(vec![36, 32]),
                }),
                Query::RegisterPromptPattern(RegisterPromptPatternRequest {
                    terminal: $k::terminal("main"),
                    pattern: $k::regex(vec![92, 36]),
                }),
                Query::AcquireInputGate(AcquireInputGateRequest {
                    terminal: $k::terminal("main"),
                    input_gate_reason: $k::reason("inject"),
                    prompt_pattern_identifier_selection: $k::selection(Some("p1")),
                }),
                Query::AcquireInputGate(AcquireInputGateRequest {
                    terminal: $k::terminal("main"),
                    input_gate_reason: $k::reason("none"),
                    prompt_pattern_identifier_selection: $k::selection(None),
                }),
                Query::WriteInjection(WriteInjectionRequest {
                    terminal: $k::terminal("main"),
                    lease: $k::lease(9),
                    input_bytes: $k::input_bytes(vec![1, 2, 3]),
                }),
                Query::ListSessions(ListSessionsRequest {}),
                Query::ResolveSession(ResolveSessionRequest {
                    name: $k::name("session"),
                }),
            ],
            responses: vec![
                Response::TerminalReady(TerminalReadyReply {
                    terminal: $k::terminal("main"),
                    generation: $k::generation(3),
                }),
                Response::TranscriptDelta(TranscriptDeltaReply {
                    terminal: $k::terminal("main"),
                    sequence: $k::sequence(11),
                    transcript_bytes: $k::transcript_bytes(vec![79, 75]),
                }),
                Response::TerminalCaptured(TerminalCapturedReply {
                    terminal: $k::terminal("main"),
                    generation: $k::generation(3),
                    transcript_bytes: $k::transcript_bytes(vec![]),
                }),
                Response::TerminalExited(TerminalExitedReply {
                    terminal: $k::terminal("main"),
                    generation: $k::generation(3),
                    terminal_exit_status: $k::exited(0),
                }),
                Response::TerminalExited(TerminalExitedReply {
                    terminal: $k::terminal("main"),
                    generation: $k::generation(3),
                    terminal_exit_status: $k::signaled(9),
                }),
                Response::TerminalRejected(TerminalRejectedReply {
                    terminal: $k::terminal("main"),
                    terminal_rejection_reason: TerminalRejectionReason::TransportFailed,
                }),
                Response::PromptPatternList(PromptPatternListReply {
                    terminal: $k::terminal("main"),
                    entries: $k::entries(vec![
                        $k::entry("p1", vec![36]),
                        $k::entry("p2", vec![62, 32]),
                    ]),
                }),
                Response::GateAcquired(GateAcquiredReply {
                    terminal: $k::terminal("main"),
                    lease: $k::lease(9),
                    prompt_state: $k::dirty(4),
                }),
                Response::SessionList(SessionListReply {
                    session_entries: $k::sessions(vec![
                        $k::session("a", "/run/a.sock"),
                        $k::session("b", "/run/b.sock"),
                    ]),
                }),
                Response::SessionResolved(SessionResolvedReply {
                    name: $k::name("a"),
                    data_socket_path: $k::data_socket("/run/a.sock"),
                }),
                Response::Event(TerminalEvent::TerminalWorkerLifecycleEvent(
                    TerminalWorkerLifecycleEventPayload {
                        terminal: $k::terminal("main"),
                        observation: $k::observation(),
                    },
                )),
            ],
            configurations: vec![
                $k::configuration($k::user(1000)),
                $k::configuration($k::principal("root")),
            ],
        }
    }};
}

struct Samples<Q, R, C> {
    queries: Vec<Q>,
    responses: Vec<R>,
    configurations: Vec<C>,
}

fn old_samples()
-> Samples<previous::Query, previous::Response, previous::TerminalDaemonConfiguration> {
    samples!(previous, oldk)
}

fn new_samples() -> Samples<current::Query, current::Response, current::TerminalDaemonConfiguration>
{
    samples!(current, newk)
}

macro_rules! archived {
    ($value:expr) => {
        rkyv::to_bytes::<Error>($value).expect("archive").to_vec()
    };
}

macro_rules! assert_equal_bytes {
    ($label:expr, $old:expr, $new:expr) => {{
        assert_eq!($old.len(), $new.len(), "{}", $label);
        for (index, (old, new)) in $old.iter().zip($new.iter()).enumerate() {
            assert_eq!(
                archived!(old),
                archived!(new),
                "{} sample {}",
                $label,
                index
            );
        }
    }};
}

#[test]
fn the_current_types_archive_to_the_bytes_of_the_410_types() {
    let (old, new) = (old_samples(), new_samples());
    assert_equal_bytes!("Query", old.queries, new.queries);
    assert_equal_bytes!("Response", old.responses, new.responses);
    assert_equal_bytes!(
        "TerminalDaemonConfiguration",
        old.configurations,
        new.configurations
    );
}

#[test]
fn an_old_archive_restores_as_the_current_value() {
    let (old, new) = (old_samples(), new_samples());
    for (index, (old, new)) in old.queries.iter().zip(&new.queries).enumerate() {
        let restored = rkyv::from_bytes::<current::Query, Error>(&archived!(old)).expect("restore");
        assert_eq!(&restored, new, "Query sample {index}");
    }
    for (index, (old, new)) in old.responses.iter().zip(&new.responses).enumerate() {
        let restored =
            rkyv::from_bytes::<current::Response, Error>(&archived!(old)).expect("restore");
        assert_eq!(&restored, new, "Response sample {index}");
    }
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn written<T: Datomizable>(value: &T) -> String {
        value.datomize(Vec::new()).protosize().textualize()
    }

    fn read<T: Composing>(text: &str) -> T {
        let mut budget = Budget {
            remaining: 100_000,
            reader: ReaderBudget::try_from(100_000usize).unwrap(),
            depth: 0,
            maximum_depth: 128,
        };
        Potential::<T>::from(text)
            .actualize(&mut budget)
            .expect("read the datom text")
    }

    #[test]
    fn the_current_types_write_the_text_of_the_410_types_and_read_it_back() {
        let (old, new) = (old_samples(), new_samples());
        for (index, (old, new)) in old.queries.iter().zip(&new.queries).enumerate() {
            let text = written(old);
            assert_eq!(text, written(new), "Query sample {index}");
            assert_eq!(&read::<current::Query>(&text), new, "Query sample {index}");
        }
        for (index, (old, new)) in old.responses.iter().zip(&new.responses).enumerate() {
            let text = written(old);
            assert_eq!(text, written(new), "Response sample {index}");
            assert_eq!(
                &read::<current::Response>(&text),
                new,
                "Response sample {index}"
            );
        }
        for (index, (old, new)) in old
            .configurations
            .iter()
            .zip(&new.configurations)
            .enumerate()
        {
            let text = written(old);
            assert_eq!(text, written(new), "Configuration sample {index}");
            assert_eq!(
                &read::<current::TerminalDaemonConfiguration>(&text),
                new,
                "Configuration sample {index}"
            );
        }
    }
}
