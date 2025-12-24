[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS MRK Match for Decrypt

## Version

0.2.2

### Changelog

- 0.2.2

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

Since the AWS Encryption SDK stores the AWS KMS key ARN
in the encrypted data key of the message format
an AWS KMS component needs to be able to evaluate
if a configured ARN matches a stored ARN.
This is especially important for multi-Region keys because the match does not need to be exact.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Implementation

The caller MUST provide:

- 2 AWS KMS key identifiers

If both identifiers are identical, this function MUST return `true`.
Otherwise if either input is not [identified as a multi-Region key](aws-kms-key-arn.md#identifying-an-aws-kms-multi-region-key),
then this function MUST return `false`.
Otherwise if both inputs are [identified as a multi-Region keys](aws-kms-key-arn.md#identifying-an-aws-kms-multi-region-key),
this function MUST return the result of comparing
the `partition`, `service`, `accountId`, `resourceType`, and `resource` parts of both ARN inputs.
NOTE: The `region` part is intentionally omitted.
