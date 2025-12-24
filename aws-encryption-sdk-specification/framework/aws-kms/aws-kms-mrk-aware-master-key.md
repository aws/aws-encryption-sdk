[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS KMS MRK Aware Master Key

## Version

0.2.2

### Changelog

- 0.2.2

  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

A Master Key which interacts with AWS Key Management Service (AWS KMS)
to create, encrypt, and decrypt data keys
using AWS KMS defined Customer Master Keys (CMKs).

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Interface

MUST implement the [Master Key Interface](../master-key-interface.md#interface)

## Initialization

On initialization, the caller MUST provide:

- An AWS KMS key identifier
- An AWS KMS SDK client.

The AWS KMS key identifier MUST NOT be null or empty.
The AWS KMS key identifier MUST be [a valid identifier](aws-kms-key-arn.md#a-valid-aws-kms-identifier).
The AWS KMS SDK client MUST not be null.
The master key MUST be able to be configured with an optional list of Grant Tokens. This configuration SHOULD be on initialization and SHOULD be immutable.

## Get Master Key

MUST be unchanged from the Master Key interface.

## Get Master Keys For Encryption

MUST be unchanged from the Master Key interface.

## Decrypt Data Key

The inputs MUST be the same as the [Master Key Decrypt Data Key](../master-key-interface.md#decrypt-data-key) interface.

The set of encrypted data keys MUST first be filtered to match this master key’s configuration.
To match the encrypted data key’s provider ID MUST exactly match the value “aws-kms”
and the the function [AWS KMS MRK Match for Decrypt](aws-kms-mrk-match-for-decrypt.md#implementation) called with the configured AWS KMS key identifier
and the encrypted data key’s provider info MUST return `true`.
Additionally each provider info MUST be a [valid AWS KMS ARN](aws-kms-key-arn.md#a-valid-aws-kms-arn) with a resource type of `key`.

For each encrypted data key in the filtered set, one at a time,
the master key MUST attempt to decrypt the data key. If this attempt results in an error,
then these errors MUST be collected.

To decrypt the encrypted data key this master key MUST use the configured AWS KMS client
to make an [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) request constructed as follows:

- `KeyId`: The configured AWS KMS key identifier.
- `CiphertextBlob`: The `ciphertext` from the encrypted data key.
- `EncryptionContext`: The encryption context included in the input.
- `GrantTokens`: The configured grant tokens.

If the call succeeds then the response’s `KeyId` MUST be equal to the configured AWS KMS key identifier
otherwise the function MUST collect an error.
The response’s `Plaintext`’s length MUST equal the length required by the requested algorithm suite
otherwise the function MUST collect an error.

If the AWS KMS response satisfies the requirements then it MUST be use and this function MUST return
and not attempt to decrypt any more encrypted data keys.

If all the input encrypted data keys have been processed
then this function MUST yield an error that includes all the collected errors.

The output MUST be the same as the [Master Key Decrypt Data Key](../master-key-interface.md#decrypt-data-key) interface.

## Generate Data Key

The inputs MUST be the same as the [Master Key Generate Data Key](../master-key-interface.md#generate-data-key) interface.
This master key MUST use the configured AWS KMS client
to make an [AWS KMS GenerateDatakey](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey.html) request constructed as follows:

- `KeyId`: The configured AWS KMS key identifier.
- `NumberOfBytes`: The key derivation input length specified by the algorithm suite included in the input.
- `EncryptionContext`: The encryption context included in the input.
- `GrantTokens`: The configured grant tokens.

If the call succeeds the AWS KMS Generate Data Key response’s `Plaintext` MUST match the key derivation input length specified by the algorithm suite included in the input. The response’s `KeyId` MUST be valid.
The response’s `Plaintext` MUST be the plaintext in the output. The response’s cipher text blob MUST be used as the returned as the ciphertext for the encrypted data key in the output.

The output MUST be the same as the [Master Key Generate Data Key](../master-key-interface.md#generate-data-key) interface.

## Encrypt Data Key

The inputs MUST be the same as the [Master Key Encrypt Data Key](../master-key-interface.md#encrypt-data-key) interface.
The master key MUST use the configured AWS KMS client to make an [AWS KMS Encrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html) request constructed as follows:

- `KeyId`: The configured AWS KMS key identifier.
- `PlaintextDataKey`: The plaintext data key obtained from the input.
- `EncryptionContext`: the encryption context included in the input.
- `GrantTokens`: The configured grant tokens.

The AWS KMS Encrypt response MUST contain a valid `KeyId`.
The response’s cipher text blob MUST be used as the `ciphertext` for the encrypted data key.

The output MUST be the same as the [Master Key Encrypt Data Key](../master-key-interface.md#encrypt-data-key) interface.
