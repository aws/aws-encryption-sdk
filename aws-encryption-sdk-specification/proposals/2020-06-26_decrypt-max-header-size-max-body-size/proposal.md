[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Decrypt Inputs Max Body Size and Max Header Size

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                 |
| --------------------------------------- |
| [Decrypt](../../client-apis/decrypt.md) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                           |
| --------------------------------------- |
| [Decrypt](../../client-apis/decrypt.md) |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| Python     | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python)         |
| Java       | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)             |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

Processing ESDK messages of unknown size introduces a risk of highly-bounded resource consumption
due to variable length fields in the message format.
This is bad for users that need to decrypt such messages,
especially for such users that care about the memory usage of their systems.

Most ESDK implementations have a way to mitigate some of these concerns through
user specified controls on the Decrypt operation,
however these controls are not described in our specification
and are not consistent across implementations.

This change clearly defines the specification for two inputs,
`Max Body Size` and `Max Header Size`,
on Decrypt which provide a way for users to put more restrictive bounds
on the messages the ESDK will process.

## Out of Scope

- Providing a control that is a 1:1 memory limit on
  what the ESDK MUST hold in memory in order to perform its operations is out of scope.
  The goal is to provide a control that is precise
  (behaves consistently and scales in expected ways),
  not a control that is accurate (closely resembles the actual memory usage of the ESDK).

- Updating the logic of the ESDK to not hold objects
  in memory for the duration of the decryption operation is not in scope.

- Additional controls to limit the memory used to process messages with invalid signature lengths
  is out of scope.
  The bounds of the signature length field are small enough (2^16 - 1 bytes)
  that they are not a concern.

## Motivation

ESDK implementations provide some controls that mitigate high resource consumption
when decrypting messages.
However, none of these mitigations behave consistently across implementations
and none of them bound the fields in the message header specifically.

This change proposes two standard controls
that bound resource consumption.

## Drawbacks

- Requires users to use two controls to mitigate high resource consumption concerns.
- Requires users to be aware of the message structure
  and requires them to consider what messages they expect when decrypting.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

This change adds operational controls that more strictly bound some fields in the
message header.
Users who are especially concerned with the memory usage of their systems should consider
setting these controls to restrict resource consumption.

This change breaks Javascript and Node.js users who currently set `Max Body Size`
and expect a message with an encrypted content field length of exactly `Max Body Size`
to fail.
This change makes `Max Body Size` inclusive,
allowing encryption content fields of that exact length.

## Guide-level Explanation

Specify two optional inputs `Max Body Size` and `Max Header Size` for Decrypt.
`Max Body Size` sets a limit on the maximum encryption content length
the ESDK allows in a message
and `Max Header Size` sets a limit on the maximum header length
the ESDK allows in a message.

The intention of these controls is to provide a way for users to
bound the resources the ESDK can consume in order to mitigate
high resource consumption when decrypting a large message.

### Why do users care about these controls?

In order to decrypt encrypted messages,
the ESDK MUST buffer variable length fields from the ESDK message format.
Some variable length fields can be large enough that they
require a significant amount of memory to process.

The message format contains the following variable length fields with maximum bounds high enough to be of concern:

1.  The Encrypted Content field in an unframed message.
    The message format allows this to be up to 2^36 - 32 bytes (~64 GiB).
    For some implementations this is further restricted
    (e.g. Java arrays cannot have more than 2^31 - 1 members).
    In order to decrypt an unframed message,
    the ESDK MUST buffer the entire Encrypted Content field.
    In order to release the plaintext to the user safely,
    the ESDK MUST hold both the plaintext and the ciphertext in memory at the same time.
    Assuming an encryption algorithm where the length of the output equals the length of the input,
    this bumps the memory requirement to process this by a factor of 2.
1.  The Encrypted Content field in a frame in a framed message.
    The message format allows this to be up to 2^32 - 1 bytes (4 GiB).
    In order to decrypt a frame, the ESDK MUST buffer the entire Encrypted Content field in that frame.
    In order to release the plaintext to the user safely,
    the ESDK MUST hold both the plaintext and the ciphertext in memory at the same time.
    Assuming an encryption algorithm where the length of the output equals the length of the input,
    this bumps the memory requirement to process this by a factor of 2.
1.  The Encrypted Data Keys field in the message header.
    The message format allows this to be up to `(2^16 - 1) * (6 + 3 * (2^16 - 1))` bytes (~12 GiB).
    As such, 12 GiB represents a lower bound on the amount of memory required to
    process the largest EDK set the message format allows.
    The actual memory required depends on how a specific language and implementation
    represents these objects in memory.
    These objects MUST be held in memory for the duration of the decryption operation
    because the decrypt operation MUST output the parsed header.

