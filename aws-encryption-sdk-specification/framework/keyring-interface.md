[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Keyring Interface

## Version

0.2.4

### Changelog

- 0.2.4

  - Update list of AWS KMS Keyrings

- 0.2.3

  - Fix stated behavior if the keyring takes no action
    to conform with [how to fail](../changes/2020-06-04_how-to-fail-with-keyrings/change.md).

- 0.2.2

  - [Define wrapping key identifier terms](../changes/2020-06-09_wrapping-key-identifiers/change.md)

- 0.2.1

  - [Clarify naming of KMS to AWS KMS](https://github.com/awslabs/aws-encryption-sdk-specification/issues/67)

- 0.2.0

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                             |
| ---------- | -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| C          | 0.1.0-preview                          | 0.1.0                     | [materials.c](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/materials.c)                                                  |
| Javascript | 0.1.0-preview                          | 0.1.0                     | [keyring.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/keyring.ts)              |
| Python     | 0.1.0-preview                          | n/a                       | [keyrings/base.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/keyrings/base.py)                   |
| Java       | 0.1.0-preview                          | n/a                       | [Keyring.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/keyrings/Keyring.java) |

## Overview

Keyrings are responsible for the generation, encryption, and decryption of data keys.

The keyring interface specified in this document describes the interface all keyrings MUST implement.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### key namespace

A configuration value for a keyring
that identifies the grouping or categorization
for the wrapping keys that the keyring can access.

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

This value MUST NOT be or start with "aws-kms"
unless this encrypted data key was produced by one of the [AWS KMS Keyrings](./aws-kms/).

### key provider info

An output value returned by a keyring on encrypt
as part of an encrypted data key structure
that provides necessary information for a keyring
to fulfill this decryption contract.

The key provider info MUST be a binary value
and SHOULD be equal to a UTF-8 encoding of the key name.

## Supported Keyrings

- [AWS KMS Keyrings](./aws-kms/)
  - [AWS KMS Keyring](./aws-kms/aws-kms-keyring.md)
  - [AWS KMS RSA Keyring](./aws-kms/aws-kms-rsa-keyring.md)
  - [AWS KMS Multi Keyrings](./aws-kms/aws-kms-multi-keyrings.md)
  - [AWS KMS Discovery Keyring](./aws-kms/aws-kms-discovery-keyring.md)
  - [AWS KMS MRK Discovery Keyring](./aws-kms/aws-kms-mrk-discovery-keyring.md)
  - [AWS KMS MRK Keyring](./aws-kms/aws-kms-mrk-keyring.md)
  - [AWS KMS MRK Multi Keyrings](./aws-kms/aws-kms-mrk-multi-keyrings.md)
  - [AWS KMS Hierarchical Keyring](./aws-kms/aws-kms-hierarchical-keyring.md)
- [Multi-Keyring](multi-keyring.md)
- [Raw AES Keyring](raw-aes-keyring.md)
- [Raw RSA Keyring](raw-rsa-keyring.md)

## Interface

### OnEncrypt

This interface MUST take [encryption materials](structures.md#encryption-materials) as input.
It MUST modify it with at least one of the following behaviors:

- [Generate data key](#generate-data-key)
- [Encrypt data key](#encrypt-data-key)

If this keyring attempted any of the above behaviors, and successfully completed those behaviors,
it MUST output the modified [encryption materials](structures.md#encryption-materials).

If the keyring did not attempt any of the above behaviors, it MUST fail
and it MUST NOT modify the [encryption materials](structures.md#encryption-materials).

The keyring SHOULD NOT attempt to store the encryption context
in the [encrypted data key's](structures.md#encrypted-data-key) properties.
This is especially important given the `Required Encryption Context Keys`
that exists on [encryption materials](structures.md#encryption-materials) and [decryption materials](structures.md#decryption-materials)

#### Generate Data Key

If the [encryption materials](structures.md#encryption-materials) do not contain a plaintext data key,
OnEncrypt MUST generate a data key.
If the encryption materials contain a plaintext data key, OnEncrypt MUST NOT generate a data key.

Generate Data Key MUST modify the following fields in the [encryption materials](structures.md#encryption-materials):

- [plaintext data key](structures.md#plaintext-data-key)

To perform this behavior, the keyring generates a [plaintext data key](structures.md#plaintext-data-key)
and sets the resulting plaintext data key on the [encryption materials](structures.md#encryption-materials).

The length of the output plaintext data key MUST be equal to the KDF input length of the [algorithm suite](algorithm-suites.md)
specified in the [encryption materials](structures.md#encryption-materials).
The value of the plaintext data key MUST consist of cryptographically secure (pseudo-)random bits.

Note: If the keyring successfully performs this behavior, this means that the keyring MAY then
perform the [Encrypt Data Key](#encrypt-data-key) behavior.

#### Encrypt Data Key

If the [encryption materials](structures.md#encryption-materials) contain a plaintext data key,
OnEncrypt MUST encrypt a data key.
If the encryption materials do not contain a plaintext data key, OnEncrypt MUST NOT encrypt a data key.

Encrypt Data Key MUST modify the following fields in the [encryption materials](structures.md#encryption-materials):

- [encrypted data keys](structures.md#encrypted-data-keys)

To perform this behavior, the keyring creates one or more [encrypted data keys](structures.md#encrypted-data-key)
using the plaintext data key from the [encryption materials](structures.md#encryption-materials) as input,
and appends the [encrypted data keys](structures.md#encrypted-data-key) to the encrypted data key list
in the [encryption materials](structures.md#encryption-materials).

The [encrypted data keys](structures.md#encrypted-data-key) produced by this keyring MUST
have [ciphertexts](structures.md#ciphertext) that can be decrypted to the plaintext data key in the
[encryption materials](structures.md#encryption-materials).

### OnDecrypt

This interface MUST take [decryption materials](structures.md#decryption-materials) and
a list of [encrypted data keys](structures.md#encrypted-data-key) as input.
It MUST modify it with the following behavior:

- [Decrypt data key](#decrypt-data-key)

If the decryption materials already contain a plaintext data key,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

If this keyring attempted the above behavior, and succeeded, it MUST output the modified [decryption materials](structures.md#decryption-materials).

If the keyring did not attempt the above behavior,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

#### Decrypt Data Key

If the encryption materials do contain a plaintext data key, OnDecrypt MUST NOT decrypt a data key.
If the [decryption materials](structures.md#decryption-materials) do not include a plaintext data key,
OnDecrypt MUST decrypt a data key.

The decrypt data key MUST modify the following fields in the [decryption materials](structures.md#decryption-materials):

- [Plaintext data key](structures.md#plaintext-data-key-1)

To perform this behavior, the keyring attempts to retrieve a plaintext data key from the input list
of [encrypted data keys](structures.md#encrypted-data-key).

If the keyring is able to successfully get at least one plaintext data key from any [encrypted data key](structures.md#encrypted-data-key)
and the [decryption materials](structures.md#decryption-materials) still do not include a plaintext data key,
it SHOULD set one resulting plaintext data key on the [decryption materials](structures.md#decryption-materials).

If the keyring is unable to get any plaintext data key using the input [encrypted data keys](structures.md#encrypted-data-key),
the keyring MUST NOT not update the [decryption materials](structures.md#decryption-materials) and MUST return failure.

## Security Considerations

Keyring implementations SHOULD provide integrity guarantees for the [encrypted data keys](structures.md#encrypted-data-key)
they return on [OnEncrypt](#onencrypt) such that tampered versions of those encrypted data keys,
if input into [OnDecrypt](#ondecrypt), are overwhelmingly likely to cause a decryption failure
(i.e. the chance of a successful decryption in this case is negligible).

Such integrity guarantees SHOULD include the integrity of the [encryption context](structures.md#encryption-context)
such that, if the encryption context used as input to OnEncrypt to produce an encrypted data key is
different than the encryption context input to OnDecrypt to decrypt that encrypted data key,
the decryption is overwhelmingly likely to fail.

Users SHOULD use a keyring that protects wrapping keys and performs cryptographic operations within a secure boundary.
Examples are:

- The built-in [AWS KMS keyrings](./aws-kms/),
  which use AWS Key Management Service (AWS KMS) customer master keys (CMKs) that never leave AWS KMS as plaintext.
- A custom keyring that uses wrapping keys that are stored in your hardware security modules (HSMs)
- A custom keyring protected by another master key service.

The [raw AES keyring](raw-aes-keyring.md) and [raw RSA keyring](raw-rsa-keyring.md) MAY be used,
however users should refer to their specification for notes on their respective security considerations.

## Appendix

### Keyring and Master Key Provider/Master Key Compatability

The following keyrings are compatible with the referenced [master key providers](master-key-provider-interface.md) or
[master keys](master-key-interface.md) when configured to use the same wrapping key.

| Keyring          | Master Key Provider: Java and Python                                                                             |
| ---------------- | ---------------------------------------------------------------------------------------------------------------- |
| AWS KMS keyrings | KMS master key (Java), KMS master key provider (Java), KMS master key (Python), KMS master key provider (Python) |
| Raw AES keyring  | When they are used with symmetric encryption keys: JceMasterKey (Java), RawMasterKey (Python)                    |
| Raw RSA keyring  | When they are used with asymmetric encryption keys: JceMasterKey (Java), RawMasterKey (Python)                   |

### Why should I use Keyrings instead of Master Key Providers and Master Keys?

Keyrings provide a simplified architecture over master keys and master key providers.
The keyring combines the similar responsibilities of master keys and master key providers into one concept,
as well as removes all key management logic from [cryptographic materials managers](cmm-interface.md).

Due to this simplified architecture, master keys and master key providers are going to be deprecated in the future,
and new implementations SHOULD use keyrings.
