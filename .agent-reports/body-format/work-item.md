# Work Item: Non-Framed Data Annotations + Framed Data Annotation Fixes

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-body.md`
- **Sections**: `#non-framed-data`, `#non-framed-data-iv`, `#non-framed-data-encrypted-content-length`, `#non-framed-data-encrypted-content`, `#non-framed-data-authentication-tag`, `#framed-data`, `#regular-frame-iv`, `#regular-frame-encrypted-content`, `#regular-frame-sequence-number`, `#final-frame-sequence-number`, `#final-frame-encrypted-content-length`
- **Duvet Targets**:
  - `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data`
  - `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-iv`
  - `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content-length`
  - `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content`
  - `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-authentication-tag`
  - `aws-encryption-sdk-specification/data-format/message-body.md#framed-data`
  - `aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-iv`
  - `aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-encrypted-content`
  - `aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-sequence-number`
  - `aws-encryption-sdk-specification/data-format/message-body.md#final-frame-sequence-number`
  - `aws-encryption-sdk-specification/data-format/message-body.md#final-frame-encrypted-content-length`

## Type of Work
FIX_ANNOTATION + ADD_TESTS + NEW_IMPLEMENTATION (annotations only — code already exists)

## Requirements to Address

---

### GROUP A: Non-Framed Data — All Missing (14 requirements)

#### Requirement A1 — non-framed-data serialization order
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Non-framed data MUST be serialized (written) as, in order,
  IV,
  Encrypted Content Length,
  Encrypted Content,
  and Authentication Tag.
  ```
- **Current State**: missing (both impl and test)
- **Note**: The ESDK does NOT encrypt non-framed data (`ContentType::Framed` is hardcoded in encrypt.rs). This requirement needs a `type=exception` annotation with `reason=` explaining the ESDK only encrypts framed data.

#### Requirement A2 — non-framed-data deserialization order
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Non-framed data MUST be deserialized (read) as, in order,
  IV,
  Encrypted Content Length,
  Encrypted Content,
  and Authentication Tag.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_and_decrypt_non_framed_message_body` which reads in this exact order.

#### Requirement A3 — non-framed-data-iv: unique IV on write
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When writing a message, the IV MUST be a unique IV within the message.
  ```
- **Current State**: missing (both impl and test)
- **Note**: The ESDK does NOT encrypt non-framed data. This needs `type=exception`.

#### Requirement A4 — non-framed-data-iv: serialize IV length
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When writing a message, the operation MUST serialize the IV to be [IV Length](message-header.md#iv-length) bytes.
  ```
- **Current State**: missing (both impl and test)
- **Note**: The ESDK does NOT encrypt non-framed data. This needs `type=exception`.

#### Requirement A5 — non-framed-data-iv: deserialize IV length
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When reading a message, the operation MUST deserialize [IV Length](message-header.md#iv-length) bytes and interpret it as the IV.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_vec(r, get_iv_length(&header.suite) as usize, raw)` in `read_and_decrypt_non_framed_message_body`.

#### Requirement A6 — non-framed-data-iv: interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When reading a message, the deserialized IV MUST be interpreted as bytes.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_vec` returning `Vec<u8>`.

#### Requirement A7 — non-framed-data-encrypted-content-length: max size
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
  due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_seq_u64_bounded(r, header::SAFE_MAX_ENCRYPT, ...)` where `SAFE_MAX_ENCRYPT = 0x000F_FFFF_FFE0 = 2^36 - 32`.

#### Requirement A8 — non-framed-data-encrypted-content-length: serialized length 8 bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When serializing the encrypted content length to a message, the length of the serialized encrypted content length MUST be 8 bytes.
  ```
- **Current State**: missing (both impl and test)
- **Note**: The ESDK does NOT encrypt non-framed data. This needs `type=exception`.

#### Requirement A9 — non-framed-data-encrypted-content-length: serialized as Uint64
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encrypted content length MUST be serialized as a Uint64.
  ```
- **Current State**: missing (both impl and test)
- **Note**: The ESDK does NOT encrypt non-framed data. This needs `type=exception`.

#### Requirement A10 — non-framed-data-encrypted-content-length: read as Uint64
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a Uint64.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_seq_u64_bounded` which reads 8 bytes as u64.

#### Requirement A11 — non-framed-data-encrypted-content: length matches field
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized encrypted content MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length) field.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_seq_u64_bounded` which reads exactly `content_length` bytes.

#### Requirement A12 — non-framed-data-encrypted-content: interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encrypted content MUST be interpreted as bytes.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by returning `Vec<u8>`.

#### Requirement A13 — non-framed-data-authentication-tag: length matches algorithm
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_vec(r, get_tag_length(&header.suite) as usize, raw)`.

