# Work Item: Add Missing `#behavior` Annotation for Signature Step Skip

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `behavior`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/decrypt.md#behavior`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If the message header does not contain an algorithm suite including a signature algorithm,
  the Decrypt operation MUST NOT perform this step.
  ```
- **Current State**: missing
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs`

The code that fulfills this requirement is in `step_verify_signature` (line ~425):

```rust
fn step_verify_signature(
    ciphertext: &mut dyn SafeRead,
    state: &DecryptState,
) -> Result<(), Error> {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# After deserializing the body, the Decrypt operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).
    if state.dec_mat.verification_key.is_some() {
        // ... signature verification ...
    }
    Ok(())
}
```

The `if state.dec_mat.verification_key.is_some()` guard means when there is no verification key (i.e., no signature algorithm), the function returns `Ok(())` without performing signature verification. This is the code that fulfills the "MUST NOT perform this step" requirement, but the annotation is missing.

The positive case ("MUST perform this step") is already annotated in `internal_decrypt` at lines 161-166:

```rust
    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 5 MUST be [Verify the signature](#verify-the-signature)
    //= specification/client-apis/decrypt.md#behavior
    //# - If the message header contains an algorithm suite including a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Decrypt operation MUST perform this step.
    step_verify_signature(ciphertext, &state)?;
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`

No test annotations exist for any `decrypt.md#behavior` requirements. The existing `test_encrypt_decrypt` test exercises the decrypt path but has no duvet annotations.

## Implementation Guidance

- Add the missing `#behavior` annotation at the `Ok(())` return after the `if` block in `step_verify_signature`, since that is the code path where signature verification is NOT performed.
- The annotation should be placed just before the `Ok(())` at the end of `step_verify_signature`, inside an `else` block or after the `if` block closes, to make it clear this is the "no signature" path.
- Follow the existing pattern in `decrypt.rs` where the `#behavior` section annotations are placed at the call site in `internal_decrypt` (lines 133-166). However, this specific requirement is about NOT performing the step, which is fulfilled by the guard in `step_verify_signature` itself.

### Spec-Aligned Structure

The spec describes the signature step flow:
1. Step 5 MUST be Verify the signature → annotated at `step_verify_signature(...)` call (line 161)
2. If signature algorithm present, MUST perform → annotated at call site (line 163-166)
3. If NO signature algorithm, MUST NOT perform → **MISSING** — annotate at the implicit skip in `step_verify_signature`

The most natural placement is:
- Add annotation before the `Ok(())` at the end of `step_verify_signature`, since when `verification_key.is_none()`, the function skips the `if` block and returns `Ok(())` — that IS the "MUST NOT perform" behavior.

Alternatively, restructure to make the skip explicit:

```rust
fn step_verify_signature(
    ciphertext: &mut dyn SafeRead,
    state: &DecryptState,
) -> Result<(), Error> {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    if state.dec_mat.verification_key.is_some() {
        // ... existing verification code ...
    } else {
        //= specification/client-apis/decrypt.md#behavior
        //# - If the message header does not contain an algorithm suite including a signature algorithm,
        //# the Decrypt operation MUST NOT perform this step.
        return Ok(());
    }
    Ok(())
}
```

- Reference pattern: The existing `step_verify_signature` function at line ~425 of `decrypt.rs`
- No dependencies on other unfinished work

### Most Likely Structural Mistake

The implementer may be tempted to place the annotation at the call site in `internal_decrypt` (next to the existing positive-case annotation at line 163). However, the "MUST NOT perform" requirement is fulfilled by the guard inside `step_verify_signature`, not at the call site. The annotation must be at the point where the skip actually happens.

## Targeted Tests
- `test_encrypt_decrypt` — exercises the full decrypt path (currently uses signing algorithm suites by default via KMS keyring)
- Test vectors in `test_vectors/do_decrypt.rs` — exercises decrypt with various algorithm suites including non-signing ones

## Success Criteria
```bash
cargo test test_encrypt_decrypt
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows the `#behavior` section requirement "MUST NOT perform this step" is covered
- [ ] The annotation uses `type=implementation` (default, omit type line)
- [ ] The annotation quote exactly matches the TOML quote
