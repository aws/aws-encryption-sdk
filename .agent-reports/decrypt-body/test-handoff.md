### Test Handoff

**Spec**: `aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`

**Commit Message**:
```
feat(decrypt): add duvet annotations for decrypt-the-message-body

Add 20 implementation/implication annotations in body.rs covering:
- Frame field deserialization (Seq Num End, Seq Num, IV, Content Length, Content, Auth Tag)
- AAD construction (message ID, body AAD content, sequence number, content length)
- AES-GCM decryption inputs (IV, cipherkey, ciphertext, tag)
- Streaming behavior (wait for bytes, signature algorithm input, plaintext release)
- Non-framed sequence number

Add 11 new test functions with 22 type=test annotations in
test_decrypt_the_message_body.rs covering all 22 requirements.

Spec: specification/client-apis/decrypt.md#decrypt-the-message-body
```
