# Test Handoff — header (Round 3)

**Spec**: `aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count`, `aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-data`, `aws-encryption-sdk-specification/data-format/message-header.md#frame-length`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`

**Commit Message**:
```
test(message-header): add missing test annotations for EDK count, suite data length, and frame length

Add type=test annotations for three message-header requirements that had
implementation annotations but no corresponding test coverage:
- encrypted-data-key-count: "This value MUST be greater than 0"
- algorithm-suite-data: suite data length must match algorithm suite
- frame-length: "When content type is non-framed, value MUST be 0"

Spec: aws-encryption-sdk-specification/data-format/message-header.md
```
