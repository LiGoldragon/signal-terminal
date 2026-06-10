use schema_rust_next::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-terminal",
        "0.2.1",
        "SIGNAL_TERMINAL_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
