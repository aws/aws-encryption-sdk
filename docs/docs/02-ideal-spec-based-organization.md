# Ideal Spec-Based File Organization

The goal: each spec document maps cleanly to one file (or one small folder), so a reviewer can look at a PR touching `client.rs` and know it covers `client-apis/client.md`, or look at `message_header/` and know it covers `data-format/message-header.md`.

## Spec Documents in Scope

### client-apis/ (4 specs)
| Spec | Key Requirement Areas |
|------|----------------------|
| `client.md` | Client initialization, commitment policy, max EDKs, encrypt/decrypt dispatch |
| `encrypt.md` | Input validation, get encryption materials, construct header, construct body, construct signature |
| `decrypt.md` | Parse header, get decryption materials, verify header, decrypt body, verify signature |
| `streaming.md` | Streaming framework, consumable bytes, release semantics |

### data-format/ (5 specs)
| Spec | Key Requirement Areas |
|------|----------------------|
| `message.md` | Top-level message structure (header + body + optional footer) |
| `message-header.md` | V1/V2 header body, header auth, all field definitions (version, type, suite ID, message ID, AAD, EDKs, content type, reserved, IV length, frame length, suite data) |
| `message-body.md` | Non-framed data, framed data (regular frames, final frames), sequence numbers, IVs, encrypted content, auth tags |
| `message-body-aad.md` | Body AAD construction (message ID, body AAD content string, sequence number, content length) |
| `message-footer.md` | Signature length + signature bytes |

---

## Proposed File Layout

```
esdk/src/
├── lib.rs                          # Module root + re-exports
├── error.rs                        # Error types (unchanged)
│
├── client.rs                       # ← client-apis/client.md
│                                   #   Client config, commitment policy, max EDKs
│                                   #   EncryptInput/DecryptInput validation
│                                   #   FrameLength, NetV400RetryPolicy
│
├── encrypt.rs                      # ← client-apis/encrypt.md
│                                   #   encrypt() entry point
│                                   #   encrypt_stream() entry point
│                                   #   Step 1: get_encryption_materials()
│                                   #   Step 2: construct_header()
│                                   #   Step 3: construct_body()
│                                   #   Step 4: construct_signature()
│
├── decrypt.rs                      # ← client-apis/decrypt.md
│                                   #   decrypt() entry point
│                                   #   decrypt_stream() entry point
│                                   #   Step 1: parse_header()
│                                   #   Step 2: get_decryption_materials()
│                                   #   Step 3: verify_header()
│                                   #   Step 4: decrypt_body()
│                                   #   Step 5: verify_signature()
│
├── streaming.rs                    # ← client-apis/streaming.md
│                                   #   Streaming encrypt/decrypt wrappers
│                                   #   (may be thin if streaming is inline in encrypt/decrypt)
│
├── materials.rs                    # MPL bridge (no spec — internal plumbing)
│                                   #   Cmm enum, legacy/modern dispatch
│                                   #   Type conversions from Dafny MPL
│
├── key_derivation.rs               # Key derivation (referenced by encrypt.md + decrypt.md)
│                                   #   derive_keys(), derive_key(), expand_key_material()
│                                   #   (Cross-cutting: used by both encrypt and decrypt)
│
├── message/                        # ← data-format/ specs
│   ├── mod.rs                      # ← data-format/message.md
│   │                               #   Top-level message structure
│   │                               #   Re-exports submodules
│   │
│   ├── header.rs                   # ← data-format/message-header.md (structure + auth)
│   │                               #   MessageFormatVersion, MessageType, ContentType
│   │                               #   V1HeaderBody, V2HeaderBody, HeaderBody enum
│   │                               #   HeaderAuth
│   │                               #   HeaderInfo
│   │                               #   write_header_body() / read_header_body()
│   │                               #   write_v1_header_body() / read_v1_header_body()
│   │                               #   write_v2_header_body() / read_v2_header_body()
│   │                               #   write_header_auth_tag() / read_header_auth_tag()
│   │                               #   Suite ID, message ID, reserved bytes helpers
│   │
│   ├── body.rs                     # ← data-format/message-body.md
│   │                               #   Frame types, sequence numbers
│   │                               #   read_and_decrypt_framed_message_body()
│   │                               #   read_and_decrypt_non_framed_message_body()
│   │                               #   Frame encryption during encrypt
│   │
│   ├── body_aad.rs                 # ← data-format/message-body-aad.md
│   │                               #   BodyAADContent enum
│   │                               #   body_aad() construction
│   │                               #   body_aad_content_type_string()
│   │
│   ├── footer.rs                   # ← data-format/message-footer.md
│   │                               #   Signature serialization/deserialization
│   │                               #   (Currently inline in encrypt/decrypt)
│   │
│   ├── encryption_context.rs       # ← message-header.md#aad subsection
│   │                               #   EC serialization/deserialization
│   │                               #   Canonical ordering
│   │                               #   Validation
│   │
│   ├── encrypted_data_keys.rs      # ← message-header.md#encrypted-data-keys subsection
│   │                               #   EDK serialization/deserialization
│   │
│   └── types.rs                    # Shared serialization types + primitives
│                                   #   ESDKEncryptionContext aliases
│                                   #   get_iv_length(), get_tag_length(), etc.
│                                   #   Binary read/write primitives (u8, u16, u32, u64)
│                                   #   DigestWriter, NoopWriter
│
├── types.rs                        # Public API types (EncryptInput, DecryptInput, etc.)
│                                   #   (Slimmed down — client config moves to client.rs)
│
└── test_vectors/                   # Test vector support (unchanged)
    └── ...
```

