# Agent 3 Notes — header (Round 3: EDK count, suite data, frame length tests)

## Step 2: Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**Test 1: `test_encrypted_data_key_count_greater_than_zero`**
- Annotation: `//# This value MUST be greater than 0.`
- Next executable line: `let ct = encrypt_default(b"edk count test").await;`
- The annotation is at the top of the test function. The actual assertion proving the requirement is `assert!(edk_count > 0, ...)` several lines later.
- For `type=test`, the convention is to place the annotation at the top of the test function, near the beginning. The test as a whole proves the requirement. The assertion at the end is the fulfillment point, but the annotation at the top is the established pattern in this codebase (see `test_header_big_endian_format`, `test_header_serialization_order`).
- VERDICT: Acceptable — follows the established test annotation pattern.

**Test 2: `test_suite_data_length_matches_algorithm_suite`**
- Annotation: `//# The length of the suite data field MUST be equal to...`
- Next executable line: `let pt = b"suite data length test";`
- Same pattern — annotation at top of test, assertion at bottom.
- The round-trip test proves the requirement because `validate_suite_data` runs during decrypt and would fail if the length were wrong.
- VERDICT: Acceptable — indirect proof via round-trip, same pattern as existing tests.

**Test 3: `test_nonframed_frame_length_must_be_zero`**
- Annotation: `//# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.`
- Next executable line: `let keyring = test_keyring().await;`
- The test mutates ciphertext to set NonFramed content type + non-zero frame length, then asserts decrypt fails.
- This is a negative test — it proves the validation rejects the invalid combination.
- VERDICT: Acceptable — negative test pattern, annotation at top of test function.

### 2. Annotation stacking check

No annotation stacking anywhere. Each test function has exactly one annotation block (target + type + quote). PASS.

### 3. Per-block isolation evaluation

**Block 1** (test_encrypted_data_key_count_greater_than_zero):
- Annotation: `specification/data-format/message-header.md#encrypted-data-key-count` / `This value MUST be greater than 0.`
- Code: encrypts, parses header, asserts edk_count > 0.
- Is it obvious? YES — the test name says "greater than zero", the annotation says "greater than 0", the assertion checks `edk_count > 0`.

**Block 2** (test_suite_data_length_matches_algorithm_suite):
- Annotation: `specification/data-format/message-header.md#algorithm-suite-data` / `The length of the suite data field MUST be equal to...`
- Code: round-trip encrypt/decrypt.
- Is it obvious? SOMEWHAT — the connection is that `validate_suite_data` runs during decrypt. The assertion message explains this. The test name also makes it clear.
- This is an indirect proof. The work item guidance explicitly says "A successful V2 encrypt+decrypt round-trip proves the suite data length matches the algorithm suite, since validate_suite_data is called during decrypt and would fail if the length were wrong."
- VERDICT: Acceptable given the indirect proof pattern used throughout the codebase.

**Block 3** (test_nonframed_frame_length_must_be_zero):
- Annotation: `specification/data-format/message-header.md#frame-length` / `When the [content type](#content-type) is non-framed, the value of this field MUST be 0.`
- Code: encrypts, mutates ciphertext to set NonFramed + non-zero frame length, asserts decrypt fails.
- Is it obvious? YES — the mutation clearly sets up the violation, and the assertion proves it's rejected.

### 4. Semantic relationship check

All three annotations semantically match their test code. PASS.

### 5. Spec sub-items check

The three requirements are standalone MUST statements, not lists or tables. No sub-item annotation needed. PASS.

### 6. Code structure mirrors spec

The three tests correspond to three distinct spec sections (encrypted-data-key-count, algorithm-suite-data, frame-length). Each test is a separate function. PASS.

### 7. Linear readability

The file reads top-to-bottom: helpers first, then tests in order. Each test is self-contained. PASS.

## Step 3: Anti-Rationalization Check

Reviewing my Step 2 notes for "but" patterns:

- Block 2 (suite data): I noted "SOMEWHAT" obvious and said "Acceptable given the indirect proof pattern." This is a potential rationalization. However, the indirect proof pattern is genuinely the established pattern in this codebase (test_header_big_endian_format, test_header_serialization_order both use round-trip). The work item guidance explicitly endorses this approach. I'm not rationalizing — I'm applying the established standard.

No other "but" patterns found.

## Step 4: Pre-Review Gate

- Test file modified: YES — `tests/test_header_structure.rs` has 3 new test functions with `type=test` annotations.
- PASS.

## Quote Verification

### Requirement 1: encrypted-data-key-count
- TOML: `This value MUST be greater than 0.`
- Annotation: `//# This value MUST be greater than 0.`
- MATCH ✅

### Requirement 2: algorithm-suite-data
- TOML: `The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value\nof the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.`
- Annotation: `//# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value\n//# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.`
- MATCH ✅

### Requirement 3: frame-length
- TOML: `When the [content type](#content-type) is non-framed, the value of this field MUST be 0.`
- Annotation: `//# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.`
- MATCH ✅

## Target Path Verification

All three annotations use `specification/data-format/message-header.md#...` which matches the TOML target prefix. ✅

## Code Reuse Assessment

Agent 2 added `parse_header_offsets()` which is similar to `parse_v2_header_field_offsets()` in `test_v2_header_body.rs`. However:
- The existing helper returns `Vec<(&str, usize, usize)>` with all fields and assertions
- The new helper returns just `(usize, usize, usize)` for the three specific offsets needed
- They serve different purposes and the new one is simpler
- Moving to shared fixtures would require making it accessible across test files, which is a larger refactor
- Non-blocking observation.

## Pre-existing Issues Noted

- `header.rs` line 99: `//= type=implementation` is unnecessary (default type). Pre-existing, non-blocking.
- Clippy warnings (missing docs, unreachable patterns) are pre-existing and not in modified files.
- `test_authentication_tag` tests fail due to expired AWS credentials — pre-existing, not related to this change.
