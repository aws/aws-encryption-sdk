[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS MRK Aware Master Key Provider

## Version

0.2.2

### Changelog

- 0.2.2

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

A Master Key Provider which interacts with AWS Key Management Service (AWS KMS)
to create, encrypt, and decrypt data keys
using AWS KMS defined Customer Master Keys (CMKs).

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Interface

MUST implement the [Master Key Provider Interface](../master-key-provider-interface.md#interface)

## Initialization

On initialization the caller MUST provide:

- An indicator of either strict or discovery mode e.g an. `isDiscovery` boolean
- A list of AWS KMS key identifiers, the list may be empty
- An optional list of AWS KMS grant tokens
- A method that can take a region string and return an AWS KMS client e.g. a regional client supplier
- An optional discovery filter that is an AWS partition and a set of AWS accounts
- An optional discovery MRK Region string

The key id list MUST NOT be empty or null in strict mode.
The key id list MUST NOT contain any null or empty string values.
All AWS KMS key identifiers are be passed to [Assert AWS KMS MRK are unique](aws-kms-mrk-are-unique.md#Implementation)
and the function MUST return success.
A discovery filter MUST NOT be configured in strict mode.
A default MRK Region MUST NOT be configured in strict mode.
In discovery mode if a default MRK Region is not configured
the AWS SDK Default Region MUST be used.
If an AWS SDK Default Region can not be obtained initialization MUST fail.

The key id list MUST be empty in discovery mode.
The regional client supplier MUST be defined in discovery mode.

## Get Master Key

The input MUST be the same as the [Master Key Provider Get Master Key](../master-key-provider-interface.md#get-master-key) interface.

The function MUST only provide master keys if the input provider id equals `aws-kms`.
In strict mode, the requested AWS KMS key ARN MUST match a member of the configured key ids
by using [AWS KMS MRK Match for Decrypt](aws-kms-mrk-match-for-decrypt.md#implementation)
otherwise this function MUST error.
In discovery mode, the requested AWS KMS key identifier MUST be a well formed AWS KMS ARN.
In discovery mode if a discovery filter is configured
the requested AWS KMS key ARN’s `partition` MUST match the discovery filter’s `partition`
and the AWS KMS key ARN’s `account` MUST exist in the discovery filter’s account id set.

If the requested AWS KMS key identifier is not a well formed ARN
the AWS Region MUST be the configured default region this SHOULD be obtained from the AWS SDK.
Otherwise if the requested AWS KMS key identifier is [identified as a multi-Region key](aws-kms-key-arn.md#identifying-an-aws-kms-multi-region-key),
then AWS Region MUST be the region from the AWS KMS key ARN stored in the provider info from the encrypted data key.
Otherwise if the mode is discovery then the AWS Region MUST be the discovery MRK region.
Finally if the provider info is [identified as a multi-Region key](aws-kms-key-arn.md#identifying-an-aws-kms-multi-region-key)
the AWS Region MUST be the region from the AWS KMS key in the configured key ids matched to
the requested AWS KMS key by using [AWS KMS MRK Match for Decrypt](aws-kms-mrk-match-for-decrypt.md#implementation).
An AWS KMS client MUST be obtained by calling the regional client supplier with this AWS Region.

In strict mode a [AWS KMS MRK Aware Master Key](aws-kms-mrk-aware-master-key.md) MUST be returned configured with

- The AWS KMS obtained by calling the regional client supplier.
- The AWS KMS key in the configured key ids matched to the requested AWS KMS key by using [AWS KMS MRK Match for Decrypt](aws-kms-mrk-match-for-decrypt.md#implementation).
- The configured grant tokens.

In discovery mode a [AWS KMS MRK Aware Master Key](aws-kms-mrk-aware-master-key.md) MUST be returned configured with

- The AWS KMS obtained by calling the regional client supplier.
- An AWS KMS key ARN where the `region` element is the AWS Region, and every other ARN element matches the requested AWS KMS key ARN.

- The configured grant tokens.

The output MUST be the same as the [Master Key Provider Get Master Key](../master-key-provider-interface.md#get-master-key) interface.

## Get Master Keys For Encryption

The input MUST be the same as the [Master Key Provider Get Master Keys For Encryption](../master-key-provider-interface.md#get-master-keys-for-encryption) interface.

If the configured mode is discovery the function MUST return an empty list.
If the configured mode is strict this function MUST return a list of master keys
obtained by calling [Get Master Key](aws-kms-mrk-aware-master-key-provider.md#get-master-key) for each AWS KMS key identifier in the configured key ids

The output MUST be the same as the [Master Key Provider Get Master Keys For Encryption](../master-key-provider-interface.md#get-master-keys-for-encryption) interface.

## Decrypt Data Key

The input MUST be the same as the [Master Key Provider Decrypt Data Key](../master-key-provider-interface.md#decrypt-data-key) interface.

The set of encrypted data keys MUST first be filtered to match this master key’s configuration.
To match the encrypted data key’s provider ID MUST exactly match the value “aws-kms”.
Additionally each provider info MUST be a [valid AWS KMS ARN](aws-kms-key-arn.md#a-valid-aws-kms-arn) with a resource type of `key`.

For each encrypted data key in the filtered set, one at a time,
the master key provider MUST call [Get Master Key](aws-kms-mrk-aware-master-key-provider.md#get-master-key)
with the encrypted data key’s provider info as the AWS KMS key ARN.
It MUST call [Decrypt Data Key](aws-kms-mrk-aware-master-key.md#decrypt-data-key) on this master key with the input algorithm,
this single encrypted data key, and the input encryption context.
If this attempt results in an error, then these errors MUST be collected.
If the decrypt data key call is successful, then this function MUST return this result
and not attempt to decrypt any more encrypted data keys.

If all the input encrypted data keys have been processed
then this function MUST yield an error that includes all the collected errors.

The output MUST be the same as the [Master Key Provider Decrypt Data Key](../master-key-provider-interface.md#decrypt-data-key) interface.
