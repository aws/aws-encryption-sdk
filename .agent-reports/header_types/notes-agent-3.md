# Agent 3 Review Notes — header_types

## Step 2: Adversarial Pre-Review

### 1. Per-annotation challenge: Does the next line actually implement THAT requirement?

**Source file annotations:**

1. `The supported versions MUST be:` (line 16) → next executable: `pub(crate) enum MessageFormatVersion {`
   - The enum definition IS the structural constraint on valid versions. ✅ Direct fulfillment.

2. `- \`01\` MUST be version 1.0` (line 21) → next executable: `V1 = 1`
   - The variant assignment IS the mapping of 0x01 to version 1.0. ✅ Direct fulfillment.

3. `- \`02\` MUST be version 2.0` (line 25) → next executable: `V2 = 2`
   - Same pattern. ✅ Direct fulfillment.

4. `The length of the serialized version field MUST be 1 byte.` (line 32) → next executable: `pub(crate) fn write_msg_format_version(`
   - The function uses `write_u8` internally, which constrains to 1 byte. The annotation is on the function signature, not on the `write_u8` call itself. However, the function's entire purpose is to serialize the version, and `write_u8` is the only operation. The reason line explains this. ✅ Acceptable.

5. `The length of the serialized type field MUST be 1 byte.` (line 42) → next executable: `pub(crate) fn write_msg_type(`
   - Same pattern as #4. ✅ Acceptable.

6. `The length of the serialized content type field MUST be 1 byte.` (line 49) → next executable: `pub(crate) fn write_content_type(`
   - Same pattern. ✅ Acceptable.

7. `The type (hex) of this field MUST be a value that exists in the following table:` (line 66) → next executable: `pub(crate) fn read_msg_type(`
   - The function validates the type byte against the match block. The annotation is on the function, and the match block inside enforces the constraint. ✅ Acceptable — follows the existing `read_content_type` pattern.

8. `The supported types MUST be:` (line 204) → next executable: `pub(crate) enum MessageType {`
   - Same pattern as #1. ✅ Direct fulfillment.

9. `- \`80\` MUST be Customer Authenticated Encrypted Data` (line 209) → next executable: `TypeCustomerAed = 0x80`
   - Wait — there's a `#[default]` attribute between the annotation and the variant. Let me check...

Actually, looking at lines 207-211:
```
    //= aws-encryption-sdk-specification/data-format/message-header.md#supported-types
    //= type=implication
    //# - `80` MUST be Customer Authenticated Encrypted Data
    #[default]
    TypeCustomerAed = 0x80,
```

The `#[default]` attribute is between the annotation and the executable line. Is `#[default]` an executable line? In Rust, attributes are part of the item they annotate — they're not comments or blank lines. The `#[default]` is part of the `TypeCustomerAed = 0x80` variant declaration. This is acceptable — the annotation is on the variant, and `#[default]` is metadata on that variant.

### 2. Annotation stacking check

**Source file**: No stacking issues. Each annotation block is followed by its own code element. Maximum is 1 annotation block per code line.

**Test file**: 
- `test_version_v2_value`: 2 blocks before `let ct = encrypt_v2(...)`. Under limit.
- `test_type_customer_aed_value`: 2 blocks before `let ct = encrypt_v1(...)`. Under limit.
- `test_content_type_framed_value`: 2 blocks before `let ct = encrypt_v2(...)`. Under limit.
- All other tests: 1 block each. ✅

### 3. Per-block isolation evaluation

All source annotations pass the context-reset test. Each annotation quote names its subject explicitly:
- "The supported versions MUST be" — clear subject
- "- `01` MUST be version 1.0" — clear subject
- "The length of the serialized version field MUST be 1 byte" — clear subject
- "The type (hex) of this field MUST be a value..." — "this field" refers to the Type field, which is the function being annotated. Slightly ambiguous but the function name `read_msg_type` makes it clear.
- All others: clear subjects. ✅

### 4. Semantic relationship check

All annotations have clear semantic relationships to their code lines. ✅

### 5. Sub-items annotated individually?

- `#supported-versions` has sub-items `01` and `02` → both annotated at their enum variants. ✅
- `#supported-types` has sub-item `80` → annotated at the enum variant. ✅
- `#supported-content-types` sub-items were pre-existing. ✅

### 6. Code structure mirrors spec structure?

The spec describes:
1. Version → enum + serialization functions
2. Type → enum + serialization functions  
3. Content Type → enum + serialization functions

Code follows this structure. ✅

### 7. Linear readability?

Reading top-to-bottom:
- Version enum with annotations → version write function with length annotation → type write function with length annotation → content-type write function with length annotation → read functions → type value validation → MessageType enum with annotations → ContentType enum (pre-existing)

The flow is logical. ✅

## Step 3: Anti-Rationalization Check

Reviewing my Step 2 notes for "but" patterns...

- I noted `#[default]` between annotation and variant. I said "This is acceptable." Let me re-examine: `#[default]` is a Rust attribute, not a comment or blank line. It's part of the item definition. The duvet rule says "no comments or blank lines between annotation and annotated code." Attributes are code. This is genuinely fine.

- I noted "this field" in the type annotation is slightly ambiguous. But the function name `read_msg_type` makes it clear. And this follows the exact same pattern as the pre-existing `read_content_type` annotation which says "The value (hex) of this field MUST be...". Consistent with existing code. Not a finding.

No anti-rationalization issues found.

## Cross-Reference Check

Scanning annotation quotes for markdown links:
- `//# - \`02\` for [Framed](message-body.md#framed-data)` — contains link to `message-body.md#framed-data`. This is in the pre-existing ContentType annotation, not Agent 2's change.
- `//# - \`01\` for [Non-Framed](message-body.md#non-framed-data)` — same, pre-existing.
- New annotations contain no markdown links. ✅

Cross-ref ratio: 0 links found in new annotations, 0 cross-refs needed. N/A.

## Potential Spec Gaps

None identified. The implementation is purely annotation work on existing code.
