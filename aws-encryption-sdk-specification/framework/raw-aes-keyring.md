[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Raw AES Keyring

## Version

0.4.1

### Changelog

- 0.4.1

  - Clarify [keyring failure on decrypt](../changes/2020-06-04_how-to-fail-with-keyrings/change.md)

- 0.4.0

  - [Specify behavior for RawAESKeyring for invalid input EC](../changes/2020-07-15_specify-behavior-for-raw-aes-keyring-for-invalid-EC/change.md)

- 0.3.0

  - [Raw AES keyring MUST NOT accept a key namespace of "aws-kms".](https://github.com/awslabs/aws-encryption-sdk-specification/issues/101)

- 0.2.0

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                    |
| ---------- | -------------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [raw_aes_keyring.c](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/raw_aes_keyring.c)                                                             |
| NodeJS     | 0.1.0-preview                          | 0.1.0                     | [raw_aes_keyring_node.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/raw-aes-keyring-node/src/raw_aes_keyring_node.ts)          |
| Browser JS | 0.1.0-preview                          | 0.1.0                     | [raw_aes_keyring_browser.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/raw-aes-keyring-browser/src/raw_aes_keyring_browser.ts) |
| Python     | 0.1.0-preview                          | n/a                       | [keyrings/raw.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/keyrings/raw.py)                                            |
| Java       | 0.1.0-preview                          | n/a                       | [RawAesKeyring.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/keyrings/RawAesKeyring.java)            |

## Overview

A keyring which does local AES-GCM encryption and decryption of data keys using a local wrapping key.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### AES-GCM

Advanced Encryption Standard in Galois/Counter Mode (AES-GCM) is an Authenticated Encryption with Associated Data (AEAD) cipher.

Advanced Encryption Standard (AES) Specification: [NIST FIPS 297](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)

Galois/Counter Mode (GCM) Specification: [NIST Special Publication 800-38D](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf)

## Initialization

On keyring initialization,
the caller MUST provide the following:

- [Key Namespace](./keyring-interface.md#key-namespace)
- [Key Name](./keyring-interface.md#key-name)
- [Wrapping Key](#wrapping-key)
- [Wrapping Algorithm](#wrapping-algorithm)

### Wrapping Key

The AES key input to be used with the configured [wrapping algorithm](#wrapping-algorithm) to encrypt plaintext data keys.

The wrapping key MUST be a secret value consisting of cryptographically secure pseudo-random bytes.
It MUST be randomly generated from a cryptographically secure entropy source.
The length of the wrapping key MUST be 128, 192, or 256.

### Wrapping Algorithm

The algorithm to be used with the configured [wrapping key](#wrapping-key) to encrypt plaintext data keys.

The keyring MUST support the following algorithm configurations:

- AES_GCM with key size 128 bits, IV length 12 bytes, and tag length 16 bytes
- AES_GCM with key size 192 bits, IV length 12 bytes, and tag length 16 bytes
- AES_GCM with key size 256 bits, IV length 12 bytes, and tag length 16 bytes

Initialization MUST fail if the length of the [wrapping key](#wrapping-key) does not match the length specified by the wrapping algorithm.

## Structure

### Key Provider Information

This structure is a sequence of bytes in big-endian format to be used as the
[key provider information](structures.md#key-provider-information) field in
[encrypted data keys](structures.md#encrypted-data-keys) produced by raw AES keyrings.

The following table describes the fields that form the raw AES keyring key provider information.
The bytes are appended in the order shown.

| Field                     | Length (bytes)                                        | Interpreted as      |
| ------------------------- | ----------------------------------------------------- | ------------------- |
| Key Name                  | length of [Key Name](./keyring-interface.md#key-name) | UTF-8 encoded bytes |
| Authentication Tag Length | 4                                                     | UInt32              |
| IV Length                 | 4                                                     | UInt32              |
| IV                        | [IV Length](#iv-length)                               | Bytes               |

#### Key Name

The [Key Name](./keyring-interface.md#key-name) of this keyring.

#### Authentication Tag Length

The length, in bits, of the authentication tag returned by the AES-GCM encryption.

This value MUST match the authentication tag length of the keyring's configured [wrapping algorithm](#wrapping-algorithm).

#### IV Length

The length, in bytes, of the initialization vector (IV) input into the AES-GCM encryption.

This value MUST match the IV length of the keyring's configured [wrapping algorithm](#wrapping-algorithm).

#### IV

The bytes to use as the IV in the AES-GCM encryption.

### Ciphertext

This structure is a sequence of bytes in big-endian format to be used as
the [ciphertext](structures.md#ciphertext) field in
[encrypted data keys](structures.md#encrypted-data-key) produced by raw AES keyrings.

The following table describes the fields that form the ciphertext for this keyring.
The bytes are appended in the order shown.

| Field              | Length (bytes)                                                   | Interpreted as |
| ------------------ | ---------------------------------------------------------------- | -------------- |
| Encrypted Key      | length of AES-GCM ciphertext output                              | Bytes          |
| Authentication Tag | [Authentication Tag Length](#authentication-tag-length) as Bytes | Bytes          |

#### Encrypted Key

The ciphertext returned by the AES-GCM encryption of the plaintext data key.

#### Authentication Tag

The authentication tag returned by the AES-GCM encryption.

## Operation

### OnEncrypt

OnEncrypt MUST take [encryption materials](structures.md#encryption-materials) as input.

If the [encryption materials](structures.md#encryption-materials) do not contain a plaintext data key,
OnEncrypt MUST generate a random plaintext data key and set it on the [encryption materials](structures.md#encryption-materials).

The keyring MUST encrypt the plaintext data key in the [encryption materials](structures.md#encryption-materials)
using AES-GCM.

The keyring MUST attempt to serialize the [encryption materials'](structures.md#encryption-materials)
[encryption context](structures.md#encryption-context-1) according to the [encryption context serialization specification](structures.md#serialization).
If the keyring cannot serialize the encryption context, OnEncrypt MUST fail.

The keyring uses AES-GCM with the following specifics:

- It MUST use the serialized [encryption context](structures.md#encryption-context-1) as the additional authenticated data (AAD).
- It MUST use this keyring's [wrapping key](#wrapping-key) as the AES-GCM cipher key.
- It MUST use a cryptographically random generated IV of length specified by this keyring's [wrapping algorithm](#wrapping-algorithm).
- It MUST use an authentication tag bit of length specified by this keyring's [wrapping algorithm](#wrapping-algorithm).

Based on the ciphertext output of the AES-GCM decryption,
the keyring MUST construct an [encrypted data key](structures.md#encrypted-data-key) with the following specifics:

- The [key provider ID](structures.md#key-provider-id) is this keyring's [key namespace](./keyring-interface.md#key-namespace).
- The [key provider information](structures.md#key-provider-information) is serialized as the
  [raw AES keyring key provider information](#key-provider-information).
- The [ciphertext](structures.md#ciphertext) is serialized as the
  [raw AES keyring ciphertext](#ciphertext).

The keyring MUST append the constructed encrypted data key to the encrypted data key list in the
[encryption materials](structures.md#encryption-materials).

OnEncrypt MUST output the modified [encryption materials](structures.md#encryption-materials).

### OnDecrypt

OnDecrypt MUST take [decryption materials](structures.md#decryption-materials) and
a list of [encrypted data keys](structures.md#encrypted-data-key) as input.

If the decryption materials already contain a plaintext data key,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

The keyring MUST attempt to serialize the [decryption materials'](structures.md#decryption-materials)
[encryption context](structures.md#encryption-context-1) according to the [encryption context serialization specification](structures.md#serialization).
If the keyring cannot serialize the encryption context, OnDecrypt MUST fail.

The keyring MUST perform the following actions on each [encrypted data key](structures.md#encrypted-data-key)
in the input encrypted data key list, serially, until it successfully decrypts one.

For each [encrypted data key](structures.md#encrypted-data-key),
the keyring MUST first attempt to deserialize the [serialized ciphertext](#ciphertext)
to obtain the [encrypted key](#encrypted-key) and [authentication tag](#authentication-tag), and
deserialize the [serialized key provider info](#key-provider-information) to obtain the [key name](./keyring-interface.md#key-name),
[IV](#iv), [IV length](#iv-length), and [authentication tag length](#authentication-tag-length).

The keyring attempts to decrypt the encrypted data key if and only if the following is true:

- The [ciphertext](#ciphertext) and [key provider information](#key-provider-information) MUST be successfully deserialized.
- The key name obtained from the encrypted data key's key provider information MUST have a value equal to this keyring's [key name](./keyring-interface.md#key-name).
- The key provider ID of the encrypted data key MUST have a value equal to this keyring's [key namespace](./keyring-interface.md#key-namespace).
- The [IV length](#iv-length) obtained from the encrypted data key's key provider information MUST have a value equal to the length specified by this keyring's [wrapping algorithm](#wrapping-algorithm).
- The [authentication tag length](#authentication-tag-length) obtained from the key provider information MUST have a value equal to the length specified by this keyring's [wrapping algorithm](#wrapping-algorithm).

If decrypting, the keyring uses AES-GCM with the following specifics:

- It MUST use the [encrypt key](#encrypted-key) obtained from deserialization as the AES-GCM input ciphertext.
- It MUST use the [authentication tag](#authentication-tag) obtained from deserialization as the AES-GCM input authentication tag.
- It MUST use this keyring's [wrapping key](#wrapping-key) as the AES-GCM cipher key.
- It MUST use the [IV](#iv) obtained from deserialization as the AES-GCM IV.
- It MUST use the serialized [encryption context](structures.md#encryption-context-1) as the AES-GCM AAD.

If a decryption succeeds, this keyring MUST
add the resulting plaintext data key to the decryption materials and return the modified materials.

If no decryption succeeds,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).
