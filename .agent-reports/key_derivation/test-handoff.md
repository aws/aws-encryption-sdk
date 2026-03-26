# Test Handoff: key_derivation

**Spec**: `specification/client-apis/encrypt.md#get-the-encryption-materials`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs` (NEW)

**Commit Message**:
```
test(encrypt): add test annotations for key derivation requirements

Add type=test duvet annotations for three key derivation requirements
in encrypt.md#get-the-encryption-materials:
- Parent: algorithm used MUST be the KDF from the algorithm suite
- Identity KDF: derived key MUST equal plaintext data key
- HKDF: derivation MUST follow HKDF Encryption Key process

Each test uses encrypt-then-decrypt round-trip verification with
the appropriate algorithm suite and commitment policy.

Spec: specification/client-apis/encrypt.md#get-the-encryption-materials
```
