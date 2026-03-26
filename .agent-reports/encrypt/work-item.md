# Work Item: Add Duvet Annotations for Authentication Tag Calculation in encrypt.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `authentication-tag`
- **Duvet Target**: `specification/client-apis/encrypt.md#authentication-tag`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  After serializing the message header body,
  this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
  over the message header body.
  ```
- **Current State**: missing
- **Code Location**: `build_header_auth_tag` function call in `build_header_for_encrypt` (line ~472)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
  specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
  ```
- **Current State**: missing
- **Code Location**: `aes_encrypt(...)` call in `build_header_auth_tag` (line ~531)

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
  and the serialization of encryption context to only authenticate.
  ```
- **Current State**: missing
- **Code Location**: `&[raw_header, serialized_req_encryption_context].concat()` in `build_header_auth_tag` (line ~535)

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
  in the [encryption materials](../framework/structures.md#encryption-materials)
  filtered to only contain key value pairs listed in
  the [encryption material's](../framework/structures.md#encryption-materials)
  [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
  serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
  ```
- **Current State**: missing
- **Code Location**: `required_encryption_context_map` construction loop in `build_header_for_encrypt` (lines ~441-447), and `write_empty_ec_or_write_aad` call (lines ~450-453)

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The IV MUST have a value of 0.
  ```
- **Current State**: missing
- **Code Location**: `let iv = vec![0; get_iv_length(suite) as usize];` in `build_header_auth_tag` (line ~529)

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The cipherkey MUST be the derived data key
  ```
- **Current State**: missing
- **Code Location**: `data_key` parameter passed to `aes_encrypt` in `build_header_auth_tag` (line ~533)

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The plaintext MUST be an empty byte array
  ```
- **Current State**: missing
- **Code Location**: `&[]` passed as plaintext to `aes_encrypt` in `build_header_auth_tag` (line ~534)

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The serialized bytes MUST NOT be released until the entire message header has been serialized.
  ```
- **Current State**: missing
- **Code Location**: Structural — `build_header_for_encrypt` builds the complete header (body + auth tag) before returning, and `step_construct_header` calls `header::serialize_header` only after the full header is built

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If this operation is streaming the encrypted message and
  the entire message header has been serialized,
  the serialized message header MUST be released.
  ```
- **Current State**: missing
- **Code Location**: `header::serialize_header(&header, ciphertext, dw)` in `step_construct_header` — writes to ciphertext output immediately after header is complete

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encrypted message output by the Encrypt operation MUST have a message header equal
  to the message header calculated in this step.
  ```
- **Current State**: missing
- **Code Location**: Structural — single code path: the header built in `step_construct_header` is serialized directly to the output ciphertext buffer

### Requirement 11
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the message headers are not equal, the Encrypt operation MUST fail.
  ```
- **Current State**: missing
- **Code Location**: Structural — single code path means inequality is impossible; the same header object is serialized to output

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the algorithm suite contains a signature algorithm and
  this operation is [streaming](streaming.md) the encrypted message output to the caller,
  this operation MUST input the serialized header to the signature algorithm as soon as it is serialized,
  such that the serialized header isn't required to remain in memory to [construct the signature](#construct-the-signature).
  ```
