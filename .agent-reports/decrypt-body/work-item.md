# Work Item: Add Non-Framed Data Deserialization Conformance Annotation

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `decrypt-the-message-body`
- **Duvet Target**: `specification/client-apis/decrypt.md#decrypt-the-message-body`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Non-framed data deserialization MUST conform to the [Non-Framed Data](../data-format/message-body.md#non-framed-data) specification.
  ```
- **Current State**: missing (no implementation annotation, no test annotation)

## Existing Code Context

### Source File: `src/message/body.rs`

The non-framed data deserialization is implemented in `read_and_decrypt_non_framed_message_body`. This function already has annotations for `specification/data-format/message-body.md#non-framed-data` but is missing the `client-apis/decrypt.md#decrypt-the-message-body` annotation for this new conformance requirement.

### Source File: `src/decrypt.rs`

The call to `read_and_decrypt_non_framed_message_body` happens in `step_decrypt_body` around the content type match. This is where the annotation should go — at the call site that dispatches to non-framed deserialization.

### Test File: `tests/test_decrypt_the_message_body.rs`

Existing tests exercise non-framed decryption. One of these should receive the `type=test` annotation.

## Implementation Guidance

1. Add an implementation annotation at the call site in `src/decrypt.rs` where `read_and_decrypt_non_framed_message_body` is called, or in `src/message/body.rs` at the function entry.

2. Add a `type=test` annotation to an existing test that exercises non-framed decryption.

## Success Criteria
- [ ] `make duvet` passes with no errors
- [ ] The requirement has `type=implementation` (or default) in source
- [ ] The requirement has `type=test` in test file
- [ ] Duvet snapshot shows the requirement as covered
