# Work Item: Add Missing Test Annotation for Final Frame Release Hold-Back

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `decrypt-the-message-body`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
  a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
  successfully completes.
  ```
- **Current State**: needs-test (implementation annotations exist in `src/decrypt.rs` at lines 215 and 452, but no `type=test` annotation exists anywhere)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs`
```rust
// Line 213-218: After step_verify_signature succeeds, last_frame is written
    serialize_functions::write_bytes(plaintext, &last_frame)?;

    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.
```

```rust
// Line 450-455: In step_decrypt_body, fail_if_multi_frame enforces the hold-back
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
            //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
            //# successfully completes.
            let fail_if_multi_frame = state.dec_mat.verification_key.is_some() && safety_needed.yes();
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`
The test file has 24 test functions covering 32 of 33 requirements. This requirement is the only one missing a `type=test` annotation.

## Implementation Guidance
- Add a single new test function to `tests/test_decrypt_the_message_body.rs`
- The test should verify that when using a signing algorithm suite, the final frame plaintext is only released after signature verification completes
- The simplest approach: encrypt with a signing suite (ECDSA P384), then tamper with the signature bytes at the end of the ciphertext. Decrypt must fail, proving the final frame was held back pending signature verification. If the final frame were released before signature verification, the plaintext would have been written to the output before the error.
- Alternatively: a successful round-trip with a signing suite where the message has only a final frame (single-frame message) proves the hold-back works because the plaintext is only available after `decrypt()` returns `Ok`.
- Follow the existing test patterns in the file: use `test_keyring()`, `encrypt_with_frame_length()`, `round_trip()`, and `EncryptInput::with_legacy_keyring()`.
- Use `aws-encryption-sdk-specification/` prefix for the annotation (matching the test file convention).

### Spec-Aligned Structure
The spec describes this flow:
1. Final frame / unframed plaintext is decrypted → held in `last_frame` variable
2. Signature verification runs → `step_verify_signature()`
3. Only after signature verification succeeds → `write_bytes(plaintext, &last_frame)`

The test annotation should be placed at the test function that exercises this exact flow.

## Targeted Tests
- `test_decrypt_streaming_releases_regular_frames` — existing test for the related SHOULD (regular frames released after tag verification with signing suite); the new test should be distinct
- New test: `test_decrypt_final_frame_held_until_signature_verification` — verifies the MUST for final frame hold-back

## Success Criteria
```bash
cargo test test_decrypt_final_frame_held_until_signature_verification
make duvet
```
- [ ] The new test passes
- [ ] duvet report shows no gaps for `decrypt-the-message-body` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All 33 TOML requirements have corresponding `type=test` annotations
