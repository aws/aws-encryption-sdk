# Discovery Notes — Cycle 2 Re-discovery for decrypt-body

## Context
Cycle 1 (commit 678f8992) added 20 implementation/implication annotations in body.rs and 11 new test functions with 22 type=test annotations. This cycle 2 re-discovery checks for remaining gaps.

## Analysis Summary

### TOML Requirements Count
The `decrypt-the-message-body.toml` file contains 33 `[[spec]]` entries:
- 27 MUST-level requirements
- 6 SHOULD-level requirements

### Coverage Status
- **Implementation annotations**: All 33 requirements have implementation or implication annotations across `src/decrypt.rs` and `src/message/body.rs`
- **Test annotations**: 32 of 33 requirements have `type=test` annotations in `tests/test_decrypt_the_message_body.rs`
- **Missing test**: 1 requirement is missing a test annotation

### The Missing Test Annotation
The requirement:
```
Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
successfully completes.
```

Has implementation annotations at:
- `src/decrypt.rs` line 215 (in `internal_decrypt`, after `step_verify_signature`)
- `src/decrypt.rs` line 452 (in `step_decrypt_body`, setting `fail_if_multi_frame`)

But has NO `type=test` annotation in any test file.

## Spec-Aligned Structure Analysis

### Q1: Logical flow of the spec section
1. Deserialize body bytes after header → content type determines framed vs non-framed
2. For framed: read first 4 bytes to determine regular vs final frame
3. Deserialize frame fields (seq num, IV, content length, encrypted content, auth tag)
4. Construct AAD (message ID, body AAD content, sequence number, content length)
5. Decrypt with AES-GCM (key, IV, ciphertext, tag, AAD)
6. Handle failure → halt immediately, no unauthenticated plaintext
7. Streaming release rules → regular frames released after tag verification; final frame/unframed held until signature verification

### Q2: Where is the missing requirement fulfilled?
The requirement is fulfilled by the combination of:
- `internal_decrypt` calling `step_decrypt_body` which returns `last_frame` (the final frame's plaintext)
- `internal_decrypt` then calling `step_verify_signature` BEFORE writing `last_frame` to the output
- The `write_bytes(plaintext, &last_frame)` call at line 213 only happens after signature verification succeeds

### Q3: Sub-items
No sub-items for this requirement.

### Q4: Most likely structural mistake
The implementer might be tempted to add the test annotation to an existing test that already covers streaming behavior (e.g., `test_decrypt_streaming_releases_regular_frames`). However, that test covers the SHOULD for regular frames being released early, not the MUST for final frame/unframed being held back. A distinct test is needed.

## Potential Spec Gaps
None identified in this cycle. The code behavior aligns well with the spec for this section.
