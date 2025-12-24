[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS Multi Keyrings

## Version

0.3.0

### Changelog

- 0.3.0

  - Incorporate [KMS Keyring Redesign](https://github.com/awslabs/aws-encryption-sdk-specification/tree/master/proposals/2020-07-01_aws-kms-keyring-redesign)

- 0.2.2

  - Rename Key IDs to Key Names for increased clarity
  - Update Key Names and Generator sections to reinforce support for all AWS KMS key identifiers
  - [Pull request link for discussions](https://github.com/awslabs/aws-encryption-sdk-specification/pull/123)

- 0.2.1

  - [Clarify naming of KMS to AWS KMS](https://github.com/awslabs/aws-encryption-sdk-specification/issues/67)

- 0.2.0

  - [Remove Keyring Trace](../../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

The AWS KMS MRK keyrings only operate on a single AWS KMS key ARN.
However customers want to easily configure multiple AWS KMS key ARNs.
These functions compose multiple AWS KMS key ARNs and return a single Multi Keyring.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

Customers need a way to compose the single AWS KMS keyrings in an easy way. These functions take configuration and produce and compose many keyrings into a Multi Keyring.

## AWS KMS Multi-Keyring

The caller MUST provide:

- An optional AWS KMS key identifiers to use as the generator.
- An optional set of AWS KMS key identifiers to use as child keyrings.
- An optional method that can take a region string and return an AWS KMS client e.g. a regional client supplier
- An optional list of AWS KMS grant tokens

If any of the AWS KMS key identifiers is not a [valid AWS KMS key ARN](aws-kms-key-arn.md#a-valid-aws-kms-arn), this function MUST fail.
If a regional client supplier is not passed, then a default MUST be created that takes a region string and generates a default AWS SDK client for the given region.

If there is a generator input then the generator keyring MUST be a [AWS KMS Keyring](aws-kms-keyring.md) initialized with

- The generator input.
- The AWS KMS client that MUST be created by the regional client supplier
  when called with the region part of the generator ARN
  or a signal for the AWS SDK to select the default region.
- The input list of AWS KMS grant tokens

If there is a set of child identifiers then a set of [AWS KMS Keyring](aws-kms-keyring.md) MUST be created for each AWS KMS key identifier by initializing each keyring with

- AWS KMS key identifier.
- The AWS KMS client that MUST be created by the regional client supplier
  when called with the region part of the AWS KMS key identifier
  or a signal for the AWS SDK to select the default region.
- The input list of AWS KMS grant tokens

NOTE: The AWS Encryption SDK SHOULD NOT attempt to evaluate its own default region.

Then a [Multi-Keyring](../multi-keyring.md#inputs) MUST be initialize by using this generator keyring as the [generator keyring](../multi-keyring.md#generator-keyring) and this set of child keyrings as the [child keyrings](../multi-keyring.md#child-keyrings).
This Multi-Keyring MUST be this function's output.

## AWS KMS Discovery Multi-Keyring

The caller MUST provide:

- A set of Region strings
- An optional discovery filter that is an AWS partition and a set of AWS accounts
- An optional method that can take a region string and return an AWS KMS client e.g. a regional client supplier
- An optional list of AWS KMS grant tokens

If an empty set of Region is provided this function MUST fail.
If any element of the set of regions is null or an empty string this function MUST fail.
If a regional client supplier is not passed, then a default MUST be created that takes a region string and generates a default AWS SDK client for the given region.

A set of AWS KMS clients MUST be created by calling regional client supplier for each region in the input set of regions.

Then a set of [AWS KMS Discovery Keyring](aws-kms-discovery-keyring.md) MUST be created for each AWS KMS client by initializing each keyring with

- The AWS KMS client
- The input discovery filter
- The input AWS KMS grant tokens

Then a [Multi-Keyring](../multi-keyring.md#inputs) MUST be initialize by using this set of discovery keyrings as the [child keyrings](../multi-keyring.md#child-keyrings).
This Multi-Keyring MUST be this functions output.
