# Pre-Implementation Reasoning — Key Derivation Tests

## 1. Logical steps in this spec section

1. The algorithm used to derive a data key MUST be the KDF from the algorithm suite
2. If identity KDF → derived key MUST equal plaintext key
3. If HKDF → derivation MUST follow HKDF Encryption Key process

## 2. Point of fulfillment for each requirement

- Req 1 (parent: algorithm MUST be KDF from suite): fulfilled at the `match &suite.kdf` in `derive_key()` — the match dispatches to the correct KDF. A round-trip test with any suite proves the correct KDF was selected.
- Req 2 (identity KDF): fulfilled at `data_key: plaintext_data_key.to_vec()` — the identity branch returns the plaintext key unchanged. A round-trip with `AlgAes256GcmIv12Tag16NoKdf` proves this.
- Req 3 (HKDF): fulfilled at the `hkdf()` call in the `Hkdf` branch. A round-trip with `AlgAes256GcmIv12Tag16HkdfSha256` proves this.

## 3. Sub-items

The parent requirement has two sub-items (identity KDF and HKDF). Each gets its own test annotation.

## 4. Reviewer readability

Three test functions, each with a single annotation block and a round-trip assertion. Straightforward.

## 5. Existing similar code

- `tests/test_v1_header_body.rs` — uses `ForbidEncryptAllowDecrypt` for v1 suites, raw AES keyring
- `tests/test_get_decryption_materials.rs` — uses `round_trip` helper
- `tests/test_construct_a_frame.rs` — similar round-trip pattern

## Notes

- Source annotations use `specification/` prefix (symlink to `aws-encryption-sdk-specification/`)
- Some test files use `aws-encryption-sdk-specification/` prefix — I'll use `specification/` to match the source and the duvet config
- The TOML quotes match exactly what's in the source annotations
