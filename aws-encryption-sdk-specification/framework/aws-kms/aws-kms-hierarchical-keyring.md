[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS Hierarchical Keyring

## Version

0.1.0

### Changelog

- 0.1.0
  - Initital record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

The Hierarchical keyring interacts with Amazon DynamoDB (AWS DDB) and AWS Key Management Service (AWS KMS).
The Hierarchical keyring uses AWS KMS Keys to protect [Branch Keys](#definitions) to establish a key hierarchy
and it uses Amazon DynamoDB to persist these [Branch Keys](#definitions).
The Hierarchical keyring allows customers to reduce AWS KMS calls by locally caching branch keys in the established hierarchy and using them to protect data keys.

## Definitions

- [Branch Key(s)](../structures.md#branch-key): Data keys that are reused to derive unique data keys for envelope encryption.
  For security considerations on when to rotate the branch key, refer to [Appendix B](#appendix-b-security-considerations-for-branch-key-rotation).
- [Keystore](../branch-key-store.md): a resource responsible for managing and protecting branch keys in DDB.
- [UUID](https://www.ietf.org/rfc/rfc4122.txt): a universally unique identifier that can be represented as a byte sequence or a string.

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Interface

MUST implement the [AWS Encryption SDK Keyring interface](../keyring-interface.md#interface)

## Initialization

On initialization, the caller:

- MUST provide a [Keystore](../branch-key-store.md)
- MUST provide a [cache limit TTL](#cache-limit-ttl)
- MUST provide either a Branch Key Identifier or a [Branch Key Supplier](#branch-key-supplier)
- MAY provide a max cache size

On initialization the Hierarchical Keyring MUST initialize a [cryptographic-materials-cache](../local-cryptographic-materials-cache.md) with the configured cache limit TTL and the max cache size.
If no max cache size is provided, the crypotgraphic materials cache MUST be configured to a
max cache size of 1000.

### Cache Limit TTL

The maximum amount of time in seconds that an entry within the cache may be used before it MUST be evicted.
The client MUST set a time-to-live (TTL) for [branch key materials](../structures.md#branch-key-materials) in the underlying cache.
This value MUST be greater than zero.

## Structure

### Ciphertext

This structure is a sequence of bytes in big-endian format to be used as
the [ciphertext](structures.md#ciphertext) field in
[encrypted data keys](structures.md#encrypted-data-key) produced by the AWS KMS Hierarchical Keyring.

This structure is formed using the 16 byte `salt` used to derive the `derivedBranchKey`
concatenated with the AES-GCM-256 12 byte `IV`
concatenated with the byte representation of the UUID branch key version from the AWS DDB response `version` value
concatenated with the AES Encryption output from the [branch key wrapping](#branch-key-wrapping).

The following table describes the fields that form the ciphertext for this keyring.
The bytes are appended in the order shown.

| Field              | Length (bytes) | Interpreted as |
| ------------------ | -------------- | -------------- |
| Salt               | 16             | bytes          |
| IV                 | 12             | bytes          |
| Version            | 16             | bytes          |
| Encrypted Key      | 32             | bytes          |
| Authentication Tag | 16             | bytes          |

#### Authentication Tag

The authentication tag returned by the AES-GCM encryption.

## Operation

### OnEncrypt

OnEncrypt MUST take [encryption materials](../structures.md#encryption-materials) as input.

The `branchKeyId` used in this operation is either the configured branchKeyId, if supplied, or the result of the `branchKeySupplier`'s
`getBranchKeyId` operation, using the encryption material's encryption context as input.

If the input [encryption materials](../structures.md#encryption-materials) do not contain a plaintext data key,
OnEncrypt MUST generate a random plaintext data key, according to the key length defined in the [algorithm suite](../algorithm-suites.md#encryption-key-length).
The process used to generate this random plaintext data key MUST use a secure source of randomness.

The hierarchical keyring MUST attempt to find [branch key materials](../structures.md#branch-key-materials)
from the underlying [cryptographic materials cache](../local-cryptographic-materials-cache.md).
The hierarchical keyring MUST use the formulas specified in [Appendix A](#appendix-a-cache-entry-identifier-formulas)
to compute the [cache entry identifier](../cryptographic-materials-cache.md#cache-identifier).

If a cache entry is found and the entry's TTL has not expired, the hierarchical keyring MUST use those branch key materials for key wrapping.

If a cache entry is not found or the cache entry is expired, the hierarchical keyring MUST attempt to obtain the branch key materials
by querying the backing branch keystore specified in the [retrieve OnEncrypt branch key materials](#query-branch-keystore-onencrypt) section.

If the keyring is not able to retrieve [branch key materials](../structures.md#branch-key-materials)
through the underlying cryptographic materials cache or
it no longer has access to them through the backing keystore, OnEncrypt MUST fail.

Otherwise, OnEncrypt:

- MUST wrap a data key with the branch key materials according to the [branch key wrapping](#branch-key-wrapping) section.

If the keyring is unable to wrap a plaintext data key, OnEncrypt MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

Otherwise, OnEncrypt MUST append a new [encrypted data key](../structures.md#encrypted-data-key)
to the encrypted data key list in the [encryption materials](../structures.md#encryption-materials), constructed as follows:

- [ciphertext](../structures.md#ciphertext): MUST be serialized as the [hierarchical keyring ciphertext](#ciphertext)
- [key provider id](../structures.md#key-provider-id): MUST be UTF8 Encoded "aws-kms-hierarchy"
- [key provider info](../structures.md#key-provider-information): MUST be the UTF8 Encoded AWS DDB response `branch-key-id`

#### Query Branch Keystore OnEncrypt

The branch keystore persists [branch keys](#definitions) that are reused to derive unique data keys for envelope encryption to
reduce the number of calls to AWS KMS through the use of the
[cryptographic materials cache](../cryptographic-materials-cache.md).

OnEncrypt MUST call the Keystore's [GetActiveBranchKey](../branch-key-store.md#getactivebranchkey) operation with the following inputs:

- the `branchKeyId` used in this operation

If the Keystore's GetActiveBranchKey operation succeeds
the keyring MUST put the returned branch key materials in the cache using the
formula defined in [Appendix A](#appendix-a-cache-entry-identifier-formulas).

Otherwise, OnEncrypt MUST fail.

#### Branch Key Wrapping

To derive and encrypt a data key the keyring will follow the same key derivation and encryption as [AWS KMS](https://rwc.iacr.org/2018/Slides/Gueron.pdf).

The hierarchical keyring MUST:

1. Generate a 16 byte random `salt` using a secure source of randomness
1. Generate a 12 byte random `IV` using a secure source of randomness
1. Use a [KDF in Counter Mode with a Pseudo Random Function with HMAC SHA 256](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-108r1.pdf) to derive a 32 byte `derivedBranchKey` data key with the following inputs:
   - Use the `salt` as the salt.
   - Use the branch key as the `key`.
   - Use the UTF8 Encoded value "aws-kms-hierarchy" as the label.
1. Encrypt a plaintext data key with the `derivedBranchKey` using `AES-GCM-256` with the following inputs:
   - MUST use the `derivedBranchKey` as the AES-GCM cipher key.
   - MUST use the plain text data key that will be wrapped by the `derivedBranchKey` as the AES-GCM message.
   - MUST use the derived `IV` as the AES-GCM IV.
   - MUST use an authentication tag byte of length 16.
   - MUST use the serialized [AAD](#branch-key-wrapping-and-unwrapping-aad) as the AES-GCM AAD.

If OnEncrypt fails to do any of the above, OnEncrypt MUST fail.

### OnDecrypt

OnDecrypt MUST take [decryption materials](../structures.md#decryption-materials) and a list of [encrypted data keys](../structures.md#encrypted-data-keys) as input.

The `branchKeyId` used in this operation is either the configured branchKeyId, if supplied, or the result of the `branchKeySupplier`'s
`getBranchKeyId` operation, using the decryption material's encryption context as input.

If the decryption materials already contain a `PlainTextDataKey`, OnDecrypt MUST fail.

The set of encrypted data keys MUST first be filtered to match this keyring’s configuration. For the encrypted data key to match:

- Its provider ID MUST match the UTF8 Encoded value of “aws-kms-hierarchy”.
- Deserialize the key provider info, if deserialization fails the next EDK in the set MUST be attempted.
  - The deserialized key provider info MUST be UTF8 Decoded and MUST match this keyring's configured `Branch Key Identifier`.

For each encrypted data key in the filtered set, one at a time, OnDecrypt MUST attempt to decrypt the encrypted data key.
If this attempt results in an error, then these errors MUST be collected.

To decrypt each encrypted data key in the filtered set, the hierarchical keyring MUST attempt
to find the corresponding [branch key materials](../structures.md#branch-key-materials)
from the underlying [cryptographic materials cache](../local-cryptographic-materials-cache.md).
The hierarchical keyring MUST use the OnDecrypt formula specified in [Appendix A](#decryption-materials)
in order to compute the [cache entry identifier](cryptographic-materials-cache.md#cache-identifier).

If a cache entry is found and the entry's TTL has not expired, the hierarchical keyring MUST use those branch key materials for key unwrapping.

If a cache entry is not found or the cache entry is expired, the hierarchical keyring
MUST attempt to obtain the branch key materials by calling the backing branch key
store specified in the [retrieve OnDecrypt branch key materials](#getitem-branch-keystore-ondecrypt) section.

If the keyring is not able to retrieve `branch key materials` from the backing keystore then OnDecrypt MUST fail.

If the keyring is able to retrieve `branch key materials` from the backing keystore, OnDecrypt:

- MUST unwrap the encrypted data key with the branch key materials according to the [branch key unwrapping](#branch-key-unwrapping) section.

If a decryption succeeds, this keyring MUST
add the resulting plaintext data key to the decryption materials and return the modified materials.

If OnDecrypt fails to successfully decrypt any [encrypted data key](../structures.md#encrypted-data-key),
then it MUST yield an error that includes all the collected errors
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

#### GetItem Branch Keystore OnDecrypt

The branch keystore persists [branch keys](#definitions) that are reused to derive unique data keys for key wrapping to
reduce the number of calls to AWS KMS through the use of the
[cryptographic materials cache](../cryptographic-materials-cache.md).

OnDecrypt MUST calculate the following values:

- Deserialize the UTF8-Decoded `branch-key-id` from the [key provider info](../structures.md#key-provider-information) of the [encrypted data key](../structures.md#encrypted-data-key)
  and verify this is equal to the configured or supplied `branch-key-id`.
- Deserialize the UUID string representation of the `version` from the [encrypted data key](../structures.md#encrypted-data-key) [ciphertext](#ciphertext).

OnDecrypt MUST call the Keystore's [GetBranchKeyVersion](../branch-key-store.md#getbranchkeyversion) operation with the following inputs:

- The deserialized, UTF8-Decoded `branch-key-id`
- The deserialized UUID string representation of the `version`

If the Keystore's GetBranchKeyVersion operation succeeds
the keyring MUST put the returned branch key materials in the cache using the
formula defined in [Appendix A](#appendix-a-cache-entry-identifier-formulas).

Otherwise, OnDecrypt MUST fail.

#### Branch Key Unwrapping

To decrypt an encrypted data key with a branch key, the hierarchical keyring MUST:

1. Deserialize the 16 byte random `salt` from the [edk ciphertext](../structures.md#ciphertext).
1. Deserialize the 12 byte random `IV` from the [edk ciphertext](../structures.md#ciphertext).
1. Deserialize the 16 byte `version` from the [edk ciphertext](../structures.md#ciphertext).
1. Deserialize the `encrypted key` from the [edk ciphertext](../structures.md#ciphertext).
1. Deserialize the `authentication tag` from the [edk ciphertext](../structures.md#ciphertext).

1. Use a [KDF in Counter Mode with a Pseudo Random Function with HMAC SHA 256](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-108r1.pdf) to derive
   the 32 byte `derivedBranchKey` data key with the following inputs:
   - Use the `salt` as the salt.
   - Use the branch key as the `key`.
1. Decrypt the encrypted data key with the `derivedBranchKey` using `AES-GCM-256` with the following inputs:
   - It MUST use the `encrypted key` obtained from deserialization as the AES-GCM input ciphertext.
   - It MUST use the `authentication tag` obtained from deserialization as the AES-GCM input authentication tag.
   - It MUST use the `derivedBranchKey` as the AES-GCM cipher key.
   - It MUST use the `IV` obtained from deserialization as the AES-GCM input IV.
   - It MUST use the serialized [encryption context](#branch-key-wrapping-and-unwrapping-aad) as the AES-GCM AAD.

If OnDecrypt fails to do any of the above, OnDecrypt MUST fail.

### Branch Key Wrapping and Unwrapping AAD

To Encrypt and Decrypt the `wrappedDerivedBranchKey` the keyring MUST include the following values as part of the AAD for
the AES Encrypt/Decrypt calls.

To construct the AAD, the keyring MUST concatenate the following values

1. "aws-kms-hierarchy" as UTF8 Bytes
1. Value of `branch-key-id` as UTF8 Bytes
1. [version](../structures.md#branch-key-version) as Bytes
1. [encryption context](structures.md#encryption-context-1) from the input
   [encryption materials](../structures.md#encryption-materials) according to the [encryption context serialization specification](../structures.md#serialization).

| Field               | Length (bytes) | Interpreted as                                       |
| ------------------- | -------------- | ---------------------------------------------------- |
| "aws-kms-hierarchy" | 17             | UTF-8 Encoded                                        |
| branch-key-id       | Variable       | UTF-8 Encoded                                        |
| version             | 16             | Bytes                                                |
| encryption context  | Variable       | [Encryption Context](../structures.md#serialization) |

If the keyring cannot serialize the encryption context, the operation MUST fail.

## Appendix A: Cache Entry Identifier Formulas

When accessing the underlying cryptographic materials cache,
the hierarchical keyring MUST use the formulas specified in this appendix
in order to compute the [cache entry identifier](../cryptographic-materials-cache.md#cache-identifier).

### Preliminaries

Each of the cache entry identifier formulas includes serialized information related to the branch key,
as defined in the [Key Provider Info](../structures.md#key-provider-information).

### Encryption Materials

When the hierarchical keyring receives an OnEncrypt request,
the cache entry identifier MUST be calculated as the first 32 bytes of the
SHA-512 hash of the following byte strings, in the order listed:

| Field                    | Length (bytes) | Interpreted as |
| ------------------------ | -------------- | -------------- |
| Length of branch-key-id  | 3              | UInt8          |
| branch-key-id            | Variable       | UTF-8 Encoded  |
| Null Byte                | 1              | `0x00`         |
| Constant string "ACTIVE" | 6              | UTF-8 Encoded  |

As a formula:

```
branch-key-id = UTF8Encode(hierarchicalKeyring.BranchKeyIdentifier)
branch-key-digest - SHA512(branch-key-id)

ENTRY_ID = SHA512(
    LengthUint8(branch-key-id) +
    branch-key-digest +
    + 0x00
    + UTF8Encode("ACTIVE")
)[0:32]
```

### Decryption Materials

When the hierarchical keyring receives an OnDecrypt request,
it MUST calculate the cache entry identifier as the first 32 bytes of the SHA-512 hash of the following byte strings, in the order listed:

| Field                   | Length (bytes) | Interpreted as |
| ----------------------- | -------------- | -------------- |
| Length of branch-key-id | 3              | UInt8          |
| branch-key-id           | Variable       | UTF-8 Encoded  |
| Null Byte               | 1              | `0x00`         |
| Branch key version      | 36             | String         |

```
branch-key-id = UTF8Encode(edk.providerInfo)
branch-key-digest - SHA512(branch-key-id)

ENTRY_ID = SHA512(
    LengthUint8(branch-key-id) +
    branch-key-digest +
    0x00 +
    branch key version
)[0:32]
```

## Branch Key Supplier

The Branch Key Supplier is an interface containing the `GetBranchKeyId` operation.
This operation MUST take in an encryption context as input,
and return a branch key id (string) as output.

This supplier may be implemented by customers in order to configure behavior where the hierarchical
keyring may decide on which branch key to use based on information in the encryption context.
This gives customers more flexibility in multi-tenant use cases.

## Appendix B: Security Considerations for Branch Key Rotation

Branch Keys are not used to wrap plaintext data keys; instead they are used to derive unique `derivedBranchKeys`.
The `derivedBranchKeys` are responsible for wrapping plaintext data keys set on the [encryption materials](../structures.md#encryption-materials).

Branch Keys have a limit on how many times they are able to derive a `derivedBranchKey` before a theoretical collision.

To derive a `derivedBranchKey` the Keyring uses a 16 byte salt for the KDF.
Additionally the keyring uses a 12-byte IV for the AES-GCM-256 for key wrapping.
We have selected to use these `salt` and `IV` parameters as they are the same parameters used
in [AWS KMS key derivation](https://rwc.iacr.org/2018/Slides/Gueron.pdf).

Overall this results in 28-bytes of randomness.

Birthday Problem calculations for current selection:

```
16 + 12 = 28-bytes or 224-bits of randomness

(224 - 32) / 2 = 96

2^96 = 7.9228163e+28 or 79,228,162,514,264,337,593,543,950,336
```

The above number is how many times one would have to wrap with the `derivedBranchKey` before a theoretical collision.
Given the magnitude of the result; it is recommended to rotate the `branchKey` once per year as this is also the cadence
at which AWS KMS rotates its [AWS Managed Keys](https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-how-it-works).
Using this combination of IV and salt for the KDF and Wrapping operations significantly extends the lifetime of the key; this allows
customers to define their rotation schedule by criteria other than cryptographic safety limits.

### Other considerations

We considered deriving a `derivedBranchKey` with the following construction.

Use a 32-byte salt for an HKDF operation using the `branchKey` as the input key material.
Use no IV for AES GCM 256 Key Wrapping

This would result in a total of 256 bits of randomness

32 = 256 bits of randomness

Birthday Problem calculations:

```
(256 - 32) / 2 = 112

2^112 = 5,192,296,858,534,827,628,530,496,329,220,096
```

The above number is how many times one would have to wrap with the `derivedBranchKey` before a theoretical collision.
Although this is a higher number we decided on the current selection of including a salt and an IV to not only
lower the overhead of bytes we have to store in the [edk ciphertext](../structures.md#ciphertext) but to
easily reason about the security properties of the key derivation since it is what AWS KMS does.
