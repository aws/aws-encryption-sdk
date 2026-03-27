# Deferred Annotations — footer (message.md#structure)

## Requirement: Unrecognized signature algorithm error

- **Spec target**: `specification/data-format/message.md#structure`
- **TOML quote**: `If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.`
- **Current state**: Has `type=implementation` annotation in `encrypt.rs` line 399 (on the `_ =>` wildcard match arm).
- **Why no `type=test`**: The `SignatureAlgorithm` enum in Rust is exhaustive — the only variants are `Ecdsa(...)` and `None`. The `_ =>` wildcard arm is unreachable from the public API because there is no way to construct an unrecognized `SignatureAlgorithm` variant. A test would need to bypass the type system to reach this code path.
- **Recommendation**: This is a defensive coding pattern. The implementation annotation is sufficient. If the enum gains new variants in the future, the compiler will flag the match as non-exhaustive, forcing an update. No `type=test` annotation is possible without unsafe/internal access.
