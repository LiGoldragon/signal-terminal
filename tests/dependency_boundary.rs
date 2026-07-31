#[test]
fn terminal_contract_is_schema_derived_without_retired_helper_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");
    let cargo_lock = include_str!("../Cargo.lock");
    let source = include_str!("../src/lib.rs");

    assert!(
        cargo_toml.contains("schema-rust"),
        "schema-rust owns generated contract emission",
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
    let stale_schema_inputs = [
        ["schema", "-next"].concat(),
        ["schema-rust", "-next"].concat(),
        ["drop", "-next"].concat(),
        ["Specified", "Schema"].concat(),
        ["emit_rust", "_from_schema"].concat(),
        ["emit_rust", "_from_specified_schema"].concat(),
    ];
    for stale_schema_input in stale_schema_inputs {
        assert!(
            !cargo_toml.contains(&stale_schema_input),
            "Cargo.toml must not carry stale pre-TrueSchema input {stale_schema_input}",
        );
        assert!(
            !cargo_lock.contains(&stale_schema_input),
            "Cargo.lock must not carry stale pre-TrueSchema input {stale_schema_input}",
        );
        assert!(
            !source.contains(&stale_schema_input),
            "source must not carry stale pre-TrueSchema helper {stale_schema_input}",
        );
    }
    assert!(
        cargo_toml.contains("default = [\"dotos-text\"]"),
        "direct signal-terminal users keep the DOTOS projection by default",
    );
    assert!(
        cargo_toml.contains("dotos-text = [\"dep:dotos\", \"signal-frame/dotos-text\"]"),
        "generated DOTOS traits and signal-frame DOTOS support are gated through the local feature",
    );
}
