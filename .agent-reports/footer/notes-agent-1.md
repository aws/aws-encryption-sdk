# Agent 1 Notes — footer.rs

## Spec-Aligned Structure Analysis

### 1. What is the spec section's logical flow?

**`data-format/message-footer.md`**: Fully covered. All 4 MUST requirements have both implementation and test annotations.

**`data-format/message.md#structure`**: Describes the overall message structure:
1. Message begins with header
2. Body follows header
3. If signing algorithm → footer MUST be present after body
4. If no signing algorithm → footer MUST NOT be present
5. If unrecognized signing algorithm → MUST raise error

Steps 3-5 are footer-relevant. All 5 have implementation annotations but ZERO test annotations.

### 2. Where will each requirement be fulfilled in code?

- "MUST also contain a message footer" → `write_footer` call in `encrypt.rs` step_construct_signature, `read_footer` call in `decrypt.rs`
- "MUST NOT contain a message footer" → the `None` match arm in `encrypt.rs` step_construct_signature
- "MUST raise an error" → the `_` wildcard match arm in `encrypt.rs` step_construct_signature

### 3. Sub-items under normative requirements?

The `message.md#structure` section has 5 bullet-level requirements, each a separate TOML `[[spec]]` entry. No nested sub-items.

### 4. Most likely structural mistake?

The implementer might try to add test annotations to `test_footer.rs` for requirements that are really about encrypt/decrypt behavior (message ordering, error on unrecognized algorithm). The test for "MUST NOT contain a footer" needs to verify the *absence* of footer bytes, not just that encryption succeeds. The test for "MUST raise an error" on unrecognized algorithm may be difficult since the current code uses a Rust enum that doesn't easily allow injecting an unrecognized variant.

## Potential Spec Gaps

### 1. Signature length overflow check
- **Code location**: `footer.rs` line 28-30 — `if signature.len() >= u16::MAX.into()`
- **Why it matters**: Correctness — prevents serializing a footer with a signature that can't be represented in 2 bytes
- **Suggested spec requirement**: "The signature length MUST be less than 2^16 - 1 bytes."

### 2. No explicit spec requirement for footer absence verification during decrypt
- **Code location**: `decrypt.rs` line 93 — `return Err("Data after message footer.".into())`
- **Why it matters**: Security — ensures no trailing data after footer
- **Suggested spec requirement**: "After deserializing the message footer, the Decrypt operation MUST verify that no additional bytes remain in the encrypted message."
