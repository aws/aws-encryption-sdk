# Pre-Implementation Reasoning: materials-decrypt

## 1. Logical steps in this spec section
1. Pre-CMM commitment policy check (parsed algorithm suite vs commitment policy)
2. CMM resolution (use input CMM, or construct default CMM from keyring)
3. Call CMM to obtain decryption materials
4. Extract algorithm suite from decryption materials
5. Derive data key from plaintext data key using KDF from algorithm suite
6. If suite supports key commitment, derive commit key and validate against header
7. Commit key equality check

## 2. Point of fulfillment for each requirement
- Req 1 (pre-CMM commitment check): `validate_commitment_policy_on_decrypt()` call in `step_get_decryption_materials`
- Req 2 (CMM used MUST be input CMM): `create_cmm_from_input()` returning the input CMM
- Req 3 (construct default CMM from keyring): `create_cmm_from_input()` constructing default CMM
- Req 4 (data key derived from plaintext data key): `key_derivation::derive_keys()` call
- Req 5 (algorithm suite from decryption materials): `let suite = &dec_mat.algorithm_suite`
- Req 6 (commit key derivation): `if v2_header_body::has_hkdf(...)` block with derive_keys
- Req 7 (commit key equality): `header::validate_suite_data()` call
- Req 8 (KDF algorithm from materials): `key_derivation::derive_keys()` using `suite` from materials

## 3. Sub-items
No sub-items requiring individual annotation — these are all standalone MUST requirements.

## 4. Test structure
All tests are round-trip: encrypt → decrypt. The test annotations go inside test functions.
- Test A: keyring round-trip (covers Req 3, 4, 5, 8)
- Test B: CMM round-trip (covers Req 2)
- Test C: pre-CMM commitment policy failure (covers Req 1)
- Test D: committing suite round-trip (covers Req 6, 7)

## 5. Existing similar code
- `test_get_decryption_materials.rs` — existing round-trip tests with `test_keyring()` and `round_trip()`
- `test_post_cmm_validation.rs` — existing tests with `encrypt_with()` and `decrypt_with()` helpers using specific suites/policies
