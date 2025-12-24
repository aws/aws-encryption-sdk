[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS Key ARN

## Version

0.2.2

### Changelog

- 0.2.2

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

AWS KMS Key ARNs generally follow [AWS ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html))
but there are a few subtle differences.
This is NOT the authoritative source for these rules,
it is just a specification for how the ESDK processes AWS KMS CMK ARNs.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## A valid AWS KMS ARN

A string with 5 `:` that delimit following 6 parts:

1. MUST start with string `arn`
2. The partition MUST be a non-empty
3. The service MUST be the string `kms`
4. The region MUST be a non-empty string
5. The account MUST be a non-empty string
6. The resource section MUST be non-empty and MUST be split by a single `/`
   any additional `/` are included in the resource id
   1. The resource type MUST be either `alias` or `key`
   2. The resource id MUST be a non-empty string

## A valid AWS KMS identifier

An AWS KMS identifer can be any of the following

- A valid AWS KMS ARN
- AWS KMS alias, the resource section of an AWS KMS alias ARN
- AWS KMS key id, the resource id of an AWS KMS key ARN

## AWS KMS multi-Region keys

AWS KMS multi-Region keys can be distinguished from a single-Region key because the key id begins with `mrk-`.
AWS KMS MRK aware components can take as input any AWS KMS identifier:

- AWS KMS key ARN (`arn:aws:kms:us-east-1:2222222222222:key/1234abcd-12ab-34cd-56ef-1234567890ab`)
- AWS KMS multi-Region key ARN (`arn:aws:kms:us-east-1:2222222222222:key/mrk-4321abcd12ab34cd56ef1234567890ab`)
- AWS KMS alias ARN (`arn:aws:kms:us-west-2:111122223333:alias/test-key`)
- AWS KMS key id (`1234abcd-12ab-34cd-56ef-1234567890ab`)
- AWS KMS multi-Region key id (`mrk-4321abcd12ab34cd56ef1234567890ab`)
- AWS KMS alias (`alias/test-key`)

Since the alias is can be any string a customer can create an alias that started with `mrk-`.
But an alias is not a multi-Region key.

## Identifying an an AWS KMS multi-Region ARN

This function MUST take a single AWS KMS ARN

If the input is an invalid AWS KMS ARN this function MUST error.

If resource type is “alias”,
this is an AWS KMS alias ARN and MUST return false.
If resource type is “key” and resource ID starts with “mrk-“,
this is a AWS KMS multi-Region key ARN and MUST return true.
If resource type is “key” and resource ID does not start with “mrk-“,
this is a (single-region) AWS KMS key ARN and MUST return false.

## Identifying an an AWS KMS multi-Region identifier

This function MUST take a single AWS KMS identifier

If the input starts with "arn:",
this MUST return the output of [identifying an an AWS KMS multi-Region ARN](aws-kms-key-arn.md#identifying-an-an-aws-kms-multi-region-arn)
called with this input.
If the input starts with “alias/“,
this an AWS KMS alias and not a multi-Region key id and MUST return false.
If the input starts with “mrk-“,
this is a multi-Region key id and MUST return true.
If the input does not start with any of the above,
this is not a multi-Region key id and MUST return false.
