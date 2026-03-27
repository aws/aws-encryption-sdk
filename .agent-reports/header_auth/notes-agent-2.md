# Pre-Implementation Reasoning — Round 4 (header_auth)

## 1. Logical Steps

This round addresses 4 remaining requirements (Req 5 already covered):

1. Fix quote mismatch in v1 annotation (Req 1) — remove comma after "1.0" in `write_header_auth_tag` match arm
2. Add IV length annotation (Req 2) — at `read_vec` for IV in `read_header_auth_tag_v1`
3. Add IV "interpreted as bytes" annotation (Req 3) — at same `read_vec` for IV
4. Add auth tag length annotation (Req 4) — at `read_vec` for tag in both v1 and v2 read functions

## 2. Point of Fulfillment

- Req 1 (quote fix): The annotation at the `1 =>` match arm in `write_header_auth_tag`
- Req 2 (IV length): `read_vec(r, get_iv_length(suite) as usize, raw)` — the `get_iv_length(suite)` enforces the length equals the algorithm suite's IV length
- Req 3 (IV as bytes): Same `read_vec` call — the IV is stored as `Vec<u8>`, i.e., raw bytes
- Req 4 (auth tag length): `read_vec(r, get_tag_length(suite) as usize, raw)` — the `get_tag_length(suite)` enforces the length equals the algorithm suite's tag length

## 3. Sub-items

No sub-items — each requirement is a standalone normative statement.

## 4. Reviewer Readability

- Req 1: Fix in-place at existing annotation location (line 17-22)
- Req 2+3: Two `type=implication` annotations before the `let header_iv = read_vec(...)` line in `read_header_auth_tag_v1` — this is exactly 2 blocks, within the stacking limit
- Req 4: One `type=implication` annotation before `let header_auth_tag = read_vec(...)` in both v1 and v2 read functions

## 5. Existing Patterns

- `body.rs` lines 88-98: Uses `type=implication` for identical IV/tag length structural requirements
- `encrypt.rs` lines 481-483: Uses `type=implication` with `reason=` for "interpreted as bytes"
- Both use the same annotation style I'll follow

## Annotation Type Decisions

All 4 new annotations (Reqs 2-4) are `type=implication`:
- "IV length MUST be equal to..." — structural, can't test "the length equals" directly, only that round-trip works
- "IV MUST be interpreted as bytes" — structural, no test can assert "interpreted as bytes"
- "auth tag length MUST be equal to..." — structural, same reasoning as IV length

This matches the body.rs pattern exactly.

## Test Annotations

Need `type=test` annotations for Reqs 2, 3, 4 in the test file. The existing `test_v1_header_auth_serialization_order` test exercises the v1 read path (round-trip). The `test_v2_header_auth_serialization` test exercises the v2 read path.

Wait — these are `type=implication` annotations. Per duvet-patterns.md: "Infrastructure requirements use `type=implication`, which satisfies both the implementation and test checks (they are not runtime-testable)." So `type=implication` annotations do NOT need corresponding `type=test` annotations. But the work item says "All implementations have corresponding `type=test`". Let me re-read...

The work item says: "All requirements have `type=implication` (not `type=todo`) for structural properties" and "All implementations have corresponding `type=test`". But the duvet patterns doc says implication satisfies both checks. I'll add test annotations anyway since the work item explicitly requests them, and it doesn't hurt.
