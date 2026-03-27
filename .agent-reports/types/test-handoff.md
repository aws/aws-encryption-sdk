### Test Handoff

**Spec**: `specification/client-apis/encrypt.md#input` and `specification/client-apis/decrypt.md#input`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`

**Commit Message**:
```
test(types): add duvet annotations for encrypt.md#input and decrypt.md#input

Add type=implication annotations on EncryptInput and DecryptInput
source fields for the SHOULD-optional requirement on CMM/keyring inputs.

Add 10 type=test annotations across 9 test functions covering all
MUST requirements in encrypt.md#input (8 requirements) and
decrypt.md#input (5 requirements, 2 already had test annotations).

Spec sections:
- specification/client-apis/encrypt.md#input
- specification/client-apis/decrypt.md#input
```
