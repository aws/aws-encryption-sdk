[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Define Required Examples

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                         |
| ----------------------------------------------------- |
| [examples](../../examples/examples.md)                |
| [examples readme](../../examples/templates/readme.md) |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| Python     | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python)         |
| Java       | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)             |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

This document defines the examples that each implementation MUST include.

## Out of Scope

The specific implementations details for each example
are out of scope for this document.
The template document for each example contains those details.

## Motivation

The primary motivations for requiring examples include:

- Providing guidance to AWS Encryption SDK customers.
- Facilitating documentation and interoperability.
- Encouraging AWS Encryption SDK developers
  to work backwards from the caller experience
  to verify that APIs and components
  provide the intended functionality,
  clarity, and ease of use.
- Demonstrating that all implementations
  can be used to solve the same problems.

## Drawbacks

Requiring each implementation to include many examples
increases the level of effort to create each implementation.
We believe that the value to customers of
improving the usability of APIs
and providing guidance on how to use them
justifies this cost.

## Security Implications

We MUST assume that any example will be used in production.
Because of this, every example MUST only contain logic
that is reasonable for production use
unless explicitly identified otherwise.

## Operational Implications

Each implementation MUST include all examples
in its continuous integration testing.

## Guide-level Explanation

Every implementation MUST include an example
for each required use-case.
These examples MUST be tested as part of
the implementation's continuous integration testing.

## Reference-level Explanation

- Every example MUST be tested
  as part of the implementation's continuous integration testing.
- Every example MUST be independent and complete.
- Every example SHOULD follow a consistent layout and framing.

  - ex: Consistent function name and input parameters across examples.

- Every example MUST only contain logic that is reasonable for production use.
- If an example MUST contain logic that is not reasonable for production use,
  it MUST include clear comments that
  MUST identify that logic as not fit for production use,
  SHOULD explain why it is not fit for production use,
  and MUST instruct the reader what they SHOULD do instead.

  - ex: Raw keyring examples generate wrapping keys as part of the example.
    These examples MUST contain guidance that
    in production those keys SHOULD be managed by an HSM.
