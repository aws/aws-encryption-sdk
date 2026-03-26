# Work Item: Fix Missing Duvet Annotations in `get_modern_decryption_materials`

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `get-the-decryption-materials`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
  by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
  ```
- **Current State**: missing (annotated in `get_legacy_decryption_materials` but not in `get_modern_decryption_materials`)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
  MUST be constructed as follows:
  ```
- **Current State**: missing (annotated in legacy path only)
- **Sub-items** (each is a MUST-level requirement in the TOML):
  ```toml
  - Algorithm Suite ID: This MUST be the parsed
  [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
  from the message header.
  ```
  ```toml
  - Commitment Policy: This MUST be the commitment policy configured on the client.
  ```
  ```toml
  - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header#encrypted-data-keys)
  from the message header.
  ```
  ```toml
  - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
  from the message header.
  ```
  ```toml
  - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
  ```

### Requirement 3 (from `decrypt.md#cryptographic-materials-manager`)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
  ```
- **Current State**: missing (annotated in legacy path only)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`

The `get_modern_decryption_materials` function (lines 167-203) has orphaned `//#` content lines that are NOT valid duvet annotations — they lack `//=` target headers and use paraphrased text instead of exact TOML quotes:

```rust
pub(crate) async fn get_modern_decryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let mut input = DecryptMaterialsInput::default();
    //#*  Algorithm Suite ID: This is the parsed algorithm suite ID   // <-- ORPHANED, no //= target
    //#   (../data-format/message-header.md#algorithm-suite-id) from the
    //#   message header.
    input.algorithm_suite_id = algorithm_suite_id;
    input.commitment_policy = aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //#*  Encrypted Data Keys: This is the parsed encrypted data keys  // <-- ORPHANED
    //#   (../data-format/message-header#encrypted-data-keys) from the
    //#   message header.
    input.encrypted_data_keys = header_body.encrypted_data_keys().into();
    //#*  Encryption Context: This is the parsed encryption context    // <-- ORPHANED
    //#   (../data-format/message-header.md#aad) from the message header.
    input.encryption_context = encryption_context;
    //#* Reproduced Encryption Context: This is the                    // <-- ORPHANED
    //# [input](#input) encryption context.
    input
        .reproduced_encryption_context
        .clone_from(reproduced_encryption_context);
    let materials = cmm.decrypt_materials(&input).await?;
    aws_mpl_legacy::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_legacy::materials::decryption_materials_with_plaintext_data_key(&materials)?;
    Ok(materials)
}
```

### Reference: Legacy path with correct annotations (same file, lines 456-525)

```rust
pub(crate) async fn get_legacy_decryption_materials(...) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/decrypt.md#cryptographic-materials-manager
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let output = cmm
        .decrypt_materials()
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Algorithm Suite ID: This MUST be the parsed
        //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
        //# from the message header.
        .algorithm_suite_id(convert_alg(algorithm_suite_id))
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Commitment Policy: This MUST be the commitment policy configured on the client.
        .commitment_policy(convert_commit(commitment_policy))
        // ... etc
```

### Test File: NEW FILE NEEDED: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`

No test annotations exist for `decrypt.md#get-the-decryption-materials` anywhere in the test suite.

## Implementation Guidance

1. **Replace orphaned content lines** in `get_modern_decryption_materials` with proper duvet annotations using exact TOML quotes. Each orphaned `//#` block must be preceded by a `//= specification/client-apis/decrypt.md#get-the-decryption-materials` target line.

2. **Add the missing parent annotations** before the `cmm.decrypt_materials(&input).await?` call:
   - `//= specification/client-apis/decrypt.md#get-the-decryption-materials` + "This operation MUST obtain..."
   - `//= specification/client-apis/decrypt.md#cryptographic-materials-manager` + "This CMM MUST obtain..."
   - `//= specification/client-apis/decrypt.md#get-the-decryption-materials` + "The call to the CMM's..."

3. **Add the missing Commitment Policy sub-item annotation** before `input.commitment_policy = ...` — the orphaned comments skipped this one entirely.

4. **Follow the pattern** in `get_legacy_decryption_materials` (same file, lines 462-495) — it has the correct annotation structure for the same requirements.

5. **Do not modify `get_legacy_decryption_materials`** — its annotations are already correct.

### Spec-Aligned Structure

The spec describes this flow for the CMM call construction:
1. Obtain decryption materials by calling Decrypt Materials on CMM → annotate at `cmm.decrypt_materials(&input).await?`
2. Algorithm Suite ID sub-item → annotate at `input.algorithm_suite_id = algorithm_suite_id;`
3. Commitment Policy sub-item → annotate at `input.commitment_policy = ...;`
4. Encrypted Data Keys sub-item → annotate at `input.encrypted_data_keys = ...;`
5. Encryption Context sub-item → annotate at `input.encryption_context = encryption_context;`
6. Reproduced Encryption Context sub-item → annotate at `input.reproduced_encryption_context.clone_from(...);`

Sub-items to annotate individually:
- "- Algorithm Suite ID: This MUST be the parsed..." → at `input.algorithm_suite_id = algorithm_suite_id;`
- "- Commitment Policy: This MUST be the commitment policy..." → at `input.commitment_policy = ...;`
- "- Encrypted Data Keys: This MUST be the parsed..." → at `input.encrypted_data_keys = ...;`
- "- Encryption Context: This MUST be the parsed..." → at `input.encryption_context = encryption_context;`
- "- Reproduced Encryption Context: This MUST be the..." → at `input.reproduced_encryption_context.clone_from(...);`

**Structural note**: The modern path uses struct field assignment (`input.field = value`) rather than builder chaining (`.field(value)`). Place each sub-item annotation immediately before its corresponding `input.field = value;` line. The parent annotations ("This operation MUST obtain..." and "The call to the CMM's... MUST be constructed as follows:") should go before the `let materials = cmm.decrypt_materials(&input).await?;` line, since that is the point of fulfillment for the CMM call.

## Targeted Tests

No existing tests cover `decrypt.md#get-the-decryption-materials` with `type=test` annotations. Integration tests in `tests/test_encrypt_decrypt.rs` exercise the decrypt path end-to-end but lack duvet annotations for this specific section.

A new test file or additions to an existing test file should include `type=test` annotations for at minimum:
- The CMM call construction requirement
- Each sub-item (Algorithm Suite ID, Commitment Policy, Encrypted Data Keys, Encryption Context, Reproduced Encryption Context)

## Success Criteria
```bash
cargo test --package esdk
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `decrypt.md#get-the-decryption-materials` sub-items annotated in `get_modern_decryption_materials`
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] Orphaned `//#` lines are replaced with proper `//= ... //# ...` annotation blocks
- [ ] All implementations have corresponding `type=test`
