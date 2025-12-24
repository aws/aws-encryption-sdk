[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Raw RSA Keyring

## Version

0.3.2

### Changelog

- 0.3.2

  - Clarify [keyring failure on decrypt](../changes/2020-06-04_how-to-fail-with-keyrings/change.md)

- 0.3.1

  - [Clarify failure language when only one key is defined](https://github.com/awslabs/aws-encryption-sdk-specification/issues/91)

- 0.3.0

  - [Raw RSA keyring MUST NOT accept a key namespace of "aws-kms".](https://github.com/awslabs/aws-encryption-sdk-specification/issues/101)

- 0.2.0

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                      |
| ---------- | -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [raw_rsa_keyring.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/raw_rsa_keyring.h)                                                |
| NodeJS     | 0.1.0-preview                          | 0.1.0                     | [raw_rsa_keyring_node.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/raw-rsa-keyring-node/src/raw_rsa_keyring_node.ts)            |
| Browser JS | 0.1.0-preview                          | 0.1.0                     | [raw_rsa_keyring_web_crypto.ts](https://github.com/aws/aws-encryption-sdk-javascript/blob/master/modules/raw-rsa-keyring-browser/src/raw_rsa_keyring_web_crypto.ts) |
| Python     | 0.1.0-preview                          | n/a                       | [keyrings/raw.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/keyrings/raw.py)                                              |
| Java       | 0.1.0-preview                          | n/a                       | [RawRsaKeyring.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/keyrings/RawRsaKeyring.java)              |

## Overview

A keyring which does local RSA encryption and decryption of data keys using a local wrapping key.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### RSA

RSA (Rivest–Shamir–Adleman) is an asymmteric cryptographic cipher.

RSA Implementation Specification: [RFC 3447](https://tools.ietf.org/html/rfc8017)

## Initialization

On keyring initialization,
the caller MUST provide the following:

- [Key Namespace](./keyring-interface.md#key-namespace)
- [Key Name](./keyring-interface.md#key-name)
- [Padding Scheme](#padding-scheme)
- [Public Key](#public-key) and/or [Private Key](#private-key)

### Padding Scheme

The RSA padding scheme to use with this keyring.

This value MUST correspond with one of the [supported padding schemes](#supported-padding-schemes).

If the padding scheme uses MGF1 Padding, the hash function used as part of MGF1 MUST be the same hash function
used to hash the plaintext data key.

#### Supported Padding Schemes

The following padding schemes are currently defined:

- [PKCS1 v1.5 Padding](https://tools.ietf.org/html/rfc8017#section-7.2)
- [OAEP with SHA-1 and MGF1 with SHA-1 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)
- [OAEP with SHA-256 and MGF1 with SHA-256 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)
- [OAEP with SHA-384 and MGF1 with SHA-384 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)
- [OAEP with SHA-512 and MGF1 with SHA-512 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)

This keyring MUST NOT use a padding scheme outside those defined above.

### Public Key

The RSA public key used by this keyring to encrypt data keys.

The public key MUST follow the [RSA specification for public keys](#rsa).
The raw RSA keyring SHOULD support loading PEM encoded [X.509 SubjectPublicKeyInfo structures](https://tools.ietf.org/html/rfc5280#section-4.1)
as public keys.

### Private Key

The RSA private key used by this keyring to decrypt data keys.

The private key MUST follow the [RSA specification for private keys](#rsa).
The raw RSA keyring SHOULD support loading PEM encoded [PKCS #8 PrivateKeyInfo structures](https://tools.ietf.org/html/rfc5958#section-2)
as private keys.
The private key SHOULD contain all Chinese Remainder Theorem (CRT) components (public exponent, prime factors, CRT exponents, and CRT coefficient).

## Operation

### OnEncrypt

OnEncrypt MUST fail if this keyring does not have a specified [public key](#public-key).
The keyring MUST NOT derive a public key from a specified [private key](#private-key).

OnEncrypt MUST take [encryption materials](structures.md#encryption-materials) as input.

If the [encryption materials](structures.md#encryption-materials) do not contain a plaintext data key,
OnEncrypt MUST generate a random plaintext data key and set it on the [encryption materials](structures.md#encryption-materials).

The keyring MUST attempt to encrypt the plaintext data key in the
[encryption materials](structures.md#encryption-materials) using RSA.
The keyring performs [RSA encryption](#rsa) with the following specifics:

- This keyring's [public key](#public-key) is the RSA public key.
- This keyring's [padding scheme](#supported-padding-schemes) is the RSA padding scheme.
- The plaintext data key is the plaintext input to RSA encryption.

If RSA encryption was successful, OnEncrypt MUST return the input
[encryption materials](structures.md#encryption-materials), modified in the following ways:

- The encrypted data key list has a new encrypted data key added, constructed as follows:
  - The [key provider ID](structures.md#key-provider-id) field is this keyring's [key namespace](keyring-interface.md#key-namespace).
  - The [key provider information](structures.md#key-provider-information) field is this keyring's [key name](keyring-interface.md#key-name).
  - The [ciphertext](structures.md#ciphertext) field is the ciphertext output by
    the RSA encryption of the plaintext data key.

### OnDecrypt

OnDecrypt MUST fail if this keyring does not have a specified [private key](#private-key).

OnDecrypt MUST take [decryption materials](structures.md#decryption-materials) and
a list of [encrypted data keys](structures.md#encrypted-data-key) as input.

If the decryption materials already contain a plaintext data key,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

The keyring MUST attempt to decrypt the input encrypted data keys, in list order, until it successfully decrypts one.

For each encrypted data key, the keyring MUST attempt to decrypt the encrypted data key into plaintext
using RSA if and only if the following is true:

- The encrypted data key's [key provider information](structures.md#key-provider-information).
  has a value equal to this keyring's [key name](keyring-interface.md#key-name).
- The encrypted data key's [key provider ID](structures.md#key-provider-id) has a value equal to
  this keyring's [key namespace](keyring-interface.md#key-namespace).

The keyring performs RSA decryption with the following specifics:

- This keyring's [private key](#private-key) is the RSA private key.
- This keyring's [padding scheme](#supported-padding-schemes) is the RSA padding scheme.
- An encrypted data key's [ciphertext](structures.md#ciphertext) is the input ciphertext to RSA decryption.

If any decryption succeeds, this keyring MUST immediately return the input
[decryption materials](structures.md#decryption-materials), modified in the following ways:

- The output of RSA decryption is set as the decryption material's plaintext data key.

If no decryption succeeds,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).
