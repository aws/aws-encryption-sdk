# Work Item: Add Missing Duvet Annotations for Plaintext Length Bound in types.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `input`
- **Duvet Target**: `specification/client-apis/encrypt.md#input`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If a caller is able to specify both an input [plaintext](#plaintext) with known length and
  a [Plaintext Length Bound](#plaintext-length-bound),
  the [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation
  and MUST be ignored.
  ```
- **Current State**: missing
- **Placement**: On `EncryptInput` struct definition — this is satisfied by construction because `EncryptInput` has `plaintext: &'a [u8]` (always known length) and no `plaintext_length_bound` field, so a caller cannot specify both. Annotate as `type=implication` with a `reason=` explaining this.

### Requirement 2
- **Level**: SHOULD
- **Exact Quote** (from TOML):
  ```toml
  Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext)
  with known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction.
  ```
- **Current State**: missing
- **Placement**: On `EncryptInput` struct definition — satisfied by construction because `EncryptInput` takes `plaintext: &[u8]` (known length) and has no `plaintext_length_bound` field. Annotate as `type=implication` with a `reason=`.

### Requirement 3
- **Level**: MAY
- **Exact Quote** (from TOML):
  ```toml
  If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a
  [Plaintext Length Bound](#plaintext-length-bound).
  ```
- **Current State**: missing
- **Placement**: On `EncryptStreamInput.data_size` field — the streaming input accepts an optional `data_size` which serves as the plaintext length bound when plaintext length is unknown. Annotate as `type=implication` with a `reason=`.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`

Relevant snippet for `EncryptInput` (Requirements 1 and 2):
```rust
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Input for [`encrypt`](crate::encrypt).
//= specification/client-apis/encrypt.md#input
//= type=implication
//# - The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument.
//= specification/client-apis/encrypt.md#input
//= type=implication
//# - The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.
//= specification/client-apis/encrypt.md#input
//= type=implication
//# - The input to the Encrypt operation MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.
//= specification/client-apis/encrypt.md#input
//= type=implication
//# - The input to the Encrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
//= specification/client-apis/encrypt.md#input
//= type=implication
//# - The input to the Encrypt operation MUST accept an optional [Frame Length](#frame-length) argument.
pub struct EncryptInput<'a> {
```

Relevant snippet for `EncryptStreamInput` (Requirement 3):
```rust
pub struct EncryptStreamInput {
    /// Algorithm Suite.
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: FrameLength,
    /// The source of cryptographic materials
    pub source: Option<MaterialSource>,
    /// The expected size of the input data stream.
    /// This is only important if you cmm or keyring care about such things, which most don't.
    pub data_size: Option<usize>,
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`
No existing tests for Plaintext Length Bound requirements.

## Implementation Guidance

- Add three `type=implication` annotation blocks to `types.rs`.
- Requirements 1 and 2 should be annotated on the `EncryptInput` struct definition, grouped with the existing `encrypt.md#input` annotations that are already on the struct.
- Requirement 3 should be annotated on the `EncryptStreamInput.data_size` field.
- All three use `type=implication` because they are satisfied by the type system / struct construction, not by runtime logic.
- Each annotation MUST include a `reason=` line explaining WHY the struct construction satisfies the requirement.
- Follow the existing annotation pattern in `types.rs` — see the `//= type=implication` + `//= reason=` pattern used on `EncryptInput.source` and `EncryptInput.max_encrypted_data_keys`.
- Since `type=implication` satisfies both implementation and test checks, no separate test annotations are needed.

### Spec-Aligned Structure
The spec describes the Plaintext Length Bound as:
1. MAY input a Plaintext Length Bound (when plaintext length unknown) → annotate at `EncryptStreamInput.data_size`
2. SHOULD ensure caller can't specify both known-length plaintext and Plaintext Length Bound → annotate at `EncryptInput` struct (satisfied by construction: no `plaintext_length_bound` field)
3. MUST NOT use Plaintext Length Bound if both specified → annotate at `EncryptInput` struct (satisfied by construction: impossible to specify both)

Sub-items to annotate individually:
- MAY quote → at `EncryptStreamInput.data_size` field
- SHOULD quote → at `EncryptInput` struct definition, after existing `encrypt.md#input` annotations
- MUST quote → at `EncryptInput` struct definition, after the SHOULD annotation

### Most Likely Structural Mistake
The implementer might be tempted to annotate Requirements 1 and 2 on `EncryptStreamInput` instead of `EncryptInput`. But the SHOULD and MUST requirements specifically reference "a plaintext with known length" — which is `EncryptInput` (where `plaintext: &[u8]` has known length). `EncryptStreamInput` has unknown-length plaintext, so the SHOULD/MUST don't apply there.

## Targeted Tests
No new tests needed — `type=implication` satisfies both implementation and test checks.

## Success Criteria
```bash
make duvet
```
- [ ] duvet report shows no gaps for `encrypt.md#input` Plaintext Length Bound requirements
- [ ] All three requirements have `type=implication` annotations
- [ ] No new test files needed (implication satisfies test check)
