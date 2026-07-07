use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-terminal",
        "0.2.3",
        "SIGNAL_TERMINAL_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
