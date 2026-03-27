# Agent 2 Notes — header/ (Fix Duplicate Annotations and Fill Gaps)

## Pre-Implementation Reasoning

### 1. Logical steps

1. Remove duplicate `#message-id` randomness annotation from `shared_header_functions.rs` (line 51-52)
2. Remove duplicate `#message-id` randomness annotation from `v1_header_body.rs` (lines 69-71)
3. Remove duplicate `#structure` big-endian annotation from `serialize_functions.rs` (lines 86-87)
4. Remove explicit `//= type=implementation` from `header.rs` line 117 (message-id)
5. Remove explicit `//= type=implementation` from `header.rs` line 99 (encrypted-data-key-count)
6. Add `#algorithm-suite-data` "interpreted as bytes" annotation in `validate_suite_data` in `header.rs`
7. Add test annotation for `#message-id` randomness in test files
8. Add test annotation for `#algorithm-suite-data` "interpreted as bytes" in test file

### 2. Point of fulfillment for each requirement

- Req 1 (message-id randomness): `generate_message_id` in header.rs — already annotated, just fix style
- Req 2 (structure big-endian): `write_header_body` in header.rs — already annotated, just remove duplicate
- Req 4 (algorithm-suite-data interpreted as bytes): `validate_suite_data` — the `!=` comparison on `&[u8]` slices
- Req 6 (encrypted-data-key-count > 0): `validate_max_encrypted_data_keys` — already annotated, just fix style

### 3. Sub-items?

No sub-items for these requirements.

### 4. Reviewer readability

All changes are annotation-only (add/remove/fix). No code logic changes.

### 5. Existing patterns

- `shared_header_functions.rs` line 55-58 has a `type=implication` with `reason=` for "interpreted as bytes" — follow this pattern for algorithm-suite-data.
- Test files use `specification/` prefix for annotations (confirmed by review round 2).

### Key observations

- The duvet config uses `specification/` prefix, which is a symlink to `aws-encryption-sdk-specification/`.
- Annotations in source code MUST use `specification/` prefix.
- Annotations in test files MUST also use `specification/` prefix (confirmed by review round 2).
- The `aws-encryption-sdk-specification/` prefix is used in some annotations (e.g., shared_header_functions.rs lines 25, 33, 55) — these are for different requirements and use the full path. Need to check which prefix duvet expects.

Actually, looking more carefully: the duvet config has `[[specification]] source = "specification/data-format/message-header.md"`. The TOML target is `aws-encryption-sdk-specification/data-format/message-header.md#message-id`. The annotations in shared_header_functions.rs use BOTH prefixes — `specification/` for the randomness one at line 51, and `aws-encryption-sdk-specification/` for the length ones at lines 25, 33. Both seem to work with duvet. I'll use `specification/` to match the existing annotations in header.rs.
