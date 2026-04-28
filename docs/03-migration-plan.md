# Migration Plan: Current → Spec-Based File Organization

This plan moves code from the current layout to the ideal spec-based layout in phases. Each phase is designed to be a reviewable, compilable PR.

## Guiding Principles

- Each phase should compile and pass tests before moving to the next.
- Use Rust module re-exports to avoid breaking internal callers during transition.
- The `prim/`, `mpl/`, and `aws-esdk-cxx/` crates are unaffected — only `esdk/src/` changes.
- Duvet annotations move with their code. No annotation should be orphaned.

---

## Phase 1: Create `message/` module from `serialize/`

This is the biggest structural change but is mostly a rename + consolidation. No logic changes.

### Step 1.1: Rename `serialize/` → `message/`

Rename the folder and update `lib.rs`:
```
serialize/  →  message/
```
Update `lib.rs`:
```rust
// Before:
pub(crate) mod serialize;
// After:
pub(crate) mod message;
```

### Step 1.2: Consolidate header files into `message/header.rs`

Merge these 6 files into one:
| Source | Destination |
|--------|------------|
| `serialize/header_types.rs` | `message/header.rs` (types section) |
| `serialize/header.rs` | `message/header.rs` (read/write header body) |
| `serialize/v1_header_body.rs` | `message/header.rs` (V1 section) |
| `serialize/v2_header_body.rs` | `message/header.rs` (V2 section) |
| `serialize/header_auth.rs` | `message/header.rs` (auth section) |
| `serialize/shared_header_functions.rs` | `message/header.rs` (shared helpers) |

The file will be large (~300 lines of types + ~400 lines of read/write logic). If desired, use internal `mod v1 { }` and `mod v2 { }` blocks within the file to keep V1/V2 visually separated without creating separate files.

### Step 1.3: Rename remaining serialize files

| Source | Destination |
|--------|------------|
| `serialize/encryption_context.rs` | `message/encryption_context.rs` |
| `serialize/encrypted_data_keys.rs` | `message/encrypted_data_keys.rs` |
| `serialize/serializable_types.rs` | `message/types.rs` (merge with below) |
| `serialize/serialize_functions.rs` | `message/types.rs` (merge into same file) |
| `serialize.rs` (module root) | `message/mod.rs` |

### Step 1.4: Move `message_body.rs` into `message/`

| Source | Destination |
|--------|------------|
| `message_body.rs` | `message/body.rs` (frame encrypt/decrypt) |
| Body AAD code from `message_body.rs` | `message/body_aad.rs` (new file) |

Extract `BodyAADContent`, `body_aad_content_type_string()`, and `body_aad()` into `message/body_aad.rs`. Leave frame encryption/decryption in `message/body.rs`.

### Step 1.5: Verify

```bash
cargo build && cargo test && cargo clippy
```

---

## Phase 2: Split `esdk_operations.rs` into `encrypt.rs` + `decrypt.rs`

### Step 2.1: Create `encrypt.rs`

Move from `esdk_operations.rs`:
- `encrypt()` (public entry point)
- `encrypt_stream()`
- `internal_encrypt()`
- `step_get_encryption_materials()`
- `step_construct_header()`
- `step_construct_body()`
- `step_construct_signature()`
- `EncryptionMaterialsResult` struct
- `get_esdk_id()`

Move from `encrypt_decrypt.rs`:
- `encrypt_and_serialize()`
- `build_header_for_encrypt()`
- `build_header_body()`
- `build_header_auth_tag()`
- `validate_encryption_context()`
- `validate_max_encrypted_data_keys()`
- `generate_message_id()`

### Step 2.2: Create `decrypt.rs`

Move from `esdk_operations.rs`:
- `decrypt()` (public entry point)
- `decrypt_stream()`
- `internal_decrypt()`
- `build_encryption_context_to_only_authenticate()`

Move from `encrypt_decrypt.rs`:
- `verify_signature()`
- `get_ecdsa_alg()`
- `validate_suite_data()`
- `read_and_decrypt_non_framed_message_body()`

