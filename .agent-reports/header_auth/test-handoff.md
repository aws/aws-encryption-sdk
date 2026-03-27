# Test Handoff — header_auth

**Spec**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v1-authentication-tag` and `aws-encryption-sdk-specification/client-apis/encrypt.md#v2-authentication-tag`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`

**Commit Message**:
```
test(encrypt): add test annotations for v1/v2 authentication tag serialization

Add type=test duvet annotations for all 5 encrypt.md header auth tag
requirements (v1 parent, v1 IV sub-item, v1 auth tag sub-item, v2 parent,
v2 auth tag sub-item) to existing round-trip tests.

Distribute annotations within test functions so no more than 2 annotation
blocks appear before any single executable line.

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md
Sections: v1-authentication-tag, v2-authentication-tag
```
