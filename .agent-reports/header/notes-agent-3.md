# Agent 3 Review Notes — header

## Step 2: Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**header.rs line 133-137 (new annotation — "interpreted as bytes")**:
- Annotation: `The algorithm suite data MUST be interpreted as bytes.`
- Code line: `if header_body.suite_data() != expected_suite_data {`
- Analysis: `suite_data()` returns `&[u8]` and `expected_suite_data` is `&[u8]`. The comparison IS a byte comparison. The `type=implication` with `reason=` is appropriate — the type system enforces byte interpretation. The code line semantically relates: it's comparing suite data as bytes.
- Verdict: PASS. The annotation is at the point where byte interpretation is exercised.

**header.rs line 98 (style fix — removed `type=implementation`)**:
- Annotation: `This value MUST be greater than 0.`
- Code line: `pub(crate) fn validate_max_encrypted_data_keys(`
- Analysis: Pre-existing placement. The function validates EDK count. The annotation is at the function that enforces the requirement. Acceptable.
- Verdict: PASS (pre-existing, only style change).

**header.rs line 115 (style fix — removed `type=implementation`)**:
- Annotation: `implementations MUST use a good source of randomness...`
- Code line: `pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {`
- Analysis: Pre-existing placement. The function generates message IDs using randomness. Direct semantic match.
- Verdict: PASS (pre-existing, only style change).

**test_v1_header_body.rs line 119-123 (new test annotation)**:
- Annotation: `While implementations cannot guarantee complete uniqueness, implementations MUST use a good source of randomness...`
- Code line: `assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");`
- Analysis: The test encrypts twice and asserts the message IDs differ. This is a probabilistic test for randomness. The annotation is immediately before the assertion. Direct semantic match.
- Verdict: PASS.

**test_v2_header_body.rs line 234-238 (new test annotation)**:
- Annotation: Same as above.
- Code line: `assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");`
- Analysis: Same pattern as V1 test. Direct semantic match.
- Verdict: PASS.

**test_header_structure.rs line 121-126 (new test)**:
- Annotation: `The algorithm suite data MUST be interpreted as bytes.`
- Code line: `let pt = b"suite data bytes test";` then `round_trip(pt).await` then `assert_eq!`
- Analysis: The test does a round-trip which exercises `validate_suite_data` during decrypt. The round-trip succeeding proves the byte comparison worked. The annotation is at the top of the test function, before the test logic. This follows Pattern 3 (general behavior at method start).
- Verdict: PASS.

### 2. Annotation stacking check

No annotation stacking found in any modified file. Each annotation block has at most 1 annotation before a code line in the areas modified by Agent 2.

Pre-existing stacking in v1_header_body.rs (not modified by Agent 2 in those areas) — not blocking.

### 3. Per-block isolation evaluation

**header.rs "interpreted as bytes" block**:
- Annotation: `The algorithm suite data MUST be interpreted as bytes.`
- Code: `if header_body.suite_data() != expected_suite_data {`
- With context reset: I see "suite data MUST be interpreted as bytes" and a byte-slice comparison. The `type=implication` with `reason=` explains the connection. Immediately obvious.
- PASS.

**test_v1_header_body.rs randomness block**:
- Annotation: `implementations MUST use a good source of randomness when generating messages IDs...`
- Code: `assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");`
- With context reset: I see a randomness requirement and an assertion that two IDs differ. Immediately obvious.
- PASS.

**test_v2_header_body.rs randomness block**:
- Same pattern as V1. PASS.

**test_header_structure.rs "interpreted as bytes" block**:
- Annotation: `The algorithm suite data MUST be interpreted as bytes.`
- Code: `let pt = b"suite data bytes test"; let result = round_trip(pt).await; assert_eq!(...)`
- With context reset: The connection is that round_trip exercises validate_suite_data which does the byte comparison. This is indirect — the test doesn't directly test byte interpretation, it tests that a round-trip succeeds. However, this is the standard pattern used throughout the test suite for testing implication-type requirements. The assertion message explains the connection.
- PASS (acceptable for implication test pattern).

### 4. Semantic relationship check

All annotations have direct semantic relationships to their code lines. No mismatches found.

### 5. Spec sub-items check

The requirements addressed are individual MUST statements, not lists or tables. No sub-item annotation needed.

### 6. Code structure mirrors spec

The changes are annotation-only (removals, style fixes, one addition). The code structure was not changed and already mirrors the spec.

### 7. Linear readability

Reading header.rs top-to-bottom, the annotations flow naturally:
- `#structure` big-endian at `write_header_body`
- `#encrypted-data-key-count` at `validate_max_encrypted_data_keys`
- `#message-id` randomness at `generate_message_id`
- `#algorithm-suite-data` "interpreted as bytes" at `validate_suite_data`
- `#algorithm-suite-data` length at the length check in `validate_suite_data`

No jumping required. PASS.

## Step 3: Anti-Rationalization Check

Reviewing my Step 2 notes for the pattern "This is [wrong] **but** [acceptable because]":

1. I noted the implementation annotation in header.rs uses a partial quote (missing "While implementations cannot guarantee complete uniqueness,"). However, this is pre-existing and the work item explicitly says to keep this annotation and only remove the `type=implementation` line. Agent 2 followed the work item guidance correctly. This is NOT a rationalization — it's a pre-existing issue outside scope.

2. I noted the test for "interpreted as bytes" is indirect (round-trip). But this is the standard pattern for `type=implication` requirements throughout the test suite, and the work item guidance explicitly suggests this approach. Not a rationalization.

No anti-rationalization issues found.

## Step 4: Pre-Review Gate

**Test file modified**: YES — Agent 2 modified `test_header_structure.rs`, `test_v1_header_body.rs`, and `test_v2_header_body.rs` with `type=test` annotations.
- PASS.

## Potential Spec Gaps

None identified. The changes are annotation-only.

## Pre-existing Issues (not blocking)

1. header.rs line 115: The implementation annotation for `#message-id` randomness uses a partial quote (missing first line). The TOML has the full 3-line quote. This predates Agent 2's changes.
2. Clippy warnings in unrelated files (missing docs on `encrypt_stream`, collapsible `if` in `v1_header_body.rs`, unreachable patterns in `materials.rs`).
3. `test_authentication_tag` tests fail due to invalid AWS credentials — pre-existing, unrelated.