### Step 2.3: Delete `esdk_operations.rs` and `encrypt_decrypt.rs`

Update `lib.rs`:
```rust
// Before:
mod esdk_operations;
pub use esdk_operations::*;
pub(crate) mod encrypt_decrypt;

// After:
mod encrypt;
pub use encrypt::*;
mod decrypt;
pub use decrypt::*;
```

### Step 2.4: Verify

```bash
cargo build && cargo test && cargo clippy
```

---

## Phase 3: Create `client.rs` and slim down `types.rs`

### Step 3.1: Create `client.rs`

Move from `types.rs`:
- `FrameLength` (client configuration)
- `NetV400RetryPolicy` (client configuration)
- `MaterialSource` enum (client-level concept)
- `mpl()` function (client singleton)

Move input validation logic:
- `EncryptInput::validate()` → `client.rs` or keep on the type
- `DecryptInput::validate()` → same

### Step 3.2: Slim `types.rs`

`types.rs` retains only the public I/O types:
- `EncryptInput` / `EncryptOutput`
- `EncryptStreamInput` / `EncryptStreamOutput`
- `DecryptInput` / `DecryptOutput`
- `DecryptStreamInput` / `DecryptStreamOutput`
- `SafeWrite` / `SafeRead` traits

### Step 3.3: Update `lib.rs`

```rust
mod client;
pub use client::*;
```

### Step 3.4: Verify

```bash
cargo build && cargo test && cargo clippy
```

---

## Phase 4: Create `message/footer.rs`

### Step 4.1: Extract footer logic

Currently, footer (signature) serialization is inline in the encrypt path (`step_construct_signature`) and footer deserialization is inline in the decrypt path (`verify_signature`). Extract the pure serialization/deserialization parts:

Create `message/footer.rs`:
- `write_footer(w, signature)` — writes signature length (u16) + signature bytes
- `read_footer(r)` — reads signature length + signature bytes

The actual signing/verification logic stays in `encrypt.rs` and `decrypt.rs` respectively (it's client-api behavior, not data-format).

### Step 4.2: Create `streaming.rs` (optional)

If the streaming logic is substantial enough to warrant its own file, extract it. If it's just thin wrappers around `encrypt_stream()` / `decrypt_stream()`, it can stay inline in `encrypt.rs` / `decrypt.rs` with Duvet annotations pointing to `streaming.md`.

### Step 4.3: Verify

```bash
cargo build && cargo test && cargo clippy
```

---

## Phase 5: Update Duvet annotations

After all moves are complete, do a sweep:

1. Run `make duvet_report` to check coverage.
2. Verify all `//= specification/...` annotations still point to the correct spec sections.
3. Annotations should now be easier to audit because each file maps to one spec.
4. Add any missing annotations that become obvious with the cleaner structure.

---

## Migration Summary

| Phase | PR Scope | Risk | Files Changed |
|-------|----------|------|---------------|
| 1 | `serialize/` → `message/`, consolidate headers | Medium (many file moves) | ~15 files |
| 2 | Split `esdk_operations.rs` → `encrypt.rs` + `decrypt.rs` | Medium (logic moves) | ~5 files |
| 3 | Create `client.rs`, slim `types.rs` | Low (type moves) | ~3 files |
| 4 | Extract `footer.rs`, optional `streaming.rs` | Low (small extractions) | ~3 files |
| 5 | Duvet annotation sweep | Low (comments only) | ~8 files |

Total: ~5 PRs, each independently reviewable and compilable.

---

## What Doesn't Change

- `error.rs` — stays as-is
- `key_derivation.rs` — stays as-is (cross-cutting, referenced by both encrypt and decrypt specs)
- `materials.rs` — stays as-is (internal plumbing, no spec mapping)
- `test_vectors/` — stays as-is
- `tests/` — stays as-is (test file names don't need to match spec structure)
- `prim/` crate — stays as-is
- `mpl/` crate — stays as-is
- `aws-esdk-cxx/` crate — stays as-is
