# Agent 2 Notes — encrypt.md#input and decrypt.md#input annotations

## Pre-Implementation Reasoning

### 1. Logical steps

1. Add `type=implication` annotations for SHOULD requirements (3, 11) on `source` fields
2. Add `type=test` annotations for structural "accept" requirements (1, 2, 6, 7, 8, 9, 10, 12) in test file
3. Add `type=test` annotations for validate/fail requirements (4, 5) on existing `test_bad_encrypt_input`

### 2. Point of fulfillment

- Req 1 (plaintext): `pub plaintext: &'a [u8]` field — already has implication annotation
- Req 2 (CMM/keyring): `pub source: Option<MaterialSource>` field — already has implication annotation
- Req 3 (SHOULD optional): `pub source: Option<MaterialSource>` on EncryptInput — MISSING, needs implication
- Req 4 (validate exactly one): `validate()` method — already has implementation annotation
- Req 5 (fail if not exactly one): `validate()` method — already has implementation annotation
- Req 6 (algorithm suite): `pub algorithm_suite_id: Option<...>` — already has implication annotation
- Req 7 (encryption context): `pub encryption_context: EncryptionContext` — already has implication annotation
- Req 8 (frame length): `pub frame_length: FrameLength` — already has implication annotation
- Req 9 (encrypted message): `pub ciphertext: &'a [u8]` — already has implementation annotation
- Req 10 (CMM/keyring decrypt): `pub source: Option<MaterialSource>` — already has implementation annotation
- Req 11 (SHOULD optional decrypt): `pub source: Option<MaterialSource>` on DecryptInput — MISSING, needs implication
- Req 12 (encryption context decrypt): `pub encryption_context: EncryptionContext` — already has implication annotation

### 3. Sub-items?

No sub-items. Each requirement is standalone.

### 4. Reviewer readability

- Source changes: Add 2 implication annotations on `source` fields in types.rs
- Test changes: Add new test functions in test_create_esdk_client.rs (structural tests) and annotations on test_bad_encrypt_input in test_encrypt_decrypt.rs

### 5. Existing similar code

- `test_create_esdk_client.rs` has `test_encrypt_input_custom_commitment_policy` — same pattern for structural field tests
- `test_encrypt_decrypt.rs` has `test_bad_decrypt_input` with duvet annotations — same pattern for validate/fail tests

### Cross-reference analysis

- Req 1 contains `[plaintext](#plaintext)` — self-referential anchor, no cross-ref needed
- Req 2 contains `[cryptographic Materials Manager (CMM)](../framework/cmm-interface.md)` and `[keyring](../framework/keyring-interface.md)` — these link to framework specs not tracked in this project's duvet config for this code. No cross-ref needed.
- Req 3 — no links
- Req 4 — no links
- Req 5 — no links
- Req 6 contains `[Algorithm Suite](#algorithm-suite)` — self-referential, no cross-ref needed
- Req 7 contains `[Encryption Context](#encryption-context)` — self-referential, no cross-ref needed
- Req 8 contains `[Frame Length](#frame-length)` — self-referential, no cross-ref needed
- Req 9 contains `[Encrypted Message](#encrypted-message)` — self-referential, no cross-ref needed
- Req 10 same as Req 2
- Req 11 — no links
- Req 12 contains `[Encryption Context](#encryption-context)` — self-referential, no cross-ref needed
