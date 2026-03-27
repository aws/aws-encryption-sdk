# Pre-Implementation Reasoning

## 1. Logical steps

This is a test-only work item (ADD_TESTS). Three requirements need `type=test` annotations:

1. `encrypted-data-key-count`: "This value MUST be greater than 0." — Encrypt produces ≥1 EDK; inspect ciphertext bytes to verify EDK count > 0.
2. `algorithm-suite-data`: "The length of the suite data field MUST be equal to the Algorithm Suite Data Length..." — V2 round-trip succeeds, proving validate_suite_data passed.
3. `frame-length`: "When the content type is non-framed, the value of this field MUST be 0." — Mutate ciphertext to set content_type=NonFramed + frame_length!=0, assert decrypt fails.

## 2. Point of fulfillment for each test

1. EDK count > 0: Assert at the byte-level inspection of the EDK count field in ciphertext (must be > 0).
2. Suite data length: Assert at successful round-trip decrypt (validate_suite_data runs during decrypt and would fail if length mismatched).
3. Non-framed frame length: Assert at decrypt failure when ciphertext is mutated to have non-framed content type with non-zero frame length.

## 3. Sub-items

No sub-items — each requirement is a single normative statement.

## 4. Structure

Three test functions in `test_header_structure.rs`, following the existing pattern:
- `test_encrypted_data_key_count_greater_than_zero` — byte inspection
- `test_suite_data_length_matches_algorithm_suite` — round-trip
- `test_nonframed_frame_length_must_be_zero` — mutation + failure

## 5. Existing similar code

- `test_header_types.rs` — byte-level ciphertext inspection with `content_type_offset()`
- `test_v2_header_body.rs` — `parse_v2_header_field_offsets()` helper
- `test_header_structure.rs` — existing round-trip pattern with `test_keyring()` and `round_trip()`

## Annotation target path

Source uses `specification/` prefix (symlink to `aws-encryption-sdk-specification/`).
Duvet config uses `specification/` as source prefix.
All annotations must use `specification/data-format/message-header.md#...`.
