[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Add Thread Safety to Cryptographic Materials Cache Specification

## Affected Specifications

| Specification                                                                                                           |
| ----------------------------------------------------------------------------------------------------------------------- |
| [Cryptographic Materials Cache](../../framework/cryptographic-materials-cache.md)                                       |
| Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md)                            |
| [Synchronized Local Cryptographic Materials Cache](../../framework/synchronized-local-cryptographic-materials-cache.md) |
| [Storm Tracking Cryptographic Materials Cache](../../framework/storm-tracking-cryptographic-materials-cache.md)         |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The [Cryptographic Materials Cache](../../framework/cryptographic-materials-cache.md) interface
and the [Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md)
were written without specifying a threading model.

The threading model detailed here addresses basic thread safety,
as well as the higher level problems of "lookup storms" where
many threads all try to get information for the cache all at once.

## Configuration

Some new configuration is needed

### Grace Period

A number of seconds (at least 1, default 10).

If an entry will expire within this amount of time,
attempts will be made to refresh the cache.

This should be less than the TTL for any item put into the cache,
or it will never come into play.

### Grace Interval

A number of seconds (at least 1, default 1).

While within the [grace period](#grace-period),
attempts to refresh the cache are made no more often than once per interval.

If the Grace Interval if greater than half of the Grace Period,
then only one attempt will be made to fetch new materials before the entry expires.

`((Period-1)/Interval) + 1` is the maximum number of attempted fetches before the entry expires.

### FanOut

A number (at least 1, default 20).

The maximum number of individual keys for which lookups can be in flight.

### In Flight TTL

A number of seconds (at least 1, default 20).

If an entry has been in flight for this long, it is no longer considered in flight.

This prevents the FanOut from being exhausted by clients that are never going to respond.

### PutCacheEntry if key already exists

The first change that must be made is to PutCacheEntry.
The spec doesn't explicitly say what to do if there is already an entry for the given key.
In a multi-threaded environment, this situation can't be avoided, as one thread might
call PutCacheEntry right after called GetCacheEntry and found nothing.
The simplest and more useful behavior is for Put to always Put the new thing,
whether or not something was already in the cache under that key.

### Basic Thread Safety

The [Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md)
has mutable state and is therefore not thread safe.

[Synchronized Local Cryptographic Materials Cache](../../framework/synchronized-local-cryptographic-materials-cache.md)
is a thin synchronized wrapper around
[Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md)
so that all operations are safe under multiple threads,

### Lookup Storms

A more subtle problem with a multi-threaded implementation involves many threads
resolving the same key at the same time. For example, many threads might each look at the cache,
find it empty, and then all make the same call to KMS to fetch a plaintext data key;
which is both slow and expensive.

To ameliorate this, we introduce the
[Storm Tracking Cryptographic Materials Cache](../../framework/storm-tracking-cryptographic-materials-cache.md)
which prevents these storms. This amelioration comes in three parts

#### Expiration Storm

During normal operation, a single cache entry will expire.
It is possible that many threads will notice this at once,
and all of them try to fill the cache with newly acquired data.

To avoid this, we introduce a [grace period](#grace-period) which is a period of time before the expiration
when refilling the cache can begin.

At the beginning of the [grace period](#grace-period), no more than once per [grace interval](#grace-interval),
a client will be told `No Such Entry` even though an entry exists,
with the expectation that this client will fetch new materials and call `PutCacheEntry`,
moving this key out of the [grace period](#grace-period).
All other clients receive the cache entry as usual.

If the cache is not refreshed for this key before the entry expires,
then the entry will expire, and it will be handles as an [initialization storm](#initialization-storm).

In a single threaded context, this results in behavior that is exactly that of the
[Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md)
except that the entries expire at the beginning of the grace period, rather than the end.

#### Initialization Storm

On startup, or any other time that there is no entry in the cache,
the [expiration storm](#expiration-storm) strategy can't work,
because there is nothing to return to the other clients.

In this case, one client per [grace interval](#grace-interval) will
receive `NoSuchEntry` as above.
The other clients will block until an entry is available.

In a single threaded context, this results in behavior that is exactly that of the
[Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md).

#### Multi-Key Storm

[expiration](#expiration-storm) and [initialization](#initialization-storm) storms
only apply to individual keys in isolation.

In the case where there are many different keys in use,
many keys might need refreshing at the same time,
and the other types of storm amelioration won't help.

The number of in flight keys is defined as
the number of keys for which we have returned `No Such Entry`
and for which `PutCacheEntry` has not yet been called.

If `GetCacheEntry` is called, and one of the above amelioration
strategies would otherwise return `No Such Entry`,
and the number of in flight keys would exceed the [FanOut](#fanout),
then we instead return as if the [grace interval](#grace-interval) had
no yet passed, that is, if an unexpired entry exists in the cache we return it,
otherwise we block.

In a single threaded context, this results in behavior that is exactly that of the
[Local Cryptographic Materials Cache](../../framework/local-cryptographic-materials-cache.md).

### Stale Entries

The solution to the [Multi-Key Storm](#multi-key-storm) is the cause of a new problem.

In somewhat rare circumstances, a client might receive NoSuchEntry from GetCacheEntry,
follow that up with a failed attempt to get new materials,
and then never get around to calling GetCacheEntry again for that key.
In this circumstance, that key will considered in flight forever.
Enough of those, and the [FanOut](#fanout) will be full of these stale entries,
and all future cache lookups will block forever.

Thus we introduce the [In Flight TTL](#in-flight-ttl) configuration.

If an entry has been sitting untouched in the In Flight state for this long,
the it is no longer considered in flight, and no longer is counted toward the
limit imposed by [FanOut](#fanout).

### Service Outage

On rare occasions, the service from which you get your plaintext keys might be down.

The Storm Tracking CMC can help keep your service running in this situation,
because with a six hour grace period,
the backend service can be down for six hours with most clients being unaffected.
So if you want your typical TTL to be six hours, but you're willing to extend that
by eight hours if the backend service is down, then set a TTL of 14 hours and
a grace time of eight hours.

One problem remains. While the backend service is down,
we need to make sure that all clients are making progress,
rather than spending all their time querying the backend service.
To remedy this, make sure the [grace interval](#grace-interval) is large compared
to the time it takes to fail to get a key,
so that the remainder of the [grace interval](#grace-interval) can be spent doing work.
