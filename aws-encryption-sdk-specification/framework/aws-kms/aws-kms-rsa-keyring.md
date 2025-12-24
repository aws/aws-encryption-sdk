[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS RSA Keyring

## Version

0.1.0-preview

### Changelog

- 0.1.0-preview

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| ---------- | -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | |

## Overview

A keyring which uses AWS KMS RSA asymmetric keys
to protect messages with envelop encryption.
A hash of the Encryption Context is stored
in the encrypted data key
to bind the encryption context to the data key.

For decryption of data keys the keyring always calls KMS
and compares the encryption context
to the hashed value stored in the encrypted data key.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

On initialized the caller can provide:

- MUST provide an AWS KMS key identifier
- MUST provide an [AWS KMS Encryption Algorithm](#supported-padding-schemes)
- MAY provide a PEM encoded Public Key
- MAY provide an AWS KMS SDK client
- MAY provide a list of Grant Tokens

The AWS KMS key identifier MUST NOT be null or empty.
The AWS KMS key identifier MUST be [a valid identifier](../../framework/aws-kms/aws-kms-key-arn.md#a-valid-aws-kms-identifier).
The AWS KMS key identifier MUST NOT be an AWS KMS alias.

If provided the Public Key
MUST have an RSA modulus bit length greater than or equal to 2048.
The configured AWS KMS key identifier
must match the public key provided.
There should not be a synchronous check to verify this.

### AWS KMS Encryption Algorithm

The RSA padding scheme to use with this keyring.

This value MUST correspond with one of the [supported padding schemes](#supported-padding-schemes).

#### Supported Padding Schemes

The following padding schemes are currently defined by AWS KMS:

- RSAES_OAEP_SHA_1
  - This is equivalent to [OAEP with SHA-1 and MGF1 with SHA-1 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)
- RSAES_OAEP_SHA_256
  - This is equivalent to [OAEP with SHA-256 and MGF1 with SHA-256 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)

This keyring MUST NOT use a padding scheme outside those defined above.
These values must match the supported values
for [AWS KMS RSA key specs](https://docs.aws.amazon.com/kms/latest/developerguide/asymmetric-key-specs.html#key-spec-rsa).

## Operation

### OnEncrypt

OnEncrypt MUST fail if this keyring does not have a specified Public Key.

OnEncrypt MUST fail if the input [encryption materials](../structures.md#encryption-materials)
contains an [algorithm suite](../algorithm-suites.md) containing an
[asymmetric signature](../algorithm-suites.md#asymmetric-signature-algorithm).
See [Security Considerations](#security-considerations).

OnEncrypt MUST take [encryption materials](../structures.md#encryption-materials) as input.

If the [encryption materials](structures.md#encryption-materials) do not contain a plaintext data key,
OnEncrypt MUST generate a random plaintext data key and set it on the [encryption materials](structures.md#encryption-materials).

OnEncrypt MUST calculate a Encryption Context Digest by:

1. Serializing The [encryption context](structures.md#encryption-context-1) from the input
   [encryption materials](../structures.md#encryption-materials) according to the [encryption context serialization specification](../structures.md#serialization).
2. Taking the SHA-384 Digest of this concatenation.

The keyring MUST determine the [Padding Scheme](#padding-scheme)
using the configured [AWS KMS Encryption Algorithm]((https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html#KMS-Decrypt-request-EncryptionAlgorithm).
in the following manner:

If `RSAES_OAEP_SHA_256` the keyring
MUST use [OAEP with SHA-256 and MGF1 with SHA-256 Padding](https://tools.ietf.org/html/rfc8017#section-7.1).

If `RSAES_OAEP_SHA_1` the keyring
MUST use [OAEP with SHA-1 and MGF1 with SHA-1 Padding](https://tools.ietf.org/html/rfc8017#section-7.1)

The keyring MUST attempt to encrypt the plaintext data key in the
[encryption materials](structures.md#encryption-materials) using RSA.

The keyring performs [RSA encryption](#rsa) with the following specifics:

- This keyring's [padding scheme](#supported-padding-schemes) calculated above.
- The Encryption Context Digest concatenated with the plaintext data key is the plaintext input to RSA encryption.
- This [public key](kms-rsa-public-key-provider-interface.md##public-key)
  is the configured RSA public key.

If RSA encryption was successful, OnEncrypt MUST return the input
[encryption materials](structures.md#encryption-materials), modified in the following ways:

- The encrypted data key list has a new encrypted data key added, constructed as follows:
  - The [ciphertext](../structures.md#ciphertext) MUST field is the ciphertext output by the RSA encryption.
  - The [key provider id](../structures.md#key-provider-id) MUST be "aws-kms-rsa".
  - The [key provider information](../structures.md#key-provider-information) MUST be
    the configured `AWS KMS key identifier`.

If RSA encryption was NOT successful, OnEncrypt MUST fail.

### OnDecrypt

OnDecrypt MUST fail if this keyring does not have a specified AWS KMS SDK client.

OnDecrypt MUST fail if the input [decryption materials](../structures.md#decryption-materials)
contains an [algorithm suite](../algorithm-suites.md) containing an
[asymmetric signature](../algorithm-suites.md#asymmetric-signature-algorithm).
See [Security Considerations](#security-considerations).

OnDecrypt MUST take [decryption materials](../structures.md#decryption-materials) and
a list of [encrypted data keys](../structures.md#encrypted-data-key) as input.

If the [decryption materials](../structures.md#decryption-materials) already contained a valid plaintext data key
OnDecrypt MUST return an error.

The set of encrypted data keys MUST first be filtered to match this keyring’s configuration. For the encrypted data key to match:

- Its provider ID MUST exactly match the value “aws-kms-rsa”.
- The provider info MUST be a [valid AWS KMS ARN](aws-kms-key-arn.md#a-valid-aws-kms-arn)
  with a resource type of `key` or OnDecrypt MUST fail.
- The function [AWS KMS MRK Match for Decrypt](aws-kms-mrk-match-for-decrypt.md#implementation)
  called with the configured AWS KMS key identifier and the provider info MUST return `true`.

OnDecrypt MUST calculate a Encryption Context Digest Prime by:

1. Serializing The [encryption context](structures.md#encryption-context-2) from the input
   [decryption materials](../structures.md#decryption-materials) according to the [encryption context serialization specification](../structures.md#serialization).
2. Taking the SHA-384 Digest of this concatenation.

For each encrypted data key in the filtered set,
one at a time,
the OnDecrypt MUST attempt to decrypt the data key.
If this attempt results in an error,
then these errors MUST be collected.

To attempt to decrypt a particular [encrypted data key](../structures.md#encrypted-data-key),
OnDecrypt MUST call [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html)
with the configured AWS KMS client.

When calling [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html),
the keyring MUST call with a request constructed as follows:

- `KeyId` MUST be the configured AWS KMS key identifier.
- `CiphertextBlob` MUST be the [encrypted data key ciphertext](../structures.md#ciphertext).
- `GrantTokens` MUST be this keyring's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).
- `EncryptionAlgorithm` MUST be configured value.

If the call to [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) succeeds,
OnDecrypt verifies:

- The `KeyId` field in the response MUST equal the configured AWS KMS key identifier.

If any decryption succeeds,
the result of this decryption MUST be split into
the encryption context digest and plaintext data key by:

- the first 48 bytes is the encryption context digest
- all bytes after that are the plain text data key.

The keyring MUST compare the decrypted encryption context digest
to the encryption context digest prime;
if the two are not equal,
the keyring MUST fail and
MUST NOT modify the [decryption materials](structures.md#decryption-materials).

Otherwise, this keyring MUST immediately return the input
[decryption materials](structures.md#decryption-materials), modified in the following ways:

- The plaintext data key is set as the decryption material's plaintext data key.

If no decryption and keyring digest check succeeds,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

### Encryption Context Digest

The Encryption Context is the SHA-384 hash of
the Encryption Materials' Encryption Context.
The fact that this digest is not truncated
means that this keyring MUST NOT
support 1024 bit keys.

### Encryption Context Digest Prime

The Encryption Context Digest Prime is the SHA-384 hash of
the Decryption Materials' Encryption Context.

## Security Considerations

The AWS KMS RSA Keyring does not support use
with an algorithm suite containing an asymmetric signature.
The purpose of the asymmetric signature algorithm is to create
the security property where decryptors of a message are able
to assert that the encrypted messaged originated from an entity
with encrypt access to the material that protects the data key
for the message.
As an example, this property is useful for customers that use
the AWS KMS Keyring (using a KMS symmetric key),
restrict encrypt and decrypt access to this key separately
via KMS Key Policies,
and the entities given decrypt access are not as trusted as
the entities given encrypt access.
In such a case it is valuable to be able to ensure that one decryptor
is not able to create a message that looks valid to a different decryptor.

By definition, access to the material that the AWS KMS RSA Keyring
uses for encryption is public.
While customers may use AWS KMS Key Policies to restrict access
to obtaining the public key through KMS, the public key
should not be considered anything other than public.
Because it should be assumed that everyone has access to the
material responsible for protecting data keys using this Keyring,
there is no additional security value to using an algorithm suite
with asymmetric signing.
In order to avoid an expensive cryptographic calculation,
as well as to avoid cases where the customer may be misinterpreting
the intent of the signature algorithm,
the AWS KMS Keyring rejects any material containing an algorithm suite
with asymmetric signing.

A similar situation exists in the [Raw AES Keyring](../raw-rsa-keyring.md)
and [Raw RSA Keyrings](../raw-aes-keyring.md).
With the Raw RSA Keyring, the material used on encrypt is public,
so there is no set up in which it makes sense to use an algorithm suite
with asymmetric signing.
With the Raw AES Keyring, because the Keyring requires
the AWS symmetric key for decryption to be available in memory
(as opposed to calling out for the decryption to happen via an HSM,
which may have access control on it),
decrypt access necessarially implies encrypt access.
Again, our asymmetric signing provides no additional value for this set up.
Currently, these two keyrings accept algorithm suites with asymmetric signing.
This is for backwards compatability reasons.
If these Keyrings are refactored in the future,
they may be updated (over a major version bump) to also reject
algorithm suites with asymmetric signing.