#### Requirement A14 — non-framed-data-authentication-tag: interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The authentication tag MUST be interpreted as bytes.
  ```
- **Current State**: missing (both impl and test)
- **Note**: Fulfilled by `read_vec` returning `Vec<u8>`.

---

### GROUP B: Framed Data — Missing Implementation Annotations (5 requirements)

#### Requirement B1 — framed-data: max frame size
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Enforced by `FrameLength` wrapping `NonZeroU32` (max value is `u32::MAX = 2^32 - 1`). Annotate at `FrameLength::new` in `types.rs` or at the `frame_length` usage in `encrypt_and_serialize_body`.

#### Requirement B2 — regular-frame-iv: unique IV
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Fulfilled by `iv_seq(input.sequence_number, iv)` in `construct_frame` — each frame gets a unique IV derived from its unique sequence number.

#### Requirement B3 — regular-frame-encrypted-content: length equals frame length
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Enforced by `encrypt_and_serialize_body` which passes `&plaintext_frame` (exactly `frame_length` bytes) to `construct_frame` for regular frames.

#### Requirement B4 — final-frame-sequence-number: equals total frames
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Fulfilled by `encrypt_and_serialize_body` which increments `sequence_number` for each regular frame and passes it to the final frame's `construct_frame` call.

#### Requirement B5 — final-frame-sequence-number: serialized same as regular
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence Number MUST be serialized to a message the same way as the
  [Regular Frame Sequence Number](#regular-frame-sequence-number).
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Fulfilled by `construct_frame` which uses the same `write_u32(w, input.sequence_number)` for both regular and final frames.

#### Requirement B6 — final-frame-sequence-number: interpreted same as regular
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence Number MUST be interpreted from a message the same way as the
  [Regular Frame Sequence Number](#regular-frame-sequence-number).
  ```
- **Current State**: needs-impl (test exists in `test_message_body_format.rs`)
- **Note**: Fulfilled by `read_and_decrypt_framed_message_body` which uses the same `read_u32(r, raw)` for both regular and final frame sequence numbers.

---

### GROUP C: Misquoted Annotations (4 fixes)

#### Requirement C1 — regular-frame-sequence-number: serialized length 4 bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When serializing the sequence number to a message, the length of the serialized sequence number MUST be 4 bytes.
  ```
- **Current State**: FIX_ANNOTATION — body.rs line 114 misquotes as `The length of the serialized sequence number MUST be 4 bytes.`

#### Requirement C2 — regular-frame-sequence-number: read as UInt32
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When reading the sequence number from a message, the sequence number MUST be interpreted as a UInt32.
  ```
- **Current State**: FIX_ANNOTATION — body.rs line 117 misquotes as `The sequence number MUST be interpreted as a UInt32.`

#### Requirement C3 — final-frame-encrypted-content-length: serialized length 4 bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When serializing the encrypted content length to a message, the length of the serialized encrypted content length field MUST be 4 bytes.
  ```
- **Current State**: FIX_ANNOTATION — body.rs line 146 misquotes as `The length of the serialized encrypted content length field MUST be 4 bytes.`

#### Requirement C4 — final-frame-encrypted-content-length: read as UInt32
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a UInt32.
  ```
- **Current State**: FIX_ANNOTATION — body.rs line 148 misquotes as `The encrypted content length MUST be interpreted as a UInt32.`

