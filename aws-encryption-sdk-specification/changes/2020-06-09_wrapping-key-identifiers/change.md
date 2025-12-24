[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Clarify Wrapping Key Identifiers

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                                             |
| --------------------------------------------------------------------------------------------------- |
| [Encrypted Data Key](../../framework/structures.md#encrypted-data-key)                              |
| [AWS KMS Keyring](../../framework/aws-kms/aws-kms-keyring.md)                                       |
| [Raw AES Keyring](../../framework/raw-aes-keyring.md)                                               |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md)                                               |
| [Keyring Decryption Contract](https://github.com/awslabs/aws-encryption-sdk-specification/pull/131) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                 |
| ------------------------------------------------------------- |
| [Structures](../../framework/structures.md)                   |
| [Keyring Interface](../../framework/keyring-interface.md)     |
| [AWS KMS Keyring](../../framework/aws-kms/aws-kms-keyring.md) |
| [Raw AES Keyring](../../framework/raw-aes-keyring.md)         |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md)         |

## Affected Implementations

The scope of this change only affects the specification.
Follow-up changes that define
whether implementations expose these concepts
MAY affect implementations.

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

We have referenced several terms throughout various specification documents
that are never defined:
"key namespace",
"key name",
"key provider ID",
and "key provider info".
This change defines each of these terms
and describes their relationships to each other
and the keyrings they represent.

## Out of Scope

- Whether, or how, each keyring exposes any of these values at runtime is out of scope.

- Specific values for these terms for any specific keyring is out of scope.

## Motivation

Various specification documents reference
"key namespace",
"key name",
"key provider ID",
and "key provider info",
but we never define what these terms mean
or how they relate to each other.

In order to ensure the specification documents accurately describe keyring behavior,
we need to define all terms that they use.

## Drawbacks

This change SHOULD NOT introduce any drawbacks.
It is identifying and describing
terms that already exist
and the existing relationship between them.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

This change SHOULD NOT have any operational implications.

## Guide-level Explanation

Key namespace and key name are configuration values that determine the behavior of a keyring.

Key provider ID and key provider info are values that identify the keyring configuration
that can fulfill the keyring's decryption contract.
The keyring attaches these values to a data key ciphertext to form an encrypted data key.

## Reference-level Explanation

"Key namespace", "key name", "key provider ID", and "key provider info"
are all concepts that identify a wrapping key.

### key namespace

A configuration value for a keyring
that identifies the grouping or categorization
for the wrapping keys that keyring can access.

The key namespace MUST be a string value.

### key name

A configuration value for a keyring
that identifies a single wrapping key
within a key namespace.

The key name MUST be a string value.

### key provider ID

An output value returned by a keyring on encrypt
as part of an encrypted data key structure
that identifies the grouping or categorization
for a keyring that can fulfill this decryption contract.

The key provider ID MUST be a binary value
and SHOULD be equal to a UTF-8 encoding of the key namespace.

### key provider info

An output value returned by a keyring on encrypt
as part of an encrypted data key structure
that provides necessary information for a keyring
to fulfill this decryption contract.

The key provider info MUST be a binary value
and SHOULD be equal to a UTF-8 encoding of the key name.

One example of a keyring where the key name and key provider info can differ
is the AWS KMS keyring.
This keyring uses its key name to identify the desired CMK in its call to AWS KMS.
However, the key provider info that this keyring writes is
the CMK identifier that AWS KMS includes in its response.
If the key name is a CMK ARN, these two values are identical
because this response value is always the CMK ARN of the CMK that AWS KMS used.
However, if the key name is some other valid CMK identifier,
such as an alias,
then they are different.
