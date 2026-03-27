# Work Item: Add client.md#initialization annotations to types.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/client.md`
- **Section**: `initialization`
- **Duvet Target**: `specification/client-apis/client.md#initialization`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - On client initialization,
  the caller MUST have the option to provide a [commitment policy](#commitment-policy).
  ```
- **Current State**: missing
- **Notes**: The `EncryptInput`, `DecryptInput`, `EncryptStreamInput`, and `DecryptStreamInput` structs all have a `pub commitment_policy: EsdkCommitmentPolicy` field. This fulfills the requirement structurally. Needs `type=implication` annotation on the field and a `type=test` annotation in a test.

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - On client initialization,
  the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
  ```
- **Current State**: missing
- **Notes**: The input structs all have `pub max_encrypted_data_keys: Option<NonZeroUsize>`. Needs `type=implication` annotation and `type=test`.

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
  the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
  ```
- **Current State**: missing
- **Notes**: `Option<NonZeroUsize>` defaults to `None` via `#[derive(Default)]`. Needs `type=implication` annotation on the field and `type=test`.

### Requirement 4 (already partially covered — needs test only)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
  ```
- **Current State**: has `type=implication` in `decrypt.rs` but no `type=test` anywhere
- **Notes**: `EsdkCommitmentPolicy` has `#[default] RequireEncryptRequireDecrypt`. The implication annotation exists in `decrypt.rs` already. A `type=test` annotation is needed.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`
```rust
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
pub struct EncryptInput<'a> {
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    pub encryption_context: EncryptionContext,
    pub frame_length: FrameLength,
    pub source: Option<MaterialSource>,
    pub plaintext: &'a [u8],
    /// default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: EsdkCommitmentPolicy,
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`
```rust
// Existing test file for client-level tests. Currently only tests NetV400RetryPolicy.
// New tests for initialization defaults should go here.
```

## Implementation Guidance
- Add `type=implication` annotations on the `commitment_policy` and `max_encrypted_data_keys` fields of `EncryptInput` for Requirements 1, 2, and 3.
- The Rust ESDK does not have a separate "client" object — the input structs serve as the per-operation configuration. The `commitment_policy` and `max_encrypted_data_keys` fields on each input struct fulfill the "client initialization" requirements because the caller sets them when constructing the input (which is the Rust equivalent of client initialization).
- For Requirement 3, annotate at the `max_encrypted_data_keys` field with `type=implication` and `reason=Option<NonZeroUsize> defaults to None via derive(Default), which means no limit`.
- For Requirement 4, the implication annotation already exists in `decrypt.rs`. Only a `type=test` annotation is needed.
- Add tests in `test_create_esdk_client.rs` that verify the default values of `EncryptInput::default()` and `DecryptInput::default()`.
- Follow the existing annotation pattern in `types.rs` — see the `encrypt.md#input` annotations on the struct for the style.

### Spec-Aligned Structure
The spec describes this flow:
1. "caller MUST have the option to provide a commitment policy" → annotate at `pub commitment_policy` field on `EncryptInput`
2. "caller MUST have the option to provide a maximum number of encrypted data keys" → annotate at `pub max_encrypted_data_keys` field on `EncryptInput`
3. "default MUST be REQUIRE_ENCRYPT_REQUIRE_DECRYPT" → already annotated in `decrypt.rs`, needs test only
4. "default MUST result in no limit" → annotate at `pub max_encrypted_data_keys` field on `EncryptInput`

Sub-items to annotate individually:
- "provide a commitment policy" → at `pub commitment_policy: EsdkCommitmentPolicy` field on `EncryptInput`
- "provide a maximum number of encrypted data keys" → at `pub max_encrypted_data_keys: Option<NonZeroUsize>` field on `EncryptInput`
- "default MUST result in no limit" → at `pub max_encrypted_data_keys` field on `EncryptInput` with `reason=`

## Targeted Tests
- New test: `test_encrypt_input_default_commitment_policy` — verify `EncryptInput::default().commitment_policy == EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
- New test: `test_encrypt_input_default_max_edks_is_none` — verify `EncryptInput::default().max_encrypted_data_keys.is_none()`
- New test: `test_decrypt_input_default_commitment_policy` — verify `DecryptInput::default().commitment_policy == EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
- New test: `test_decrypt_input_default_max_edks_is_none` — verify `DecryptInput::default().max_encrypted_data_keys.is_none()`
- New test: `test_encrypt_input_custom_commitment_policy` — verify caller can set a non-default commitment policy
- New test: `test_encrypt_input_custom_max_edks` — verify caller can set a max EDK limit

## Success Criteria
```bash
cargo test test_encrypt_input_default -- --exact
cargo test test_decrypt_input_default -- --exact
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `client.md#initialization`
- [ ] All requirements have `type=implication` (structural, not runtime)
- [ ] All implementations have corresponding `type=test`
