[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Message Body AAD

## Version

See [Message Version](message.md#version).

## Implementations

- [C](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/cipher.c)
- [JavaScript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/serialize/src/aad_factory.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/internal/formatting/encryption_context.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/internal/Utils.java)

## Overview

The message body AAD is the serialization of the AAD to be used as input to encryption of the message body.

## Structure

The following describes the fields that form the message body AAD.
The bytes are appended in the order shown.

| Field            | Length (bytes) | Interpreted as |
| ---------------- | -------------- | -------------- |
| Message ID       | 16             | Bytes          |
| Body AAD Content | Variable.      | UTF-8 Bytes    |
| Sequence Number  | 4              | Uint32         |
| Content Length   | 8              | Uint64         |

### Message ID

An identifier for the [message](message.md) this message body AAD is associated with.

### Body AAD Content

An identifier for the content type of the data this message body AAD is associated with.

This value depends on the [content type](message-header.md#content-type) of the [message](message.md):

- [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
- The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
- The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.

### Sequence Number

The sequence number of the data this message body AAD belongs to.
For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#sequence-number).
For [non-framed data](message-body.md#non-framed-data), the value of this field MUST be `1`.

### Content Length

The length, in bytes, of the plaintext data being encrypted that this message body AAD is associated with.

More specifically, depending on the [content type](message-header.md#content-type) of the [message](message.md):

- For [non-framed data](message-body.md#non-framed-data), this value MUST equal the length, in bytes,
  of the plaintext data provided to the algorithm for encryption.
- For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes,
  of the plaintext being encrypted in this frame.
  - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of
    the [frame length](message-header.md#frame-length) field in the message header.
  - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to
    0 and less than or equal to the value of the [frame length](message-header.md#frame-length)
    field in the message header.
