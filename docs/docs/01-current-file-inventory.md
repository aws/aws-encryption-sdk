# Current File Inventory — Native Rust ESDK

This document describes what each source file in the native Rust ESDK workspace contains.

## Workspace Root (`esdk_rust/`)

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace manifest defining the 5 member crates |
| `Cargo.lock` | Dependency lockfile |
| `Makefile` | Build orchestration |

---

## `esdk/` — Core ESDK Crate

The main encrypt/decrypt implementation. This is the heart of the native Rust ESDK.

### `esdk/src/lib.rs`
Module root. Declares all submodules and re-exports public API types. Configures extensive clippy/rustc lint settings. Public modules: `error`, `esdk_operations`, `types`, `test_vectors` (feature-gated). Internal modules: `encrypt_decrypt`, `key_derivation`, `materials`, `message_body`, `serialize`.

### `esdk/src/types.rs`
All public-facing types for the ESDK API:
- `EncryptInput` / `EncryptOutput` — encrypt operation I/O
- `EncryptStreamInput` / `EncryptStreamOutput` — streaming encrypt I/O
- `DecryptInput` / `DecryptOutput` — decrypt operation I/O
- `DecryptStreamInput` / `DecryptStreamOutput` — streaming decrypt I/O
- `FrameLength` — validated frame length wrapper (must be > 0, ≤ u32::MAX)
- `MaterialSource` — enum for Legacy vs Modern CMM/keyring
- `NetV400RetryPolicy` — .NET v4.0.0 compatibility retry policy
- `SafeWrite` / `SafeRead` — trait aliases for `std::io::Write`/`Read`
- Helper: `mpl()` — returns the Dafny MPL client singleton

### `esdk/src/esdk_operations.rs`
Top-level encrypt/decrypt orchestration. Contains:
- `encrypt()` / `decrypt()` — public async entry points
- `encrypt_stream()` / `decrypt_stream()` — streaming variants
- `internal_encrypt()` — the 4-step encrypt pipeline (get materials → construct header → construct body → construct signature)
- `internal_decrypt()` — the 5-step decrypt pipeline (parse header → get materials → verify header → decrypt body → verify signature)
- `step_get_encryption_materials()` — calls CMM for encryption materials
- `step_construct_header()` — serializes the message header
- `step_construct_body()` — encrypts plaintext into framed body
- `step_construct_signature()` — computes ECDSA signature (if required by suite)
- `build_encryption_context_to_only_authenticate()` — filters EC for required-EC-keys feature

### `esdk/src/encrypt_decrypt.rs`
Mid-level encrypt/decrypt helpers that bridge between operations and serialization:
- `encrypt_and_serialize()` — encrypts plaintext and writes the complete message (header + body + footer)
- `build_header_for_encrypt()` — constructs the full `HeaderInfo` for encryption
- `build_header_body()` — builds V1 or V2 header body from encryption materials
- `build_header_auth_tag()` — computes the header authentication tag
- `validate_encryption_context()` — checks EC for `aws-crypto-` prefix violations
- `validate_max_encrypted_data_keys()` — enforces max EDK limit
- `generate_message_id()` — generates random message ID (16 or 32 bytes)
- `validate_suite_data()` — validates commitment key matches suite data
- `verify_signature()` — verifies ECDSA signature during decrypt
- `read_and_decrypt_non_framed_message_body()` — decrypts non-framed (legacy) message bodies
- `get_ecdsa_alg()` — maps algorithm suite to ECDSA algorithm

### `esdk/src/key_derivation.rs`
Key derivation logic:
- `derive_keys()` — top-level dispatcher: delegates to v1 or v2 based on message version
- `derive_key()` — v1-style key derivation (HKDF or Identity KDF, no commitment)
- `expand_key_material()` — v2-style key derivation (HKDF with key commitment)
- `ExpandedKeyMaterial` — holds derived data key + optional commitment key

### `esdk/src/message_body.rs`
Message body encryption/decryption:
- `BodyAADContent` — enum for body AAD content type strings ("Single Block", "Frame", "Final Frame")
- `body_aad()` — constructs the body AAD bytes per the message-body-aad spec
- `iv_seq()` — converts sequence number to IV bytes
- `get_encrypt()` — extracts AES-GCM algorithm from suite
- `read_and_decrypt_framed_message_body()` — reads and decrypts framed message body during decrypt

### `esdk/src/materials.rs`
Materials bridge between native Rust types and the Dafny-transpiled MPL:
- `Cmm` enum — Legacy vs Modern CMM wrapper
- `create_cmm_from_input()` — creates appropriate CMM from encrypt/decrypt input
- `get_encryption_materials()` / `get_decryption_materials()` — dispatchers
- `get_modern_encryption_materials()` / `get_modern_decryption_materials()` — native MPL path
- `get_legacy_encryption_materials()` / `get_legacy_decryption_materials()` — Dafny MPL path
- ~20 `from_legacy_*` conversion functions — convert Dafny MPL types to native types

