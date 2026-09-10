# Architecture

`ethos/signal.ethos` is the authored terminal Signal contract. `src/generated/signal.rs` is checked-in Ethos output and the build script rejects stale output. The public API exports the generated named `Query` and `Response` types plus the source-borne `Signalizable`, `Signal`, `ByteViewable`, and `Restorable` binary transport capabilities.

The default dependency graph is portable rkyv only. Enabling `datom` adds generated Datom text actualization for every declared contract type.
