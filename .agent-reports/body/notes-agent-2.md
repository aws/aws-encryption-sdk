# Pre-Implementation Reasoning: decrypt-the-message-body tests

## 1. Logical steps in this spec section

1. Deserialize body bytes according to message body spec (framed vs non-framed based on content type)
2. For framed data: read first 4 bytes to determine frame type (regular vs final)
3. If 0xFFFFFFFF → final frame; otherwise → regular frame
4. Validate final frame content length <= frame length
5. Decrypt and authenticate each frame using AES-GCM with specific inputs (AAD, IV, key, ciphertext, tag)
6. Sequence numbers: first frame = 1, subsequent = prev + 1
7. Content length in AAD = length of plaintext encrypted
8. On decryption failure → halt and fail
9. Do not release unauthenticated plaintext
10. Streaming: release regular frame plaintext after tag verification (with signing suite)

## 2. Point of fulfillment for each requirement

- Req 1 (regular frame deserialization): fulfilled by successful round-trip of multi-frame message
- Req 2 (final frame deserialization): fulfilled by successful round-trip proving final frame parsed
- Req 3 (first 4 bytes determine frame type): fulfilled by multi-frame decrypt succeeding
- Req 4 (0xFFFF → final frame): fulfilled by single-frame decrypt (only final frame)
- Req 5 (otherwise → regular frame): fulfilled by multi-frame decrypt (regular frames parsed)
- Req 6 (final frame content length <= frame length): fulfilled by tampering content length to exceed frame length
- Req 7 (decrypt and authenticate): fulfilled by round-trip decrypt
- Req 8 (first frame seq=1): fulfilled by single-frame decrypt
- Req 9 (subsequent seq = prev+1): fulfilled by multi-frame decrypt
- Req 10 (content length in AAD): fulfilled by round-trip (wrong content length would fail auth)
- Req 11 (decryption fails → halt): fulfilled by tampered auth tag causing error
- Req 12 (no unauthenticated plaintext): fulfilled by tampered ciphertext producing no output
- Req 13 (SHOULD release regular frames after tag verification): fulfilled by multi-frame round-trip with signing suite

## 3. Sub-items

Req 7 has sub-items (AAD, IV, cipherkey, ciphertext, tag) but these are tested implicitly via round-trip.

## 4. Reviewer readability

Each test function covers 1-2 requirements. Annotations are placed near the assertion that proves the requirement.

## 5. Existing similar code

- `tests/test_construct_the_body.rs` — same pattern of test_keyring(), encrypt_with_frame_length(), round_trip()
- `tests/test_construct_a_frame.rs` — same helpers
- `tests/test_decrypt_behavior.rs` — decrypt test pattern with signing suite
