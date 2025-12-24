[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Refactor Cryptographic Materials Cache Specification

## Affected Features

| Feature                                                                                       |
| --------------------------------------------------------------------------------------------- |
| [Cryptographic Materials Cache](../../framework/cryptographic-materials-cache.md)             |
| [Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md) |

## Affected Specifications

| Specification                                                                                 |
| --------------------------------------------------------------------------------------------- |
| [Cryptographic Materials Cache](../../framework/cryptographic-materials-cache.md)             |
| [Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md) |

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

Before this change,
the specification of the Cryptographic Materials Cache (CMC) interface
gave a only brief explanation of the local CMC:

> The local CMC is a configurable, in-memory, least recently used (LRU) cache.
> It provides non-blocking, locking, cache entries per cache identifier.

This change captures a stricter set of behaviors
upon which existing local CMC implementations already agree,
within a specification separate from the abstract CMC interface.
These behaviors include how the local CMC bounds its space and time usage,
its eviction strategy,
and parameters with which users can configure those behaviors.

## Out of Scope

- Changing the shape of any CMC interface APIs is out of scope.
- Changing the shape of any CMM APIs is out of scope.

## Motivation

### Separating the Local CMC From Its Interface

The local CMC serves as a built-in in-memory cache
for use with the caching Cryptographic Materials Manager (caching CMM),
but its behavior is underspecified
when compared to the those shared by the existing implementations.
In order to ensure consistency between existing and future implementations of the local CMC,
this change creates a separate specification for the local CMC.
The separation helps to draw a distinction between the abstract CMC interface
and concrete types of CMCs,
which currently includes both the local CMC
and the null CMC (in some implementations).
This pattern of separation can also be seen
in the specification of the CMM interface and its implementations,
the default CMM and the caching CMM.

Since the caching CMM requires some CMC implementation to work,
and because the local CMC is a simple type of CMC
that already exists in AWS Encryption SDK implementations today,
this change mandates that all AWS Encryption SDK implementations MUST
provide a local CMC implementation.
This does not require any up-front effort
to add to existing AWS Encryption SDK implementations,
but helps ensure that AWS Encryption SDK users
can easily get started with the caching CMM
without having to separately obtain a CMC implementation.

### Bounding Space Usage and Entry Lifetime

Unbounded caches are bad in general,
but ones that hold cryptographic materials are especially dangerous.
In order to ensure that memory usage of the local CMC does not grow without bound,
this change specifies that the local CMC
MUST set an _entry capacity_ that it will enforce throughout its lifetime.
We mandate that the user provide the entry capacity upon initialization of the local CMC instance,
so that the user makes a conscious decision
about the amount of memory they are willing to allocate for cache entries.

We also mandate that the local CMC must accept any non-negative value
(up to an implementation-defined limit),
so that in addition to typical use cases (for entry capacity at least 2),
a user can simulate a null cache (by using entry capacity 0)
or a "debouncing" cache (by using entry capacity 1).
By "debouncing" cache,
we mean one that caches materials
upon consecutive cache requests for those materials.
Although these are not necessarily common use cases,
they are natural generalizations of the local CMC,
and so without evidence to the contrary,
we have decided that they are worth supporting.
Because the null cache is a special case of the local CMC as defined above,
it is also unnecessary to individually specify the null CMC as its own entity,
which in turn reduces specification and implementation effort.

Continuing with the theme of avoiding unbounded caches,
we note that it is dangerous for cryptographic materials to remain in the cache
for longer than necessary,
since doing so increases the temporal attack surface.
This change therefore mandates that
the local CMC associates a time-to-live (TTL) control to each cache entry,
and never returns a cache entry that is _expired_ with respect to its TTL value
(via a Get Cache Entry operation).
Existing local CMC implementations already implement this.

It is unlikely that any default TTL value can prove sane for most customer use cases.
Furthermore, we make the simplifying assumption that the caching CMM is the only user of the local CMC,
and the caching CMM MUST be configured with a TTL value.
So this change mandates that the user (i.e., the caching CMM)
MUST provide a cache entry's TTL value in the corresponding Put Cache Entry call.

### Eviction

With the above rules in place to dictate how long and in what quantity
the local CMC can retain cache entries,
we turn our attention to when and how the local CMC evicts entries.

This change mandates that the local CMC evicts some TTL-expired entries
on every Get Cache Entry and Put Cache Entry operation,
and also that it evicts any additional entries as necessary
to not exceed the entry capacity
on every Put Cache Entry operation.
This "lazy eviction" is sufficient to ensure that the local CMC
neither returns TTL-expired entries
nor exceeds the entry capacity.

The change also recommends that the local CMC evict TTL-expired entries on a periodic basis,
if the implementation's programming language and runtime make doing so feasible.
By doing so,
the local CMC ensures that no entry lives for more than a constant amount of time past its TTL;
without periodic eviction,
a TTL-expired entry could remain in the cache indefinitely
if no user makes any Get Cache Entry or Put Cache Entry call
after the entry enters the cache.
The change does not mandate periodic eviction because
concurrency is complex to implement.

This change specifies that the local CMC
evicts entries in least recently used (LRU) order.
Prior to this change,
the specification already stated
that the local CMC is a least recently used LRU cache.
Since LRU eviction is a sane default strategy for many use cases,
and since LRU cache implementations exist in most mainstream programming language ecosystems,
this change continues to mandate that the local CMC uses LRU eviction.

This change specifies that during an eviction opportunity,
the local CMC first checks a fixed number of entries from the tail of the LRU cache
and evicts any TTL-expired entries,
and then evicts as many entries from the tail as is necessary to not exceed the entry capacity.
Limiting the number of entries checked helps to avoid latency spikes
in the case that many entries expire around the same time.

## Drawbacks

Mandating that the local CMC support entry capacity values of 0 and 1,
may result in edge cases in LRU removal logic
(as hinted in the [C implementation](https://github.com/aws/aws-encryption-sdk-c/blob/aa85ca224d550cfe110e2112821a84506b9aca3e/source/local_cache.c#L924)).
However, we believe that the functional benefits
outweigh the complexity of the edge cases.

By allowing the local CMC not to periodically evict TTL-expired entries,
a scenario can arise in which
an expired cache entry remains in the cache indefinitely
because the cache is untouched after the entry enters.
The specification represents a best-effort attempt to avoid retaining cache entries for long periods of time,
and since this case is a relatively uncommon usage pattern,
we are willing to accept this weakness.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

This change will break any customer
who relies on the local CMC behavior in the AWS Encryption SDK for C
if the customer attempts to set an entry capacity less than 2.

## Guide-level Explanation

The AWS Encryption SDK provides a build-in local Cryptographic Materials Cache (local CMC),
which implements the CMC interface.
The local CMC is an in-memory, least recently used (LRU) cache.
It provides atomic access to [cache entries](cryptographic-materials-cache.md#cache-entry)
per [cache identifier](cryptographic-materials-cache.md#cache-identifier).

When initializing the local CMC,
the user MUST provide an _entry capacity_ value,
which determines the maximum number of cache entries
that the local CMC will contain at any time.
When calling the Put Cache Entry operation,
the user MUST provide a _time-to-live_ (TTL) value,
which determines for how long the inserted cache entry will be retrievable from the cache.
The local CMC will evict entries as needed in order to keep its size at or below the entry capacity,
and will also lazily evict TTL-expired entries
(up to a constant quantity, to avoid latency spikes when many entries expire).
The user can set the entry capacity value to zero in order to disable caching altogether.

## Reference-level Explanation

The AWS Encryption SDK specification MUST specify the CMC interface and the local CMC separately.
AWS Encryption SDK implementations MUST provide a local CMC implementation.

During initialization,
the user MUST provide an entry capacity value to the local CMC.
The local CMC MUST NOT store more entries than this value,
except temporarily while performing a Put Cache Entry operation.
The local CMC MUST accept entry capacity values between zero
and an implementation-defined maximum, inclusive.

During each Put Cache Entry operation
before returning the inserted cache entry,
the local CMC MUST evict least recently used entries
until the number of stored entries does not exceed the entry capacity.

When calling the Put Cache Entry operation,
the user MUST provide a time-to-live (TTL) value.
After the inserted entry's TTL has elapsed,
the local CMC MUST NOT allow any user to retrieve the entry.

During each Get Cache Entry and Put Cache Entry operation,
the local CMC MUST evict all TTL-expired entries
in the `N` least recently used entries,
where `N` is an implementation-defined positive integer constant.
The local CMC SHOULD periodically evict TTL-expired entries
in the `N` least recently used entries.
