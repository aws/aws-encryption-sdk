# Agent 1 Notes — header.rs

## Discovery Summary

Analyzed `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs` against
`aws-encryption-sdk-specification/data-format/message-header.md`.

### Annotations in header.rs (current state)
1. `#structure` — "MUST be in big-endian format" — implementation ✓, test ✓
2. `#structure` — "MUST be serialized as, in order, Header Body, and Header Authentication" — implementation ✓, test ✓
3. `#encrypted-data-key-count` — "This value MUST be greater than 0" — implementation ✓, **test MISSING**
4. `#message-id` — "MUST use a good source of randomness" — implementation ✓, **test MISSING** (but tests exist in test_v1_header_body.rs and test_v2_header_body.rs)
5. `#algorithm-suite-data` — "length MUST be equal to Algorithm Suite Data Length" — implementation ✓, **test MISSING**

### Priority Assessment
- Priority 2 (missing test annotations) applies to requirements 3, 4, and 5 above.
- Requirement 4 (#message-id randomness) already has tests in test_v1_header_body.rs and test_v2_header_body.rs that test the same quote, so it may already be covered by duvet (the test annotations use `aws-encryption-sdk-specification` prefix while the implementation uses `specification` prefix — these are equivalent via symlink).
- Requirements 3 and 5 have NO test annotations anywhere.

### Broader Gaps in message-header.md (not in header.rs)
Many serialization-format requirements (field lengths, UInt types, "interpreted as bytes") are missing both implementation and test annotations across the codebase. These are structural/implication-type requirements that could be annotated with `type=implication` at the relevant struct fields or serialization functions. However, per the task scope, only header.rs gaps are addressed.

## Spec-Aligned Structure Analysis

### Q1: Logical flow of encrypted-data-key-count section
1. The count field is 2 bytes (serialization format)
2. Serialized as UInt16 (encoding)
3. Value MUST be > 0 (validation constraint)

### Q2: Code constructs fulfilling each requirement
- "2 bytes" → `write_u16` / `read_u16` calls in encrypted_data_keys.rs
- "serialized as UInt16" → same `write_u16` / `read_u16`
- "greater than 0" → `validate_max_encrypted_data_keys` in header.rs (checks `edks.is_empty()`)

### Q3: Sub-items
No sub-items for the targeted requirements.

### Q4: Most likely structural mistake
The implementer might be tempted to add the test annotation at a round-trip test that doesn't actually exercise the validation path. The `validate_max_encrypted_data_keys` function only checks emptiness when `max_encrypted_data_keys` is `Some(...)`. A round-trip test without setting `max_encrypted_data_keys` would NOT exercise the `edks.is_empty()` check. The test should either:
- Set `max_encrypted_data_keys` to trigger the validation path, OR
- Verify at the byte level that the EDK count in the ciphertext is > 0

## Potential Spec Gaps

### 1. validate_max_encrypted_data_keys only checks when max is set
- **Code location**: `header.rs:validate_max_encrypted_data_keys`
- **Behavior**: The "greater than 0" check (`edks.is_empty()`) is only executed when `max_encrypted_data_keys` is `Some(...)`. If no max is configured, an empty EDK list would not be caught by this function.
- **Why it matters**: Correctness — the spec says the count MUST be > 0 unconditionally, but the code only enforces it conditionally.
- **Suggested spec clarification**: The implementation should validate EDK count > 0 regardless of whether a max is configured. This may be a code bug rather than a spec gap.

### 2. Frame length vs content type cross-validation
- **Code location**: `header.rs:read_header_body` lines 74-84
- **Behavior**: The code validates that framed content must have frame_length > 0 and non-framed content must have frame_length == 0. The spec only states the non-framed case ("When the content type is non-framed, the value of this field MUST be 0") but doesn't explicitly state the framed case must be > 0.
- **Why it matters**: Interoperability — other implementations might not enforce the framed case.
- **Suggested spec requirement**: "When the content type is framed, the value of this field MUST be greater than 0."

## Self-Verification

1. ✅ TOML content was read from `compliance/aws-encryption-sdk-specification/data-format/message-header/encrypted-data-key-count.toml` — confirmed exact quote matches
2. ✅ Source file `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs` exists and was read
3. ✅ Duvet snapshot was read from `.duvet/snapshot.txt` — confirmed coverage state
4. ✅ Test files were read and verified to not contain test annotations for the targeted requirements
