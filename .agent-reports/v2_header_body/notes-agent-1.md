# Agent 1 Notes — v2_header_body.rs

## Coverage Analysis

### Specs Analyzed
1. `aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header` — 18 TOML specs
2. `aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-2-0` — 2 TOML specs

### Implementation Annotations Present (in v2_header_body.rs)
- `encrypt.md#v2-header`: 9 annotation blocks covering 17 of 18 specs
- `data-format/message-header.md#header-body-version-2-0`: 1 annotation block covering 1 of 2 specs

### Missing Implementation Annotations
1. `encrypt.md#v2-header`: "The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification."
2. `data-format/message-header.md#header-body-version-2-0`: "The V2 Header Body MUST be serialized as, in order, Version, Algorithm Suite ID, Message ID, AAD, Encrypted Data Keys, Content Type, Frame Length, and Algorithm Suite Data."

### Missing Test Annotations
ALL implementation annotations in this file lack corresponding `type=test` annotations. No test file exists for v2 header body serialization.

### Priority Decision
Selected the two missing serialization-order implementation annotations as the work item because:
- They are tightly coupled (both describe the same ordering constraint)
- They are the only COMPLETELY missing annotations (no implementation at all)
- Adding them is a small, self-contained unit
- They also need test annotations

The broader missing-test-annotation gap (for all 18+ specs) is a separate, larger work item.

## Spec-Aligned Structure Analysis

### Q1: Logical flow of the spec section
The `header-body-version-2-0` section describes:
1. Version field value constraint (MUST be `02`)
2. A table of fields with lengths and interpretations (non-normative)
3. Serialization order constraint (MUST be in order: Version, Algorithm Suite ID, Message ID, AAD, Encrypted Data Keys, Content Type, Frame Length, Algorithm Suite Data)

The `encrypt.md#v2-header` section describes:
1. Condition: algorithm suite format version is 2.0
2. Per-field serialization requirements (8 fields)
3. Overall serialization order constraint (MUST follow header-body-version-2-0)

### Q2: Where each requirement is fulfilled in code
- Serialization order → the sequential `write_*` calls in `write_v2_header_body` and sequential `read_*` calls in `read_v2_header_body`
- The order is structurally enforced by the function body

### Q3: Sub-items
The data-format spec lists the fields in order:
- Version
- Algorithm Suite ID
- Message ID
- AAD
- Encrypted Data Keys
- Content Type
- Frame Length
- Algorithm Suite Data

These are already individually annotated with the encrypt.md#v2-header per-field requirements.

### Q4: Most likely structural mistake
The implementer might be tempted to annotate the serialization order at the function signature rather than at the first `write_*` call. The annotation should be at the function body start (before the first write call) since the ORDER of the function body IS the fulfillment.

For `read_v2_header_body`, the same ordering constraint applies to deserialization — the read calls must be in the same order.

## Potential Spec Gaps

### 1. Commitment algorithm suite validation on deserialize
- **Code location**: `read_v2_header_body` line ~90: `if !has_hkdf(&algorithm_suite.commitment) { return ser_err(...) }`
- **Behavior**: V2 header body deserialization rejects algorithm suites that don't support commitment (HKDF)
- **Why it matters**: Security — V2 format requires commitment; accepting non-committing suites in V2 would be a security vulnerability
- **Suggested spec requirement**: "When deserializing a Version 2.0 header body, the algorithm suite MUST support key commitment. If the algorithm suite does not support key commitment, deserialization MUST fail."

### 2. Suite data length derived from algorithm suite
- **Code location**: `read_v2_header_body` line ~108: `let len = get_hkdf(&algorithm_suite.commitment).output_key_length;`
- **Behavior**: The length of suite_data read is determined by the HKDF output key length from the algorithm suite, not self-describing
- **Why it matters**: Interoperability — implementations must agree on how to determine suite data length
- **Suggested spec requirement**: "The length of the Algorithm Suite Data field MUST be equal to the output key length of the key commitment derivation function specified by the algorithm suite."
