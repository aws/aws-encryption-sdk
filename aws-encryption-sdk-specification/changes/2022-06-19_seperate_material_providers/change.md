[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Material Providers Library (MPL)

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                       |
| ----------------------------- |
| [Framework](../../framework/) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                                 |
| ----------------------------------------------------------------------------- |
| [Structures](../../framework/structures.md)                                   |
| [Keyring Interface](../../framework/keyring-interface.md)                     |
| [AWS KMS Keyring](../../framework/kms-keyring.md)                             |
| [Raw AES Keyring](../../framework/raw-aes-keyring.md)                         |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md)                         |
| [Cryptographic Materials Manager Interface](../../framework/cmm-interface.md) |
| [Default Cryptographic Materials Manager](../../framework/default-cmm.md)     |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Version Introduced | Version Removed | Repository                                                                            |
| ---------- | ------------------ | --------------- | ------------------------------------------------------------------------------------- |
| C          | 0.1.0              | n/a             | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | 0.1.0              | n/a             | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |
| Dafny      | 0.1.0              | n/a             | [aws-encryption-sdk-dafny](https://github.com/aws/aws-encryption-sdk-dafny)           |
| C#         | 0.1.0              | n/a             | [aws-encryption-sdk-dafny](https://github.com/aws/aws-encryption-sdk-dafny)           |

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

We will rename the `framework` directory to `material_providers`,
add a Readme.md,
and update the algorithm-suites
to be able to identify algorithm suites implemented by different libraries.

## Out of Scope

- Algorithm suites for additional libraries.
- Integrating this specification and the published library
  with ESDK-JS and ESDK-C.
- Integrating the MPL with ESDK-Java and ESDK-Python

## Motivation

We developed Cryptographic Materials Managers and Keyrings
as a way for customers to easily configure their key distribution.
Being able to reuse this logic in our other libraries
will simplify our customers experience and use of our libraries.
It will also simplify our development process
and alow us to focus more on new formats and libraries.

## Drawbacks

We will lose the freedom to add key distribution features
that target specific libraries or storage platforms.
However, [upon review](background.md) we concluded that
we have not found any such features to date
across the S3 Encryption Client (S3EC), DynamoDB Encryption Client (DDBEC),
and the Encryption SDK (ESDK).

## Security Implications

The main security implication of this change is the
responsibility for the implementation details
of a given Algorithm Suite move from being in the same library
to being in a calling library.
This is mitigated by having a tight coupling between
the Material Providers Library and the callers (S3EC, DDBEC, ESDK).

## Operational Implications

This will be a breaking change for the ESDK-Net.
As we publish MPL integrating this with ESDK-JS and ESDK-C
would also be breaking changes.
But this affords us the opportunity
to offer Keyring in both ESDK-Java and ESDK-Python.

This change will affect custom keyrings/CMMs.
This will have the largest impact on ESDK-JS and ESDK-C.
The ESDK-Net has not existed long enough
for significate custom keyring development.

## Guide-level Explanation

CMMs and Keyings provide customers
with a way to make security decisions for their plaintext,
and configure their key distribution.
After [reviewing the generality of this interface](background.md),
we have come to the conclusion that all our libraries should
use the same general interface to configure key distribution.

From the specifications perspective we are mostly moving files.
At a later date we may move the entire MPL directory
into a separate github repo.

However changes to the algorithm-suites will include
adding an additional element of `library`
to each suite to identify what `library` implements it.

## Reference-level Explanation

### Code Change

Code changes required to make this change include:

- Create a new set of `enums` for the ESDKs suites.
- Change the current `AlgorithmSuiteId` into a union.
- Update all references to `AlgorithmSuiteId` to disambiguate
  ESDK enums from other library enums.

### Examples

All existing examples MUST be updated.
