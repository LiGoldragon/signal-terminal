use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("signal-terminal"),
        SchemaVersion::new("0.3.0"),
        UpdateEnvironmentVariable::new("SIGNAL_TERMINAL_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::SignalSpirit,
    )
    .expect_fresh();
}
