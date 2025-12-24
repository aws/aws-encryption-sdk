[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS Discovery Keyring

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

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

A keyring which interacts with AWS Key Management Service (AWS KMS)
to decrypt data keys using a filter to identify Customer Master Keys (CMKs).

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Interface

MUST implement that [AWS Encryption SDK Keyring interface](../keyring-interface.md#interface)

## Initialization

On initialization the caller MUST provide:

- An AWS KMS client
- An optional discovery filter that is an AWS partition and a set of AWS accounts
- An optional list of AWS KMS grant tokens

The AWS KMS SDK client MUST NOT be null.

## OnEncrypt

This function MUST fail.

## OnDecrypt

OnDecrypt MUST take [decryption materials](../structures.md#decryption-materials) and
a list of [encrypted data keys](../structures.md#encrypted-data-key) as input.

If the [decryption materials](../structures.md#decryption-materials) already contained a valid plaintext data key,
they keyring MUST fail and MUST NOT modify the [decryption materials](../structures.md#decryption-materials).

The set of encrypted data keys MUST first be filtered to match this keyring’s configuration.
For the encrypted data key to match

- Its provider ID MUST exactly match the value “aws-kms”.
- The provider info MUST be a [valid AWS KMS ARN](aws-kms-key-arn.md#a-valid-aws-kms-arn) with a resource type of `key` or OnDecrypt MUST fail.
- If a discovery filter is configured, its partition and the provider info partition MUST match.
- If a discovery filter is configured, its set of accounts MUST contain the provider info account.

For each encrypted data key in the filtered set, one at a time, the OnDecrypt MUST attempt to decrypt the data key. If this attempt results in an error, then these errors are collected.

To attempt to decrypt a particular [encrypted data key](../structures.md#encrypted-data-key),
OnDecrypt MUST call [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) with the configured AWS KMS client.

When calling [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html), the keyring MUST call with a request constructed as follows:

- `KeyId`: The AWS KMS ARN from the provider info
- `CiphertextBlob`: The [encrypted data key ciphertext](../structures.md#ciphertext).
- `EncryptionContext`: The [encryption context](../structures.md#encryption-context) included in the input [decryption materials](../structures.md#decryption-materials).
- `GrantTokens`: this keyring's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token)

If the call to [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) succeeds OnDecrypt verifies

- The `KeyId` field in the response MUST equal the AWS KMS ARN from the provider info
- The length of the response’s `Plaintext` MUST equal the [key derivation input length](../algorithm-suites.md#key-derivation-input-length)
  specified by the [algorithm suite](../algorithm-suites.md) included in the input [decryption materials](../structures.md#decryption-materials).

If the response does not satisfy these requirements
then an error is collected and the next encrypted data key in the filtered set MUST be attempted.

If the response does satisfy these requirements then OnDecrypt MUST do the following with the response:

- set the plaintext data key on the [decryption materials](../structures.md#decryption-materials) as the response `Plaintext`.
- immediately return the modified [decryption materials](../structures.md#decryption-materials).

If OnDecrypt fails to successfully decrypt any [encrypted data key](../structures.md#encrypted-data-key),
then it MUST yield an error that includes all collected errors.
