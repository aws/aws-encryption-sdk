# Agent 3 Review Notes — header_types

## Round 2 Adversarial Pre-Review

### Round 1 Feedback Verification

1. **NonFramed annotation misplacement (Critical)**: FIXED ✅
   - NonFramed `type=test` annotation moved from `test_content_type_invalid_value_rejected` to new dedicated `test_content_type_nonframed_value` test.
   - New test corrupts content type byte to 0x01 and verifies error is NOT "Unsupported Content Type", proving 0x01 is accepted.
   - Semantic match: annotation says "01 for Non-Framed", test exercises value 0x01.

2. **Invalid-value test annotation (Critical)**: FIXED ✅
   - Changed from `supported-content-types` NonFramed sub-item to `content-type` section quote.
   - Now reads: `The value (hex) of this field MUST be a value that exists in the following table:` — correct for a rejection test.

3. **Blank lines between annotations (Placement)**: FIXED ✅
   - No blank lines between annotation blocks in any test function.
   - Annotations flow directly into code.

### Per-Block Context-Reset Evaluation (Test File)

**Block 1** (`test_content_type_framed_value`, lines 56-59):
- Annotation: "The supported content types MUST be:" + "- `02` for Framed"
- Code: encrypts, finds content type offset, asserts `ct[offset] == 0x02`
- Obvious? YES — test verifies Framed content type byte is 0x02. ✅

**Block 2** (`test_content_type_nonframed_value`, lines 70-72):
- Annotation: "- `01` for Non-Framed"
- Code: corrupts byte to 0x01, verifies NOT "Unsupported Content Type"
- Obvious? YES — test proves 0x01 is a valid/supported content type. ✅

**Block 3** (`test_content_type_invalid_value_rejected`, lines 87-89):
- Annotation: "The value (hex) of this field MUST be a value that exists in the following table:"
- Code: corrupts byte to 0x00, asserts decryption fails
- Obvious? YES — test proves invalid value (0x00) is rejected. ✅

### Stacking Check
- Block 1: 2 annotation blocks (parent MUST + Framed sub-item). Within limit. ✅
- Block 2: 1 annotation block. ✅
- Block 3: 1 annotation block. ✅

### Anti-Rationalization Check
No "but" patterns found. All Round 1 issues were cleanly addressed. No new concerns.

