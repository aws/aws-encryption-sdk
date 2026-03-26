# Agent 1 Notes — key_derivation.rs

## Spec-Aligned Structure Analysis

### Q1: Spec section logical flow

The encrypt.md#get-the-encryption-materials key derivation section describes:
1. The data key used for encryption MUST be derived from the plaintext data key
2. The algorithm used MUST be the key derivation algorithm from the algorithm suite
3. If identity KDF → derived key = plaintext key
4. If HKDF → use the HKDF Encryption Key process

The decrypt.md#get-the-decryption-materials key derivation section describes:
1. The data key used for decryption MUST be derived from the plaintext data key
2. The algorithm suite used MUST be from the decryption materials
3. The key derivation algorithm MUST be from the algorithm suite
4. If identity KDF → derived key = plaintext key
(Note: decrypt spec does NOT have an explicit HKDF sub-item like encrypt does)

### Q2: Where each requirement is fulfilled in code

- `derive_key()` handles v1 (non-commitment) key derivation
- `expand_key_material()` handles v2 (commitment) key derivation
- `derive_keys()` dispatches based on `suite.message_version`
- Identity KDF → `DerivationAlgorithm::Identity` match arm in `derive_key()`
- HKDF → `DerivationAlgorithm::Hkdf` match arm in `derive_key()`

### Q3: Sub-items under normative requirements

The encrypt spec has two sub-items under the key derivation algorithm requirement:
- Identity KDF sub-item (annotated at line 75-77)
- HKDF sub-item (annotated at line 82-84)

The decrypt spec has one sub-item:
- Identity KDF sub-item (NOT annotated anywhere)

### Q4: Most likely structural mistake

The implementer might be tempted to add the decrypt identity KDF annotation
at the same `DerivationAlgorithm::Identity` match arm where the encrypt annotation already is.
This is actually correct — the same code fulfills both specs.
The risk is forgetting that the decrypt quote does NOT have the leading `- ` list marker
that the encrypt quote has.

## Potential Spec Gaps

### 1. Decrypt spec missing HKDF sub-item
- **Code location**: `derive_key()` HKDF match arm (line 85-100)
- **Behavior**: When decrypting with HKDF algorithm suites, the same HKDF derivation process is used
- **Why it matters**: Interop — the decrypt path must use the same HKDF process as encrypt
- **Suggested spec requirement**: The decrypt spec should include a sub-item parallel to encrypt:
  "If the key derivation algorithm is HKDF, the derivation process used MUST be the process described in HKDF Encryption Key."

### 2. V2 key derivation not referenced in key_derivation.rs annotations
- **Code location**: `expand_key_material()` (lines 108-152)
- **Behavior**: V2 algorithm suites use a different key derivation process with commitment
- **Why it matters**: The v2 key derivation is annotated in decrypt.rs but the expand_key_material function itself has no annotations
- **Suggested**: Consider whether expand_key_material should carry annotations from the encrypt spec about commit key derivation

## Coverage Summary for key_derivation.rs

### Implementation annotations present (encrypt-side):
1. ✅ `encrypt.md#get-the-encryption-materials` — "The algorithm used to derive a data key..."
2. ✅ `encrypt.md#get-the-encryption-materials` — "If the key derivation algorithm is the identity KDF..."
3. ✅ `encrypt.md#get-the-encryption-materials` — "If the key derivation algorithm is HKDF..."

### Implementation annotations missing (decrypt-side):
4. ❌ `decrypt.md#get-the-decryption-materials` — "If the key derivation algorithm is the identity KDF..."

### Test annotations missing:
5. ❌ No test annotations exist for ANY of the 3 encrypt-side requirements
6. ❌ No test annotations exist for the decrypt identity KDF requirement

### Priority assessment:
- Missing test annotations (priority 2) > Missing implementation annotations (priority 4)
- The 3 encrypt-side implementation annotations all lack corresponding test annotations
- Selecting the tightest cluster: all 3 encrypt-side key derivation requirements can be tested together in a single test file
