# Agent 1 Notes — materials-decrypt

## Discovery Summary

After commit 695713fd (8 new test annotations), re-ran duvet and found 3 remaining gaps in `decrypt.md#get-the-decryption-materials`:
- Requirement 860: ESDK suite support check (post-CMM)
- Requirement 861: Commitment policy check (post-CMM)
- Requirement 865: Identity KDF passthrough

## Root Cause Analysis

The primary issue is a **path prefix mismatch**. Annotations in `src/materials.rs` and `tests/test_post_cmm_validation.rs` use the `specification/` symlink prefix, while duvet extracts requirements from `aws-encryption-sdk-specification/`. Duvet treats these as separate specification entries and does not cross-match annotations between them.

This means:
- Implementation annotations using `specification/` prefix → matched to a spec entry with 0 requirements
- Test annotations using `aws-encryption-sdk-specification/` prefix → matched to the actual requirements

The fix is straightforward: change the path prefix in the affected test annotations from `specification/` to `aws-encryption-sdk-specification/`, and add a new test annotation for requirement 865.

## Spec-Aligned Structure Analysis

1. **Logical flow**: The "Get the decryption materials" section follows this order:
   - Pre-CMM: Check parsed algorithm suite against commitment policy
   - Obtain materials: Call CMM's Decrypt Materials with constructed input
   - Post-CMM: Validate returned algorithm suite (ESDK support + commitment policy)
   - Key derivation: Derive data key from plaintext data key (including identity KDF case)
   - Key commitment: Verify commit key if applicable

2. **Code construct mapping**:
   - Req 860 → `validate_commitment_policy_on_decrypt` call (materials.rs line 218)
   - Req 861 → same `validate_commitment_policy_on_decrypt` call
   - Req 865 → `derive_data_key` function in key_derivation.rs (identity KDF branch)

3. **Sub-items**: None — these are standalone requirements, not list items.

4. **Most likely structural mistake**: Fixing only the `materials.rs` annotations without also fixing the test file annotations, or forgetting that requirement 865 needs a decrypt-specific test annotation (the encrypt-side annotation exists but doesn't cover the decrypt spec section).

## Potential Spec Gaps

1. **Requirement 860 says "encrypt MUST yield an error"** in the decrypt section. This is clearly a spec typo — it should say "decrypt MUST yield an error". The code correctly errors on decrypt. This is a known spec issue and does not affect the annotation (the exact quote must be used regardless).

2. **No spec requirement for validating plaintext data key length on decrypt**. The code in `materials.rs` calls `decryption_materials_with_plaintext_data_key` which validates the key exists, but there's no explicit spec requirement for checking the key length matches the algorithm suite's key derivation input length on the decrypt path (unlike encrypt, which has explicit length checks). This is a potential spec gap — the code does enforce this through the CMM interface contract, but it's not explicitly stated in the decrypt section.
