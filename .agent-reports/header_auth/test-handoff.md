# Test Handoff — header_auth

**Spec**: `aws-encryption-sdk-specification/data-format/message-header.md#header-authentication-version-1-0` and `aws-encryption-sdk-specification/data-format/message-header.md#header-authentication-version-2-0`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`

**Commit Message**:
```
feat(message-header): add data-format header-authentication annotations and tests

Add duvet annotations for header-authentication-version-1-0 and
header-authentication-version-2-0 spec sections in the data-format
specification.

- Annotate write_header_auth_tag_v1 with V1 serialization order requirement
- Annotate write_header_auth_tag_v2 with V2 serialization requirement
- Reformat write_header_auth_tag_v2 match arm to block body for annotation placement
- Add round-trip encrypt/decrypt tests for both V1 and V2 header auth

Spec: aws-encryption-sdk-specification/data-format/message-header.md
Sections: header-authentication-version-1-0, header-authentication-version-2-0
```
