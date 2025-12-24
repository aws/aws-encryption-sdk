[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Cryptographic Materials Cache Interface

## Version

0.5.0

### Changelog

- 0.5.2
  - Add specific language for multi-threading, and PutItem where the item already exists.
- 0.5.1
  - Rename Hierarchical Materials to Branch Key Materials.
  - Add Beacon Key Materials to allowed materials in the cache.
- 0.5.0
  - Add Hierarchical Materials to the materials
    allowed in the cache.
- 0.4.0
  - Updating the interface. Moving elements from the Local CMC to the interface.
- 0.3.0
  - [Return Nothing from Put Cache Entry in Cryptographic Materials Cache](../changes/2020-07-20_put-cache-entry-returns-nothing/change.md)
- 0.2.0
  - [Refactor Cryptographic Materials Cache Specification](../changes/2020-07-14_refactor-cmc-spec/change.md)
- 0.1.0-preview
  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                       |
| ---------- | -------------------------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [cache.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/cache.h)                                                                     |
| Javascript | 0.1.0-preview                          | 0.1.0                     | [cryptographic_materials_cache.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/cache-material/src/cryptographic_materials_cache.ts) |
| Python     | 0.1.0-preview                          | 1.3.0                     | [caches/base.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/caches/base.py)                                                 |
| Java       | 0.1.0-preview                          | 1.3.0                     | [CryptoMaterialsCache.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/caching/CryptoMaterialsCache.java)  |

## Overview

Cryptographic materials cache (CMC) is used by the [caching cryptographic materials manager (CMM)](caching-cmm.md)
to store cryptographic materials for reuse.
This document describes the interface that all CMCs MUST implement.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Concepts

### Cache Identifier

The cache identifier used to uniquely identify a single cache entry
in the cryptographic materials cache.

### Cache Entry

A cache entry represents an entry in the cryptographic materials cache
and MUST have the following information.

- [Materials](#materials)
- [Creation Time](#creation-time)
- [Expiry Time](#expiry-time)
- [Usage Metadata](#usage-metadata)

#### Materials

The cryptographic materials,
[encryption](structures.md#encryption-materials), [decryption](structuresmd#decryption-materials),
[branch key](structures.md#branch-key-materials), or [beacon key](structures.md#beacon-key-materials)
materials to be cached along with other metadata.

#### Creation Time

Timestamp when the cache entry was created.

#### Expiry Time

Timestamp when the cache entry expires.

#### Usage Metadata

The usage metadata is of the following two types:

- [Messages Usage](#message-usage)
- [Bytes Usage](#bytes-usage)

Updating usage metadata SHOULD be atomic.

##### Message Usage

The number of messages encrypted
by the [encryption materials](structures.md#encryption-materials)
cached in this cache entry.

##### Bytes Usage

The number of bytes encrypted by the [encryption materials](structures.md#encryption-materials)
cached in this cache entry.

### Time-to-live (TTL)

Each cache entry has a time-to-live (TTL)
that represents a point in time at which the cache entry
MUST be considered invalid.
This is generale the [expiry time](#expiry-time).
After a cache entry's TTL has elapsed,
we say that the entry is _TTL-expired_,
and a CMC MUST NOT return the entry to any caller.

However the [creation time](#creation-time) is also include
in case a stricter view of TTL is enforced by a caller.
This can be done by deleting the entry.

## Thread Safety

The CMC interface says nothing about thread safety.
Specific implementations provide different levels of thread safety,
and client code must select the appropriate implementation for their use case.

## Supported CMCs

The AWS Encryption SDK provides a built-in [local cryptographic materials cache](local-cryptographic-materials-cache.md) (local CMC).
The local CMC is a configurable, in-memory, least recently used (LRU) cache.
It provides non-blocking, locking, [cache entries](#cache-entry) per [cache identifier](#cache-identifier),
and is NOT thread safe.

Also provided are :

- SynchronizedLocalCMC : a thread safe wrapper around the local CMC.
- StormTrackerCMC : a thread safe wrapper around the local CMC, which also ameliorates KMS storms,
  by preventing multiple clients from resolving the same KMS key at the same time.

## Behaviors

The Cryptographic Materials Cache
provides behaviors for putting cache entries,
getting cache entries and deleting cache entries.

### Put Cache Entry

Attempts to put a cache entry for the specified cache ID.
If a cache entry for the given cache ID exists in the cache, it must be removed.
The CMC MUST create a new cache entry for the specified cache ID.
This operation MUST NOT return the inserted cache entry.
The cache entry MUST include all [usage metadata](#usage-metadata)
since this information can not be updated after the put operation.

If used in a multi-threaded context,
the next [Get Cache Entry](#get-cache-entry) operation
MAY not return the entry just added.

### Get Cache Entry

Attempts to get a cache entry for the specified cache ID.
The CMC MUST validate that the cache entry
has not exceeded it's stored [TTL](#time-to-live-ttl).
A successful call to Get Entry returns the [cache entry](#cache-entry)
and an unsuccessful call returns a cache miss.

If used in a multi-threaded context :

- Get Cache Entry MAY return a cache miss when the TTL has net yet been exceeded.

- Get Cache Entry MAY not return immediately if no cache entry exists for the specified cache ID,
  and a cache miss was recently returned for another thread.

### Delete Cache Entry

Attempts to delete a cache entry from the CMC.

If no cache entry exists for the specified cache ID, Delete Cache Entry must return successfully.

## Background Processing

If a cache sees no activity for a long time,
then even though all of the entries may have expired,
they still exist in the cache.

An implementation SHOULD provide a way to avoid this, for example,
by spawning a background thread to occasionally remove expired entries.
