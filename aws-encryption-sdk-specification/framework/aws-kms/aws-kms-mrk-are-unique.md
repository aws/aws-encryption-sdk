[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Assert AWS KMS MRK are unique

## Version

0.2.2

### Changelog

- 0.2.2

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

Duplicate multi-region key ids create ambiguity about which Region to contact.
This is better resolved outside of the AWS Encryption SDK.
While preferring the "local" Region is a seductive solution,
this can not be 100% correct.
First, because not all code runs in an AWS Region,
and second because customers may want create privacy controls around their data.
In this case nearby region may exists and store ciphertext,
but may be restricted from having the AWS KMS keys.
In any complicated deployment determining the closest,
or preferred region is itself a complicated task.
It will be easier for customers to reason about this criteria
and ensure their code is correct
if the AWS Encryption SDK offers one and only one configuration for a behavior.
e.g. If the preferred region logic results in an ambiguous configuration
is informed and they can resolve the ambiguity.
Rather than attempting to resolve it ourselves.

Preferring the local region does not suffice:
not all code runs in an AWS Region boundary,
and customers need to assert intentions to use a region.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Implementation

The caller MUST provide:

- A list of AWS KMS key identifiers

If the list does not contain any [multi-Region keys](aws-kms-key-arn.md#identifying-an-aws-kms-multi-region-key)
this function MUST exit successfully.

If there are zero duplicate resource ids between the multi-region keys,
this function MUST exit successfully

If any duplicate multi-region resource ids exist,
this function MUST yield an error
that includes all identifiers with duplicate resource ids
not only the first duplicate found.
