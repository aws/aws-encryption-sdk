# Agent 2 Notes — materials-decrypt Cycle 2

## Pre-Implementation Reasoning

### 1. What are the 3 gaps?

1. **Req 860** (ESDK suite not supported): `test_post_cmm_validation.rs` has a test but uses `specification/` prefix instead of `aws-encryption-sdk-specification/`. No test annotation visible to duvet.
2. **Req 861** (commitment policy on decrypt): Same issue — `test_post_cmm_validation.rs` uses `specification/` prefix.
3. **Req 865** (identity KDF): `test_post_cmm_validation.rs` has `test_identity_kdf_decrypt` but uses `specification/` prefix.

### 2. Fix plan

All 3 fixes are in `test_post_cmm_validation.rs`:
- Change `specification/client-apis/decrypt.md` → `aws-encryption-sdk-specification/client-apis/decrypt.md` in all decrypt test annotations
- Also fix the encrypt annotations (`specification/client-apis/encrypt.md` → `aws-encryption-sdk-specification/client-apis/encrypt.md`) for consistency

Wait — looking more carefully:
- Req 860: No test annotation exists for this. The `test_post_cmm_esdk_suite_decrypt` test doesn't exist in the file. Need to check if there's a test for ESDK suite validation.
- Actually, looking at the file, there's no `test_post_cmm_esdk_suite_decrypt` function. The work item says it should exist but it doesn't.

Let me re-read the file... The tests in `test_post_cmm_validation.rs` are:
1. `test_post_cmm_commitment_policy_encrypt` — encrypt spec, `specification/` prefix
2. `test_post_cmm_commitment_policy_decrypt` — decrypt spec, `specification/` prefix ← Req 861
3. `test_encrypt_non_committing_with_require_policy_fails` — encrypt spec, `specification/` prefix
4. `test_decrypt_non_committing_with_require_policy_fails` — decrypt spec, `specification/` prefix ← also Req 861
5. `test_identity_kdf_decrypt` — decrypt spec, `specification/` prefix ← Req 865

For Req 860 (ESDK suite not supported), there's NO existing test. Need to add one.

### 3. Actions

1. Fix path prefix `specification/` → `aws-encryption-sdk-specification/` in all annotations in `test_post_cmm_validation.rs`
2. Add a new test for Req 860 (ESDK suite not supported on decrypt)
3. Also fix the `materials.rs` implementation annotations from `specification/` → `aws-encryption-sdk-specification/`

### 4. Exact quotes from TOML

- Req 860: `If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)\nencrypt MUST yield an error.`
- Req 861: `If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)\nconfigured in the [client](client.md) decrypt MUST yield an error.`
- Req 865: `If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),\nthen the derived data key MUST be the same as the plaintext data key.`
