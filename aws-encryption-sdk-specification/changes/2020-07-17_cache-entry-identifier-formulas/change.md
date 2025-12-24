[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Specify Cache Entry Identifier Formulas for Caching Cryptographic Materials Manager

## Affected Features

| Feature                                                                   |
| ------------------------------------------------------------------------- |
| [Caching Cryptographic Materials Manager](../../framework/caching-cmm.md) |

## Affected Specifications

| Specification                                                             |
| ------------------------------------------------------------------------- |
| [Caching Cryptographic Materials Manager](../../framework/caching-cmm.md) |

## Affected Implementations

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Java       | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)             |
| JavaScript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |
| Python     | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python)         |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The [caching Cryptographic Materials Manager (caching CMM)](../../framework/caching-cmm.md)
computes [identifiers](../../framework/cryptographic-materials-cache.md#cache-identifier)
for each cache entry it places into its underlying Cryptographic Materials Cache (CMC).
Existing caching CMM implementations agree on the set of formulas
they use to compute these identifiers,
and this change adds those formulas to the specification.

## Out of Scope

- Changing the shape of the caching CMM operations is out of scope.
- Changing the shape of the CMC interface operations is out of scope.

## Motivation

In all generally-available implementations of the AWS Encryption SDK (ESDK),
the caching Cryptographic Materials Manager (caching CMM)
uses a particular set of formulas
to compute the identifiers for cache entries when interacting with the Cryptographic Materials Cache (CMC).
Although these implementations agree on this set of formulas,
the formulas are neither explicitly stated nor mandated in the specification.

Mandating that implementations share cache entry identifier formulas
has a number of benefits:

1.  Implementers will have a definitive reference
    for precisely which cache entry fields MUST be used to distinguish cache entries,
    and also provides a concrete data format by which to serialize them.
1.  It is easier to reason about the caching CMM across implementations
    if one can assume that they distinguish cache entries by the same criteria.
1.  If it becomes necessary to change cache identifiers in the future
    (e.g., in order to distinguish cache entries by more features),
    then we can do so uniformly across implementations.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

We MUST add ESDK test vectors to verify that caching CMM implementations
correctly implement the cache entry identifier formulas.

## Guide-level Explanation

See [Reference-level Explanation](#reference-level-explanation).

## Reference-level Explanation

### Preliminaries

Each of the cache entry identifier formulas includes a serialized encryption context,
as defined in the
[Key Value Pairs specification](../../data-format/message-header.md#key-value-pairs).
In the following sections we use `SerializeEncryptionContext`
to denote the function that,
given an encryption context,
returns the serialization of the encryption context.

Some of the cache entry identifier formulas include
the two-byte algorithm suite ID for the algorithm suite in a materials request.
The algorithm suite IDs are defined in the
[Supported Algorithm Suites specification](../../framework/algorithm-suites.md#supported-algorithm-suites).
In the following sections we use `AlgorithmSuiteId`
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
    where serialization is as defined in the [Encrypted Data Key Entries specification](../../data-format/message-header.md#encrypted-data-key-entries).
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