Messages with the above fields at their size limits, when processed,
can exhaust many memory runtime limits.
As such, customers who are concerned with the stability and memory usage of their systems
may care about providing stricter bounds to these fields.

The Encryption Context field in the header
and the Signature Length field in the footer are also variable length
but they cannot exceed 2^16 - 1 bytes (64 KiB).
This is well within the bounds of memory runtime limits for the most languages,
so we do not consider the bounds of these fields a concern.

Note that all implementations dynamically allocate resources while processing messages.

### When should users set this value?

Users should set this value if they need to process
messages of unknown size
or messages from an untrusted source.

### When should users NOT set this value?

This control should be considered strictly an operational control that can
mitigate highly-bounded resource consumption for certain use cases.

If a user has a security requirement to not decrypt messages with a certain content length
or message header length
(ex, to enforce a “sign only” use case by only decrypting zero length messages),
they SHOULD meet that security requirement using
a custom CMM that only allows desired requests.

### What value should they set it to?

Users SHOULD choose reasonable values for their use case and
the messages that they expect to decrypt.
They can expect the memory usage of the ESDK to scale linearly with this control,
and SHOULD performance test their application to determine
the ESDKs memory cost in practice for their use case.

### Why not just provide one control that “does the right thing” for most use cases?

The two controls mitigate two different resource consumption concerns.
`Max Body Size` limits short term memory consumption.
This can be used to ensure that the decryption of a frame or
an unframed message doesn't immediately exhaust memory linear to a factor of that value.

`Max Header Size` has a fuzzier implication on memory that
depends on the implementation
and represents a memory requirement that exists for the duration of the decryption operation.

Separating these controls
keeps the purpose and affect of each control clear and simple,
so that they can be easily understood and applied.

### Should there be a lower limit for Max Header Size?

To allow for the smallest possible message header
(no AAD, one EDK but the EDK fields themselves are empty),
the `Max Header Size` MUST NOT be less than 40 bytes.
However, because future message format revisions might have different lower limits,
we cannot know if a limit is too low until we start processing a message.

### Will this value still be valid if/when we update the message format?

This control does not introduce any one-way doors that would limit
how we might update the message format in the future.

Conceptually, this should apply nicely to any new message format.
With a new message format, our concerns about memory limits should be similar.
There is no way to avoid processing a whole frame before releasing any plaintext,
meaning that we will always want some control on the encrypted content size.
The message header will always contain some metadata that the ESDK might need to hold in memory,
meaning that a control on the header size would still be useful.

## Reference-level Explanation

Specify two optional inputs on Decrypt, `Max Header Size` and `Max Body Size`.

### Max Body Size

If `Max Body Size` is set on Decrypt, then the operation:

- MUST halt and fail if processing an unframed message with encrypted content field
  of a length greater than (>) Max Body Size, and MUST NOT attempt decryption on such a message.
  - This means that when decrypting an unframed message,
    if Max Body Size is set then the ESDK MUST fail if the encrypted content length field
    in the message body is greater than (>) Max Body Size.
- MUST halt and fail if processing a message frame with an encrypted content field
  of a length greater than (>) Max Body Size and MUST NOT attempt decryption on any such frame.
  - This means that when decrypting a regular frame in a framed message,
    if Max Body Size is set then the ESDK MUST fail if the frame length
    set in the header is greater than Max Body Size.
  - This means that when decrypting a final frame in a framed message,
    if Max Body Size is set then the ESDK MUST fail if the encrypted content length field
    set in the final frame is greater than Max Body Size.
  - Note that there is an edge case for messages with bodies that only consist of a final frame.
    If a message body contains a single frame,
    that final frame MAY have a content length less then (<) the frame size in the header.
    For this reason, the implementation MUST NOT fail
    based on the frame size in the message header alone
    and MUST check the first frame in the message.
    Such a message is valid because the Frame Size is set on encrypt
    while Max Body Size is set only on decrypt.

### Max Header Size

If `Max Header Size` is set on Decrypt, then the operation:

- MUST halt and fail if processing a message with a message header of a length
  greater than (>) `Max Header Size`,
  and MUST NOT attempt to parse greater than (>) `Max Header Size` bytes of a message header.
  - Note that the message header does not contain a field that describes its total length,
    and thus implementations MUST attempt to process the message header until
    it either succeeds parsing the message header
    or it can determine that the total length of the header exceeds `Max Header Size` and fails.
    For example, if the amount of bytes it would take to process the next field in the header
    would make the total bytes in the header exceed `Max Header Size`,
    then the ESDK MUST immediately fail instead of attempting to process those bytes.