### `esdk/src/error.rs`
Error types:
- `Error` — base error with kind, message, backtrace, optional cause
- `ErrorKind` — enum: Esdk, SerializationError, CryptographicError, MplError, LegacyError, ValidationError
- `val_err()` — convenience constructor for validation errors
- `From` impls for various MPL/Smithy error types

### `esdk/src/serialize.rs`
Serialization module root. Declares submodules and provides shared utilities:
- `ser_err()` — convenience constructor for serialization errors
- `NoopWriter` — writer that discards bytes (for length calculations)
- `DigestWriter` — writer that feeds bytes into a digest context (for signature computation)

### `esdk/src/serialize/header_types.rs`
Core header type definitions:
- `MessageFormatVersion` — V1/V2 enum
- `V1HeaderBody` / `V2HeaderBody` — version-specific header body structs
- `HeaderBody` — enum wrapping V1/V2 with accessor methods
- `HeaderAuth` — AES-MAC authentication tag (IV + tag)
- `MessageType` — CustomerAed (0x80)
- `ContentType` — NonFramed/Framed
- Read/write functions for version, type, content type fields
- Constants: `MESSAGE_ID_LEN_V1` (16), `MESSAGE_ID_LEN_V2` (32)

### `esdk/src/serialize/header.rs`
Header-level serialization/deserialization:
- `HeaderInfo` — complete parsed header (body + raw bytes + EC + suite + auth)
- `write_header_body()` — dispatches to V1/V2 writer
- `read_header_body()` — reads version byte, dispatches to V1/V2 reader, validates frame length vs content type
- `header_version_supports_commitment()` — validates commitment compatibility
- Constants: `START_SEQUENCE_NUMBER`, `ENDFRAME_SEQUENCE_NUMBER`, `NONFRAMED_SEQUENCE_NUMBER`, `SAFE_MAX_ENCRYPT`

### `esdk/src/serialize/v1_header_body.rs`
V1 header body serialization/deserialization:
- `write_v1_header_body()` — writes all V1 header fields in order (version, type, suite ID, message ID, AAD, EDKs, content type, reserved, IV length, frame length)
- `read_v1_header_body()` — reads and parses V1 header fields
- `read_v1_reserved_bytes()` — validates the 4-byte reserved field
- `read_v1_header_iv_length()` — validates IV length matches suite

### `esdk/src/serialize/v2_header_body.rs`
V2 header body serialization/deserialization:
- `write_v2_header_body()` — writes all V2 header fields (version, suite ID, message ID, AAD, EDKs, content type, frame length, suite data)
- `read_v2_header_body()` — reads and parses V2 header fields, validates commitment support
- `get_hkdf()` / `has_hkdf()` — helpers for extracting HKDF from derivation algorithm

### `esdk/src/serialize/serializable_types.rs`
Shared serializable type definitions and validation:
- Type aliases: `ESDKEncryptionContext`, `ESDKCanonicalEncryptionContext`
- `get_iv_length()` / `get_tag_length()` / `get_encrypt_key_length()` — extract lengths from algorithm suite
- `length()` — compute serialized EC byte length
- `to_canonical_pairs()` / `from_canonical_pairs()` — sort EC by key for canonical form
- `is_esdk_encryption_context()` — validate EC fits ESDK size constraints
- `is_esdk_encrypted_data_key()` / `is_esdk_encrypted_data_keys()` — validate EDK size constraints

### `esdk/src/serialize/serialize_functions.rs`
Low-level binary read/write primitives:
- `write_bytes()`, `write_u8()`, `write_u16()`, `write_u32()`, `write_str_u16()`, `write_seq_u16()` — big-endian writers
- `read_bytes()`, `read_u8()`, `read_u16()`, `read_u32()`, `read_u64()`, `read_vec()`, `read_str_u16()`, `read_seq_u16()` — big-endian readers
- `read_up_to()` / `read_up_to_peek()` — partial read helpers
- `read_seq_u32_bounded()` / `read_seq_u64_bounded()` — bounded-length sequence readers

### `esdk/src/serialize/encryption_context.rs`
Encryption context serialization:
- `read_canonical_ec()` — deserializes canonical EC from bytes
- `write_aad_section()` — writes the full AAD section (length prefix + key-value pairs)
- `write_aad()` — writes key-value pairs (count + pairs)
- `write_empty_ec_or_write_aad()` — conditional write for body AAD (omits empty EC)

### `esdk/src/serialize/header_auth.rs`
Header authentication tag serialization:
- `write_header_auth_tag()` — dispatches to V1/V2
- `write_header_auth_tag_v1()` — writes IV + auth tag
- `write_header_auth_tag_v2()` — writes auth tag only (IV is implicit 0)
- `read_header_auth_tag()` — dispatches to V1/V2
- `read_header_auth_tag_v1()` / `read_header_auth_tag_v2()` — version-specific readers

