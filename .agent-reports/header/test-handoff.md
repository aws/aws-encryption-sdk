### Test Handoff

**Spec**: `aws-encryption-sdk-specification/data-format/message-header.md#structure`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`

**Commit Message**:
```
fix(message-header): add missing serialization order annotation and tests

Add the missing implementation annotation for the header serialization
order requirement inside serialize_header() in header.rs. Add test file
test_header_structure.rs with type=test annotations for both structure
requirements using correct specification/ target path prefix.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#structure
```
