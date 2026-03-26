# Agent 1 Notes — header_types.rs

## Spec Structure Traceability (Step 6.8)

### 1. Spec Section Logical Flow

The `supported-versions` section defines:
1. A parent requirement: "The supported versions MUST be:"
2. Sub-item: "`01` MUST be version 1.0"
3. Sub-item: "`02` MUST be version 2.0"

This maps directly to the `MessageFormatVersion` enum definition in header_types.rs,
which has variants `V1 = 1` and `V2 = 2`.

### 2. Where Each Requirement Is Fulfilled

- "The supported versions MUST be:" → the `MessageFormatVersion` enum declaration itself
- "`01` MUST be version 1.0" → `V1 = 1` variant
- "`02` MUST be version 2.0" → `V2 = 2` variant

These are structural/type-system properties — the enum constrains the valid values.
The `read_msg_format_version` function enforces this at runtime during deserialization.

### 3. Sub-items

Yes — the spec has two sub-items (list items) under the parent requirement.
Each maps to a specific enum variant.

### 4. Most Likely Structural Mistake

The implementer might be tempted to annotate only the `read_msg_format_version` function
(runtime validation) and miss the enum definition (type-system constraint).
Both should be annotated — the enum with `type=implication` and the read function
with `type=implementation`.

Also: the implementer might place the parent annotation on the enum
and forget to annotate each sub-item at the individual variant.

## Duplicate Annotation Found

The annotation:
```
//= specification/data-format/message-header.md#content-type
//# The value (hex) of this field MUST be a value that exists in the following table:
```
appears at TWO locations in header_types.rs:
1. Line 53-54: before `read_content_type` function (correct — runtime validation)
2. Line 183-184: before `ContentType` enum definition (should be `supported-content-types` instead)

The enum definition should be annotated with the `supported-content-types` section,
not the `content-type` section, because the enum defines WHAT the supported types are,
while the `content-type` section describes the field format and validation.

## Coverage Summary for header_types.rs-relevant sections

| Section | Requirements | Annotations | Status |
|---------|-------------|-------------|--------|
| supported-versions | 3 MUST | 0 | ❌ All missing |
| supported-types | 2 MUST | 0 | ❌ All missing |
| supported-content-types | 1 MUST | 0 | ❌ Missing |
| content-type | 2 MUST | 1 impl (duplicate) | ⚠️ Duplicate + missing length |
| version | 1 MUST | 0 | ❌ Missing |
| type | 2 MUST | 0 | ❌ Missing |

## Potential Spec Gaps

### 1. MessageFormatVersion enum restricts to exactly V1/V2
- **Code location**: `header_types.rs` line 13-17, `MessageFormatVersion` enum
- **Behavior**: The enum only has V1 and V2 variants. `read_msg_format_version` returns an error for any other value.
- **Why it matters**: Interoperability — if a future version 3 is added, old code will reject it. The spec says "supported versions MUST be" but doesn't explicitly say "all other versions MUST be rejected."
- **Suggested spec requirement**: "If the version field contains a value that is not a supported version, the implementation MUST return an error."

### 2. MessageType enum restricts to exactly TypeCustomerAed (0x80)
- **Code location**: `header_types.rs` line 175-178, `MessageType` enum
- **Behavior**: `read_msg_type` returns error for any value other than 0x80.
- **Why it matters**: Interoperability — the spec says "supported types MUST be" but doesn't explicitly say unsupported types must be rejected.
- **Suggested spec requirement**: "If the type field contains a value that is not a supported type, the implementation MUST return an error."
