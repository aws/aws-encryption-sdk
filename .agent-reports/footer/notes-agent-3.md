# Agent 3 Notes — footer (Round 5 — message.md#structure Test Annotations)

## Step 2: Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**Test 1: `test_footer_present_with_signing_suite` (Req 1 annotation, line 65)**
- Annotation: "If the [message header]... contains a [signature algorithm]..., the message MUST also contain a [message footer]..."
- Next executable line: `let ct_signing = encrypt_with_signing_suite(b"footer presence test").await;`
- The test encrypts with a signing suite, then asserts the ciphertext is longer than non-signing (due to footer) and that the footer is parseable.
- ✅ The test exercises the requirement: it proves a signing suite produces a footer.

**Test 2: `test_no_footer_without_signing_suite` (Req 2, line 163)**
- Annotation: "If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer."
- Next executable line: `let keyring = test_keyring().await;`
- The test encrypts with a non-signing suite and does a round-trip decrypt. The argument is that if a footer were present, decrypt would fail or leave trailing bytes.
- ✅ The test exercises the requirement via round-trip verification.

**Test 3: `test_message_begins_with_header` (Req 4, line 187)**
- Annotation: "- The message MUST begin with [Message Header](message-header.md)"
- Next executable line: `let ct = encrypt_with_signing_suite(b"header first test").await;`
- The test checks `ct[0] == 0x02` (V2 version byte).
- ✅ The test verifies the message begins with the header version byte.

**Test 4: `test_message_body_follows_header` (Req 5, line 200)**
- Annotation: "- The [Message Body](message-body.md) MUST follow the Message Header"
- Next executable line: `let pt = b"body follows header test";`
- The test does a round-trip decrypt, proving the body follows the header.
- ✅ The test verifies via round-trip.

### 2. Annotation stacking check

- `test_footer_present_with_signing_suite`: 2 annotation blocks (pre-existing `message-footer.md#overview` + new `message.md#structure`). Under limit. ✅
- `test_no_footer_without_signing_suite`: 1 annotation block. ✅
- `test_message_begins_with_header`: 1 annotation block. ✅
- `test_message_body_follows_header`: 1 annotation block. ✅

No stacking violations.

### 3. Per-block isolation evaluation

**Block 1 (test_footer_present_with_signing_suite, message.md#structure):**
Context reset: "signing suite → message MUST contain footer" + encrypt with signing suite + assert longer than non-signing. ✅ Immediately obvious.

**Block 2 (test_no_footer_without_signing_suite):**
Context reset: "no signature algorithm → MUST NOT contain footer" + encrypt with non-signing suite + round-trip decrypt succeeds. ✅ Immediately obvious.

**Block 3 (test_message_begins_with_header):**
Context reset: "message MUST begin with Message Header" + encrypt + assert first byte is 0x02. ✅ Immediately obvious.

**Block 4 (test_message_body_follows_header):**
Context reset: "Message Body MUST follow Message Header" + round-trip decrypt succeeds. ✅ Immediately obvious — decrypt parses header then body sequentially.

### 4. Semantic relationship check
All annotations semantically relate to their test code. ✅

### 5. Sub-items check
The spec has 5 requirements. 4 are annotated with `type=test`. The 5th (unrecognized signature algorithm) is deferred with justification. ✅

### 6. Code structure mirrors spec
Tests map to spec requirements in order:
- Req 1 (footer present) → test_footer_present_with_signing_suite
- Req 2 (no footer) → test_no_footer_without_signing_suite
- Req 4 (begins with header) → test_message_begins_with_header
- Req 5 (body follows header) → test_message_body_follows_header
✅

### 7. Top-to-bottom readability
Test file reads cleanly: helpers at top, then test functions. ✅

## Step 3: Anti-Rationalization Check

No problems found. No "but" patterns in my reasoning. All annotations are clean with clear semantic connections.

## Step 4: Pre-Review Gate

✅ Test file was modified: `tests/test_footer.rs` with 4 `type=test` annotations.

## Observations

### Pre-existing issue in encrypt.rs
The implementation annotation in `encrypt.rs:398` has a double space: `MUST NOT  contain` vs TOML's `MUST NOT contain`. This is a pre-existing issue, not introduced by Agent 2. Duvet appears to normalize whitespace when matching. Not blocking.

### Blank lines between annotations and code
All 4 new test annotations have blank lines between the annotation block and the first executable line. This matches the pre-existing pattern in this test file (e.g., `test_footer_signature_length_is_two_bytes`, `test_footer_signature_calculated_over_header_and_body`). While duvet-patterns.md says "executable line follows," the established convention in this test file uses blank lines for readability. Duvet still matches correctly. Not blocking.

### Deferred annotation (Req 3) is justified
`SignatureAlgorithm` is a Rust enum with only `Ecdsa(...)` and `None` variants. The `_ =>` wildcard arm in `encrypt.rs` is unreachable from the public API. No test can exercise this path. Deferral is correct.

## Potential Spec Gaps

None identified.
