[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Specify behavior for Raw AES Keyring for invalid input encryption context

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                                                                                                   |
| --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Raw AES Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/623992d999db0b309d8a8adbd664f0d72feee813/framework/raw-aes-keyring.md) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                                                                                                             |
| --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Raw AES Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/623992d999db0b309d8a8adbd664f0d72feee813/framework/raw-aes-keyring.md) |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

## Definitions

"serialize", "serialization", and "serializability" all refer to
[message header serialization of AAD key value pairs](../../data-format/message-header.md#key-value-pairs)

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The Raw AES Keyring must serialize the encryption context for use as AAD.
The specification did not describe how the keyring should behave when given an unserializable encryption context.

With this change, the Raw AES Keyring MUST attempt to serialize the encryption context,
prior to attempting encryption/decryption.
If the Raw AES Keyring cannot serialize the encryption context, it MUST fail.

## Out of Scope

- All keyrings aside from the Raw AES Keyring
- The encryption context serialization format

## Motivation

The C and Javascript implementations of the Raw AES Keyring check for serializability at different points in OnDecrypt.
This has the potential to lead to different behaviors under specific circumstances.

## Drawbacks

Since we attempt to serialize the encryption context before attempting any decryption,
the Raw AES Keyring may attempt to serialize an encryption context in cases where no EDK matches the keyring.
This scenario results in slower operation when compared to the current Javascript implementation,
which only checks serializability when attempting to decrypt an EDK.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

This change will improve behavioral consistency between Raw AES Keyring implications.
This should improve the customer experience and reduce support engagements.

## Guide-level Explanation

Serialize the encryption/decryption materials' encryption context before attempting encryption/decryption.
If serialization fails, the OnEncrypt/OnDecrypt operation MUST fail.
When encrypting/decryption the datakey, use the already serialized encryption context as the AAD.

## Reference-level Explanation

### OnEncrypt

Before encrypting the plaintext datakey, OnEncrypt MUST attempt to serialize the encryption context.
If serialization fails, OnEncrypt MUST fail.
This is not a new change, but is more explicit now.

### OnDecrypt

Before iterating through the list of EDKs, OnDecrypt MUST attempt to serialize the encryption context.
If serialization fails, OnDecrypt MUST fail.
If serialization succeeds,
the resulting serialized encryption context MUST be used as additional authenticated
data (AAD) for any AES decrypt operations the keyring attempts.
