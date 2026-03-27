## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs` — Added 7 new test functions with 8 `type=test` annotations covering all 8 missing requirements for `get-the-decryption-materials`. Added `encrypt_with` and `make_keyring` helpers. Added imports for `EsdkCommitmentPolicy` and `EsdkAlgorithmSuiteId`.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs
```

### Requirements Addressed
- ✅ Req 1: "If the parsed [algorithm suite ID]... is not supported by the [commitment policy]... decrypt MUST yield an error." — tested (negative test with non-committing suite + RequireEncryptRequireDecrypt)
- ✅ Req 2: "The CMM used MUST be the input CMM, if supplied." — tested (round-trip with explicit CMM input)
- ✅ Req 3: "If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM]... from the input [keyring]..." — tested (round-trip with keyring input)
- ✅ Req 4: "The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key..." — tested (round-trip with HKDF suite)
- ✅ Req 5: "The algorithm suite used as input for all decryption described below MUST be the algorithm suite included in the [decryption materials]..." — tested (round-trip with HKDF suite)
- ✅ Req 6: "If the [algorithm suite]... supports [key commitment]... then the [commit key]... MUST be derived from the plaintext data key..." — tested (round-trip with committing suite)
- ✅ Req 7: "The derived commit key MUST equal the commit key stored in the message header." — tested (round-trip with committing suite)
- ✅ Req 8: "The algorithm suite used to derive a data key from the plaintext data key MUST be the [key derivation algorithm]... included in the [algorithm suite]..." — tested (round-trip with HKDF suite)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`
- **Number of `type=test` annotations added**: 8 for 8 requirements
- **Test function names**:
  - `test_pre_cmm_commitment_policy_check` (Req 1)
  - `test_cmm_used_is_input_cmm` (Req 2)
  - `test_default_cmm_constructed_from_keyring` (Req 3)
  - `test_data_key_derived_from_plaintext_data_key` (Req 4)
  - `test_algorithm_suite_from_decryption_materials` (Req 5)
  - `test_commit_key_derived_and_validated` (Req 6 + Req 7)
  - `test_kdf_algorithm_from_materials_suite` (Req 8)

### Proposed Commit Message

```
test(decrypt): add test annotations for get-the-decryption-materials

Add 7 test functions with 8 type=test annotations covering all
missing requirements in decrypt.md#get-the-decryption-materials:
- Pre-CMM commitment policy check (negative test)
- CMM resolution (input CMM vs keyring → default CMM)
- Data key derivation from plaintext data key
- Algorithm suite from decryption materials
- Commit key derivation and equality validation
- KDF algorithm from materials suite

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
```

### Duvet Verification (actual command output)
```
$ make duvet
[extract + report completed successfully]
Scanned 543 sources 22ms
Parsed 2644 annotations 101ms
Loaded 68 specifications 24ms
Mapped 494 sections 11ms
Matched 4463 references 5ms
Sorted 4463 references 23ms
Wrote specification_compliance_report.html 15ms
```

### Test Results (actual command output)
```
$ cargo test --test test_get_decryption_materials
running 15 tests
test test_commit_key_derived_and_validated ... ok
test test_cmm_call_algorithm_suite_id ... ok
test test_decrypt_fails_with_wrong_keyring ... ok
test test_cmm_call_constructed_as_follows ... ok
test test_default_cmm_constructed_from_keyring ... ok
test test_cmm_used_is_input_cmm ... ok
test test_cmm_call_encrypted_data_keys ... ok
test test_algorithm_suite_from_decryption_materials ... ok
test test_data_key_derived_from_plaintext_data_key ... ok
test test_cmm_call_encryption_context ... ok
test test_cmm_call_reproduced_encryption_context ... ok
test test_cmm_call_commitment_policy ... ok
test test_kdf_algorithm_from_materials_suite ... ok
test test_pre_cmm_commitment_policy_check ... ok
test test_obtain_decryption_materials_via_cmm ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- All 8 requirements only needed `type=test` annotations — implementation annotations already existed in `decrypt.rs` and `materials.rs`
- `test_commit_key_derived_and_validated` covers both Req 6 (commit key derivation) and Req 7 (commit key equality) since they are exercised by the same code path with a committing algorithm suite
- Pre-existing clippy warnings in `encrypt.rs` and `materials.rs` are unrelated to these changes
