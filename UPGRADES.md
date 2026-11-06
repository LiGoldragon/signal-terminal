# Upgrades

## 4.1.0

- The optional `datom` feature repins datom-codec 0.33.0 (eabead4) and protos
  0.33.0 (3c7ade3), up from 0.31.0. A consumer enabling `datom` repins both to
  those revisions and runs its tests.
- protos 0.33.0 reads a bare unbalanced guillemet inside a guillemet string, or
  a literal backslash before a guillemet, differently, and prints a balanced
  guillemet pair inside a string without an escape. A consumer holding datom text
  with either form re-reads it and updates the expectation.
- The generated contract is unchanged; the build script still checks it against
  `ethos/signal.ethos` with ethos-zero 10.0.0.
- A consumer building a `ReaderBudget` by its field replaces it with
  `ReaderBudget::try_from(n).unwrap()`; the field is private in protos 0.32.0 and later.
