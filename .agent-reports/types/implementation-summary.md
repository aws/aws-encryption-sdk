## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs` — Added 2 `type=implication` annotations for SHOULD requirements on `source` fields of `EncryptInput` and `DecryptInput`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs` — Added 8 new test functions with `type=test` annotations for encrypt.md#input and decrypt.md#input structural requirements
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs` — Added 2 `type=test` annotations to existing `test_bad_encrypt_input` for validate/fail requirements

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs
```

### Requirements Addressed
- ✅ `- The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument.` — implication (existing) + test (new)
- ✅ `- The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.` — implication (existing) + test (new)
- ✅ `The keyring and CMM inputs SHOULD be optional.` (encrypt) — implication (new on EncryptInput.source)
- ✅ `The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.` — implementation (existing) + test (new)
- ✅ `If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.` — implementation (existing) + test (new)
- ✅ `- The input to the Encrypt operation MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.` — implication (existing) + test (new)
- ✅ `- The input to the Encrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.` — implication (existing) + test (new)
- ✅ `- The input to the Encrypt operation MUST accept an optional [Frame Length](#frame-length) argument.` — implication (existing) + test (new)
- ✅ `- The input to the Decrypt operation MUST accept a required [Encrypted Message](#encrypted-message) argument.` — implementation (existing) + test (new)
- ✅ `- The input to the Decrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.` — implementation (existing) + test (new)
- ✅ `The keyring and CMM inputs SHOULD be optional.` (decrypt) — implication (new on DecryptInput.source)
- ✅ `- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.` — implication (existing) + test (new)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`, `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
- **Number of `type=test` annotations added**: 10 for 10 requirements
- **Test function names**:
  - `test_encrypt_input_accepts_plaintext`
  - `test_encrypt_input_accepts_cmm_and_keyring`
  - `test_encrypt_input_accepts_optional_algorithm_suite`
  - `test_encrypt_input_accepts_optional_encryption_context`
  - `test_encrypt_input_accepts_optional_frame_length`
  - `test_decrypt_input_accepts_encrypted_message`
  - `test_decrypt_input_accepts_cmm_and_keyring`
  - `test_decrypt_input_accepts_optional_encryption_context`
  - `test_bad_encrypt_input` (2 annotations added to existing test)

### Proposed Commit Message

```
test(types): add duvet annotations for encrypt.md#input and decrypt.md#input

Add type=implication annotations on EncryptInput and DecryptInput
source fields for the SHOULD-optional requirement on CMM/keyring inputs.

Add 10 type=test annotations across 9 test functions covering all
MUST requirements in encrypt.md#input (8 requirements) and
decrypt.md#input (5 requirements, 2 already had test annotations).

Spec sections:
- specification/client-apis/encrypt.md#input
- specification/client-apis/decrypt.md#input
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 31ms
    Scanning sources
     Scanned 158 sources 9ms
     Parsing annotations
      Parsed 1043 annotations 39ms
     Loading specifications
      Loaded 14 specifications 19ms
     Mapping sections
      Mapped 138 sections 15ms
    Matching references
     Matched 1909 references 2ms
     Sorting references
      Sorted 1909 references 13ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 18ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 1ms
```

### Test Results (actual command output)
```
$ cargo test --test test_create_esdk_client
running 15 tests
test test_decrypt_input_accepts_encrypted_message ... ok
test test_decrypt_input_accepts_cmm_and_keyring ... ok
test test_decrypt_input_accepts_optional_encryption_context ... ok
test test_decrypt_input_default_commitment_policy ... ok
test test_encrypt_input_accepts_cmm_and_keyring ... ok
test test_decrypt_input_default_max_edks_is_none ... ok
test test_encrypt_input_accepts_optional_algorithm_suite ... ok
test test_encrypt_input_accepts_optional_encryption_context ... ok
test test_encrypt_input_custom_commitment_policy ... ok
test test_encrypt_input_accepts_plaintext ... ok
test test_encrypt_input_accepts_optional_frame_length ... ok
test test_encrypt_input_custom_max_edks ... ok
test test_encrypt_input_default_commitment_policy ... ok
test test_encrypt_input_default_max_edks_is_none ... ok
test test_net_retry_flag ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- The SHOULD requirements (3, 11) are annotated as `type=implication` with `reason=` because the optionality is structural (Option<MaterialSource>), not runtime-testable.
- Pre-existing clippy warnings in encrypt.rs and materials.rs are unrelated to these changes.
- The `test_bad_decrypt_input` test already had duvet annotations for decrypt validate/fail requirements — no changes needed there.
- Duvet snapshot confirms all encrypt.md#input and decrypt.md#input requirements have appropriate coverage (implication+test or implementation+test for MUSTs, implication for SHOULDs).
