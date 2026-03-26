# Work Item: Add duvet annotations for Key Provider ID Length in serializable_types.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `key-provider-id-length`
- **Duvet Target**: `specification/data-format/message-header.md#key-provider-id-length`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized key provider ID length field MUST be 2 bytes.
  ```
- **Current State**: missing
- **Sub-items**: none

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The key provider ID length MUST be serialized as a UInt16.
  ```
- **Current State**: missing
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serializable_types.rs`
```rust
pub(crate) fn is_esdk_encrypted_data_key(edk: &EncryptedDataKey) -> bool {
    u16::try_from(edk.key_provider_id.len()).is_ok()
        && u16::try_from(edk.key_provider_info.len()).is_ok()
}
```

The `is_esdk_encrypted_data_key` function validates that `key_provider_id` length fits in a u16, which enforces the UInt16 serialization constraint. The actual serialization happens in `encrypted_data_keys.rs` via `write_str_u16` → `write_u16` (2 bytes, big-endian).

### Related serialization: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs`
```rust
pub(crate) fn write_edk(w: &mut dyn SafeWrite, edk: &EncryptedDataKey) -> Result<(), Error> {
    write_str_u16(w, &edk.key_provider_id)?;
    write_seq_u16(w, &edk.key_provider_info)?;
    write_seq_u16(w, &edk.ciphertext)
}
```

### Related primitive: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serialize_functions.rs`
```rust
pub(crate) fn write_str_u16(w: &mut dyn SafeWrite, data: &str) -> Result<(), Error> {
    write_seq_u16(w, data.as_bytes())
}
pub(crate) fn write_seq_u16(w: &mut dyn SafeWrite, data: &[u8]) -> Result<(), Error> {
    match u16::try_from(data.len()) {
        Ok(len) => {
            write_u16(w, len)?;
            write_bytes(w, data)
        }
        Err(_) => ser_err("Sequence length too long for 16 bits"),
    }
}
```

### Test File: NEW FILE NEEDED: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_serializable_types.rs` (or inline `#[cfg(test)]` module in the source file)

## Implementation Guidance
- Add `implication` annotations on the `u16::try_from(edk.key_provider_id.len()).is_ok()` line in `is_esdk_encrypted_data_key` for both requirements, with `reason=` explaining the validation ensures the key provider ID length is representable as UInt16.
- Add `implication` annotations on the `write_str_u16(w, &edk.key_provider_id)?;` line in `encrypted_data_keys.rs` for the "2 bytes" requirement — `write_str_u16` calls `write_u16` which writes exactly 2 bytes in big-endian format.
- Add `type=test` annotations on a new test that exercises the round-trip serialization of an EDK, verifying the key provider ID length field is 2 bytes and encoded as UInt16.
- Follow the pattern in `test_v1_header_body.rs` for integration-style tests, or use a unit test that directly calls `write_edk`/`read_edk`.

### Spec-Aligned Structure
The spec describes this flow:
1. Key Provider ID Length field is 2 bytes → annotate at `write_str_u16` call (writes u16 length prefix = 2 bytes)
2. Key Provider ID Length is serialized as UInt16 → annotate at `u16::try_from(edk.key_provider_id.len()).is_ok()` (validates UInt16 representability) and at `write_u16` call (encodes as big-endian u16)

Sub-items to annotate individually:
- "The length of the serialized key provider ID length field MUST be 2 bytes." → at `write_str_u16(w, &edk.key_provider_id)?;` in `encrypted_data_keys.rs`
- "The key provider ID length MUST be serialized as a UInt16." → at `u16::try_from(edk.key_provider_id.len()).is_ok()` in `serializable_types.rs`

## Targeted Tests
- NEW TEST: `test_edk_key_provider_id_length_serialized_as_uint16` — write an EDK with a known key_provider_id, verify the first 2 bytes of output are the big-endian u16 length, then read it back and confirm round-trip.

## Success Criteria
```bash
cargo test test_edk_key_provider_id_length
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `#key-provider-id-length` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
