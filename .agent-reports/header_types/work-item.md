# Work Item: Fix Duplicate Content-Type Annotation and Add Supported-Content-Types Coverage

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `supported-content-types`
- **Duvet Target**: `specification/data-format/message-header.md#supported-content-types`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1 (FIX — duplicate annotation)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "specification/data-format/message-header.md#content-type"

  [[spec]]
  level = "MUST"
  quote = '''
  The value (hex) of this field MUST be a value that exists in the following table:
  '''
  ```
- **Current State**: duplicate — this annotation appears at TWO locations in `header_types.rs`:
  1. Line 53-54: before `read_content_type` function (CORRECT — keep this one)
  2. Line 183-184: before `ContentType` enum definition (WRONG — should be `supported-content-types`)

### Requirement 2 (NEW — missing annotation)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "specification/data-format/message-header.md#supported-content-types"

  [[spec]]
  level = "MUST"
  quote = '''
  The supported content types MUST be:
  '''
  ```
- **Current State**: missing — no annotation exists anywhere in the codebase for this requirement
- **Sub-items** (from the spec, not normative MUST but useful for fine-grained traceability):
  ```
  - `01` for [Non-Framed](message-body.md#non-framed-data)
  - `02` for [Framed](message-body.md#framed-data)
  ```

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs`

Current duplicate annotation on the enum (lines 183-190):
```rust
//= specification/data-format/message-header.md#content-type
//# The value (hex) of this field MUST be a value that exists in the following table:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum ContentType {
    NonFramed = 1,
    #[default]
    Framed = 2,
}
```

Correct annotation on the read function (lines 53-54):
```rust
//= specification/data-format/message-header.md#content-type
//# The value (hex) of this field MUST be a value that exists in the following table:
pub(crate) fn read_content_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ContentType, Error> {
```

### Test File: `NEW FILE NEEDED: AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs` (or add to existing test file)

No tests currently exist for the `supported-content-types` section.

## Implementation Guidance

1. **Remove the duplicate**: Replace the annotation on the `ContentType` enum (line 183-184) with the correct `supported-content-types` annotation.

2. **Annotate the enum with `type=implication`**: The enum definition is a structural/type-system property — it defines what the supported content types ARE. Use `type=implication` with a `reason=` explaining the enum constrains valid values.

3. **Annotate sub-items at each variant**: Place fine-grained annotations at each enum variant for the spec's list items (these are non-normative but improve traceability).

4. **Add a test annotation**: Write a test that verifies the `ContentType` enum has the correct values and that `read_content_type` rejects invalid values.

5. **Follow the pattern in `v2_header_body.rs`** (lines 22-37) for `type=implication` annotations on structural properties.

### Spec-Aligned Structure
The spec describes this flow:
1. "The supported content types MUST be:" → annotate at `enum ContentType` declaration with `type=implication`
2. "`01` for Non-Framed" → annotate at `NonFramed = 1` variant (non-normative sub-item)
3. "`02` for Framed" → annotate at `Framed = 2` variant (non-normative sub-item)

Sub-items to annotate individually:
- `- \`01\` for [Non-Framed](message-body.md#non-framed-data)` → at `NonFramed = 1` enum variant
- `- \`02\` for [Framed](message-body.md#framed-data)` → at `Framed = 2` enum variant

### Most Likely Structural Mistake
The implementer might keep the existing `content-type` annotation on the enum and just add a second annotation. The existing `content-type` annotation on the enum MUST be replaced, not supplemented. Only the `read_content_type` function should have the `content-type` annotation.

## Targeted Tests
- `test_content_type_enum_values` — verify `ContentType::NonFramed as u8 == 1` and `ContentType::Framed as u8 == 2`
- `test_read_content_type_rejects_invalid` — verify `read_content_type` returns error for value 0x00, 0x03, 0xFF

## Success Criteria
```bash
cargo test test_content_type
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `supported-content-types` section
- [ ] The duplicate `content-type` annotation on the `ContentType` enum is removed
- [ ] The `content-type` annotation remains ONLY on `read_content_type` function
- [ ] The `supported-content-types` annotation on the enum has `type=implication`
- [ ] Test annotations exist with `type=test` for the `supported-content-types` section
