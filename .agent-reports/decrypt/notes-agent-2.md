# Pre-Implementation Reasoning

## 1. What are the logical steps in this spec section?

This is a single missing annotation for the `#behavior` section of `decrypt.md`.
The requirement: "If the message header does not contain an algorithm suite including a signature algorithm, the Decrypt operation MUST NOT perform this step."

The code already fulfills this — `step_verify_signature` has an `if state.dec_mat.verification_key.is_some()` guard that skips signature verification when there's no verification key (i.e., no signature algorithm). The annotation is simply missing.

## 2. For each requirement, where is the point of fulfillment?

- "MUST NOT perform this step" → fulfilled at the implicit else path in `step_verify_signature` where `verification_key.is_none()` causes the `if` block to be skipped, and `Ok(())` is returned.

The best placement: restructure to add an explicit `else` block with the annotation, making the skip path visible.

## 3. Does the spec have sub-items?

No sub-items for this requirement.

## 4. Can a reviewer read this top-to-bottom?

Yes. The annotation will be in the `else` branch of the existing `if state.dec_mat.verification_key.is_some()` check, making it immediately obvious that this is the "no signature algorithm" path.

## 5. What existing code does something similar?

The positive case annotation is at the call site in `internal_decrypt` (lines 161-166). The negative case should be at the guard inside `step_verify_signature` where the skip actually happens.

The pattern in `test_construct_a_frame.rs` shows how to write test annotations with round-trip tests using raw AES keyrings (no KMS needed). I'll use `AlgAes256GcmHkdfSha512CommitKey` (non-signing) to test the skip path.
