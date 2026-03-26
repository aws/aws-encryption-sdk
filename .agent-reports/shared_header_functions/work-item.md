# Work Item: Add Missing Message ID Length and Interpretation Annotations

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
  The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
  ```
- **Current State**: missing
- **Sub-items**: none

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
  ```
- **Current State**: missing
- **Sub-items**: none

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The message ID MUST be interpreted as bytes.
  ```
- **Current State**: missing
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/shared_header_functions.rs`
```rust
pub(crate) fn read_message_id_v1(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)
}
pub(crate) fn read_message_id_v2(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)
}
```

Constants in `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs`:
```rust
pub(crate) const MESSAGE_ID_LEN_V1: u32 = 16;
pub(crate) const MESSAGE_ID_LEN_V2: u32 = 32;
```

The `write_message_id` function already has a randomness annotation but no length annotation:
```rust
//= specification/data-format/message-header.md#message-id
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    write_bytes(w, message_id)
}
```

The `generate_message_id` function in `header.rs` also enforces the length:
```rust
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
#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_message_id() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    let ct1 = encrypt_v1(b"msg id test", EncryptionContext::new()).await;
    let ct2 = encrypt_v1(b"msg id test", EncryptionContext::new()).await;
    let msg_id_1 = &ct1[4..20];
    let msg_id_2 = &ct2[4..20];
    assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");
    assert_eq!(msg_id_1.len(), 16, "V1 Message ID must be 16 bytes");
}
```

## Implementation Guidance
- Add `type=implication` annotations with `reason=` for the length requirements, since the length is structurally enforced by the constant passed to `read_vec` / the buffer size, not by a runtime check.
- Place the V1 length annotation on `read_message_id_v1` at the `read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)` call.
- Place the V2 length annotation on `read_message_id_v2` at the `read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)` call.
- Place the "interpreted as bytes" annotation on `write_message_id` at the `write_bytes(w, message_id)` call, since `MessageId` is `Vec<u8>` — the type system enforces byte interpretation.
- Add corresponding `type=test` annotations to the existing `test_v1_header_message_id` test (for V1 16-byte length) and add a new test for V2 32-byte message ID length.
- Follow the annotation pattern used elsewhere in this file (see the `write_esdk_suite_id` annotations at lines 34-39).

### Spec-Aligned Structure
The spec describes these properties of Message ID:
1. Randomness → already annotated at `write_message_id` and `generate_message_id`
2. V1 length = 16 bytes → annotate at `read_message_id_v1` call to `read_vec`
3. V2 length = 32 bytes → annotate at `read_message_id_v2` call to `read_vec`
4. Interpreted as bytes → annotate at `write_message_id` call to `write_bytes`

Sub-items to annotate individually:
- `The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.` → at `read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)` in `read_message_id_v1`
- `The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.` → at `read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)` in `read_message_id_v2`
- `The message ID MUST be interpreted as bytes.` → at `write_bytes(w, message_id)` in `write_message_id`

### Structural Pitfall
Do NOT annotate the constants in `header_types.rs` — annotate at the point of use in `shared_header_functions.rs` where the length is actually enforced during serialization/deserialization. The constants define the value; the `read_vec`/`write_bytes` calls are where the requirement is fulfilled.

## Targeted Tests
- `test_v1_header_message_id` — already asserts `msg_id_1.len() == 16`; needs `type=test` annotation for the V1 16-byte length requirement
- NEW: `test_v2_header_message_id` — encrypt with V2 suite, verify message ID is 32 bytes and unique; needs `type=test` annotations for V2 32-byte length and "interpreted as bytes"

## Success Criteria
```bash
cargo test test_v1_header_message_id test_v2_header_message_id
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `message-header.md#message-id` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
