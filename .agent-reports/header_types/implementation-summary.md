## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs` — Added duvet annotations for supported-versions, supported-types, version field length, type field length, type value validation, and content-type field length
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs` — Added tests for version V1/V2 values, type customer AED value, invalid type rejection; restructured with V1/V2 encrypt helpers

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs
```

### Requirements Addressed
- ✅ `The supported versions MUST be:` — implication on enum + test
- ✅ `- \`01\` MUST be version 1.0` — implication on V1 variant + test
- ✅ `- \`02\` MUST be version 2.0` — implication on V2 variant + test
- ✅ `The supported types MUST be:` — implication on enum + test
- ✅ `- \`80\` MUST be Customer Authenticated Encrypted Data` — implication on variant + test
- ✅ `The length of the serialized version field MUST be 1 byte.` — implication on write_msg_format_version
- ✅ `The length of the serialized type field MUST be 1 byte.` — implication on write_msg_type
- ✅ `The type (hex) of this field MUST be a value that exists in the following table:` — implementation on read_msg_type + test
- ✅ `The length of the serialized content type field MUST be 1 byte.` — implication on write_content_type

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs`
- **Number of `type=test` annotations added**: 10 for 7 requirements (some tests cover parent + sub-item)
- **Test function names**:
  - `test_version_v2_value` — covers supported-versions parent + V2 sub-item
  - `test_version_v1_value` — covers V1 sub-item
  - `test_type_customer_aed_value` — covers supported-types parent + 0x80 sub-item
  - `test_type_invalid_value_rejected` — covers type value validation
  - `test_content_type_framed_value` — covers supported-content-types parent + Framed sub-item
  - `test_content_type_nonframed_value` — covers NonFramed sub-item
  - `test_content_type_invalid_value_rejected` — covers content-type value validation

### Proposed Commit Message

```
feat(message-header): add duvet annotations for version, type, and content-type definitions

Add implementation annotations (type=implication) for:
- supported-versions: enum MessageFormatVersion with V1=0x01, V2=0x02
- supported-types: enum MessageType with TypeCustomerAed=0x80
- version field length: 1 byte via write_u8
- type field length: 1 byte via write_u8
- type value validation: match block in read_msg_type
- content-type field length: 1 byte via write_u8

Add test annotations covering all requirements with 7 tests
verifying version bytes, type bytes, and invalid value rejection.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#supported-versions
Spec: aws-encryption-sdk-specification/data-format/message-header.md#supported-types
Spec: aws-encryption-sdk-specification/data-format/message-header.md#version
Spec: aws-encryption-sdk-specification/data-format/message-header.md#type
Spec: aws-encryption-sdk-specification/data-format/message-header.md#content-type
```

### Duvet Verification (actual command output)
```
$ make duvet
  Extracting requirements from aws-encryption-sdk-specification/data-format/message-header.md
   Extracted 73 requirements across 30 sections 3ms
  ...
    Scanning sources
     Scanned 537 sources 16ms
     Parsing annotations
      Parsed 2435 annotations 85ms
     Loading specifications
      Loaded 68 specifications 20ms
     Mapping sections
      Mapped 487 sections 9ms
    Matching references
     Matched 4136 references 5ms
     Sorting references
      Sorted 4136 references 20ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 16ms
```

### Test Results (actual command output)
```
$ cargo test --test test_header_types
running 7 tests
test test_type_invalid_value_rejected ... ok
test test_type_customer_aed_value ... ok
test test_version_v1_value ... ok
test test_content_type_nonframed_value ... ok
test test_content_type_framed_value ... ok
test test_content_type_invalid_value_rejected ... ok
test test_version_v2_value ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- Field-length requirements (version, type, content-type) use `type=implication` because the 1-byte constraint is structural (enforced by `write_u8`/`read_u8`) and not independently testable.
- Enum definition requirements (supported-versions, supported-types) use `type=implication` because the enum variants structurally constrain the valid values.
- The `read_msg_type` value validation annotation uses default type (implementation) since it's testable by providing an invalid byte.
- Pre-existing clippy warnings in encrypt.rs, materials.rs, v1_header_body.rs, v2_header_body.rs are unrelated to these changes.
