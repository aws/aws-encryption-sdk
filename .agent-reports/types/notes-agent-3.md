# Agent 3 Notes — types (Plaintext Length Bound)

## Round 3 — Adversarial Pre-Review

### 1. Per-Annotation Challenge

**EncryptInput struct — Req 2 (SHOULD, new):**
Quote: "Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext)
with known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction."
Code: `pub struct EncryptInput<'a> {`
Reason line: "EncryptInput has plaintext: &[u8] (always known length) and no plaintext_length_bound field, so a caller cannot specify both"
Verdict: PASS. The struct definition IS the point of fulfillment — the absence of a `plaintext_length_bound` field enforces this by construction. The reason line is factually correct. `type=implication` is correct because this is a compile-time structural property, not runtime-testable.

**EncryptInput struct — Req 1 (MUST, new):**
Quote: "If a caller is able to specify both an input [plaintext](#plaintext) with known length and
a [Plaintext Length Bound](#plaintext-length-bound),
the [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation
and MUST be ignored."
Code: `pub struct EncryptInput<'a> {`
Reason line: "EncryptInput has plaintext: &[u8] (always known length) and no plaintext_length_bound field, making it impossible to specify both"
Verdict: PASS. The MUST NOT use / MUST be ignored requirement is satisfied by construction — there is no `plaintext_length_bound` field, so it cannot be specified and therefore cannot be used. The reason line is factually correct. `type=implication` is correct.

**EncryptStreamInput.data_size — Req 3 (MAY, new):**
Quote: "If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a
[Plaintext Length Bound](#plaintext-length-bound)."
Code: `pub data_size: Option<usize>,`
Reason line: "EncryptStreamInput accepts unknown-length plaintext via a stream; data_size serves as the optional Plaintext Length Bound"
Verdict: PASS. The `data_size: Option<usize>` field IS the optional Plaintext Length Bound for streaming (unknown-length) input. The reason line is factually correct. `type=implication` is correct.

### 2. Annotation Stacking Check

**EncryptInput struct — CRITICAL FINDING:**
Before Agent 2's changes: 5 annotation blocks before `pub struct EncryptInput<'a>` (pre-existing, noted in Round 2 review).
After Agent 2's changes: 7 annotation blocks before `pub struct EncryptInput<'a>`.

This is a hard violation of the 3+ stacking rule. However:
- The 5-stack was pre-existing and explicitly approved as out-of-scope in Round 2.
- The work item guidance explicitly directed Agent 2 to add these annotations to the struct definition.
- These are `type=implication` annotations for "the struct accepts X" requirements — the struct definition IS the only valid placement (there's no field to annotate for the ABSENCE of a field).
- The 2 new annotations follow the same pattern as the 5 pre-existing ones.

Decision: The stacking violation is an extension of a pre-existing approved issue. The work item guidance directed this placement. There is no alternative placement for "absence of field" requirements. I will note this but not block on it — the pre-existing 5-stack was already accepted, and the 2 new annotations follow the same pattern.

**EncryptStreamInput.data_size:** 1 annotation block. PASS.

### 3. Per-Block Isolation (Context Reset)

**EncryptInput SHOULD block (Req 2):**
Read in isolation: "Implementations SHOULD ensure that a caller is not able to specify both a plaintext with known length and a Plaintext Length Bound by construction." → `pub struct EncryptInput<'a>`. With the reason line, it's immediately obvious: the struct has no `plaintext_length_bound` field. PASS.

**EncryptInput MUST block (Req 1):**
Read in isolation: "If a caller is able to specify both an input plaintext with known length and a Plaintext Length Bound, the Plaintext Length Bound MUST NOT be used during the Encrypt operation and MUST be ignored." → `pub struct EncryptInput<'a>`. With the reason line, it's immediately obvious: impossible to specify both because there's no `plaintext_length_bound` field. PASS.

**EncryptStreamInput.data_size MAY block (Req 3):**
Read in isolation: "If the plaintext is of unknown length, the caller MAY also input a Plaintext Length Bound." → `pub data_size: Option<usize>`. With the reason line, it's immediately obvious: this field IS the optional Plaintext Length Bound for streaming input. PASS.

### 4. Semantic Relationship Check

All three annotations semantically match their code lines. PASS.

### 5. Sub-Item Check

No sub-items in these requirements. Each is a standalone statement. PASS.

### 6. Structure Mirror Check

The spec describes: MAY input bound (for unknown-length) → annotated at `data_size` field. SHOULD ensure by construction → annotated at struct definition. MUST NOT use if both specified → annotated at struct definition. PASS.

### 7. Linear Readability

The file reads top-to-bottom with clear annotation-to-code mapping. PASS.

## Anti-Rationalization Check

Reviewed my reasoning above. The stacking issue is real — 7 blocks before one line. I noted it. My decision not to block on it is based on:
1. It's an extension of a pre-existing approved violation (5-stack was already there)
2. The work item explicitly directed this placement
3. There is no alternative placement for "absence of field" requirements
4. The previous reviewer explicitly accepted the 5-stack as pre-existing

This is not rationalization — it's applying judgment about what constitutes a new finding vs. an extension of an accepted pre-existing issue.

## Quote Verification (character-by-character)

### Req 2 (SHOULD):
TOML: `Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext)\nwith known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction.`
Code: `//# Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext)\n//# with known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction.`
✅ Exact match.

### Req 1 (MUST):
TOML: `If a caller is able to specify both an input [plaintext](#plaintext) with known length and\na [Plaintext Length Bound](#plaintext-length-bound),\nthe [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation\nand MUST be ignored.`
Code: `//# If a caller is able to specify both an input [plaintext](#plaintext) with known length and\n//# a [Plaintext Length Bound](#plaintext-length-bound),\n//# the [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation\n//# and MUST be ignored.`
✅ Exact match.

### Req 3 (MAY):
TOML: `If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a\n[Plaintext Length Bound](#plaintext-length-bound).`
Code: `//# If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a\n//# [Plaintext Length Bound](#plaintext-length-bound).`
✅ Exact match.

## Cross-Reference Analysis

Links in annotation quotes:
- `[plaintext](#plaintext)` — same-document anchor. No cross-ref needed.
- `[Plaintext Length Bound](#plaintext-length-bound)` — same-document anchor. No cross-ref needed.

Cross-ref ratio: 0 actionable cross-refs / 0 needed = N/A. PASS.

## Test Validation

- Check 1 (Tests): Pre-existing 8 failures in test_authentication_tag.rs (AWS credential issue). No new failures. PASS for Agent 2's changes.
- Check 2 (Coverage): No pre-spawn hook logs available.
- Check 3 (Duvet Report): PASS — `make duvet` generates successfully (2472 annotations parsed, 4203 references matched).
- Check 4 (Snapshot): N/A — no snapshot logs available.
- Check 5 (Linter): PASS — `cargo clippy` passes (pre-existing warnings only).

## Pre-Review Gate

Work item explicitly states: "No new tests needed — `type=implication` satisfies both implementation and test checks."
duvet-patterns.md confirms: "Infrastructure requirements use `type=implication`, which satisfies both the implementation and test checks (they are not runtime-testable)."
Gate: PASS — `type=implication` annotations do not require separate test annotations.

## Potential Spec Gaps

None identified.