### Cross-Reference Check
- 2 markdown links found in test annotations (message-body.md#framed-data, message-body.md#non-framed-data)
- Both are definitional links (what Framed/Non-Framed data ARE), not requirement cross-references
- No cross-reference annotations needed. ✅

### New Issues Introduced by Round 2 Changes
None found. The changes are minimal and targeted.

## Step 2: Adversarial Pre-Review

### 1. Per-annotation challenge: Does the next line actually implement THAT requirement?

**Annotation Block A** (lines 183-186 in source):
```
//= specification/data-format/message-header.md#supported-content-types
//= type=implication
//= reason=The enum definition structurally constrains valid content types to exactly these two variants.
//# The supported content types MUST be:
```
Next line: `#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]`
Then: `pub(crate) enum ContentType {`

Challenge: Does the enum definition fulfill "The supported content types MUST be:"?
Yes — the enum defines exactly which content types exist. The `type=implication` is correct because this is a structural/type-system constraint. The `reason=` line explains the connection. The derive macro line is not executable code per se, but the enum declaration IS the structural fulfillment. This follows the same pattern as v2_header_body.rs for implication annotations on structural properties.

**Annotation Block B** (on NonFramed variant):
```
//= specification/data-format/message-header.md#supported-content-types
//= type=implication
//# - `01` for [Non-Framed](message-body.md#non-framed-data)
```
Next line: `NonFramed = 1,`

Challenge: Does `NonFramed = 1` fulfill "- `01` for Non-Framed"?
Yes — the variant is named NonFramed and has value 1 (0x01). Direct semantic match. ✅

**Annotation Block C** (on Framed variant):
```
//= specification/data-format/message-header.md#supported-content-types
//= type=implication
//# - `02` for [Framed](message-body.md#framed-data)
```
Next line: `#[default]` then `Framed = 2,`

Challenge: Does `Framed = 2` fulfill "- `02` for Framed"?
Yes — the variant is named Framed and has value 2 (0x02). Direct semantic match. ✅

Note: The `#[default]` attribute is between the annotation and the `Framed = 2` line. This is an attribute on the variant, not a comment or blank line. It's part of the variant definition. Acceptable.

### 2. Annotation stacking check

**Block A**: 4 lines (target, type, reason, quote) before `#[derive...]`.
This is a single annotation block with metadata lines, not multiple annotation blocks stacked.
The `//= type=implication`, `//= reason=...` are metadata for the SAME annotation, not separate annotations.
This is 1 annotation block, not 3+ stacked annotations. ✅

**Block B**: 3 lines (target, type, quote) — single annotation block. ✅
**Block C**: 3 lines (target, type, quote) — single annotation block. ✅

No stacking violations found.

### 3. Context-reset evaluation per block

**Block A + code until Block B**:
Reading ONLY: "The supported content types MUST be:" + `enum ContentType {`
Is it obvious? Yes — the enum defines what the supported content types are. The `type=implication` and `reason=` make the connection explicit. ✅

**Block B + code until Block C**:
Reading ONLY: "- `01` for Non-Framed" + `NonFramed = 1,`
Is it obvious? Yes — variant name matches, value matches. ✅

**Block C + code until end**:
Reading ONLY: "- `02` for Framed" + `Framed = 2,`
Is it obvious? Yes — variant name matches, value matches. ✅

### 4. Semantic relationship check

All annotations semantically relate to their code lines. ✅

### 5. Sub-items annotated individually?

The spec lists two sub-items under "The supported content types MUST be:":
- `01` for Non-Framed → annotated at `NonFramed = 1` ✅
- `02` for Framed → annotated at `Framed = 2` ✅

Pattern 4 correctly applied.

### 6. Code structure mirrors spec structure?

Spec: "The supported content types MUST be: [list]"
Code: enum with variants matching the list.
Structure mirrors spec. ✅

### 7. Linear readability?

Reading top-to-bottom: enum annotation → enum declaration → variant 1 annotation → variant 1 → variant 2 annotation → variant 2.
Perfectly linear. ✅

### Test Review

**test_content_type_framed_value**:
- Has `type=test` for "The supported content types MUST be:" ✅
- Has `type=test` for "- `02` for Framed" ✅
- Actually encrypts data and checks the content type byte in the output is 0x02 ✅
- Tests observable behavior (output bytes), not internal state ✅

**test_content_type_invalid_value_rejected**:
- Has `type=test` for "- `01` for Non-Framed" 
- CONCERN: The test annotation says it tests `01` for Non-Framed, but the test actually corrupts a byte to 0x00 and verifies rejection. It does NOT test that NonFramed = 0x01.
- The test verifies that invalid content types are rejected, which is related to the `content-type` section ("The value (hex) of this field MUST be a value that exists in the following table"), not specifically the `supported-content-types` section.
- However, the test does exercise `read_content_type` which validates against the ContentType enum variants. The enum constraining to {1, 2} means rejecting 0x00 proves the enum constraint is enforced at runtime.
- The `type=test` annotation for the NonFramed sub-item is semantically weak here — the test doesn't actually verify that NonFramed has value 0x01.

### Cross-reference check

Sub-item quotes contain markdown links:
- `[Non-Framed](message-body.md#non-framed-data)` — links to message-body.md#non-framed-data
- `[Framed](message-body.md#framed-data)` — links to message-body.md#framed-data

Agent 2's notes say these are definitional links and adding cross-reference annotations would be misleading. Let me verify: these links point to what Non-Framed and Framed data ARE in the message body spec. They are not requirements about content types — they define the data format structures. Adding annotations from message-body.md at the enum variant level would be incorrect since the enum doesn't implement message body serialization.

I agree with Agent 2's assessment. These are definitional cross-references, not requirement cross-references. No cross-reference annotations needed.

Cross-reference ratio: 2 links found, 0 cross-refs needed (definitional links). Acceptable.

## Step 3: Anti-Rationalization Check

Reviewing my notes for "but" patterns:

1. "The test annotation for NonFramed sub-item is semantically weak here — the test doesn't actually verify that NonFramed has value 0x01."

I DID notice this. The `test_content_type_invalid_value_rejected` test has `type=test` for `- \`01\` for [Non-Framed]` but the test doesn't verify NonFramed = 0x01. It verifies that 0x00 is rejected. These are different things.

However — the `test_content_type_framed_value` test verifies Framed = 0x02 by checking the output byte. There is no corresponding test that verifies NonFramed = 0x01 by checking output bytes. The NonFramed sub-item's test annotation is on a test that doesn't exercise NonFramed at all.

This IS a finding. The `type=test` for the NonFramed sub-item should be on a test that actually verifies NonFramed produces 0x01 in the output, or at minimum asserts `ContentType::NonFramed as u8 == 1`.

But wait — the sub-items are non-normative (not `[[spec]]` entries in the TOML). The TOML only has one `[[spec]]` entry: "The supported content types MUST be:". The sub-items are additional traceability annotations. Duvet doesn't require test coverage for non-normative text.

Still, the `type=test` annotation claims this test covers the NonFramed sub-item, which is misleading. The test doesn't exercise NonFramed at all.

This is a finding I should flag.

## Potential Spec Gaps

None identified. The implementation is minimal and matches the spec.
