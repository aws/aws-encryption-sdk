[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Required Examples

## Version

0.2.0

### Changelog

- 0.2.0

  - Add [Example Templates](#example-templates) section

- 0.1.0

  - [Require examples](../changes/2020-06-16_required-examples/change.md)

## Implementations

- [Python (DEV)](https://github.com/aws/aws-encryption-sdk-python/blob/keyring/examples/)
- [Java (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/)

## Overview

Every implementation MUST include an example
for each use-case described in [example templates](./templates).

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Requirements

- Every example MUST be tested
  as part of the continuous integration testing.
- Every example MUST be independent and complete.
- Every example SHOULD follow a consistent layout and framing.

  - ex: Consistent function name and input parameters across examples.

- Every example MUST only contain logic that is reasonable for production use.
- If an example MUST contain logic that is not reasonable for production use,
  it MUST include clear comments identifying that logic as such
  and instructing the reader what they SHOULD do instead.

  - ex: Raw keyring examples generate wrapping keys as part of the example.
    These examples MUST contain guidance that
    in production those keys SHOULD be managed by an HSM.

### Example Templates

Each example is defined by a [template](./templates).
Templates MUST include sufficient code to clearly demonstrate
how to implement an example.
This code MAY be in any language.

Each template MUST contain the following sections:

1. Header :
   This is a detailed description
   that independently describes the purpose of the example
   and what it describes.
   It includes any links to supporting documentation.

   - Implementations MUST include this text verbatim
     at the top of the file,
     adjusted appropriately for language comment syntax.

1. Summary :
   One-sentence summary of the header.

   - Implementations MUST include this text verbatim
     in the documentation for the example entry point
     function/method/etc,
     adjusted appropriately for language comment syntax.

1. Inputs :
   List of inputs that the example entry point MUST accept,
   with an accompanying description.

   - Implementations MUST provide all of these inputs in CI
     as part of their testing framework.
   - Implementations MUST name their inputs to match the naming of
     the input in the template.
   - Implementations MUST use the description text verbatim,
     adjusted appropriately for language comment syntax.

1. Steps :
   List of steps that define the example.

- Implementations MUST include every step.
- Implementations MUST include any comments verbatim,
  adjusted appropriately for language comment syntax.
- Implementations MAY alter the order of steps
  if another order is more appropriate for that language.
