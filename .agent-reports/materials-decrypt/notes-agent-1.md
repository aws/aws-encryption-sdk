# Discovery Notes — materials-decrypt

## Spec Section Logical Flow

The `### Get the decryption materials` section describes this flow:

1. **Pre-CMM commitment policy check**: Validate parsed algorithm suite ID against commitment policy before calling CMM
2. **CMM resolution**: Use input CMM if supplied, else construct default CMM from keyring
3. **CMM call construction**: Build DecryptMaterialsInput with 5 fields (EC, suite ID, EDKs, reproduced EC, commitment policy)
4. **Post-CMM data key derivation**: Derive data key from plaintext data key in materials
5. **Post-CMM algorithm suite usage**: Use algorithm suite from materials for all decryption
6. **Post-CMM ESDK support check**: Verify algorithm suite is ESDK-supported
7. **Post-CMM commitment policy check**: Validate materials' algorithm suite against commitment policy
8. **Key commitment verification**: If suite supports commitment, derive commit key and compare to header
9. **KDF algorithm usage**: Use the KDF from the materials' algorithm suite
10. **Identity KDF special case**: If identity KDF, derived key = plaintext key

## Where Each Requirement Is Fulfilled

| Requirement | Code Construct | File |
|---|---|---|
| Pre-CMM commitment check | `validate_commitment_policy_on_decrypt()` call | `decrypt.rs:step_get_decryption_materials` |
| CMM resolution (input CMM / default CMM) | `create_cmm_from_input()` match arms | `materials.rs:create_cmm_from_input` |
| CMM call construction | `DecryptMaterialsInput` field assignments | `materials.rs:get_modern_decryption_materials` and `get_legacy_decryption_materials` |
| Data key derivation | `key_derivation::derive_keys()` call | `decrypt.rs:step_get_decryption_materials` |
| Algorithm suite from materials | `let suite = &dec_mat.algorithm_suite` | `decrypt.rs:step_get_decryption_materials` |
| ESDK support check | `type=implication` annotation (CMM enforces) | `materials.rs` |
| Post-CMM commitment check | `validate_commitment_policy_on_decrypt()` | `materials.rs:get_modern_decryption_materials` |
| Key commitment derivation | `v2_header_body::has_hkdf` + `validate_suite_data` | `decrypt.rs:step_get_decryption_materials` |
| Commit key equality | `header::validate_suite_data()` | `decrypt.rs:step_get_decryption_materials` |
| KDF algorithm usage | `derive_keys()` uses `suite.kdf` | `key_derivation.rs:derive_keys` |
| Identity KDF | `DerivationAlgorithm::Identity` match arm | `key_derivation.rs:derive_key` |

## Most Likely Structural Mistake

The implementer may be tempted to add test annotations in `materials.rs` test code.
However, many of the requirements that need tests are fulfilled in `decrypt.rs:step_get_decryption_materials`,
not in `materials.rs`. The test annotations should reference the spec section
`specification/client-apis/decrypt.md#get-the-decryption-materials` regardless of which code file
implements the behavior.

Also: the "CMM used MUST be the input CMM" and "construct a default CMM" requirements
are annotated in `decrypt.rs` (line ~253-258) but the actual code is in `materials.rs:create_cmm_from_input`.
The annotations are split across both files. Tests should cover the decrypt.rs orchestration.

## Potential Spec Gaps

### 1. Algorithm suite mismatch between header and materials
- **Code location**: `decrypt.rs:step_get_decryption_materials` lines checking `suite != header_body.algorithm_suite()`
- **Behavior**: The code checks that the algorithm suite in the decryption materials matches the one parsed from the header, and returns an error if they differ.
- **Why it matters**: Correctness — prevents using wrong algorithm suite for decryption
- **Suggested spec requirement**: "The algorithm suite in the decryption materials MUST match the algorithm suite parsed from the message header."

### 2. Plaintext data key presence validation
- **Code location**: `materials.rs:get_modern_decryption_materials` calls `decryption_materials_with_plaintext_data_key()`
- **Behavior**: Validates that the decryption materials contain a plaintext data key before proceeding
- **Why it matters**: Correctness — prevents null pointer / missing key errors during key derivation
- **Suggested spec requirement**: "The decryption materials MUST contain a plaintext data key."

### 3. Encryption context serializability check
- **Code location**: `materials.rs:get_decryption_materials` checks `is_esdk_encryption_context()`
- **Behavior**: Validates that the encryption context returned by the CMM is serializable for ESDK
- **Why it matters**: Interop — ensures encryption context can be properly serialized in the message format
- **Suggested spec requirement**: "The encryption context in the decryption materials MUST be serializable according to the ESDK message format."
