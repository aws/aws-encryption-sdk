# Work Item: Add missing implementation and test annotations for Encrypted Data Keys serialization order

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `encrypted-data-keys`
- **Duvet Target**: `specification/data-format/message-header.md#encrypted-data-keys`

## Type of Work
NEW_IMPLEMENTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Encrypted Data Keys MUST be serialized as, in order,
  Encrypted Data Key Count,
  and Encrypted Data Key Entries.
  ```
- **Current State**: missing
- **Sub-items** (the spec describes the serialization order):
  ```toml
  Encrypted Data Key Count
  ```
  ```toml
  Encrypted Data Key Entries
  ```

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs`
```rust
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    write_u16(w, edks.len() as u16)?;
    for edk in edks {
        write_edk(w, edk)?;
    }
    Ok(())
}

pub(crate) fn read_edks(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<EncryptedDataKey>, Error> {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# This value MUST be greater than 0.
    let count = read_u16(r, raw)?;
    //= specification/client-apis/decrypt.md#parse-the-header
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    if let Some(max_edks) = max_edks
        && count as usize > max_edks.get()
    {
        return ser_err("Ciphertext encrypted data keys exceed maxEncryptedDataKeys");
    }
    let mut edks = Vec::with_capacity(count as usize);
    for _ in 0..count {
        edks.push(read_edk(r, raw)?);
    }
    Ok(edks)
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` (existing pattern to follow)
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_header_big_endian_format() {
    //= specification/data-format/message-header.md#structure
    //= type=test
    //# The message header is a sequence of bytes that MUST be in big-endian format.
    let pt = b"big-endian header test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful decrypt proves header was serialized in big-endian format");
}
```

## Implementation Guidance

### 1. Add implementation annotation to `write_edks`

The `write_edks` function fulfills the serialization order requirement: it writes the count first (`write_u16`), then iterates over entries (`write_edk`). Add the annotation immediately before the first line of the function body:

```rust
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //# The Encrypted Data Keys MUST be serialized as, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.
    write_u16(w, edks.len() as u16)?;
    for edk in edks {
        write_edk(w, edk)?;
    }
    Ok(())
}
```

### 2. Remove misplaced annotation from `read_edks`

The annotation `//= specification/data-format/message-header.md#encrypted-data-key-count` / `//# This value MUST be greater than 0.` on line 37-38 of `encrypted_data_keys.rs` is misplaced. The `read_u16` call does NOT enforce that the count is > 0. The actual enforcement is in `header.rs::validate_max_encrypted_data_keys`. Remove this annotation from `encrypted_data_keys.rs` to eliminate the duplicate.

### 3. Add test annotation

Create a new test in a new or existing test file. Follow the round-trip pattern used in `test_header_structure.rs` and `test_v1_header_body.rs`. A successful encrypt→decrypt round-trip proves the EDKs were serialized in the correct order (because decrypt parses them back successfully).

The test should be in a new test file `tests/test_encrypted_data_keys.rs` or added to `tests/test_header_structure.rs`.

### Spec-Aligned Structure
The spec describes this flow:
1. Encrypted Data Key Count (written first) → annotate at `write_u16(w, edks.len() as u16)?` in `write_edks`
2. Encrypted Data Key Entries (written second) → annotate at the `for edk in edks` loop in `write_edks`

The parent requirement covers the ordering, so a single annotation at the top of `write_edks` is sufficient.

### Patterns to Follow
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — round-trip test pattern
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — EDK-specific test pattern (see `test_v1_header_encrypted_data_keys`)
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs` — annotation style for serialization order requirements

### Most Likely Structural Mistake
Do NOT annotate `read_edks` with this requirement. The serialization order requirement is about writing (serialization), and `write_edks` is the correct location. The deserialization in `read_edks` implicitly follows the same order but the spec says "serialized as" — annotate at the serialization point.

## Targeted Tests
- `test_encrypted_data_keys_serialization_order` — round-trip test proving EDKs are serialized as count + entries in order (new test)

## Success Criteria
```bash
cargo test test_encrypted_data_keys_serialization_order
make duvet
```
- [ ] Test passes
- [ ] duvet report shows no gaps for `specification/data-format/message-header.md#encrypted-data-keys`
- [ ] The requirement has `type=implementation` annotation in `write_edks`
- [ ] The requirement has `type=test` annotation in the test
- [ ] The misplaced `#encrypted-data-key-count` / `This value MUST be greater than 0.` annotation is removed from `encrypted_data_keys.rs` (it correctly exists in `header.rs`)
