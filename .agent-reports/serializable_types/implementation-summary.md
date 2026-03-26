## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs` — Added `type=implication` annotation for "The length of the serialized key provider ID length field MUST be 2 bytes" on the `write_str_u16` call in `write_edk`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serializable_types.rs` — Added `type=implication` annotation for "The key provider ID length MUST be serialized as a UInt16" on the `u16::try_from` validation in `is_esdk_encrypted_data_key`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_serializable_types.rs` — NEW FILE: 2 tests verifying key provider ID length serialization

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serializable_types.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_serializable_types.rs
```

### Requirements Addressed
- ✅ "The length of the serialized key provider ID length field MUST be 2 bytes." — implemented (implication) + tested
- ✅ "The key provider ID length MUST be serialized as a UInt16." — implemented (implication) + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_serializable_types.rs`
- **Number of `type=test` annotations added**: 2 for 2 requirements
- **Test function names**: `test_edk_key_provider_id_length_is_2_bytes`, `test_edk_key_provider_id_length_serialized_as_uint16`

### Proposed Commit Message

```
feat(message-header): add duvet annotations for key provider ID length serialization

Add implementation and test annotations for the key-provider-id-length
section of the message header specification:
- "The length of the serialized key provider ID length field MUST be 2 bytes"
  annotated at write_str_u16 call in encrypted_data_keys.rs
- "The key provider ID length MUST be serialized as a UInt16"
  annotated at u16::try_from validation in serializable_types.rs

Both annotations use type=implication since they describe structural
serialization format properties.

Tests verify the serialized output bytes: the key provider ID length
field occupies exactly 2 bytes and is encoded as big-endian UInt16.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#key-provider-id-length
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 30ms
    Scanning sources
     Scanned 153 sources 3ms
     Parsing annotations
      Parsed 831 annotations 28ms
     Loading specifications
      Loaded 12 specifications 15ms
     Mapping sections
      Mapped 114 sections 12ms
    Matching references
     Matched 1522 references 4ms
     Sorting references
      Sorted 1522 references 10ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 16ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 2ms
```

Duvet snapshot shows:
```
  SECTION: [Key Provider ID Length](#key-provider-id-length)
    TEXT[implication,test]: The length of the serialized key provider ID length field MUST be 2 bytes.
    TEXT[implication,test]: The key provider ID length MUST be serialized as a UInt16.
```

### Test Results (actual command output)
```
$ cargo test test_edk_key_provider_id_length -- --nocapture
running 2 tests
test test_edk_key_provider_id_length_serialized_as_uint16 ... ok
test test_edk_key_provider_id_length_is_2_bytes ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- Both requirements use `type=implication` because they describe structural serialization format properties ("serialized as UInt16", "field MUST be 2 bytes") which are format definitions rather than runtime behavior.
- The `reason=` lines explain the non-obvious connection between the annotation and the code.
- Pre-existing clippy errors (missing docs, unused doc comments, etc.) and KMS authentication test failures in `test_authentication_tag.rs` are unrelated to these changes.
- The `specification/` prefix target in the duvet snapshot shows `[!MUST]` for these requirements — this is a pre-existing issue with duplicate spec targets, not caused by these changes.
