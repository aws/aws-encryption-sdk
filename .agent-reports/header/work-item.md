# Work Item: Fix Duplicate Annotations and Fill Gaps in header.rs for message-header.md

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Sections**: `#message-id`, `#structure`, `#algorithm-suite-data`, `#encrypted-data-key-count`
- **Duvet Targets**:
  - `aws-encryption-sdk-specification/data-format/message-header.md#message-id`
  - `aws-encryption-sdk-specification/data-format/message-header.md#structure`
  - `aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-data`
  - `aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1 — DUPLICATE: message-id randomness (triplicated)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  While implementations cannot guarantee complete uniqueness,
  implementations MUST use a good source of randomness when generating messages IDs in order to make
  the chance of duplicate IDs negligible.
  ```
- **Current State**: duplicate — annotated in 3 places as `type=implementation`:
  1. `header.rs` line 116 (generate_message_id) — **CORRECT location, keep this one**
  2. `shared_header_functions.rs` line 51 (write_message_id) — **REMOVE**
  3. `v1_header_body.rs` line 69 (write_message_id call) — **REMOVE**
- **Additional fix**: Remove the explicit `//= type=implementation` line from header.rs (it's the default).

### Requirement 2 — DUPLICATE: structure big-endian (duplicated)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The message header is a sequence of bytes that MUST be in big-endian format.
  ```
- **Current State**: duplicate — annotated in 2 places as `type=implementation`:
  1. `header.rs` line 29 (write_header_body) — **keep**
  2. `serialize_functions.rs` line 86 (write_u16) — **REMOVE**
- **Rationale**: `write_header_body` is the entry point for header serialization and the more appropriate location for this structural requirement. `write_u16` is a generic utility.

### Requirement 3 — MISSING TEST: message-id randomness
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  While implementations cannot guarantee complete uniqueness,
  implementations MUST use a good source of randomness when generating messages IDs in order to make
  the chance of duplicate IDs negligible.
  ```
- **Current State**: needs-test — implementation annotation exists in header.rs but no `type=test` annotation exists for this exact data-format spec quote. (Tests in test_v1_header_body.rs and test_v2_header_body.rs annotate the encrypt.md version of this requirement, not the data-format spec version.)

### Requirement 4 — MISSING IMPL: algorithm-suite-data interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The algorithm suite data MUST be interpreted as bytes.
  ```
- **Current State**: missing — no annotation anywhere in the codebase. Natural placement is in `validate_suite_data` in header.rs where `suite_data` is compared as `&[u8]`.

### Requirement 5 — MISSING TEST: algorithm-suite-data interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The algorithm suite data MUST be interpreted as bytes.
  ```
- **Current State**: needs-test — no test annotation exists.

### Requirement 6 — STYLE FIX: encrypted-data-key-count explicit type=implementation
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  This value MUST be greater than 0.
  ```
- **Current State**: incomplete — annotation at header.rs line 98 has explicit `//= type=implementation` which should be removed per duvet-patterns.md ("Do NOT include `//= type=implementation` — omit the type line entirely.")

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`

Duplicate annotation #1 (message-id randomness, keep but fix style):
```rust
//= specification/data-format/message-header.md#message-id
//= type=implementation
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
```

Duplicate annotation #2 (structure big-endian, keep):
```rust
//= specification/data-format/message-header.md#structure
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_header_body(w: &mut dyn SafeWrite, body: &HeaderBody) -> Result<(), Error> {
```

Style fix (encrypted-data-key-count, remove explicit type):
```rust
//= specification/data-format/message-header.md#encrypted-data-key-count
//= type=implementation
//# This value MUST be greater than 0.
pub(crate) fn validate_max_encrypted_data_keys(
```

Missing annotation location (algorithm-suite-data interpreted as bytes):
```rust
pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header_body: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    if header_body.suite_data() != expected_suite_data {
        return Err("Commitment key does not match".into());
    }
    //= specification/data-format/message-header.md#algorithm-suite-data
    //# The length of the suite data field MUST be equal to ...
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/shared_header_functions.rs` (remove duplicate)
```rust
//= specification/data-format/message-header.md#message-id
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs` (remove duplicate)
```rust
    //= specification/data-format/message-header.md#message-id
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    write_message_id(w, &body.message_id)?;
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serialize_functions.rs` (remove duplicate)
```rust
//= specification/data-format/message-header.md#structure
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_u16(w: &mut dyn SafeWrite, data: u16) -> Result<(), Error> {
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_length_matches_algorithm_suite() {
    //= specification/data-format/message-header.md#algorithm-suite-data
    //= type=test
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length]...
    let pt = b"suite data length test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "...");
}
```

## Implementation Guidance

### Duplicate Removal
1. **Remove** the `#message-id` randomness annotation from `shared_header_functions.rs` line 51-53 (2 comment lines before `write_message_id`).
2. **Remove** the `#message-id` randomness annotation from `v1_header_body.rs` lines 69-71 (2 comment lines before `write_message_id(w, &body.message_id)?;`).
3. **Remove** the `#structure` big-endian annotation from `serialize_functions.rs` lines 86-87 (2 comment lines before `write_u16`).

### Style Fixes in header.rs
4. **Remove** `//= type=implementation` from the `#message-id` annotation at line 117.
5. **Remove** `//= type=implementation` from the `#encrypted-data-key-count` annotation at line 99.

### Add Missing Annotation in header.rs
6. **Add** `#algorithm-suite-data` "interpreted as bytes" annotation in `validate_suite_data`, before the byte comparison `header_body.suite_data() != expected_suite_data`:
```rust
pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header_body: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#algorithm-suite-data
    //= type=implication
    //= reason=suite_data is Vec<u8> and compared as byte slices; the type system enforces byte interpretation
    //# The algorithm suite data MUST be interpreted as bytes.
    if header_body.suite_data() != expected_suite_data {
```

### Add Missing Test Annotations in test_header_structure.rs
7. **Add** test annotation for `#message-id` randomness in a new test or existing test that verifies message IDs are unique across encryptions. The test in `test_v1_header_body.rs::test_v1_header_message_id` already checks `assert_ne!(msg_id_1, msg_id_2)` — add the data-format annotation there.
8. **Add** test annotation for `#algorithm-suite-data` "interpreted as bytes" in the existing `test_suite_data_length_matches_algorithm_suite` test (or a new test).

### Spec-Aligned Structure
The changes are annotation-only (no logic changes):
1. Remove duplicate `#message-id` randomness → at `shared_header_functions.rs` and `v1_header_body.rs`
2. Remove duplicate `#structure` big-endian → at `serialize_functions.rs`
3. Fix style `type=implementation` → at `header.rs` lines 99, 117
4. Add `#algorithm-suite-data` "interpreted as bytes" → at `validate_suite_data` in `header.rs`
5. Add test annotations → at test files

### Patterns to Follow
- See `shared_header_functions.rs` line 55-58 for an example of `type=implication` with `reason=` for "interpreted as bytes" annotations:
  ```rust
  //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
  //= type=implication
  //= reason=MessageId is Vec<u8>; write_bytes treats it as raw bytes
  //# The message ID MUST be interpreted as bytes.
  ```
- See `header_types.rs` line 183-193 for an example of `type=implication` for structural type constraints.

## Targeted Tests
- `test_header_structure::test_suite_data_length_matches_algorithm_suite` — add `#algorithm-suite-data` "interpreted as bytes" test annotation
- `test_v1_header_body::test_v1_header_message_id` — add `#message-id` randomness test annotation
- `test_v2_header_body::test_v2_header_message_id` — add `#message-id` randomness test annotation

## Success Criteria
```bash
cargo test test_header_structure test_v1_header_body test_v2_header_body
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no duplicate annotations for `#message-id` randomness
- [ ] duvet report shows no duplicate annotations for `#structure` big-endian
- [ ] duvet report shows coverage for `#algorithm-suite-data` "interpreted as bytes" (impl + test)
- [ ] duvet report shows test coverage for `#message-id` randomness
- [ ] No annotations in header.rs use explicit `type=implementation`
