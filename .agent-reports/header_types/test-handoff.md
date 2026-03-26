# Test Handoff — header_types

**Spec**: `specification/data-format/message-header.md#supported-content-types`
**Spec**: `specification/data-format/message-header.md#content-type`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs`

**Commit Message**:
```
fix(message-header): fix duplicate content-type annotation and add supported-content-types coverage

Replace duplicate content-type annotation on ContentType enum with
correct supported-content-types annotation using type=implication.
Add fine-grained sub-item annotations at each enum variant.
Add tests verifying Framed=0x02, NonFramed=0x01 acceptance,
and invalid content type rejection.

Spec: specification/data-format/message-header.md#supported-content-types
Spec: specification/data-format/message-header.md#content-type
```
