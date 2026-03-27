# Work Item: Add Missing Test Annotations for client.md#encrypt and client.md#decrypt

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/client.md`
- **Section**: `encrypt`, `decrypt`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/client.md#encrypt`, `aws-encryption-sdk-specification/client-apis/client.md#decrypt`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
  that adheres to [encrypt](./encrypt.md).
  ```
- **Current State**: needs-test
- **Implementation annotation exists** in `src/encrypt.rs` line 28 (default `type=implementation`). No `type=test` annotation exists anywhere.

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
  that adheres to [decrypt](./decrypt.md).
  ```
- **Current State**: needs-test
- **Implementation annotation exists** in `src/decrypt.rs` line 71 (default `type=implementation`). No `type=test` annotation exists anywhere.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs`
```rust
/// This is the public-facing entry point for the ESDK encrypt method.
//= specification/client-apis/client.md#encrypt
//# The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
//# that adheres to [encrypt](./encrypt.md).
pub async fn encrypt(input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs`
```rust
//= specification/client-apis/client.md#decrypt
//# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
//# that adheres to [decrypt](./decrypt.md).
/// Decrypt slice into Vec
pub async fn decrypt(input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    // ... sets up KMS keyring ...
    let encrypt_input = EncryptInput::with_legacy_keyring(asdf.as_bytes(), ec, kms_keyring);
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;

    let decrypt_input = DecryptInput::from_encrypt(&esdk_ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());
}
```

## Implementation Guidance
- Add `type=test` duvet annotations to the existing `test_encrypt_decrypt` test in `tests/test_encrypt_decrypt.rs`.
- The `client.md#encrypt` test annotation should be placed immediately before the `encrypt()` call (or the `assert` that confirms it succeeded).
- The `client.md#decrypt` test annotation should be placed immediately before the `decrypt()` call (or the `assert` that confirms it succeeded).
- Follow the pattern already used in this file for other test annotations (e.g., the `encrypt.md#input` test annotations in `test_bad_encrypt_input`).
- No new test functions are needed — the existing `test_encrypt_decrypt` already exercises both `encrypt()` and `decrypt()` successfully.

### Spec-Aligned Structure
The spec describes this flow:
1. `client.md#encrypt` — "Client MUST provide an encrypt function" → annotate at the `encrypt()` call in the test
2. `client.md#decrypt` — "Client MUST provide a decrypt function" → annotate at the `decrypt()` call in the test

### Annotation Placement
- `//= specification/client-apis/client.md#encrypt` + `//= type=test` → before `let encrypt_output = encrypt(&encrypt_input).await.unwrap();`
- `//= specification/client-apis/client.md#decrypt` + `//= type=test` → before `let decrypt_output = decrypt(&decrypt_input).await.unwrap();`

## Targeted Tests
- `test_encrypt_decrypt::test_encrypt_decrypt` — existing integration test that calls both `encrypt()` and `decrypt()` successfully

## Success Criteria
```bash
cargo test test_encrypt_decrypt
make duvet
```
- [ ] `test_encrypt_decrypt` passes
- [ ] duvet report shows no gaps for `client.md#encrypt`
- [ ] duvet report shows no gaps for `client.md#decrypt`
- [ ] Both requirements have `type=implementation` annotations (already exist)
- [ ] Both requirements have corresponding `type=test` annotations (to be added)
