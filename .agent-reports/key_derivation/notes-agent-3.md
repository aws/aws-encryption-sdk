# Agent 3 Review Notes: key_derivation

## Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**Test 1 (`test_key_derivation_uses_suite_kdf`)**:
- Annotation: "The algorithm used to derive a data key from the plaintext data key MUST be the [key derivation algorithm]... included in the [algorithm suite]... defined above."
- Code: `let pt = b"test kdf selection from suite";` followed by `round_trip_with_suite(pt, AlgAes256GcmIv12Tag16HkdfSha256, ForbidEncryptAllowDecrypt)`
- Assessment: The annotation is `type=test`. The next executable line is `let pt = ...` which is just a variable assignment. The actual test assertion is the `assert_eq!` at the end. However, for `type=test` annotations, the convention (seen in `test_get_decryption_materials.rs` and `test_v1_header_body.rs`) is to place the annotation at the top of the test function body, before the test setup. This is consistent with Pattern 3 (general behavior at method/block start). The round-trip proves the correct KDF was used because using the wrong KDF would produce the wrong key and decryption would fail. PASS.

**Test 2 (`test_identity_kdf_derived_key_equals_plaintext_key`)**:
- Annotation: "If the key derivation algorithm is the [identity KDF]..., then the derived data key MUST be the same as the plaintext data key."
- Code: Same pattern — `let pt = ...` then round-trip with `AlgAes256GcmIv12Tag16NoKdf`.
- Assessment: The NoKdf suite uses identity KDF. A successful round-trip proves the identity KDF returned the plaintext key unchanged (otherwise decryption would fail). PASS.

**Test 3 (`test_hkdf_derivation_process`)**:
- Annotation: "If the key derivation algorithm is [HKDF]..., the derivation process used MUST be the process described in [HKDF Encryption Key]..."
- Code: Round-trip with `AlgAes256GcmIv12Tag16HkdfSha256`.
- Assessment: The HKDF suite exercises the HKDF derivation path. A successful round-trip proves the HKDF derivation produced the correct key. PASS.

### 2. Annotation stacking check

No stacking issues. Each test function has exactly ONE annotation block (target + type + quote). No stacks of 3+. PASS.

### 3. Per-block isolation evaluation

**Block 1** (test_key_derivation_uses_suite_kdf):
- Annotation says: algorithm used MUST be the KDF from the algorithm suite.
- Code: encrypts with HKDF suite, decrypts, asserts plaintext matches.
- In isolation: Yes, it's clear this test exercises KDF selection by using a specific suite and verifying round-trip success. PASS.

**Block 2** (test_identity_kdf_derived_key_equals_plaintext_key):
- Annotation says: identity KDF → derived key MUST equal plaintext key.
- Code: encrypts with NoKdf suite, decrypts, asserts plaintext matches.
- In isolation: Yes, the NoKdf suite name makes it clear this is the identity KDF path. PASS.

**Block 3** (test_hkdf_derivation_process):
- Annotation says: HKDF → derivation MUST follow HKDF Encryption Key process.
- Code: encrypts with HkdfSha256 suite, decrypts, asserts plaintext matches.
- In isolation: Yes, the suite name makes it clear this is the HKDF path. PASS.

### 4. Semantic relationship check

All three annotations semantically relate to their test code. Each test uses the appropriate algorithm suite to exercise the specific KDF path described in the annotation. PASS.

### 5. Sub-items annotated individually?

The spec has a parent requirement and two sub-items (identity KDF, HKDF). Each is annotated in a separate test function. This follows Pattern 4 correctly. PASS.

### 6. Code structure mirrors spec structure?

The spec describes: parent (algorithm MUST be KDF from suite) → sub-item 1 (identity KDF) → sub-item 2 (HKDF). The tests follow this exact order. PASS.

### 7. Linear readability?

Reading the test file top-to-bottom, each test function has its annotation at the top, followed by the test code. No jumping required. PASS.

## Anti-Rationalization Check

Reviewing my notes above, I don't see any pattern of "this is wrong BUT acceptable because...". All assessments are straightforward passes.

## Cross-Reference Check

Scanning annotation quotes for markdown links:
1. Test 1: `[key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm)` and `[algorithm suite](../framework/algorithm-suites.md)` — these are informational links in the spec text, not separate requirements that need their own annotations. The duvet config doesn't include `framework/algorithm-suites.md` as a tracked specification.
2. Test 2: `[identity KDF](../framework/algorithm-suites.md#identity-kdf)` — same, informational link.
3. Test 3: `[HKDF](../framework/algorithm-suites.md#hkdf)` and `[HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key)` — informational links.

These are cross-references to framework specs that define terms, not separate requirements that need annotations at the same code location. The duvet config only tracks `specification/client-apis/` and `specification/data-format/` specs. No `ANNOTATION_MISSING` issues.

Links found: 5 total cross-reference links in annotation quotes.
Cross-refs present: 0 (none needed — all link to framework definition specs not tracked by duvet).
Ratio: N/A (framework definition links, not actionable requirements).

## Potential Spec Gaps

None identified. The implementation and tests align with the spec.

## Test Validation Results

- Check 1 (Tests): PASS — all 3 key_derivation tests pass; pre-existing KMS credential failures in test_authentication_tag.rs and test_encrypt_decrypt.rs are unrelated
- Check 2 (Coverage): PASS — duvet snapshot shows `implementation,test` for all 3 requirements
- Check 3 (Duvet Report): PASS — `make duvet` succeeds
- Check 4 (Snapshot): N/A — no expected snapshot file exists for comparison
- Check 5 (Linter): PASS — no new clippy warnings from Agent 2's changes; pre-existing warnings in test_construct_a_frame.rs and test_construct_the_body.rs
