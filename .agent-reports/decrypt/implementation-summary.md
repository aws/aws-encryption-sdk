## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs` — Added missing `#behavior` annotation for the "MUST NOT perform this step" requirement in `step_verify_signature`, using an explicit `else` block
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_behavior.rs` — New test file verifying decrypt skips signature verification for non-signing algorithm suites

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_behavior.rs
```

### Requirements Addressed
- ✅ "If the message header does not contain an algorithm suite including a signature algorithm, the Decrypt operation MUST NOT perform this step." — implemented + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_behavior.rs`
- **Number of `type=test` annotations added**: 1 for 1 requirement
- **Test function names**: `test_decrypt_skips_signature_step_for_non_signing_algorithm`

### Proposed Commit Message

```
feat(decrypt): add missing #behavior annotation for signature step skip

Add the missing duvet annotation for the decrypt.md#behavior requirement:
"If the message header does not contain an algorithm suite including a
signature algorithm, the Decrypt operation MUST NOT perform this step."

The annotation is placed in an explicit `else` block in
`step_verify_signature`, making the skip path visible and annotated.

Added test using AlgAes256GcmHkdfSha512CommitKey (non-signing) to verify
the decrypt operation succeeds without signature verification.

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#behavior
```

### Duvet Verification (actual command output)
```
$ make duvet
[Pre-existing 2 errors in compliance_exceptions/encrypt.txt — not related to this change]
```

### Test Results (actual command output)
```
$ cargo test --test test_decrypt_behavior
running 1 test
test test_decrypt_skips_signature_step_for_non_signing_algorithm ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### Notes
- The 2 duvet errors in `make duvet` are pre-existing (in `compliance_exceptions/encrypt.txt`) and unrelated to this change
- The `test_authentication_tag` test failures are due to expired AWS credentials (ExpiredTokenException), not related to this change
- The `test_v2_header_body` failure is pre-existing, not related to this change
- The annotation uses `implementation` type (default, type line omitted) as specified
- The quote exactly matches the TOML entry
- No cross-references needed (the quoted text contains no markdown links)