---

## Spec → File Mapping Summary

| Spec Document | Proposed File | Current File(s) |
|---------------|---------------|-----------------|
| `client-apis/client.md` | `client.rs` | `types.rs` (scattered) |
| `client-apis/encrypt.md` | `encrypt.rs` | `esdk_operations.rs` + `encrypt_decrypt.rs` |
| `client-apis/decrypt.md` | `decrypt.rs` | `esdk_operations.rs` + `encrypt_decrypt.rs` |
| `client-apis/streaming.md` | `streaming.rs` | `esdk_operations.rs` (inline) |
| `data-format/message.md` | `message/mod.rs` | (implicit) |
| `data-format/message-header.md` | `message/header.rs` | `serialize/header.rs` + `header_types.rs` + `header_auth.rs` + `v1_header_body.rs` + `v2_header_body.rs` + `shared_header_functions.rs` |
| `data-format/message-body.md` | `message/body.rs` | `message_body.rs` + parts of `encrypt_decrypt.rs` |
| `data-format/message-body-aad.md` | `message/body_aad.rs` | `message_body.rs` (inline) |
| `data-format/message-footer.md` | `message/footer.rs` | `esdk_operations.rs` (inline) |
| `message-header.md#aad` | `message/encryption_context.rs` | `serialize/encryption_context.rs` |
| `message-header.md#encrypted-data-keys` | `message/encrypted_data_keys.rs` | `serialize/encrypted_data_keys.rs` |
| (no spec — internal) | `materials.rs` | `materials.rs` |
| (cross-cutting) | `key_derivation.rs` | `key_derivation.rs` |
| (shared primitives) | `message/types.rs` | `serialize/serializable_types.rs` + `serialize_functions.rs` + `serialize.rs` |

---

## Why This Organization

1. **1:1 spec-to-file for client-apis**: A reviewer looking at `encrypt.rs` knows exactly which spec requirements to check against. No hunting across `esdk_operations.rs` and `encrypt_decrypt.rs`.

2. **`message/` folder mirrors `data-format/`**: The folder structure directly reflects the spec structure. `message/header.rs` = `data-format/message-header.md`. Simple.

3. **Header consolidation**: Currently the header logic is spread across 6 files (`header.rs`, `header_types.rs`, `header_auth.rs`, `v1_header_body.rs`, `v2_header_body.rs`, `shared_header_functions.rs`). Since they all implement one spec (`message-header.md`), they consolidate into one file. If that file gets too large, the V1/V2 split can be internal modules within `header.rs`.

4. **Body AAD gets its own file**: It has its own spec doc and its own distinct requirements. Currently buried inside `message_body.rs`.

5. **Footer gets its own file**: Currently the footer (signature) serialization is inline in `esdk_operations.rs`. Giving it a dedicated file makes the spec mapping obvious.

6. **Encrypt/decrypt split**: Currently `esdk_operations.rs` contains both encrypt and decrypt orchestration, and `encrypt_decrypt.rs` contains shared helpers. Splitting into `encrypt.rs` and `decrypt.rs` means each file maps to exactly one spec. Shared helpers (like `validate_encryption_context`) can live in whichever file "owns" them per the spec, or in a small shared module.

7. **`client.rs` for client config**: The client spec defines commitment policy, max EDKs, and the encrypt/decrypt dispatch. Currently these are scattered across `types.rs` and `esdk_operations.rs`. A dedicated `client.rs` makes the mapping clear.

8. **Serialization primitives consolidate**: `serialize_functions.rs`, `serializable_types.rs`, and the `serialize.rs` module root are all internal plumbing with no direct spec mapping. They merge into `message/types.rs`.
