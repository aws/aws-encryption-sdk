# Work Item: Add Missing Test Annotations for V2 Header Body

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md` and `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `v2-header` and `header-body-version-2-0`
- **Duvet Target**: `specification/client-apis/encrypt.md#v2-header` and `specification/data-format/message-header.md#header-body-version-2-0`

## Type of Work
ADD_TESTS

## Requirements to Address

All 18 requirements below have implementation annotations in `v2_header_body.rs` but are **missing `type=test` annotations** in the test file. The existing test `test_v2_header_body_serialization_order` already exercises the serialization of all fields (it parses ciphertext and verifies field order/positions), and `test_v2_header_message_id` tests the message ID. New individual test functions are needed — following the exact pattern used in `test_v1_header_body.rs`.

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
  then the [message header body](../data-format/message-header.md#header-body-version-2-0) MUST be serialized with the following specifics:
  ```
- **Current State**: needs-test

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Version](../data-format/message-header.md#version): MUST be serialized according to the
  [Version](../data-format/message-header.md#version) specification.
  ```
- **Current State**: needs-test

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
  ```
- **Current State**: needs-test

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value of the `Version` field MUST be `02` in the Version 2.0 header body.
  ```
- **Current State**: needs-test
- **Duvet Target**: `specification/data-format/message-header.md#header-body-version-2-0`

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
  [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
  ```
- **Current State**: needs-test

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
  ```
- **Current State**: needs-test

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
  [Message ID](../data-format/message-header.md#message-id) specification.
  ```
- **Current State**: needs-test

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The process used to generate this identifier MUST use a good source of randomness
  to make the chance of duplicate identifiers negligible.
  ```
- **Current State**: needs-test

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
  [AAD](../data-format/message-header.md#aad) specification.
  ```
- **Current State**: needs-test

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
  in the [encryption materials](../framework/structures.md#encryption-materials),
  and this serialization MUST NOT contain any key value pairs listed in
  the [encryption material's](../framework/structures.md#encryption-materials)
  [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
  ```
- **Current State**: needs-test

### Requirement 11
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
  [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
  ```
- **Current State**: needs-test

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the serialization of the
  [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
  ```
- **Current State**: needs-test

### Requirement 13
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
  [Content Type](../data-format/message-header.md#content-type) specification.
  ```
- **Current State**: needs-test

### Requirement 14
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be [02](../data-format/message-header.md#supported-content-types).
  ```
- **Current State**: needs-test

### Requirement 15
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
  [Frame Length](../data-format/message-header.md#frame-length) specification.
  ```
- **Current State**: needs-test

### Requirement 16
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the value of the frame size determined above.
  ```
- **Current State**: needs-test

### Requirement 17
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be serialized according to the
  [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data) specification.
  ```
- **Current State**: needs-test

### Requirement 18
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
  derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
  ```
- **Current State**: needs-test

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs`
All implementation annotations are already present and correctly placed. No changes needed to this file.

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`
```rust
// Existing tests:
// - test_v2_header_body_serialization_order (covers serialization order only)
// - test_v2_header_message_id (covers message-id length and uniqueness)
//
// Missing: individual field-level test functions with duvet annotations
// for each of the 18 requirements listed above.
```

## Implementation Guidance
- Follow the exact pattern in `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — each field gets its own test function with the exact TOML quotes as `type=test` annotations.
- Reuse the existing `encrypt_default` helper and `parse_v2_header_field_offsets` helper already in the test file.
- For fields that can be verified by byte inspection (Version, Algorithm Suite ID, Content Type), assert the expected byte values directly.
- For fields that require round-trip verification (AAD, EDKs, Frame Length, Algorithm Suite Data), encrypt then decrypt and assert the plaintext matches.
- For Message ID randomness, encrypt twice and assert the IDs differ (the existing `test_v2_header_message_id` already does this — just add the missing `encrypt.md#v2-header` test annotations to it or create a new test that references both specs).
- Group tightly coupled requirements (e.g., "MUST be serialized according to" + "The value MUST be") into the same test function, with separate annotation blocks for each quote.

### Spec-Aligned Structure
The spec describes this flow:
1. Gate condition (version 2.0) → annotate at a test that encrypts with a V2 suite
2. Version field → annotate at test asserting `ct[0] == 0x02`
3. Algorithm Suite ID → annotate at test asserting suite ID bytes match
4. Message ID → annotate at test asserting 32-byte length and randomness
5. AAD → annotate at round-trip test with encryption context
6. Encrypted Data Keys → annotate at round-trip test (decrypt uses EDKs)
7. Content Type → annotate at round-trip test or byte inspection
8. Frame Length → annotate at round-trip test
9. Algorithm Suite Data → annotate at round-trip test (commit key verified on decrypt)

Sub-items to annotate individually:
- Each "MUST be serialized according to" quote → at the test function for that field
- Each "value MUST be/correspond to" quote → at the assertion within that test function

## Targeted Tests
- `test_v2_header_body_serialization_order` — existing, already has 2 test annotations
- `test_v2_header_message_id` — existing, covers message-id spec
- NEW: `test_v2_header_serialized` — gate condition test
- NEW: `test_v2_header_version` — version byte test
- NEW: `test_v2_header_algorithm_suite_id` — suite ID test
- NEW: `test_v2_header_message_id_randomness` — or add annotations to existing test
- NEW: `test_v2_header_aad` — AAD round-trip test
- NEW: `test_v2_header_encrypted_data_keys` — EDK round-trip test
- NEW: `test_v2_header_content_type` — content type test
- NEW: `test_v2_header_frame_length` — frame length test
- NEW: `test_v2_header_algorithm_suite_data` — suite data round-trip test

## Success Criteria
```bash
cargo test test_v2_header
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `encrypt.md#v2-header`
- [ ] duvet report shows no gaps for `data-format/message-header.md#header-body-version-2-0`
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
