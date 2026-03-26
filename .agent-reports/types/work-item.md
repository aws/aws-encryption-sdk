# Work Item: Add Missing Duvet Annotations for DecryptInput Validation and Encryption Context

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `input`
- **Duvet Target**: `specification/client-apis/decrypt.md#input`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Decrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
  ```
- **Current State**: missing (code exists in `DecryptInput::validate()` but has no duvet annotation)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
  ```
- **Current State**: missing (code exists in `DecryptInput::validate()` but has no duvet annotation)

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
  ```
- **Current State**: missing (field `encryption_context` exists on `DecryptInput` but has no duvet annotation)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`

The `DecryptInput::validate()` method (around line 488) has the validation logic but no annotations:
```rust
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.source.is_none() {
            Err(val_err("A Materials Source must be provided."))
        } else {
            Ok(())
        }
    }
```

The `encryption_context` field on `DecryptInput` (around line 382) has no annotation:
```rust
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
```

Compare with the already-annotated `EncryptInput::validate()` (around line 282):
```rust
    //= specification/client-apis/encrypt.md#input
    //# The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
    //= specification/client-apis/encrypt.md#input
    //# If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.source.is_none() {
            Err(val_err("A Materials Source must be provided."))
        } else {
            Ok(())
        }
    }
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`

The test `test_bad_decrypt_input` (line 39) exercises the validation but has no duvet test annotations:
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_bad_decrypt_input() {
    // ... setup ...
    decrypt_input.source = None;
    let bad_decrypt_output = decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());
}
```

## Implementation Guidance

- Follow the exact pattern from `EncryptInput::validate()` for the decrypt validate annotations
- Add `type=implication` annotation for the `encryption_context` field on the `DecryptInput` struct, following the same pattern as `EncryptInput` struct annotations
- Add `type=test` annotations to `test_bad_decrypt_input` for the validate/fail requirements
- Reference `EncryptInput` annotations in `types.rs` lines 281-285 as the exact pattern

### Spec-Aligned Structure

The spec describes this flow for `decrypt.md#input`:
1. Required arguments (encrypted message, CMM/keyring) → annotate at struct fields (ALREADY DONE)
2. Validation of exactly one CMM/keyring → annotate at `DecryptInput::validate()` method
3. Failure on invalid input → annotate at `Err(val_err(...))` inside validate
4. Optional arguments (encryption context) → annotate at struct definition or field

Sub-items to annotate individually:
- `The Decrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.` → at `DecryptInput::validate()` method, before `pub(crate) fn validate`
- `If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.` → at `DecryptInput::validate()` method, before `pub(crate) fn validate`
- `- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.` → at `DecryptInput` struct definition (as `type=implication`), or at the `encryption_context` field

### Most Likely Structural Mistake

The implementer might place the encryption context annotation on the `encryption_context` field directly instead of on the struct definition. Either placement works, but the `EncryptInput` pattern places all "MUST accept" annotations on the struct definition (before `pub struct EncryptInput`). For consistency, the `DecryptInput` should follow the same pattern — BUT the existing `DecryptInput` annotations are on individual fields (not the struct). So for this file, annotate at the field level to match the existing `DecryptInput` convention.

## Targeted Tests
- `test_bad_decrypt_input` — Tests that decrypt fails when `source` is set to `None`

## Success Criteria
```bash
cargo test test_bad_decrypt_input
make duvet
```
- [ ] `test_bad_decrypt_input` passes
- [ ] duvet report shows no gaps for `decrypt.md#input` requirements addressed here
- [ ] All three requirements have `type=implementation` (or `type=implication` for the encryption context field)
- [ ] All implementations have corresponding `type=test`
