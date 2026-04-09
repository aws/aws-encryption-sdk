# Agent 1 Notes — decrypt-the-message-body

## Discovery Summary

Analyzed duvet coverage for `specification/client-apis/decrypt.md#decrypt-the-message-body`
after commit ae86cf7a.

### Coverage Status

All 43 requirements (IDs 37–79) in the `decrypt-the-message-body` section
have both `citation` (implementation) and `test` annotations.

All 10 requirements (IDs 127–136) in the `un-framed-message-body-decryption` section
have both `citation` (implementation) and `test` annotations.

### Snapshot Format Clarification

The duvet snapshot format `TEXT[!MUST,implementation,test]` was initially
misread as indicating uncovered requirements. After parsing the HTML report's
JSON data structure, the `!` prefix on the level indicates a normative
requirement, NOT a coverage failure. The actual coverage status is determined
by the `statuses` object in the report data, which shows `citation` and `test`
keys for every requirement in these sections.

### Annotation Locations

- **Implementation annotations**: `src/decrypt.rs` (5 annotations), `src/message/body.rs` (44 annotations)
- **Test annotations**: `tests/test_decrypt_the_message_body.rs` (43 annotations)

### Spec-Aligned Structure

The spec describes this flow:
1. After header parsing, deserialize body bytes → annotated at `step_decrypt_body` in `decrypt.rs`
2. Use content type to determine framed vs non-framed → annotated at `match state.header.body.content_type()`
3. For framed: loop reading frames, using first 4 bytes to detect final frame → annotated in `read_and_decrypt_framed_message_body`
4. Deserialize each frame's fields (seq num, IV, content, auth tag) → annotated at each `read_*` call
5. Construct AAD and decrypt with AES-GCM → annotated at `body_aad` and `aes_decrypt` calls
6. Hold final frame until signature verification → annotated at return from `step_decrypt_body`

## Potential Spec Gaps

No significant spec gaps identified. The code behaviors align well with the specification.
The non-framed path is read-only (ESDK only encrypts framed data), which is correctly
annotated with an exception for the write-path IV uniqueness requirement.