---

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`

Non-framed decrypt function (where most Group A annotations go):
```rust
pub(crate) fn read_and_decrypt_non_framed_message_body(
    r: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    //= specification/data-format/message-header.md#frame-length
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    if header.body.frame_length() != 0 {
        return Err("Non-framed message contains non-zero frame length.".into());
    }
    let iv = serialize_functions::read_vec(r, get_iv_length(&header.suite) as usize, raw)?;
    let enc_content = serialize_functions::read_seq_u64_bounded(
        r,
        header::SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        raw,
    )?;
    let auth_tag = serialize_functions::read_vec(r, get_tag_length(&header.suite) as usize, raw)?;
```

Misquoted annotations in read path (Group C fixes):
```rust
    loop {
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# The length of the serialized sequence number MUST be 4 bytes.   // <-- WRONG
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# The sequence number MUST be interpreted as a UInt32.   // <-- WRONG
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs`

Existing test infrastructure (reuse for non-framed tests):
```rust
async fn round_trip(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}
```

Note: Non-framed data tests require a non-framed ciphertext. Since the ESDK only encrypts framed data, tests must either:
1. Use a pre-built non-framed ciphertext from test vectors, or
2. Construct a non-framed ciphertext manually in the test

## Implementation Guidance

### Group A: Non-Framed Data Annotations

**Write-path requirements (A1, A3, A4, A8, A9)**: The ESDK does NOT encrypt non-framed data. These need `type=exception` annotations with `reason=The ESDK only encrypts framed data per encrypt.md`. Place these in `body.rs` near the top of the file or in a compliance document.

**Read-path requirements (A2, A5, A6, A7, A10, A11, A12, A13, A14)**: These are fulfilled by `read_and_decrypt_non_framed_message_body`. Annotate at the specific code lines:

- A2 (deserialization order) → at the function entry, before the first `read_vec` call
- A5 (deserialize IV length bytes) → at `let iv = serialize_functions::read_vec(r, get_iv_length(...))`
- A6 (IV interpreted as bytes) → at the same `read_vec` call (returns `Vec<u8>`)
- A7 (max content length) → at `read_seq_u64_bounded(r, header::SAFE_MAX_ENCRYPT, ...)`
- A10 (content length as Uint64) → at the same `read_seq_u64_bounded` call
- A11 (content length matches field) → at the same `read_seq_u64_bounded` call (reads exactly that many bytes)
- A12 (content interpreted as bytes) → at the same `read_seq_u64_bounded` call (returns `Vec<u8>`)
- A13 (auth tag length matches algorithm) → at `let auth_tag = serialize_functions::read_vec(r, get_tag_length(...))`
- A14 (auth tag interpreted as bytes) → at the same `read_vec` call

Use `type=implication` for structural/interpretation requirements (A6, A10, A12, A14).

**Test annotations for non-framed read-path**: The ESDK's test vector infrastructure decrypts non-framed ciphertexts from other implementations. Check if existing test vector tests can be annotated, or create a dedicated test that constructs a non-framed ciphertext manually and decrypts it.

### Group B: Missing Implementation Annotations

- B1 (max frame size) → annotate at `FrameLength::new` in `types.rs` with `type=implication` and `reason=FrameLength wraps NonZeroU32 which has max value 2^32-1`
- B2 (unique IV) → annotate at `iv_seq(input.sequence_number, iv)` in `construct_frame` with `type=implication` and `reason=Each frame's IV is derived from its unique sequence number`
- B3 (regular frame content length = frame length) → annotate at `plaintext: &plaintext_frame` in the regular frame `construct_frame` call with `type=implication` and `reason=plaintext_frame is exactly frame_length bytes`
- B4 (final frame seq num = total frames) → annotate at `sequence_number` passed to final frame `construct_frame` call with `type=implication` and `reason=sequence_number is incremented for each frame and equals the total frame count at the final frame`
- B5 (final frame seq num serialized same as regular) → annotate at `write_u32(w, input.sequence_number)` in `construct_frame` with `type=implication` and `reason=construct_frame uses the same write_u32 for both regular and final frames`
- B6 (final frame seq num interpreted same as regular) → annotate at `read_u32(r, raw)` in the final frame branch of `read_and_decrypt_framed_message_body` with `type=implication` and `reason=read_u32 is used for both regular and final frame sequence numbers`

### Group C: Misquoted Annotation Fixes

Fix the 4 misquoted annotations in `body.rs` to match the exact TOML quotes. These are in the `read_and_decrypt_framed_message_body` function.

### Spec-Aligned Structure

Non-framed data deserialization flow:
1. Read IV (IV Length bytes) → annotate at `read_vec(r, get_iv_length(...))`
2. Read Content Length (8 bytes as Uint64) → annotate at `read_seq_u64_bounded(...)`
3. Read Content (Content Length bytes) → annotate at same `read_seq_u64_bounded` (it reads the content)
4. Read Auth Tag (tag length bytes) → annotate at `read_vec(r, get_tag_length(...))`

Non-framed data serialization (exception):
- Annotate all write-path requirements as `type=exception` with `reason=`

### Pattern References

Follow the existing annotation patterns in `body.rs`:
- `type=implication` with `reason=` for structural properties (see lines 90-96, 112-123)
- `type=exception` with `reason=` for requirements the ESDK intentionally does not implement

## Targeted Tests

For non-framed data tests, the test vector decrypt tests exercise the non-framed path. Check:
- `test_vectors/` test files for non-framed decrypt tests
- If none exist, create a test that manually constructs a non-framed ciphertext

For framed data, existing tests in `test_message_body_format.rs` already cover all Group B requirements (test annotations exist).

## Success Criteria
```bash
cargo test
make duvet
```
- [ ] All misquoted annotations fixed (Group C)
- [ ] All non-framed-data requirements annotated (Group A)
- [ ] All missing framed-data impl annotations added (Group B)
- [ ] duvet report shows no gaps for `message-body.md` sections: `#non-framed-data`, `#non-framed-data-iv`, `#non-framed-data-encrypted-content-length`, `#non-framed-data-encrypted-content`, `#non-framed-data-authentication-tag`, `#framed-data`, `#regular-frame-iv`, `#regular-frame-encrypted-content`, `#regular-frame-sequence-number`, `#final-frame-sequence-number`, `#final-frame-encrypted-content-length`
- [ ] All implementations have corresponding `type=test` (except `type=exception` and `type=implication`)
