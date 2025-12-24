[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Synchronized Local Cryptographic Materials Cache

## Version

### Changelog

- 0.1.0
  - Initial record
  - [Thread Safe Cache](../changes/2023-06-19_thread_safe_cache/change.md)

## Overview

The synchronized local Cryptographic Materials Cache (synchronized local CMC)
is a built-in implementation of the [CMC interface](cryptographic-materials-cache.md)
provided by the AWS Encryption SDK.

It provides thread safe access to a [Local CMC](local-cryptographic-materials-cache.md)

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

On initialization of the synchronized local CMC,
the caller MUST provide exactly what is required by a
[Local CMC](local-cryptographic-materials-cache.md).

## Behaviors

All behaviors MUST be exactly the same as a [Local CMC](local-cryptographic-materials-cache.md),
even if used in a multi-threaded context.
