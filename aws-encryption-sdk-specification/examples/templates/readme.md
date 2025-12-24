[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Examples Section Readme

## Version

0.1.0

### Changelog

- 0.1.0

  - [Require examples](../../changes/2020-06-16_required-examples/change.md)

## Implementations

- [Python (DEV)](https://github.com/aws/aws-encryption-sdk-python/blob/keyring/examples/README.md)
- [Java (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/README.md)

## Overview

Every implementation MUST include a readme
that introduces all examples
and identifies which use-cases
map to which examples.

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Requirements

The readme MUST contain the following information:

1. A listing of each [API example](./api)
   with a short description of the use-case
   and a link to the example.
1. A listing of each [configuration example](./configuration)
   with a short description of the use-case
   and a link to the example.
1. Instructions for developers writing examples
   that describe any requirements
   (organization/layout/naming/etc)
   and any actions that need to be taken
   to make sure the continuous integration testing
   tests the new examples.
