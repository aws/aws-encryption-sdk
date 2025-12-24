[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Structures

## Version

0.6.0

### Changelog

- 0.6.0

  - Update keystore structure and add encryption context options

- 0.5.0

  - Rename Hierarchical Materials to Branch Key Materials.
  - Add Beacon Key Materials

- 0.4.0

  - Add symmetric signature keys to materials
  - Add Hierarchical Materials.

- 0.3.0

  - Clarify handling of the `aws-crypto-public-key` encryption context key.

- 0.2.1

  - [Clarify naming of KMS to AWS KMS](https://github.com/awslabs/aws-encryption-sdk-specification/issues/67)

- 0.2.0

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Overview

This document includes the specifications for common structures referenced throughout the
AWS Encryption SDK specification.
These structures define a group of related fields that MUST hold certain properties.
Wherever these structures are referenced in this specification,
implementations MUST ensure that all properties of a structure's fields are upheld.

Note that this specification does not specify how these structures should be represented or passed
throughout the AWS Encryption SDK framework.
While these structures will usually be represented as objects, lower level languages MAY represent
these fields in a less strictly defined way as long as all field properties are still upheld.

Structures defined in this document:

- [Encrypted Data Key](#encrypted-data-key)
- [Encryption Context](#encryption-context)
- [Encryption Materials](#encryption-materials)
- [Decryption Materials](#decryption-materials)
- [Branch Key Materials](#branch-key-materials)
- [Beacon Key Materials](#beacon-key-materials)

### Encrypted Data Key

#### Implementations

- [C](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/edk.h)
- [Javascript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/encrypted_data_key.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/structures.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/EncryptedDataKey.java)

#### Structure

An encrypted data key is comprised of the following fields:

- [Key Provider ID](#key-provider-id)
- [Key Provider Information](#key-provider-information)
- [Ciphertext](#ciphertext)

Note: "Encrypted" is a misnomer here, as the process by which a key provider may obtain the plaintext data key
from the ciphertext and vice versa does not have to be an encryption and decryption cipher.
This specification uses the terms "encrypt" and "decrypt" for simplicity,
but the actual process by which a key provider obtains the plaintext data key from the ciphertext
and vice versa MAY be any reversible operation, though we expect that most will use encryption.

##### Key Provider ID

The [key provider ID](keyring-interface.md#key-provider-id) value
for the keyring that wrote this encrypted data key.

##### Key Provider Information

The [key provider info](keyring-interface.md#key-provider-info) value
for the keyring that wrote this encrypted data key.

##### Ciphertext

An opaque value from which an appropriate key provider can obtain the plaintext data key.

Some key provider MUST be capable of deterministically obtaining the plaintext key from the ciphertext.

Most commonly this is an encrypted form of the plaintext data key.
Alternatively, it could be the public input to a KDF that derives the plaintext data key or
an identifier into a key store that will return the plaintext data key.

### Encryption Context

#### Implementations

- [C](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/enc_ctx.h)
- [Javascript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/types.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/internal/formatting/encryption_context.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/internal/EncryptionContextSerializer.java)

#### Structure

The encryption context is a key-value mapping of arbitrary, non-secret, UTF-8 encoded strings.
It is used during [encryption](../client-apis/encrypt.md) and [decryption](../client-apis/decrypt.md) to provide additional authenticated data (AAD).

Users SHOULD use the encryption context to store:

- Non-secret data that MUST remain associated with the [message](../data-format/message.md) ciphertext.
- Data that is useful in logging and tracking, such as data about the file type, purpose, or ownership.

Users MUST NOT use the encryption context to store secret data.

#### Serialization

If the encryption context is empty, its serialization MUST be
an empty byte sequence.

If the encryption context is not empty, its serialization MUST
take the following form:

| Field                                             | Length (bytes)                                                       | Interpreted as                                    |
| ------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------- |
| [Key Value Pair Count](#key-value-pair-count)     | 2                                                                    | UInt16                                            |
| [Key Value Pair Entries](#key-value-pair-entries) | Variable. Determined by the count and length of each key-value pair. | [Key Value Pair Entries](#key-value-pair-entries) |

###### Key Value Pair Count

The number of key-value pairs within the [Key Value Pair Entries](#key-value-pair-entries) field.
The value of this field MUST be greater than 0.

###### Key Value Pair Entries

A sequence of one or more key-value pair entries.

This sequence MUST NOT contain duplicate entries.

These entries MUST be sorted, by key,
in ascending order according to the UTF-8 encoded binary value.

The following table describes the fields that form each key value pair entry.
The bytes are appended in the order shown.

| Field        | Length (bytes)                                                                 | Interpreted as      |
| ------------ | ------------------------------------------------------------------------------ | ------------------- |
| Key Length   | 2                                                                              | UInt16              |
| Key          | Variable. Equal to the value specified in the previous 2 bytes (Key Length).   | UTF-8 encoded bytes |
| Value Length | 2                                                                              | UInt16              |
| Value        | Variable. Equal to the value specified in the previous 2 bytes (Value Length). | UTF-8 encoded bytes |

### Encryption Materials

#### Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                  |
| ---------- | -------------------------------------- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [materials.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/materials.h)                                                        |
| Javascript | 0.1.0-preview                          | 0.1.0                     | [cryptographic_material.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/cryptographic_material.ts)     |
| Python     | 0.1.0-preview                          | n/a                       | [materials_managers](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/materials_managers/__init__.py)                        |
| Java       | 0.1.0-preview                          | n/a                       | [EncryptionMaterials.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/model/EncryptionMaterials.java) |

#### Structure

Encryption materials are a structure containing materials needed for [encryption](../client-apis/encrypt.md).
This structure MUST include the following fields:

- [Algorithm Suite](#algorithm-suite)
- [Encrypted Data Keys](#encrypted-data-keys)
- [Encryption Context](#encryption-context-1)
- [Required Encryption Context Keys](#required-encryption-context-keys)

This structure MAY include any of the following fields:

- [Plaintext Data Key](#plaintext-data-key)
- [Signing Key](#signing-key)
- [Symmetric Signing Keys](#symmetric-signing-key)

##### Algorithm Suite

The [algorithm suite](algorithm-suites.md) to be used for [encryption](../client-apis/encrypt.md).

##### Encrypted Data Keys

A list of the [encrypted data keys](#encrypted-data-key) that correspond to the plaintext data key.

The [ciphertext](#ciphertext) of each encrypted data key in this list MUST be an opaque form of the
plaintext data key from this set of encryption materials.

If the plaintext data key is not included in this set of encryption materials, this list MUST be empty.

##### Encryption Context

The [encryption context](#encryption-context) associated with this [encryption](../client-apis/encrypt.md).

If an [encryption material](#encryption-materials) contains a [signing key](#signing-key),
the [encryption context](#encryption-context) SHOULD include the reserved key `aws-crypto-public-key`.
The mapped value from the reserved key `aws-crypto-public-key` SHOULD be the signature verification key
corresponding to the [signing key](#signing-key) stored on the [encryption material](#encryption-materials).
If an [encryption material](#encryption-materials) does not contains a [signing key](#signing-key),
the [encryption context](#encryption-context) SHOULD NOT include the reserved key `aws-crypto-public-key`.

##### Plaintext Data Key

A data key to be used as input for [encryption](../client-apis/encrypt.md).

The plaintext data key MUST:

- Fit the specification for the [key derivation algorithm](algorithm-suites.md#key-derivation-algorithm)
  included in this decryption material's [algorithm suite](#algorithm-suite).
- Consist of cryptographically secure (pseudo-)random bits.
- Be kept secret.

The plaintext data key SHOULD be stored as immutable data.

The plaintext data key SHOULD offer an interface to zero the plaintext data key.

##### Signing Key

The key to be used as the signing key for asymmetric signature verification during [encryption](../client-apis/encrypt.md).

The signing key MUST fit the specification described by the [asymmetric signature algorithm](algorithm-suites.md#asymmetric-signature-algorithm)
included in this encryption material's [algorithm suite](#algorithm-suite).
If the algorithm suite does not contain an asymmetric signing algorithm, the signing key MUST NOT be present.

The value of this key MUST be kept secret.

##### Required Encryption Context Keys

Communicates to higher level `encrypt` APIs
of [supported formats](algorithm-suites.md#supp#supported-formats)
how to split the encryption context
into elements that are authenticated and stored
from elements that are only authenticated and not stored.
Keys in this set should not be stored
with ciphertext protected by these materials.
These keys are required to be reproduced on decrypt.

Every key in Required Encryption Context Keys
MUST be a key in the [encryption context](#encryption-context-1).

##### Symmetric Signing Keys

A list of symmetric signing keys, such that each key in this list corresponds to the encrypted data key in the [encrypted data key list](#encrypted-data-keys) at the same index.
These keys are used to generate symmetric signatures during encryption.

If the algorithm suite does not contain a symmetric signing algorithm, this list MUST NOT be included in the materials.
If the algorithm suite does contain a symmetric signing algorithm, this list MUST have length equal to the [encrypted data key list](#encrypted-data-keys).

The symmetric signature keys MUST adhere to the specification for [symmetric signature algorithms](./algorithm-suites.md#symmetric-signature-algorithm)
included in this encryption material's [algorithm suite](#algorithm-suite).

The value of keys in this list MUST be kept secret.

### Decryption Materials

#### Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                  |
| ---------- | -------------------------------------- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [materials.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/materials.h)                                                        |
| Javascript | 0.1.0-preview                          | 0.1.0                     | [cryptographic_material.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/cryptographic_material.ts)     |
| Python     | 0.1.0-preview                          | n/a                       | [materials_managers](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/materials_managers/__init__.py)                        |
| Java       | 0.1.0-preview                          | n/a                       | [DecryptionMaterials.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/model/DecryptionMaterials.java) |

#### Fields

Decryption materials are a structure containing materials needed for [decryption](../client-apis/decrypt.md).
This structure MUST include the following fields:

- [Algorithm Suite](#algorithm-suite-1)
- [Encryption Context](#encryption-context-2)
- [Required Encryption Context Keys](#required-encryption-context-keys-1)

This structure MAY include any of the following fields:

- [Plaintext Data Key](#plaintext-data-key-1)
- [Verification Key](#verification-key)
- [Symmetric Signing Key](#symmetric-signing-key)

##### Algorithm Suite

The [algorithm suite](algorithm-suites.md) to be used for [decryption](../client-apis/decrypt.md).

##### Encryption Context

The [encryption context](#encryption-context) associated with this [decryption](../client-apis/decrypt.md).

If a [decryption materials](#decryption-materials) contains a [verification key](#verification-key),
the [encryption context](#encryption-context) SHOULD include the reserved key `aws-crypto-public-key`.
The mapped value from the reserved key `aws-crypto-public-key`
SHOULD be the signature verification key stored on the [decryption materials](#decryption-materials).
If a [decryption materials](#decryption-materials) does not contain a [verification key](#verification-key),
the [encryption context](#encryption-context) SHOULD NOT include the reserved key `aws-crypto-public-key`.

##### Plaintext Data Key

The data key to be used as input for [decryption](../client-apis/decrypt.md).

The plaintext data key MUST:

- Fit the specification for the [encryption algorithm](algorithm-suites.md#encryption-algorithm)
  included in this decryption material's [algorithm suite](#algorithm-suite-1).
- Consist of cryptographically secure (pseudo-)random bits.
- Be kept secret.

The plaintext data key SHOULD be stored as immutable data.

The plaintext data key SHOULD offer an interface to zero the plaintext data key.

##### Verification Key

The key to be used as the verification key for asymmetric signature verification during [decryption](../client-apis/decrypt.md).

The verification key MUST fit the specification for the [asymmetric signature algorithm](algorithm-suites.md#asymmetric-signature-algorithm)
included in this decryption material's [algorithm suite](#algorithm-suite-1).

##### Required Encryption Context Keys

A set of strings to communicate to higher level `decrypt` APIs
of [supported formats](algorithm-suites.md#supp#supported-formats)
how to split the [encryption context](#encryption-context-2)
into elements that are authenticated and stored
from elements that are only authenticated
and not stored in the encrypted message.
Keys in this set should not have been stored
with the ciphertext protected by these materials.
These keys should have been reproduced by the caller on decrypt.

Every key in Required Encryption Context Keys
MUST be a key in the [encryption context](#encryption-context-2).

##### Symmetric Signing Key

The key to be used to validate a symmetric signature during decryption.

If the algorithm suite does not contain a symmetric signing algorithm,
the symmetric signing key MUST NOT be included in the materials.

If the algorithm suite does contain a symmetric signing algorithm,
the symmetric signing key MUST also be included in the materials
if and only if the materials also include a [plaintext data key](#plaintext-data-key-1).

If included, the symmetric signature key MUST fit the specification for the [symmetric signature algorithm](algorithm-suites.md#symmetric-signature-algorithm)
included in this decryption material's [algorithm suite](#algorithm-suite-1).

This value MUST be kept secret.

### Branch Key Materials

#### Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

#### Structure

Branch Key materials are a structure containing materials that are used to establish a key hierarchy in order to
reuse a data key that wraps other data keys.
This structure MUST include all of the following fields:

- [Branch Key](#branch-key)
- [Branch Key Id](#branch-key-id)
- [Branch Key Version](#branch-key-version)
- [Encryption Context](#encryption-context-3)

##### Branch Key

Data keys that are reused to derive unique data keys for envelope encryption.
This data key MUST only be generated through
AWS KMS using the [`GenerateDataKeyWithoutPlaintext`](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyWithoutPlaintext.html) API.
This key MUST be 32 bytes long.

##### Branch Key Id

The UTF8 Encoded value of the ID of the corresponding [branch key](#branch-key).
The plaintext value of which is stored in the [branch key store](./branch-key-store.md).

##### Branch Key Version

The UTF8 Encoded value of the version of the corresponding [branch key](#branch-key).
The plaintext value of which is stored in the [branch key store](./branch-key-store.md).
This value MUST be a version 4 [UUID](https://www.ietf.org/rfc/rfc4122.txt).

##### Encryption Context

The [custom encryption context](#encryption-context) associated with this branch key.

## Beacon Key Materials

#### Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

#### Structure

Beacon Key materials are a structure containing materials
that are used in structured encryption.
They contain HMAC keys derived from a Beacon Key
stored in the [Keystore](branch-key-store.md)

This structure MUST include the following fields:

- [Beacon Key Id](#beacon-key-id)
- [Encryption Context](#encryption-context-4)

This structure MAY include the following fields:

- [Beacon Key](#beacon-key)
- [HMAC Keys](#hmac-keys)

##### Beacon Key

The Beacon Key that is stored
in the [Keystore](branch-key-store.md).

The beacon key is optional,
because the beacon key is used to derive [HMAC Keys](#hmac-keys).
Once this has been complete,
the beacon key can be discarded.

##### Encryption Context

The [custom encryption context](#encryption-context) associated with this beacon key.

##### Beacon Key Id

The Beacon key id that was used to obtain
the beacon key from a [Keystore](branch-key-store.md)

##### HMAC Keys

A key-value mapping of arbitrary strings
to bytes.
Where the string is the name of the beacon and
the bytes are the HMAC Key derived from the beacon key
concatenated with the UTF8 Encoding of the beacon name.
