# Work Item: Fix Incomplete message-id Annotation in header.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `message-id`
- **Duvet Target**: `specification/data-format/message-header.md#message-id`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  quote = '''
  While implementations cannot guarantee complete uniqueness,
  implementations MUST use a good source of randomness when generating messages IDs in order to make
  the chance of duplicate IDs negligible.
  '''
  ```
- **Current State**: incomplete — the implementation annotation in `header.rs` is missing the first line of the TOML quote (`While implementations cannot guarantee complete uniqueness,`). The test annotations in `test_v1_header_body.rs` and `test_v2_header_body.rs` already include the full quote.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
```rust
//= specification/data-format/message-header.md#message-id
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
    let length = if suite.message_version == 1 {
        MESSAGE_ID_LEN_V1
    } else {
        MESSAGE_ID_LEN_V2
    };
    let mut rand_bytes: Vec<u8> = vec![0; length as usize];
    aws_mpl_legacy::primitives::generate_random_bytes(&mut rand_bytes)?;
    Ok(rand_bytes)
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs`
```rust
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`
```rust
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");
```

## Implementation Guidance
- Add the missing first line `//# While implementations cannot guarantee complete uniqueness,` to the existing annotation block in `header.rs` at `generate_message_id`.
- The annotation must use the exact TOML quote — three lines, starting with "While implementations cannot guarantee complete uniqueness,".
- Do NOT change the annotation target path (`specification/data-format/message-header.md#message-id`) — it is correct.
- Do NOT add `type=implementation` — the default type is `implementation`.
- Follow the existing pattern in `header.rs` where the annotation is immediately before the function signature.

### Spec-Aligned Structure
The spec describes this flow:
1. Message ID uniqueness caveat → annotate at `generate_message_id` function (the `//# While implementations...` line)
2. Randomness requirement → annotate at `generate_message_id` function (already present, just incomplete)

Sub-items to annotate individually:
- "While implementations cannot guarantee complete uniqueness," → at `generate_message_id` function signature (add to existing annotation block)

### What the fix looks like

Change the annotation from:
```rust
//= specification/data-format/message-header.md#message-id
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
```

To:
```rust
//= specification/data-format/message-header.md#message-id
//# While implementations cannot guarantee complete uniqueness,
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
```

## Targeted Tests
- `test_v1_header_message_id` in `test_v1_header_body.rs` — verifies V1 message IDs are unique across encryptions
- `test_v2_header_message_id` in `test_v2_header_body.rs` — verifies V2 message IDs are unique across encryptions

Both tests already include the full quote with `type=test`. No test changes needed.

## Success Criteria
```bash
make duvet
```
- [ ] duvet snapshot shows `TEXT[!MUST,implementation,test]` for the full `#message-id` randomness requirement (all three lines)
- [ ] No `TEXT[!MUST,test]` entry for "While implementations cannot guarantee complete uniqueness," (the first line should now have implementation coverage)
- [ ] All existing tests still pass
- [ ] No other regressions in the duvet report for `specification/data-format/message-header.md`
