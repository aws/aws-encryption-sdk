# Work Item: Add Missing Implementation and Test Annotations for AAD / Key Value Pairs in encryption_context.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Sections**: `#aad`, `#key-value-pairs-length`, `#key-value-pairs`
- **Duvet Targets**:
  - `aws-encryption-sdk-specification/data-format/message-header.md#aad`
  - `aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs-length`
  - `aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs`

## Type of Work
ADD_TESTS + FIX_ANNOTATION

## Requirements to Address

### Requirement 1 — AAD Serialization Order
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#aad"

  [[spec]]
  level = "MUST"
  quote = '''
  The AAD MUST be serialized as, in order,
  Key Value Pairs Length,
  and Key Value Pairs.
  '''
  ```
- **Current State**: missing (no implementation annotation exists)
- **Sub-items**:
  - "Key Value Pairs Length" → `write_u16(w, bytes as u16)` in `write_aad_section`
  - "Key Value Pairs" → `write_aad(w, data)` call in `write_aad_section`

### Requirement 2 — Key Value Pairs Length Field Size
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs-length"

  [[spec]]
  level = "MUST"
  quote = '''
  The length of the serialized key value pairs length field MUST be 2 bytes.
  '''
  ```
- **Current State**: missing (no implementation annotation exists)

### Requirement 3 — Key Value Pairs Length as UInt16
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs-length"

  [[spec]]
  level = "MUST"
  quote = '''
  The key value pairs length MUST be serialized as a UInt16.
  '''
  ```
- **Current State**: missing (no implementation annotation exists)

### Requirement 4 — Empty Encryption Context Length Value
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs-length"

  [[spec]]
  level = "MUST"
  quote = '''
  When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.
  '''
  ```
- **Current State**: incomplete — annotation exists in `write_aad` (line 66) but is misplaced; `write_aad` is never called with empty data. The correct location is `write_aad_section` where `write_u16(w, 0)` is called.

### Requirement 5 — Key Value Pairs Serialization
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs"

  [[spec]]
  level = "MUST"
  quote = '''
  The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).
  '''
  ```
- **Current State**: needs-test — implementation annotation exists at line 70, but no `type=test` annotation exists

### Requirement 6 — Empty Encryption Context Exclusion
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#key-value-pairs"

  [[spec]]
  level = "MUST"
  quote = '''
  When the [encryption context](../framework/structures.md#encryption-context) is empty,
  this field MUST NOT be included in the [AAD](#aad).
  '''
  ```
- **Current State**: needs-test — implementation annotation exists at line 27, but no `type=test` annotation exists

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encryption_context.rs`
```rust
pub(crate) fn write_aad_section(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    if data.is_empty() {
        write_u16(w, 0)?;
        return Ok(());
    }
    let bytes = get_length(data);
    write_u16(w, bytes as u16)?;
    write_aad(w, data)
}

pub(crate) fn write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#key-value-pairs-length
    //# When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.
    write_u16(w, data.len() as u16)?;
    for pair in data {
        //= specification/data-format/message-header.md#key-value-pairs
        //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).
        write_u16(w, pair.0.len() as u16)?;
        write_bytes(w, pair.0.as_bytes())?;
        write_u16(w, pair.1.len() as u16)?;
        write_bytes(w, pair.1.as_bytes())?;
    }
    Ok(())
}
```

### Test File: `NEW FILE NEEDED: AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encryption_context_aad.rs`
No existing test file covers these specific data-format requirements.

## Implementation Guidance
- Add implementation annotations to `write_aad_section` for requirements 1, 2, 3, 4
- Move the misplaced annotation for requirement 4 from `write_aad` to `write_aad_section` (the `write_u16(w, 0)` line)
- Keep existing annotations for requirements 5 and 6 in place
- Create a new test file with unit tests that exercise `write_aad_section`, `write_aad`, and `write_empty_ec_or_write_aad` directly
- Follow the test pattern from `test_header_structure.rs` — use round-trip encrypt/decrypt to prove serialization correctness, OR use direct serialization tests with byte-level assertions
- Since `write_aad_section`, `write_aad`, and `write_empty_ec_or_write_aad` are `pub(crate)`, tests in the `tests/` directory cannot call them directly. Use round-trip encrypt/decrypt tests (like `test_header_structure.rs`) that prove the AAD section is correctly serialized by successfully decrypting.
- For the "2 bytes" and "UInt16" requirements (328, 329), these are structural properties of `write_u16`. Use `type=implication` with a `reason=` explaining that `write_u16` always writes exactly 2 bytes as UInt16.

### Spec-Aligned Structure
The spec describes this flow:
1. AAD = Key Value Pairs Length + Key Value Pairs → annotate at `write_aad_section` function body
2. Key Value Pairs Length is 2 bytes / UInt16 → annotate at `write_u16(w, bytes as u16)` in `write_aad_section` (use `type=implication`)
3. Empty EC → length = 0 → annotate at `write_u16(w, 0)` in `write_aad_section`
4. Key Value Pairs serialization → already annotated in `write_aad` loop
5. Empty EC → no KVP field → already annotated at `write_empty_ec_or_write_aad`

Sub-items to annotate individually:
- "Key Value Pairs Length" → at `write_u16(w, bytes as u16)` in `write_aad_section`
- "and Key Value Pairs" → at `write_aad(w, data)` call in `write_aad_section`

## Targeted Tests
- `test_aad_serialization_order` — Encrypt with non-empty EC, decrypt successfully, proving AAD was serialized as Key Value Pairs Length then Key Value Pairs
- `test_aad_empty_encryption_context_length_zero` — Encrypt with empty EC, decrypt successfully, proving empty EC produces length 0
- `test_aad_empty_encryption_context_no_kvp_field` — Encrypt with empty EC, decrypt successfully, proving Key Value Pairs field is excluded
- `test_aad_key_value_pairs_serialization` — Encrypt with non-empty EC, decrypt successfully, proving key-value pairs are serialized per spec
- `test_aad_key_value_pairs_length_field_size` — Encrypt with non-empty EC, decrypt successfully, proving the length field is 2 bytes (implication test)
- `test_aad_key_value_pairs_length_uint16` — Encrypt with non-empty EC, decrypt successfully, proving the length is serialized as UInt16 (implication test)

## Success Criteria
```bash
cargo test test_encryption_context_aad
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `#aad`, `#key-value-pairs-length`, `#key-value-pairs`
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
