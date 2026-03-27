### Test Handoff

**Spec**: `specification/data-format/message-header.md#message-id`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`

**Commit Message**:
```
fix(message-header): complete message-id randomness annotation quote

Add missing first line "While implementations cannot guarantee complete
uniqueness," to the implementation annotation on generate_message_id()
in header.rs. The annotation now matches the exact TOML quote from
specification/data-format/message-header.md#message-id.

Test annotations in test_v1_header_body.rs and test_v2_header_body.rs
already included the full quote.

Spec: specification/data-format/message-header.md#message-id
```
