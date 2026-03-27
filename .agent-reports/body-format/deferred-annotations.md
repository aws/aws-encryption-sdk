# Deferred Annotations

## Non-Framed Data Requirements (Reqs 1-14)

The Rust ESDK does not produce non-framed messages (`Implementations of the AWS Encryption SDK MUST NOT encrypt using the Non-Framed content type`). Testing non-framed deserialization requires constructing a valid non-framed ESDK message from raw bytes, which requires:
1. A valid ESDK header with ContentType::NonFramed and frame_length=0
2. A valid non-framed body (IV + encrypted content length as Uint64 + encrypted content + auth tag)
3. A valid footer (signature)

This is non-trivial and would require either:
- A test vector file containing a known non-framed message, or
- A manual message builder that constructs all header/body/footer bytes

These requirements should be addressed in a follow-up work item that adds non-framed test vector support.

### Deferred Requirements

1. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data`
   **Quote**: `Non-framed data MUST be serialized (written) as, in order, IV, Encrypted Content Length, Encrypted Content, and Authentication Tag.`
   **Reason**: Rust ESDK does not produce non-framed messages.

2. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data`
   **Quote**: `Non-framed data MUST be deserialized (read) as, in order, IV, Encrypted Content Length, Encrypted Content, and Authentication Tag.`
   **Reason**: Requires a non-framed test message to exercise the deserialization path.

3. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-iv`
   **Quote**: `When writing a message, the IV MUST be a unique IV within the message.`
   **Reason**: Rust ESDK does not produce non-framed messages.

4. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-iv`
   **Quote**: `When writing a message, the operation MUST serialize the IV to be [IV Length](message-header.md#iv-length) bytes.`
   **Reason**: Rust ESDK does not produce non-framed messages.

5. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-iv`
   **Quote**: `When reading a message, the operation MUST deserialize [IV Length](message-header.md#iv-length) bytes and interpret it as the IV.`
   **Reason**: Requires a non-framed test message.

6. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-iv`
   **Quote**: `When reading a message, the deserialized IV MUST be interpreted as bytes.`
   **Reason**: Requires a non-framed test message.

7. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content-length`
   **Quote**: `The length MUST NOT be greater than 2^36 - 32, or 64 gibibytes (64 GiB), due to restrictions imposed by the implemented algorithms.`
   **Reason**: Requires a non-framed test message; testing the upper bound is impractical.

8. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content-length`
   **Quote**: `When serializing the encrypted content length to a message, the length of the serialized encrypted content length MUST be 8 bytes.`
   **Reason**: Rust ESDK does not produce non-framed messages.

9. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content-length`
   **Quote**: `The encrypted content length MUST be serialized as a Uint64.`
   **Reason**: Rust ESDK does not produce non-framed messages.

10. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content-length`
    **Quote**: `When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a Uint64.`
    **Reason**: Requires a non-framed test message.

11. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content`
    **Quote**: `The length of the serialized encrypted content MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length) field.`
    **Reason**: Requires a non-framed test message.

12. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-encrypted-content`
    **Quote**: `The encrypted content MUST be interpreted as bytes.`
    **Reason**: Requires a non-framed test message.

13. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-authentication-tag`
    **Quote**: `The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.`
    **Reason**: Requires a non-framed test message.

14. **Spec target**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data-authentication-tag`
    **Quote**: `The authentication tag MUST be interpreted as bytes.`
    **Reason**: Requires a non-framed test message.
