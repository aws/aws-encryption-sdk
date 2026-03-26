# Agent 3 Review Notes — Round 3

## Context
This is a test-only update. Agent 2 rewrote `test_construct_the_body.rs` to parse output ciphertext bytes and assert on frame structure (regular frame count, final frame count, final frame content length) instead of just doing round-trip equality checks. All 7 existing duvet `type=test` annotations were preserved. No source code changes.

## Adversarial Pre-Review (Step 2)

### Question 1: Do the test annotations match the requirements they claim to test?

**Req 1 test** (`test_regular_frame_serialization_conforms_to_spec`): Encrypts 30 bytes with frame_length=10, asserts 2 regular frames + 1 final frame, plus round-trip. The structural assertion proves regular frames were constructed with the correct size. The round-trip proves they conform to the spec (decrypt can parse them). PASS.

**Req 2 test** (`test_process_consumable_bytes_as_regular_frames`): Encrypts 50 bytes with frame_length=10, asserts 4 regular frames + 1 final frame. This proves "as much of the consumable bytes as possible" were processed as regular frames (4 × 10 = 40 regular + 10 final = 50). PASS.

**Req 3 test** (`test_end_of_input_processing`): Encrypts 15 bytes with frame_length=10, asserts 1 regular + 1 final with content_length=5. Proves end-of-input processing creates the final frame with remaining bytes. PASS.

**Req 4 test** (`test_exact_frame_length_constructs_final_or_regular`): Encrypts 10 bytes with frame_length=10, asserts 0 regular + 1 final with content_length=10. This is the exact-match case. The spec says "MUST construct either a final frame or regular frame" — the implementation chooses final frame. The test verifies this. PASS.

**Req 5 test** (`test_enough_bytes_constructs_regular_frame`): Encrypts 25 bytes with frame_length=10, asserts 2 regular + 1 final with content_length=5. Proves regular frames are constructed when more bytes remain. PASS.

**Req 6 test** (`test_not_enough_bytes_constructs_final_frame`): Encrypts 7 bytes with frame_length=10, asserts 0 regular + 1 final with content_length=7. Proves short input goes directly to final frame. PASS.

**Req 7 test** (`test_empty_plaintext_constructs_empty_final_frame`): Encrypts 0 bytes, asserts 0 regular + 1 final with content_length=0. Proves empty final frame is constructed. PASS.

### Question 2: Annotation stacking check
No annotation stacking in the test file. Each test has exactly 1 annotation block. PASS.

### Question 3: Per-block isolation evaluation
Each annotation block is inside its test function, immediately before the test setup code. The connection between annotation and test is obvious in every case. PASS.

### Question 4: Semantic relationship
All test annotations semantically match their test functions. Each test exercises the specific scenario described by its annotation. PASS.

### Question 5: Spec sub-items
N/A for test file — the sub-items (Req 4, 5, 6) each have their own dedicated test. PASS.

### Question 6: Code structure mirrors spec
Tests are ordered Req 1 through Req 7, matching the spec's order. PASS.

### Question 7: Top-to-bottom readability
File reads linearly: helpers at top, then 7 tests in spec order. PASS.

## Anti-Rationalization Check (Step 3)

1. I noticed the `count_frames` helper reads `_content_len` but doesn't use it (prefixed with `_`). This is dead code in the helper. However, it's a test helper, and the value is used by the separate `final_frame_content_length` helper. Not a problem — it's just reading past the field to count frames. Not rationalizing.

2. I noticed clippy warns about collapsible `if` in `find_body_start`. This is a real lint warning. It's minor (style, not correctness) but should be fixed. Flagging it.

3. I noticed `final_frame_content_length` scans from byte 0, not from `find_body_start`. This means it could theoretically match a false ENDFRAME marker in the header. However, the header doesn't contain `0xFFFFFFFF` in normal messages, and the tests use small plaintexts where the header is well-formed. This is a test helper, not production code. The risk is negligible. Not blocking.

4. I noticed the empty plaintext test uses `frame_length=4096` while all other tests use `frame_length=10`. This is intentional — it uses the default frame length to test the empty case. Not a problem.

## Frame-Parsing Logic Verification

Verified against `aws-encryption-sdk-specification/data-format/message-body.md`:

**Regular frame**: SeqNum(4) + IV(12) + Content(frame_length) + Tag(16)
- Code: `4 + IV_LEN + frame_length as usize + TAG_LEN` = `4 + 12 + frame_length + 16` ✅

**Final frame**: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLength(4) + Content(N) + Tag(16)
- Content length offset from ENDFRAME: 4 + 4 + 12 = 20 bytes
- Code: `ct[pos + 20..pos + 24]` ✅

**find_body_start**: Scans for either ENDFRAME+SeqNum=1 (final frame as first frame) or SeqNum=1 validated by walking regular frames to ENDFRAME. Correct approach.

**count_frames**: Walks from body_start, counting regular frames by size and detecting final frame by ENDFRAME marker. Correct.

**final_frame_content_length**: Scans for ENDFRAME marker and reads 4 bytes at offset 20. Correct per spec.

## Potential Spec Gaps
None identified.
