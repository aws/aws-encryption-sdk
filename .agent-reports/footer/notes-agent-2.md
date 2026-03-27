# Agent 2 Notes — footer (message.md#structure test annotations)

## Step 1.5: Pre-Implementation Reasoning

### 1. Logical steps in this spec section

The `message.md#structure` section describes 5 requirements about message serialization order and footer presence:

1. Message MUST begin with Message Header
2. Message Body MUST follow the Message Header
3. Signing suite → message MUST contain footer after body
4. Non-signing suite → message MUST NOT contain footer
5. Unrecognized signature algorithm → MUST raise error

### 2. Point of fulfillment for each requirement (test annotations)

- Req 1 ("begin with Message Header"): Test verifies first byte of ciphertext is the version byte (0x02 for V2). Fulfilled at the assertion checking the first byte.
- Req 2 ("Body MUST follow Header"): Test verifies round-trip decrypt succeeds, proving body follows header in correct order. Fulfilled at the decrypt assertion.
- Req 3 ("MUST also contain footer"): Already tested in `test_footer_present_with_signing_suite` — add a second annotation block for `message.md#structure`. Fulfilled at the assertion that signing ciphertext is longer.
- Req 4 ("MUST NOT contain footer"): Test encrypts with non-signing suite and verifies no footer bytes trail the body. Fulfilled at the assertion checking ciphertext structure.
- Req 5 ("unrecognized signature algorithm MUST raise error"): This is difficult to test from the public API because `SignatureAlgorithm` is a Rust enum. The `_` wildcard arm is unreachable from normal usage. Will use `type=implication` with reason explaining the exhaustive match ensures this.

### 3. Sub-items?

No sub-items — each requirement is a standalone MUST statement.

### 4. Reviewer readability

Tests will be added to `test_footer.rs` for requirements 1-4 (footer-related and message-structure).
- Req 1 & 2 are about message ordering — they fit in `test_footer.rs` since the test file already tests message structure.
- Req 3: Add `message.md#structure` annotation to existing `test_footer_present_with_signing_suite`.
- Req 4: Add a new test `test_no_footer_without_signing_suite`.
- Req 5: Use `type=implication` — no test needed (Rust enum exhaustiveness).

Actually, re-reading the work item: Requirements 4 and 5 are about header/body ordering. The work item says "Agent 2 should evaluate whether to add them to test_footer.rs or defer." Since these are about overall message structure and the test file already has the helpers, I'll add them to `test_footer.rs` — it's the most natural place since it already tests message structure properties.

### 5. Existing similar code

`test_footer.rs` — existing test patterns with `encrypt_with_signing_suite`, `encrypt_without_signing_suite`, `find_footer_offset`.
`test_header_structure.rs` — similar pattern of round-trip tests for structural properties.
