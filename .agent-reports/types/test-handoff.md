### Test Handoff

**Spec**: `aws-encryption-sdk-specification/client-apis/encrypt.md#input`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`

**Commit Message**:
```
feat(encrypt): add duvet annotations for Plaintext Length Bound requirements

Add three missing type=implication annotations to types.rs for the
Plaintext Length Bound requirements in encrypt.md#input:

- SHOULD ensure caller cannot specify both known-length plaintext and
  Plaintext Length Bound (on EncryptInput struct)
- MUST NOT use/MUST ignore Plaintext Length Bound when both specified
  (on EncryptInput struct)
- MAY input Plaintext Length Bound for unknown-length plaintext
  (on EncryptStreamInput.data_size field)

All three are satisfied by construction: EncryptInput has no
plaintext_length_bound field, and EncryptStreamInput.data_size
serves as the optional bound for streaming input.

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#input
```
