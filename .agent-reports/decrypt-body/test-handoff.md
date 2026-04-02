# Test Handoff — decrypt-body Cycle 2

**Spec**: `aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`

**Commit Message**:
```
test(decrypt): add type=test annotation for final frame hold-back requirement

Add test verifying that final frame plaintext is not released until
signature verification completes. The test encrypts with ECDSA P384,
tampers with the signature, and asserts decrypt fails — proving the
final frame was held back pending signature verification.

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
```

**Commit**: `ff3bc056`
