[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Caching Cryptographic Materials Manager

## Version

0.3.0

### Changelog

- 0.3.0
  - [Specify Cache Entry Identifier Formulas for Caching Cryptographic Materials Manager](../changes/2020-07-17_cache-entry-identifier-formulas/change.md)
- 0.2.0
  - [Clarify Caching Cryptographic Materials Manager Initialization Parameters](../changes/2020-07-15_clarify-caching-cmm-init-params/change.md)
- 0.1.0-preview
  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                                                  |
| ---------- | -------------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.2.0                                  | 0.1.0                     | [cache.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/cache.h)                                                                                                |
| NodeJS     | 0.2.0                                  | 0.1.0                     | [caching_materials_manager_node.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/caching-materials-manager-node/src/caching_materials_manager_node.ts)          |
| Browser JS | 0.2.0                                  | 0.1.0                     | [caching_materials_manager_browser.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/caching-materials-manager-browser/src/caching_materials_manager_browser.ts) |
| Python     | 0.2.0                                  | 1.3.0                     | [materials_managers/caching.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/materials_managers/caching.py)                                              |
| Java       | 0.2.0                                  | 1.3.0                     | [CachingCryptoMaterialsManager.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/caching/CachingCryptoMaterialsManager.java)           |

## Overview

The Caching Cryptographic Materials Manager (CMM) is a built-in implementation of the [CMM interface](cmm-interface.md) provided by the AWS Encryption SDK.
The caching CMM wraps around another CMM and caches the results of its underlying key provider's encryption and decryption operations.
A caching CMM reduces the number of calls made to the underlying key provider, thereby reducing cost and/or improving performance.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

On caching CMM initialization,
the caller MUST provide the following values:

- [Underlying Cryptographic Materials Cache (CMC)](#underlying-cryptographic-materials-cache)
- [Cache Limit TTL](#cache-limit-ttl)

Additionally, the caller MUST provide one of the following values:

- [Underlying Cryptographic Materials Manager (CMM)](#underlying-cryptographic-materials-manager)
- [Keyring](keyring-interface.md)

If the caller provides a keyring,
then the caching CMM MUST set its underlying CMM
to a [default CMM](default-cmm.md) initialized with the keyring.

Finally, the caching CMM MUST optionally accept the following values:

- [Partition ID](#partition-id)
- [Limit Bytes](#limit-bytes)
- [Limit Messages](#limit-messages)

### Underlying Cryptographic Materials Cache

The caching CMM intercepts requests for [encryption](cmm-interface.md#encryption-materials-request) and
[decryption](cmm-interface.md#decrypt-materials-request) materials
and forwards them to the underlying [cryptographic materials cache (CMC)](cryptographic-materials-cache.md).

Multiple caching CMMs MAY share the same cryptographic materials cache,
but by default MUST NOT use each other's cache entries.
If multiple caching CMMs are attached to the same cryptograhic materials cache,
they will share cache entries if and only if the [partition ID](#partition-id) parameter is set to the same string.

### Underlying Cryptographic Materials Manager

The underlying [Cryptographic Materials Manager (CMM)](cmm-interface.md#supported-cmms)
to query for encryption/decryption materials on a cache miss.

### Cache Limit TTL

The maximum amount of time in seconds that a data key can be used.
The Caching CMM MUST set a time-to-live (TTL) for data keys in the CMC.
This value must be greater than zero.

### Partition ID

A string that is used to avoid collisions with other CMMs.
If this parameter is not set, the caching CMM MUST set a partition ID
that uniquely identifies the respective caching CMM.
The Partition ID MUST NOT be changed after initialization.

### Limit Bytes

The maximum number of bytes that MAY be encrypted by a single data key.
It is interpreted as UInt64.
If this parameter is not set, the caching CMM MUST set it to a value no more than 2^63-1.
This parameter is set as an additional security thresholds to ensure that
the data keys expire and are refreshed periodically.

### Limit Messages

The maximum number of messages that MAY be encrypted by a single data key.
It is interpreted as Uint64.
If this parameter is not set, the caching CMM MUST set it to 2^32.
This parameter is set as an additional security thresholds to ensure that
the data keys expire and are refreshed periodically.

The caching CMM MUST provide a structure as defined below,
to track usage statistics.

### Usage Stats

The usage stats contains two fields:

- [Messages Encrypted](#messages-encrypted)
- [Bytes Encrypted](#bytes-encrypted)

When the caching CMM stores encryption materials into the cryptographic materials cache,
the caching CMM MUST set the initial usage stats for the cache entry.

When the caching CMM obtains encryption materials from the cryptographic materials cache,
the caching CMM MUST update the usage stats for the cache entry retrieved.

#### Messages Encrypted

The number of messages encrypted by the [encryption](structures.md#encryption-materials) materials.

#### Bytes Encrypted

The number of bytes encrypted by the [encryption](structures.md#encryption-materials) materials.

## Behaviors

### Get Encryption Materials

If the [algorithm suite](algorithm-suites.md) requested contains a [Identity KDF](algorithm-suites.md#identity-kdf),
the caching CMM MUST obtain the encryption materials by making a call to the underlying CMM's [Get Encryption Materials](cmm-interface.md#get-encryption-materials) function.

Otherwise, the caching CMM MUST attempt to find the [encryption materials](structures.md#encryption-materials)
from the underlying [cryptographic materials cache (CMC)](#underlying-cryptographic-materials-cache).
The caching CMM MUST use the formulas specified in [Appendix A](#appendix-a-cache-entry-identifier-formulas)
in order to compute the [cache entry identifier](cryptographic-materials-cache.md#cache-identifier).

If a cache entry is found, the caching CMM MUST return the encryption materials retrieved.
If a cache entry is not found or the cache entry is expired, the caching CMM MUST then attempt to obtain the encryption materials
by making a call to the underlying CMM's [Get Encryption Materials](cmm-interface.md#get-encryption-materials).

If the [algorithm suite](algorithm-suites.md) requested does not contain an [Identity KDF](algorithm-suites.md#identity-kdf),
the caching CMM MUST add the encryption materials obtained from the underlying CMM into the underlying CMC.

If the [algorithm suite](algorithm-suites.md) requested contains an Identity KDF,
the caching CMM MUST NOT store the encryption materials in the underlying CMC.

### Decrypt Materials

If the [algorithm suite](algorithm-suites.md) requested contains a [Identity KDF](algorithm-suites.md#identity-kdf),
the caching CMM MUST obtain the decryption materials by making a call to the underlying CMM's [Decrypt Materials](cmm-interface.md#decrypt-materials) function.

Otherwise, the caching CMM MUST attempt to find the [decryption materials](structures.md#decryption-materials)
from the [underlying CMC](#underlying-cryptographic-materials-cache).
The caching CMM MUST use the formulas specified in [Appendix A](#appendix-a-cache-entry-identifier-formulas)
in order to compute the [cache entry identifier](cryptographic-materials-cache.md#cache-identifier).

If a cache entry is found, the caching CMM MUST return the decryption materials retrieved.
If a cache entry is not found or the cache entry is expired, the caching CMM MUST attempt to obtain the decryption materials
by making a call to the underlying CMM's [Decrypt Materials](cmm-interface.md#decrypt-materials).

If the [algorithm suite](algorithm-suites.md) requested does not contain an [Identity KDF](algorithm-suites.md#identity-kdf),
the caching CMM MUST add the decryption materials obtained from the underlying CMM into the underlying CMC.

If the [algorithm suite](algorithm-suites.md) requested contains an Identity KDF,
the caching CMM MUST NOT store the decryption materials in the underlying CMC.

## Appendix A: Cache Entry Identifier Formulas

When accessing the underlying CMC,
the caching CMM MUST use the formulas specified in this appendix
in order to compute the [cache entry identifier](cryptographic-materials-cache.md#cache-identifier).

### Preliminaries

Each of the cache entry identifier formulas includes a serialized encryption context,
as defined in the [encryption context serialization specification](structures.md#serialization).
In the following appendix sections we use `SerializeEncryptionContext`
to denote the function that,
given an encryption context,
returns the serialization of the encryption context.

Some of the cache entry identifier formulas include
the two-byte algorithm suite ID for the algorithm suite in a materials request.
The algorithm suite IDs are defined in the
[Supported Algorithm Suites specification](./algorithm-suites.md#supported-algorithm-suites).
In the following appendix sections we use `AlgorithmSuiteId`
to the denote the function that,
given an algorithm suite as specified in a materials request,
returns the corresponding two-byte algorithm suite ID.

### Encryption Materials, Without Algorithm Suite

If the Get Encryption Materials request does not specify an algorithm suite,
then the cache entry identifier MUST be calculated
as the SHA-512 hash of the concatenation of the following byte strings,
in the order listed:

1. The SHA-512 hash of a UTF-8 encoding of the caching CMM’s Partition ID
2. One null byte (`0x00`)
3. The SHA-512 hash of the serialized encryption context

As a formula:

```
ENTRY_ID = SHA512(
    SHA512(UTF8Encode(cachingCMM.partitionId))
    + 0x00
    + SHA512(SerializeEncryptionContext(getEncryptionMaterialsRequest.encryptionContext))
)
```

### Encryption Materials, With Algorithm Suite

If the Get Encryption Materials request does specify an algorithm suite,
then the cache entry identifier MUST be calculated
as the SHA-512 hash of the concatenation of the following byte strings,
in the order listed:

1.  The SHA-512 hash of a UTF-8 encoding of the caching CMM’s Partition ID
2.  One byte with value 1 (`0x01`)
3.  The two-byte algorithm suite ID corresponding to the algorithm suite in the request
4.  The SHA-512 hash of the serialized encryption context

As a formula:

```
ENTRY_ID = SHA512(
    SHA512(UTF8Encode(cachingCMM.partitionId))
    + 0x01
    + AlgorithmSuiteId(getEncryptionMaterialsRequest.algorithmSuite)
    + SHA512(SerializeEncryptionContext(getEncryptionMaterialsRequest.encryptionContext))
)
```

### Decryption Materials

When the caching CMM receives a Decrypt Materials request,
it MUST calculate the cache entry identifier as
the SHA-512 hash of the concatenation of the following byte strings,
in the order listed:

1.  The SHA-512 hash of a UTF-8 encoding of the caching CMM’s Partition ID
2.  The two-byte algorithm suite ID corresponding to the algorithm suite in the request
3.  The concatenation of the lexicographically-sorted SHA-512 hashes of the serialized encrypted data keys,
    where serialization is as defined in the [Encrypted Data Key Entries specification](../data-format/message-header.md#encrypted-data-key-entries).
4.  A sentinel field of 512 zero bits (or equivalently, 64 null bytes), indicating the end of the key hashes
5.  The SHA-512 hash of the serialized encryption context

As a formula:

```
EDK_HASHES = [SHA512(SerializeEncryptedDataKey(key)) for key in decryptMaterialsRequest.encryptedDataKeys]
ENTRY_ID = SHA512(
    SHA512(UTF8Encode(cachingCMM.partitionId))
    + AlgorithmSuiteId(decryptMaterialsRequest.algorithmSuite)
    + CONCATENATE(SORTED(EDK_HASHES))
    + PADDING_OF_512_ZERO_BITS
    + SHA512(SerializeEncryptionContext(decryptMaterialsRequest.encryptionContext))
)
```
