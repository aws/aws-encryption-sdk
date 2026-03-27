# Agent 3 Notes — materials-decrypt

## Adversarial Pre-Review (Step 2)

### 1. Does each annotation's next line actually implement THAT requirement?

These are all `type=test` annotations. The question is: does the test actually exercise the requirement?

- **Req 1 (commitment policy error)**: `test_pre_cmm_commitment_policy_check` — encrypts with non-committing suite (`AlgAes256GcmIv12Tag16HkdfSha256`), then decrypts with `RequireEncryptRequireDecrypt`. This exercises the pre-CMM `validate_commitment_policy_on_decrypt()` call in `decrypt.rs`. The test asserts `result.is_err()`. ✅ Directly exercises the requirement.

- **Req 2 (input CMM used)**: `test_cmm_used_is_input_cmm` — creates a CMM explicitly, encrypts and decrypts with it. The decrypt path uses `create_cmm_from_input()` which returns the input CMM. Successful round-trip proves the input CMM was used. ✅ Directly exercises the requirement.

- **Req 3 (default CMM from keyring)**: `test_default_cmm_constructed_from_keyring` — encrypts and decrypts with keyring (not CMM). The decrypt path calls `create_cmm_from_input()` which constructs a default CMM from the keyring. Successful round-trip proves default CMM was constructed. ✅ Directly exercises the requirement.

- **Req 4 (data key derivation)**: `test_data_key_derived_from_plaintext_data_key` — uses HKDF suite to ensure key derivation actually happens (not identity KDF). Successful round-trip proves the derived data key was correct. ✅ Exercises the requirement.

- **Req 5 (algorithm suite from materials)**: `test_algorithm_suite_from_decryption_materials` — uses HKDF suite. Successful round-trip proves the algorithm suite from materials was used. ✅ Exercises the requirement. Note: this test is nearly identical to Req 4's test — same suite, same flow. The only difference is the annotation quote. This is acceptable since they are distinct spec requirements.

- **Req 6 (commit key derivation)**: `test_commit_key_derived_and_validated` — uses committing suite (`AlgAes256GcmHkdfSha512CommitKey`). Successful round-trip proves commit key was derived correctly. ✅ Exercises the requirement.

- **Req 7 (commit key equality)**: Same test as Req 6. Successful round-trip with committing suite proves the derived commit key matched the header's commit key (otherwise `validate_suite_data()` would fail). ✅ Exercises the requirement.

- **Req 8 (KDF algorithm from materials)**: `test_kdf_algorithm_from_materials_suite` — uses HKDF suite. Successful round-trip proves the KDF algorithm from the materials suite was used. ✅ Exercises the requirement. Note: this test is nearly identical to Req 4 and Req 5 tests.

### 2. Annotation stacking check

- `test_commit_key_derived_and_validated`: 2 annotation blocks before first code line. Within the hard limit of <3. Both are tested by the same code path. ✅ Acceptable.
- All other tests: 1 annotation block each. ✅ No stacking issues.

### 3. Per-block isolation evaluation

All annotation blocks are at the top of their respective test functions, immediately before the test setup code. For `type=test` annotations, this is the correct pattern — the annotation describes what the test proves, and the test body follows.

Each annotation quote is self-contained and clearly describes what is being tested. No ambiguous references.

### 4. Semantic relationship

All annotations semantically relate to their test functions:
- Error test (Req 1) asserts error on policy mismatch
- CMM tests (Req 2, 3) exercise CMM resolution paths
- Derivation tests (Req 4, 5, 8) exercise key derivation with HKDF
- Commitment tests (Req 6, 7) exercise commit key derivation and validation

### 5. Sub-items

No sub-items applicable — these are all standalone MUST requirements.

### 6. Code structure mirrors spec

The tests follow the spec's logical flow: commitment policy check → CMM resolution → key derivation → commit key validation.

### 7. Top-to-bottom readability

The test file reads top-to-bottom with clear test names and annotations. ✅

## Anti-Rationalization Check (Step 3)

Reviewing my notes for "but" patterns:

- Req 4/5/8 tests are nearly identical in structure (same suite, same flow, different annotation). I noted this but did NOT flag it. Let me reconsider: Is this a problem? The work item guidance explicitly says "Requirements 4-5 (data key derivation, algorithm suite from materials) are proven by successful round-trip decryption" and "Requirement 8 (KDF algorithm from materials) is proven by successful round-trip with an HKDF suite." The suggested groupings say "Test A: Round-trip with keyring input (covers Req 3, 4, 5, 8)." Agent 2 chose to create separate tests instead of grouping. This is a style choice — separate tests provide clearer traceability. The work item said "Group related requirements into minimal tests to avoid redundancy" but also provided separate test suggestions. The duplication is minor (test setup code) and doesn't affect correctness. I'll note this as a suggestion, not a blocking issue.

- `make_keyring` duplicates `test_keyring` functionality. `make_keyring(0)` ≡ `test_keyring()`. This is minor code quality — could have reused `test_keyring()` for the key_byte=0 case. Non-blocking.

No other "but" patterns found.

## Pre-Review Gate (Step 4)

- ✅ Test file was modified: `test_get_decryption_materials.rs` with 8 `type=test` annotations.

## Test Results

- All 15 tests in `test_get_decryption_materials` pass (verified by running `cargo test --test test_get_decryption_materials`)
- Pre-existing failures in `test_authentication_tag` and `test_encrypt_decrypt` are KMS credential issues, not related to Agent 2's changes
- Clippy passes (pre-existing warnings only, none in modified file)

## Cross-Reference Check

Scanning annotation quotes for markdown links:
- Req 1: `[algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)`, `[commitment policy](client.md#commitment-policy)`, `[client](client.md)` — these are spec cross-references in the quote text. For `type=test` annotations, cross-reference annotations from linked specs are not required (the test proves the behavior, not the linked spec's definition).
- Req 3: `[default CMM](../framework/default-cmm.md)`, `[keyring](../framework/keyring-interface.md)` — same reasoning.
- Req 4: `[decryption materials](../framework/structures.md#decryption-materials)` — same.
- Req 5: `[decryption materials](../framework/structures.md#decryption-materials)` — same.
- Req 6: Multiple links to algorithm-suites.md — same.
- Req 8: Multiple links to algorithm-suites.md — same.

Cross-references in test annotations are informational (they describe what the spec references), not actionable (the test doesn't need to separately annotate the linked spec). This is consistent with existing test patterns in the file.

## Potential Spec Gaps

None identified. The implementation and tests align with the spec requirements.
