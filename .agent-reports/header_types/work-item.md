# Work Item: Fix Annotation Path Prefix for content-type and supported-content-types

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `content-type`, `supported-content-types`
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#content-type`, `aws-encryption-sdk-specification/data-format/message-header.md#supported-content-types`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#content-type"

  [[spec]]
  level = "MUST"
  quote = '''
  The value (hex) of this field MUST be a value that exists in the following table:
  '''
  ```
- **Current State**: incomplete — annotation exists but uses wrong path prefix `specification/` instead of `aws-encryption-sdk-specification/`

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  target = "aws-encryption-sdk-specification/data-format/message-header.md#supported-content-types"

  [[spec]]
  level = "MUST"
  quote = '''
  The supported content types MUST be:
  '''
  ```
- **Current State**: incomplete — annotation exists but uses wrong path prefix `specification/` instead of `aws-encryption-sdk-specification/`
- **Sub-items** (annotated but with wrong path):
  ```toml
  //# - `01` for [Non-Framed](message-body.md#non-framed-data)
  //# - `02` for [Framed](message-body.md#framed-data)
  ```

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs`

Four annotations use `specification/` instead of `aws-encryption-sdk-specification/`:

**Line 77** — `read_content_type` function:
```rust
//= specification/data-format/message-header.md#content-type
//# The value (hex) of this field MUST be a value that exists in the following table:
pub(crate) fn read_content_type(
```

**Line 214** — `ContentType` enum definition:
```rust
//= specification/data-format/message-header.md#supported-content-types
//= type=implication
//= reason=The enum definition structurally constrains valid content types to exactly these two variants.
//# The supported content types MUST be:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum ContentType {
```

**Line 220** — `NonFramed` variant:
```rust
    //= specification/data-format/message-header.md#supported-content-types
    //= type=implication
    //# - `01` for [Non-Framed](message-body.md#non-framed-data)
    NonFramed = 1,
```

**Line 224** — `Framed` variant:
```rust
    //= specification/data-format/message-header.md#supported-content-types
    //= type=implication
    //# - `02` for [Framed](message-body.md#framed-data)
    #[default]
    Framed = 2,
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs`

Tests already use the correct `aws-encryption-sdk-specification/` prefix. No changes needed.

## Implementation Guidance
- This is a simple find-and-replace in `header_types.rs` only
- Replace `//= specification/data-format/message-header.md#` with `//= aws-encryption-sdk-specification/data-format/message-header.md#` at lines 77, 214, 220, and 224
- Do NOT change any other files — the broader codebase has the same issue but this work item is scoped to `header_types.rs` only
- Reference the correct annotations already in this file (e.g., lines 13, 19, 23, 29, 39, 46, 65, 201, 207) which use `aws-encryption-sdk-specification/`

### Spec-Aligned Structure
The spec describes these sections:
1. `#content-type` → annotate at `read_content_type` (line 77) and `write_content_type` (line 46, already correct)
2. `#supported-content-types` → annotate at `ContentType` enum (line 214) and its variants (lines 220, 224)

Sub-items to fix individually:
- `The value (hex) of this field MUST be a value that exists in the following table:` → at `read_content_type` (line 77)
- `The supported content types MUST be:` → at `ContentType` enum (line 214)
- `- '01' for [Non-Framed](message-body.md#non-framed-data)` → at `NonFramed = 1` (line 220)
- `- '02' for [Framed](message-body.md#framed-data)` → at `Framed = 2` (line 224)

## Targeted Tests
- `test_content_type_framed_value` — verifies framed content type byte is 0x02
- `test_content_type_nonframed_value` — verifies non-framed content type byte 0x01 is accepted
- `test_content_type_invalid_value_rejected` — verifies invalid content type is rejected

No test changes needed — tests already use the correct path prefix.

## Success Criteria
```bash
make duvet
```
- [ ] duvet report shows no gaps for `#content-type` section
- [ ] duvet report shows no gaps for `#supported-content-types` section
- [ ] All 4 annotations in `header_types.rs` use `aws-encryption-sdk-specification/` prefix
- [ ] No functional code changes — only annotation path prefixes
