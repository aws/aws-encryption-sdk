[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Clarify Caching Cryptographic Materials Manager Initialization Parameters

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

Upon initialization,
the [caching Cryptographic Materials Manager (caching CMM)](../../framework/caching-cmm.md) MUST define
an underlying CMM,
an underlying Cryptographic Materials Cache (CMC),
and a [time-to-live (TTL)](../../framework/caching-cmm.md) for cache entries.
This change clarifies that the caller MUST provide these parameters.

## Out of Scope

- Changing the shape of any CMC APIs is out of scope.
- Changing the shape of the CMM interface is out of scope.

## Motivation

Before this change,
the [caching CMM](../../framework/caching-cmm.md) specification
lists several properties that the caching CMM MUST define upon initialization.
Among these,
we intend for the caller to provide
the [underlying CMC](../../framework/caching-cmm.md#underlying-cryptographic-materials-cache),
the [underlying CMM](../../framework/caching-cmm.md#underlying-cryptographic-materials-manager),
and the [TTL](../../framework/caching-cmm.md#cache-limit-ttl);
but the specification does not state that intent.
This change adds this missing intent.

## Security Implications

This change SHOULD NOT have any security implications.

## Operational Implications

This change can potentially break any customer using an AWS Encryption SDK implementation
that does not require the user to provide the TTL value upon caching CMM initialization.
However, all existing implementations require the user to provide the TTL value,
so in practice this is not an issue.

## Guide- and Reference-level Explanation

When initializing a caching CMM, the caller MUST provide
an [underlying CMC](../../framework/caching-cmm.md#underlying-cryptographic-materials-cache)
and a [TTL](../../framework/caching-cmm.md#cache-limit-ttl).

The caller MUST either provide
an [underlying CMM](../../framework/caching-cmm.md#underlying-cryptographic-materials-manager)
or a [keyring](../../framework/keyring-interface.md).
If the caller provides a keyring,
then the caching CMM MUST set its underlying CMM
to a [default CMM](../../framework/default-cmm.md) initialized with the keyring.
