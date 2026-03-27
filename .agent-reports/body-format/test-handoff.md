### Test Handoff

**Spec**: `specification/data-format/message-body.md#regular-frame-sequence-number`, `#regular-frame-iv`, `#regular-frame-encrypted-content`, `#final-frame-sequence-number`, `#final-frame-iv`, `#final-frame-encrypted-content`, `#final-frame-encrypted-content-length`, `#sequence-number-end`, `#final-frame-authentication-tag`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`

**Commit Message**: `fix(message-body): fix 48 annotation prefixes, add 2 annotations, fix B2 stacking`
