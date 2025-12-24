[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Clarify Streaming Encrypt and Decrypt

## Affected Features

This change affects no features.

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                           |
| --------------------------------------- |
| [Encrypt](../../client-apis/encrypt.md) |
| [Decrypt](../../client-apis/decrypt.md) |

## Affected Implementations

This change affects no implementations.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### Authenticated Data

Plaintext or associated data is considered authenticated if the associated
[authentication tag](../data-format/message-body.md#authentication-tag) is successfully checked
as defined by the algorithm suite indicated in the message header.

### Signed Data

Plaintext and associated data is considered signed if the associated [message signature](../data-format/message-footer.md)
is successfully verified using the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
of the algorithm suite indicated in the message header.

### Consumable Bytes

In the scope of an operation, bytes are considered consumable if:

- The operation has not yet processed those bytes.
- The operation has access to those bytes.
- Those bytes are intended to be processed.
  This intention is expressed through the specific streaming interface.

For example, in a framework where a customer is sending input bytes to an operation
and that operation must write the output bytes to some sink,
the input bytes received from the customer are considered consumable.
Here the customer is expressing intent to process their supplied bytes.

For a framework where a customer is requesting output bytes from an operation
and that operation must read from some source in order to produce bytes,
this is slightly more complicated.
Bytes are considered consumable if:

- Those bytes have not yet been processed.
- Those bytes are able to be read by the operation from the source.
- Those bytes are required to be processed in order for the operation
  to release the output requested by the customer.
  Here the customer expresses intent for the operation to process
  whatever the operation needs to consume to produce its complete output

### Release

An operation releases bytes when the operation intends those bytes to be considered output.

For example, in a framework where a customer is sending input bytes to an operation
and that operation must write the output bytes to some sink,
bytes are considered released once the operation writes those bytes into the sink.

For a framework where a customer is requesting output bytes from an operation
and that operation must read from some source in order to produce bytes,
bytes are considered released once those bytes are available to be read by the customer.

If bytes are processed by an operation, that does not imply that the operation is allowed to
release any result of that processing.
The decrypt and encrypt operations specify when output bytes MUST NOT be released
and when they SHOULD be released.

## Summary

We specify how to perform encryption and decryption in
the streaming context,
specifically specifying what processing MUST occur when,
and what conditions are REQUIRED before releasing output.

## Out of Scope

Any feature updates are out of scope for this change.

## Motivation

The Encrypt and Decrypt specifications do not currently make it clear how these operations
MUST be performed in order to support streaming use cases.
This change clarifies how to operate on partial data
and what conditions are REQUIRED to release output.

## Drawbacks

There SHOULD be no drawbacks for specifying the streaming use case.

## Security Implications

There SHOULD be no security implications for specifying the streaming use case.

## Operational Implications

There SHOULD be no operational implications for specifying the streaming use case.

## Guide-level Explanation

This change clarifies the specification of the encrypt and decrypt operations,
focusing on defining their behavior in the streaming use cases.

This change introduces no new features for customers,
however it does clarify important concepts around the release
of authenticated and signed data for the decrypt streaming use case.

The Decrypt operation MUST NOT release any unauthenticated data.

In the case of streaming messages with signatures,
any plaintext released by the Decrypt operation MUST NOT be
considered signed data until the operation completes and succeeds.

This means that customers that process such released plaintext MUST NOT consider any processing successful
until this operation completes successfully.
Additionally, if this operation fails, customers MUST discard the released plaintext and encryption context
and MUST rollback any processing done due to the released plaintext or encryption context.

This change adds these security considerations in the Decrypt specification.

## Reference-level Explanation

The following sections describe how this change is updating the specification
to answer important questions for implementors of the specification.

### How should the operation process data in parts?

This change rewrites these specifications to
better specify how these operations are allowed to process data in parts.
Specifically, we attempt to be completely language agnostic and describe streaming as a way for
bytes to be read or written sequentially, over time.

With this framing, the things the encrypt and decrypt operations care about are:

- Can any more data be read or written?
- What data is currently consumable?
- What data can I release?

To do this we precisely define what we mean by [consume](#consumable-bytes) and [release](#release),
as well as the overall framework for thinking about streaming in a new streaming document.

In the context of the encrypt or decrypt operation, we specify the conditions
for when the operation MUST perform some processing.
These conditions include how many input bytes are consumable,
whether more bytes MAY be consumable,
at what step in the operation you are in,
and the result of various calculations.

Additionally, we use similar conditions to specify when the output of the operation MUST NOT
or SHOULD be released.

This change SHOULD be agnostic to streaming interface implementations.
Any streaming interface which is capable of moving bytes over time from one place to another,
and is able to indicate an end to the bytes MUST be able to implement this specification as written.

### How should verification and signing be performed?

For one step decryption or encryption,
where the entire message is available in memory,
verification and signing can be performed in one step,
after the message header and message body has been serialized or
deserialized.

For streaming use cases,
in order to not require holding the entire message in memory to either create or verify a signature,
this change specifies that streaming use cases input the message
to the signature algorithm in parts, as it is being parsed or serialized.

As such, this change states that once the header or a frame is serialized or parsed,
it SHOULD be inputted to the signature algorithm (if included) such that it
isn't required to remain in memory for the duration of the operation.

If an implementation is unable to do this,
it SHOULD NOT extend an API that supports streaming.

### How and when should plaintext be released?

The ESDK MUST NOT release any unauthenticated plaintext.
If the plaintext comes from a message with a signature,
the answer depends on whether the decryption is being streamed
or done in one step.

For one step decryption,
where the entire message is available in memory,
it is obvious that the operation MUST NOT release any plaintext until the
signature (if included) is verified.

For the streaming case, this gets more complicated.
Streaming is a way to operate on large messages without requiring the entire message to be
loaded into memory.
This allows the processing of arbitrarily long messages with finite memory.
However, when processing messages with a signature,
the operation obtains plaintext in parts as it processes the message,
but has no way of knowing whether the message signature can be verified until the end of the operation.
So what should we do with the plaintext decrypted thus far?
Because streaming is being used, we MUST NOT buffer the entire plaintext,
as that would defeat the point of streaming.
Thus, the plaintext needs to go somewhere other than memory.
Because this is a client-side library, we MUST give it to the customer.

The customer MUST understand that all released plaintext cannot be considered signed data
until the operation completes and succeeds.
The customer MUST discard any released plaintext if the operation fails, and MUST roll back
any processing done with that released plaintext.
This change states these security considerations.

We should consider improvements to the format to further reduce or eliminate the need to release
such plaintext in the streaming use case, however that is outside the scope of this document.

### How and when does the header information become available?

Any information from the header MUST NOT be released before the message header is authenticated.

In the one step case, the operation MUST verify the whole message before outputting the header information.

In the streaming case,
the operation SHOULD release header information before the message signature is verified,
and it MUST NOT be interperted as signed data until the operation completes successfully.

This is important to release because a common use case for the header information is
to check the data within it in order to ensure that the correct thing is being decrypted.

Requiring customers to make such a check after already performing decryption on a large message
could be very expensive, when they could instead immediately see possibly unexpected data within
the header information.

This change specifies the encryption context, algorithm suite ID, and parsed header as an output,
and makes the above security considerations clear.
