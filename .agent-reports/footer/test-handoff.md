# Test Handoff — footer

**Spec**: `aws-encryption-sdk-specification/data-format/message-footer.md#overview`, `aws-encryption-sdk-specification/data-format/message-footer.md#signature-length`, `aws-encryption-sdk-specification/data-format/message-footer.md#signature`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs`

**Commit Message**:
```
test(footer): add duvet test annotations for message footer data format

Add type=test annotations for all 4 normative requirements in
message-footer.md: overview (footer presence with signing suite),
signature-length (2 bytes, UInt16 interpretation), and signature
(calculated over header+body in serialization order).

Tests use AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 signing suite
with a raw AES keyring (no KMS needed). Footer bytes are parsed
from ciphertext to verify structural properties; round-trip
decrypt proves signature correctness.

Refs: specification/data-format/message-footer.md#overview
Refs: specification/data-format/message-footer.md#signature-length
Refs: specification/data-format/message-footer.md#signature
```
