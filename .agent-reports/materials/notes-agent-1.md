# Agent 1 Notes — materials.rs Analysis

## Spec-Aligned Structure Analysis

### Q1: What is the spec section's logical flow?

For `decrypt.md#get-the-decryption-materials`:
1. Validate algorithm suite against commitment policy → error check
2. Obtain decryption materials by calling Decrypt Materials on CMM
3. Construct the CMM call with: Algorithm Suite ID, Encrypted Data Keys, Encryption Context, Reproduced Encryption Context, Commitment Policy
4. Use the algorithm suite from returned materials for all subsequent decryption
5. Derive data key from plaintext data key
6. Validate key commitment if supported

### Q2: Where will each requirement be fulfilled in code?

In `get_modern_decryption_materials`:
- "This operation MUST obtain this set of decryption materials..." → `cmm.decrypt_materials(&input).await?`
- Sub-items (Algorithm Suite ID, EDKs, EC, Reproduced EC, Commitment Policy) → `input.field = value` assignments
- Commitment policy validation → `validate_commitment_policy_on_decrypt(...)` call
- Plaintext data key validation → `decryption_materials_with_plaintext_data_key(...)` call

### Q3: Does the spec contain sub-items?

Yes. The "MUST be constructed as follows:" requirement has 5 sub-items:
- Algorithm Suite ID
- Encrypted Data Keys
- Encryption Context
- Reproduced Encryption Context
- Commitment Policy

Each maps to a distinct `input.field = value` assignment.

### Q4: What is the most likely structural mistake?

The implementer may be tempted to:
1. Place all sub-item annotations at the `DecryptMaterialsInput::default()` line instead of at each individual field assignment
2. Use the paraphrased text from the existing orphaned comments instead of exact TOML quotes
3. Miss the Commitment Policy sub-item annotation (currently has no orphaned comment at all)

## Potential Spec Gaps

### Missing commitment policy validation annotation in modern path

- **Code location**: `get_modern_decryption_materials` lines 194-199 — calls `validate_commitment_policy_on_decrypt`
- **Why it matters**: The legacy path annotates this with `decrypt.md#get-the-decryption-materials` sub-items, but the modern path does the same validation without any annotation
- **Suggested spec requirement**: Already covered by existing spec — just needs annotation

### No annotation for `decryption_materials_with_plaintext_data_key` call

- **Code location**: `get_modern_decryption_materials` line 201
- **Why it matters**: This validates that the returned materials contain a plaintext data key, which is a correctness requirement
- **Suggested spec requirement**: This may be covered by framework/structures.md requirements rather than decrypt.md

## Discovery Method

- Shell commands were unavailable; analysis performed via file reading and grep
- TOML content verified by reading actual compliance TOML files
- Source file paths verified by reading actual files
- No `make duvet` output available; gap analysis performed by comparing TOML requirements against annotations found in source
