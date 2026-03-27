### Test Handoff

**Spec**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header` and `aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-2-0`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`

**Commit Message**:
```
test(v2-header): add missing type=test annotations for V2 header body requirements

Add 8 new test functions with 18 type=test annotations covering all
V2 header body field serialization requirements from encrypt.md#v2-header
and message-header.md#header-body-version-2-0.

Tests verify: version byte (0x02), algorithm suite ID (0x0578),
message ID (32-byte length + randomness), AAD (round-trip with EC),
encrypted data keys (round-trip), content type (round-trip),
frame length (round-trip), and algorithm suite data/commit key (round-trip).

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
Spec: aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-2-0
```
