# Pre-Implementation Reasoning

## 1. Logical steps in this spec section

1. The `supported-content-types` section defines what content types are valid
2. `01` maps to Non-Framed
3. `02` maps to Framed

## 2. Point of fulfillment for each requirement

- "The supported content types MUST be:" → fulfilled at the `enum ContentType` definition itself (structural — the enum constrains valid values)
- Sub-item `01` for Non-Framed → fulfilled at `NonFramed = 1` variant
- Sub-item `02` for Framed → fulfilled at `Framed = 2` variant

## 3. Sub-items?

Yes — the spec has two list items under the MUST requirement:
- `01` for Non-Framed
- `02` for Framed
Each should be annotated at the corresponding enum variant.

## 4. Reviewer readability

The enum is small (3 lines). Annotations will be:
- `supported-content-types` MUST annotation before the enum
- Sub-item annotations before each variant
- Total ~10 lines, very readable top-to-bottom

## 5. Existing similar code

`v2_header_body.rs` lines 22-37 show `type=implication` pattern for structural properties.
`test_header_structure.rs` shows the test pattern: encrypt/decrypt round-trip with duvet `type=test` annotations.

## Implementation Plan

### Source change (header_types.rs)
- Replace the duplicate `content-type` annotation on `ContentType` enum with `supported-content-types` annotation
- Add `type=implication` since enum definition is structural
- Add sub-item annotations at each variant

### Test file (tests/test_header_types.rs)
- Test that encrypted message contains correct content type byte (0x02 for framed)
- Test that corrupting the content type byte causes decryption failure
- Both tests use `type=test` annotations for `supported-content-types`

### Cross-references
The sub-item quotes contain links to `message-body.md#non-framed-data` and `message-body.md#framed-data`.
These are references to the message body spec, not normative requirements about content types.
The cross-reference rule says to chase links one level. Let me check if those TOML files exist.

## Round 2 — Addressing Review Feedback

### Issues to fix:
1. **NonFramed test annotation misplaced**: The `type=test` annotation for `- \`01\` for [Non-Framed]...` is on `test_content_type_invalid_value_rejected`, but that test corrupts to 0x00 — it doesn't exercise NonFramed at all.
   - Fix: Create a dedicated `test_content_type_nonframed_value` that verifies 0x01 is accepted as a valid content type by corrupting the content type byte to 0x01 and confirming the error is NOT "Unsupported Content Type" but rather a frame-length mismatch.
   - Move the NonFramed `type=test` annotation to this new test.

2. **Blank lines between annotations**: Remove all blank lines between annotation blocks and between annotations and first executable line.
   - Fix: Stack annotations directly with no blank lines, matching the pattern in `test_v1_header_body.rs`.

3. **Invalid value test annotation**: Use `content-type` section annotation instead of `supported-content-types` NonFramed sub-item for the rejection test.
