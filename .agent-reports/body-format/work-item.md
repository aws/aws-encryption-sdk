# Work Item: Add Missing Implementation Annotation for Framed Data Max Frame Size

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-body.md`
- **Section**: `framed-data`
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-body.md#framed-data`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
  ```
- **Current State**: needs-implementation-annotation (has `type=test` in `test_message_body_format.rs` but no `type=implementation` or `type=implication` annotation)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
```rust
// Line 644:
    let frame_length = header.body.frame_length() as usize;
```

The `frame_length()` returns `u32`, which is bounded by `2^32 - 1`. The `FrameLength` type in `src/types.rs` wraps `NonZeroU32`, enforcing the same bound at construction time.

### Related Code: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`
```rust
// Line 56:
pub struct FrameLength(pub std::num::NonZeroU32);
// ...
// Line 70-71:
    //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
    pub fn new(val: u32) -> Result<Self, Error> {
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs`
```rust
// Line 125-131 (already exists):
    //= specification/data-format/message-body.md#framed-data
    //= type=test
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
```

## Implementation Guidance
- Add a single `type=implication` annotation for this requirement
- Best placement: `src/message/body.rs` in `encrypt_and_serialize_body`, at the line where `frame_length` is obtained from the header (`let frame_length = header.body.frame_length() as usize;`)
- The constraint is enforced by the type system: `frame_length()` returns `u32` (max `2^32 - 1`), and `FrameLength` wraps `NonZeroU32`
- Use `reason=` to explain: `frame_length() returns u32, which is bounded by 2^32 - 1`
- Follow the existing pattern in the same function (e.g., the `#framed-data` annotation at line 703 for the frame count requirement)

### Spec-Aligned Structure
The spec describes this flow:
1. Frame size constraint (`2^32 - 1`) → annotate at `let frame_length = header.body.frame_length() as usize;` in `encrypt_and_serialize_body`
2. Frame count constraint (`2^32 - 1`) → already annotated at `if sequence_number == ENDFRAME_SEQUENCE_NUMBER`

### Exact Annotation to Add
```rust
    //= specification/data-format/message-body.md#framed-data
    //= type=implication
    //= reason=frame_length() returns u32, which is bounded by 2^32 - 1
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
    let frame_length = header.body.frame_length() as usize;
```

## Targeted Tests
- `test_framed_data_max_frame_size` — already exists in `test_message_body_format.rs`, verifies frame size constraint

## Success Criteria
```bash
cargo test test_framed_data_max_frame_size
make duvet
```
- [ ] Test passes
- [ ] duvet report shows no gaps for `#framed-data` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
