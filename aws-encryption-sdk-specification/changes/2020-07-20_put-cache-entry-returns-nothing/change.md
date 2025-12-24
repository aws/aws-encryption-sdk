[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Return Nothing from Put Cache Entry in Cryptographic Materials Cache

## Affected Features

| Feature                                                                                     |
| ------------------------------------------------------------------------------------------- |
| [Cryptographic Materials Cache Interface](../../framework/cryptographic-materials-cache.md) |

## Affected Specifications

| Specification                                                                               |
| ------------------------------------------------------------------------------------------- |
| [Cryptographic Materials Cache Interface](../../framework/cryptographic-materials-cache.md) |

## Affected Implementations

| Language | Repository                                                                    |
| -------- | ----------------------------------------------------------------------------- |
| Java     | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)     |
| Python   | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The [Cryptographic Materials Cache (CMC)](../../framework/cryptographic-materials-cache.md) specification
does not specify whether or not the Put Cache Entry operation
returns the cache entry that it inserts.
Some implementations do and others do not.
This change specifies that the Put Cache Entry operation
MUST NOT return the cache entry that it inserts.

## Out of Scope

- Changing the shape of the caching CMM operations is out of scope.

## Motivation

The [Cryptographic Materials Cache (CMC)](../../framework/cryptographic-materials-cache.md) specification
does not specify whether or not the Put Cache Entry operation
returns the cache entry that it inserts.
Noting that the only user of the CMC is the
[caching Cryptographic Materials Manager](../../framework/caching-cmm.md),
we suppose that the operation does return the inserted entry
and examine what the caching CMM might do with it.

The caching CMM acquires cryptographic materials
from an underlying CMM before placing them in the CMC
via the Put Cache Entry operation.
It also provides usage metadata to the operation
so that the CMC can attach the metadata to the cache entry
before storing the entry.
But in the interest of separating concerns,
the CMC does not include any other data in the cache entry,
so the inserted cache entry ultimately contains only
data that was originally provided by the caching CMM.
It follows that the caching CMM has no reason
to _read_ the inserted cache entry if the entry were returned.

Furthermore,
the caching CMM aims to provide only atomic operations,
which necessitates that it reads and modifies the CMC state atomically.
Suppose the caching CMM were to place materials in the CMC
via the Put Cache Entry operation,
receive the inserted entry via return value,
and then modify the entry
before returning the cryptographic materials to its caller.
On the one hand,
it is unsafe for the caching CMM to return the materials
without writing the modified entry back to the CMC.
On the other hand,
it is not atomic if the caching does write the modified entry back to the CMC.
It follows that the caching CMM has no safe way
to _write_ to the inserted cache entry if the entry were returned,
and so it has no good reason to.

The caching CMM has reason
neither to read nor write to a returned cache entry,
so we conclude that the operation MUST NOT return the inserted cache entry.

Indeed,
the caching CMM ignores the operation's return value
in every existing implementation
where the operation returns the inserted cache entry
(namely, the Java and Python implementations).
This solidifies our view that the return value is unnecessary.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

We MUST change the Java and Python implementations of Put Cache Entry
to not return the inserted cache entry.

## Guide-level and Reference-level Explanation

The Put Cache Entry operation MUST NOT return the inserted cache entry.
