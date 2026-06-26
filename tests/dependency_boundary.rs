#[test]
fn terminal_contract_is_schema_derived_without_retired_helper_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");
    let source = include_str!("../src/lib.rs");

    assert!(
        cargo_toml.contains("schema-rust-next"),
        "schema-rust-next owns generated contract emission",
    );
    assert!(
        cargo_toml
            .lines()
            .any(|line| line.trim() == "build        = \"build.rs\""),
        "contract artifacts must be generated from schema/lib.schema",
    );
    assert!(
        !cargo_toml.contains("signal-engine-management"),
        "wire contracts must not drag old engine-management helper types forward",
    );
    assert!(
        !cargo_toml.contains("signal-persona-origin"),
        "owner/socket vocabulary is schema-local until a schema-derived shared origin contract exists",
    );
    assert!(
        !source.contains("signal_channel!"),
        "signal_channel! is deprecated; signal-terminal is schema-derived",
    );
    assert!(
        cargo_toml.contains("default = [\"nota-text\"]"),
        "direct signal-terminal users keep the NOTA projection by default",
    );
    assert!(
        cargo_toml.contains("nota-text = [\"dep:nota\", \"signal-frame/nota-text\"]"),
        "generated NOTA traits and signal-frame NOTA support are gated through the local feature",
    );
}