### `esdk/src/serialize/encrypted_data_keys.rs`
EDK serialization:
- `write_edk()` / `write_edks()` — serialize individual/multiple EDKs
- `read_edk()` / `read_edks()` — deserialize EDKs with max-EDK validation


### `esdk/src/serialize/shared_header_functions.rs`
Functions shared between V1 and V2 header paths:
- `read_esdk_suite_id()` — reads 2-byte suite ID and looks up the algorithm suite
- `read_message_id_v1()` / `read_message_id_v2()` — reads 16/32-byte message IDs
- `write_esdk_suite_id()` — writes suite binary ID
- `write_message_id()` — writes message ID bytes

---

## `esdk/tests/` — Integration Tests

| File | Purpose |
|------|---------|
| `fixtures.rs` | Shared test fixtures (keyring setup, encryption contexts, etc.) |
| `test_create_esdk_client.rs` | Tests for ESDK client creation and configuration |
| `test_encrypt_decrypt.rs` | Round-trip encrypt/decrypt integration tests |
| `test_reproduced_enc_context.rs` | Tests for reproduced encryption context on decrypt |
| `test_required_encryption_context.rs` | Tests for required encryption context keys feature |

---

## `esdk/src/test_vectors/` — Test Vector Support

| File | Purpose |
|------|---------|
| `mod.rs` (via `test_vectors.rs`) | Module root for test vector support |
| `do_decrypt.rs` | Decrypt test vector execution |
| `do_encrypt.rs` | Encrypt test vector execution |
| `parse_encrypt.rs` | Parse encrypt test vector manifests |
| `parse_keys.rs` | Parse key material from test vectors |
| `run_tests.rs` | Test vector runner orchestration |
| `types.rs` | Test vector data types |
| `static_keystore.rs` | Static keystore for test vectors |
| `legacy_static_keystore.rs` | Legacy static keystore compatibility |

---

## `prim/` — Crypto Primitives Crate

Thin wrappers around `aws-lc-rs` providing the cryptographic primitives needed by the ESDK.

| File | Contents |
|------|----------|
| `lib.rs` | AES-GCM encrypt/decrypt, HMAC, digest, random bytes, constant-time compare. Defines `AesGcm` enum, `DigestAlg` enum, `Error` type. |
| `hkdf.rs` | HKDF extract/expand/combined. `Prk` type for intermediate key material. |
| `ecdsa.rs` | ECDSA sign/verify (P-256, P-384). `DigestContext` for incremental hashing. `EcdsaSignatureAlgorithm` enum. |
| `ecdh.rs` | ECDH key agreement (not currently used by ESDK encrypt/decrypt path). |
| `aes_kdf_ctr.rs` | AES-KDF-CTR (not currently used by ESDK encrypt/decrypt path). |
| `format.rs` | Formatting utilities. |
| `memory_tracker.rs` | Memory tracking utilities for performance analysis. |
| `use_memory_tracker.rs` | Feature-gated memory tracker usage. |

---

## `mpl/` — Native MPL Types Crate (Stubs)

Type definitions that mirror the Dafny-transpiled MPL. These are stubs/wrappers — the actual MPL logic comes from the Dafny-transpiled `aws_mpl_dafny` crate.

| File | Contents |
|------|----------|
| `lib.rs` | Re-exports `aws_mpl_primitives` and `aws_mpl_dafny`. Defines `EncryptionContext`, `EncryptionMaterials`, `DecryptionMaterials`, `EncryptedDataKey`, `Secret`. |
| `suites.rs` | Algorithm suite definitions and types. |
| `keyring.rs` | `KeyringRef` trait/type. |
| `cmm.rs` | `CryptographicMaterialsManagerRef` trait/type. |
| `materials.rs` | Materials-related types. |
| `error.rs` | MPL error types. |
| `commitment.rs` | Commitment policy types. |
| `agreement.rs` | Key agreement types. |
| `cmc.rs` | Cryptographic materials cache types. |
| `client_supplier.rs` | KMS client supplier (feature-gated on `kms`). |
| `keystore.rs` | Keystore types (feature-gated on `ddb`). |
| `kms_keyring.rs` | KMS keyring types (feature-gated on `kms`). |

---

## `aws-esdk-cxx/` — C++ Bindings Crate

| File | Contents |
|------|----------|
| `src/lib.rs` | CXX bridge definitions. Defines C++ FFI types (`EncryptInput`, `DecryptInput`, `EncryptOutput`, `DecryptOutput`, etc.) and bridge functions (`encrypt`, `decrypt`, `create_hierarchical_keyring`, `create_keystore`, `create_kms_client`, `create_ddb_client`). Handles type conversion between C++ and Rust. |
| `build.rs` | CXX build script for generating C++ headers and compiling the bridge. |
| `main.cpp` / `main_idiomatic.cpp` | Example C++ programs using the bindings. |
| `Makefile` | Build orchestration for C++ examples. |

---

## `aws_structured_encryption/` — Out of Scope

Partial structured encryption implementation. Not part of the current ESDK deliverable.
