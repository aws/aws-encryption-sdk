### Test Handoff

**Spec**: `specification/client-apis/client.md#initialization`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`

**Commit Message**:
```
feat(client): add duvet annotations for client.md#initialization requirements

Add type=implication annotations on EncryptInput fields for three
client initialization requirements: commitment policy option,
maximum encrypted data keys option, and max EDKs default behavior.

Add six type=test annotations in test_create_esdk_client.rs covering
all four MUST requirements in the initialization section, including
the pre-existing commitment policy default requirement.

Spec section: specification/client-apis/client.md#initialization
```
