# Agent 2 Notes — v2_header_body (Round 2)

## Review Feedback Summary

Three issues from Agent 3:
1. **ANNOTATION_MISSING**: Both required annotations are absent from `v2_header_body.rs`
2. **TEST_MISSING**: Test file exists but is untracked (needs to be committed)
3. **CODE_QUALITY**: Duvet config may need `./tests/**/*.rs` pattern; also clippy warnings in test

## Pre-Implementation Reasoning

### 1. Logical steps
1. Add two annotation blocks to `write_v2_header_body` after the existing "MUST be serialized with the following specifics" annotation
2. Add `./tests/**/*.rs` source pattern to duvet config so test annotations are picked up
3. Fix clippy warnings in test file (`pos + 1 <= ct.len()` → `pos < ct.len()`)

### 2. Point of fulfillment
- Requirement 1 (serialization order MUST follow): Fulfilled by the entire function body's sequential write calls. Annotate at function body start.
- Requirement 2 (V2 Header Body MUST be serialized as, in order...): Same — the sequential write calls ARE the ordering. Annotate at function body start.

### 3. Sub-items
Requirement 2 lists the fields in order. The individual field annotations already exist. The ordering annotation is the "parent" that covers the overall sequence.

### 4. Reviewer readability
Both annotations go right after the existing "MUST be serialized with the following specifics" annotation, before the first write call. This is where the ordering begins.

### 5. Existing patterns
`test_v1_header_body.rs` has the exact same pattern for V1 — `test_v1_header_serialization_order` with both the encrypt.md and data-format annotations. Follow that pattern exactly.

## Cross-reference check
- Requirement 1 quote: "The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification."
  - Contains link to `message-header.md#header-body-version-20` → Requirement 2 IS the cross-reference. Both annotations placed together.
- Requirement 2 quote: "The V2 Header Body MUST be serialized as, in order, Version, Algorithm Suite ID, ..." — No markdown links in this quote.

## Annotation type
Both are `type=implication` with reason — the ordering is structural (enforced by sequential function body), not runtime-testable in isolation.