- **Current State**: missing
- **Code Location**: `header::serialize_header(&header, ciphertext, dw)` — the `dw` (`DigestWriter`) receives header bytes during serialization, feeding them to the signature algorithm

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs`

#### `build_header_for_encrypt` (lines ~430-478) — prepares AAD components:
```rust
fn build_header_for_encrypt(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    derived_data_keys: &key_derivation::ExpandedKeyMaterial,
) -> Result<header::HeaderInfo, Error> {
    let mut stored_encryption_context = encryption_context.clone();
    let mut required_encryption_context_map: EncryptionContext = EncryptionContext::new();
    for key in required_encryption_context_keys {
        if stored_encryption_context.contains_key(key) {
            required_encryption_context_map
                .insert(key.clone(), stored_encryption_context.remove(key).unwrap());
        }
    }
    // ...
    let header_auth = build_header_auth_tag(
        suite, &derived_data_keys.data_key,
        &raw_header, &serialized_req_encryption_context,
    )?;
    // ...
}
```

#### `build_header_auth_tag` (lines ~517-543) — the core calculation:
```rust
fn build_header_auth_tag(
    suite: &AlgorithmSuite,
    data_key: &[u8],
    raw_header: &[u8],
    serialized_req_encryption_context: &[u8],
) -> Result<HeaderAuth, Error> {
    let key_length = get_encrypt_key_length(suite);
    if data_key.len() != key_length as usize {
        return Err("Incorrect data key length".into());
    }

    let iv = vec![0; get_iv_length(suite) as usize];
    let mut auth_tag = Vec::new();
    aes_encrypt(
        body::get_encrypt(suite),
        &iv, data_key, &[],
        &[raw_header, serialized_req_encryption_context].concat(),
        &mut auth_tag,
    )?;

    Ok(HeaderAuth::AESMac {
        header_iv: iv,
        header_auth_tag: auth_tag,
    })
}
```

#### `step_construct_header` (lines ~302-318) — serialization and signature feeding:
```rust
fn step_construct_header(
    mat_result: &EncryptionMaterialsResult,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: FrameLength,
    ciphertext: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<header::HeaderInfo, Error> {
    let header = build_header_for_encrypt(/* ... */)?;
    header::serialize_header(&header, ciphertext, dw)?;
    Ok(header)
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
```rust
// Existing integration tests exercise the full encrypt path including header auth tag
// calculation, but have no duvet test annotations for #authentication-tag
```

## Implementation Guidance
- Add `type=implementation` annotations (default, omit type line) to `build_header_auth_tag` for requirements 1-7
- Add `type=implication` annotations for structural requirements 8-12 in `step_construct_header` and `build_header_for_encrypt`
- Add `type=implication` with `reason=` for requirements 10-11 since they are fulfilled by the single code path structure
- Requirement 4 (required EC filtering) should be annotated at the `required_encryption_context_map` loop in `build_header_for_encrypt`, NOT at the `aes_encrypt` call
- Requirement 12 (signature feeding) should be annotated at the `header::serialize_header(&header, ciphertext, dw)` call in `step_construct_header`, with `reason=` explaining that `dw` (DigestWriter) feeds header bytes to the signature algorithm
- Follow the pattern in `step_construct_signature` for annotation style
- Add `type=test` annotations to existing tests in `test_encrypt_decrypt.rs` that exercise the header auth tag path

### Spec-Aligned Structure
The spec describes this flow:
1. "MUST calculate an authentication tag over the message header body" → annotate at `build_header_auth_tag(...)` call in `build_header_for_encrypt`
2. "MUST be the output of the authenticated encryption algorithm" → annotate at `aes_encrypt(...)` call
3. "AAD MUST be the concatenation" → annotate at `&[raw_header, serialized_req_encryption_context].concat()`
4. "encryption context to only authenticate MUST be..." → annotate at `required_encryption_context_map` loop
5. "IV MUST have a value of 0" → annotate at `let iv = vec![0; ...]`
6. "cipherkey MUST be the derived data key" → annotate at `data_key` parameter in `aes_encrypt`
7. "plaintext MUST be an empty byte array" → annotate at `&[]` in `aes_encrypt`
8. "serialized bytes MUST NOT be released until entire header serialized" → annotate at `build_header_for_encrypt` return (implication)
9. "serialized message header MUST be released" → annotate at `header::serialize_header` call (implication)
10. "MUST have a message header equal" → annotate at `header::serialize_header` call (implication + reason)
11. "headers not equal, MUST fail" → annotate at same location (implication + reason)
12. "MUST input serialized header to signature algorithm" → annotate at `header::serialize_header(&header, ciphertext, dw)` (implication + reason about DigestWriter)

Sub-items to annotate individually:
- "AAD MUST be the concatenation" → at `&[raw_header, serialized_req_encryption_context].concat()`
- "encryption context to only authenticate MUST be..." → at `required_encryption_context_map` loop
- "IV MUST have a value of 0" → at `let iv = vec![0; ...]`
- "cipherkey MUST be the derived data key" → at `data_key` in `aes_encrypt` call
- "plaintext MUST be an empty byte array" → at `&[]` in `aes_encrypt` call

## Targeted Tests
- `test_encrypt_decrypt` — exercises full encrypt path including header auth tag with V2 (commitment) algorithm suite
- `test_encrypt_decrypt_ec` — exercises header auth tag with encryption context
- `test_encrypt_decrypt_single_full_frame` — exercises header auth tag with various frame lengths

## Success Criteria
```bash
cargo test test_encrypt_decrypt
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `specification/client-apis/encrypt.md#authentication-tag`
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
