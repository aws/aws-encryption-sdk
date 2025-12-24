[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Default Cryptographic Materials Manager

## Version

0.3.0

### Changelog

- 0.3.0

  - Clarify handling of the `aws-crypto-public-key` encryption context key.

- 0.2.1

  - [Record how the default CMM uses master key providers](https://github.com/awslabs/aws-encryption-sdk-specification/issues/98)

- 0.2.0

  - [Remove Keyring Trace](../changes/2020-05-13_remove-keyring-trace/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                                                        |
| ---------- | -------------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0-preview                          | 0.1.0                     | [default_cmm.h](https://github.com/aws/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/default_cmm.h)                                                                                          |
| NodeJS     | 0.1.0-preview                          | 0.1.0                     | [node_cryptographic_materials_manager.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management-node/src/node_cryptographic_materials_manager.ts)          |
| Browser JS | 0.1.0-preview                          | 0.1.0                     | [browser_cryptographic_materials_manager.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management-browser/src/browser_cryptographic_materials_manager.ts) |
| Python     | 0.1.0-preview                          | 1.3.0                     | [materials_managers/default.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/materials_managers/default.py)                                                    |
| Java       | 0.1.0-preview                          | 1.3.0                     | [DefaultCryptoMaterialsManager.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/DefaultCryptoMaterialsManager.java)                         |

## Overview

The Default Cryptographic Materials Manager (CMM)
is a built-in implementation of the
[CMM interface](cmm-interface.md) provided by the AWS Encryption SDK.

It is used by default to wrap the key provider.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

On default CMM initialization,
the caller MUST provide the following value:

- [Keyring](#keyring)

### Keyring

The [keyring](keyring-interface.md) this CMM uses to
[get encryption materials](#get-encryption-materials) or [decrypt materials](#decrypt-materials).

## Behaviors

### Get Encryption Materials

This operation will add a key-value pair
to the encryption context included in the [encryption materials request](cmm-interface.md#encryption-materials-request)
using the key `aws-crypto-public-key`.
Adding the key `aws-crypto-public-key` SHOULD be done to a copy of the encryption context
so that the caller's encryption context is not mutated.
If the encryption context included in the [encryption materials request](cmm-interface.md#encryption-materials-request)
already contains the `aws-crypto-public-key` key,
this operation MUST fail rather than overwrite the associated value.

- If the [encryption materials request](cmm-interface.md#encryption-materials-request) does not contain an algorithm suite,
  the operation MUST add the default algorithm suite for the [commitment policy](./commitment-policy.md#supported-commitment-policy-enum)
  as the algorithm suite in the encryption materials returned.
- If the [encryption materials request](cmm-interface.md#encryption-materials-request) does contain an algorithm suite,
  the request MUST fail if the algorithm suite is not supported by the [commitment policy](./commitment-policy.md#supported-commitment-policy-enum) on the request.
- If the [encryption materials request](cmm-interface.md#encryption-materials-request) does contain an algorithm suite,
  the encryption materials returned MUST contain the same algorithm suite.

If the algorithm suite contains a [signing algorithm](algorithm-suites.md#signature-algorithm),
the default CMM MUST Generate a [signing key](structures.md#signing-key).

If the algorithm suite contains a [signing algorithm](algorithm-suites.md#signature-algorithm),
the default CMM MUST Add the key-value pair of
key `aws-crypto-public-key`,
value `base64-encoded public verification key`
to the [encryption context](structures.md#encryption-context).

On each call to Get Encryption Materials,
the default CMM MUST make a call to its [keyring's](#keyring)
[On Encrypt](keyring-interface.md#onencrypt) operation.

The default CMM MUST obtain the Plaintext Data Key from the
On Encrypt Response and include it in the
[encryption materials](structures.md#encryption-materials) returned.

The default CMM MUST obtain the
[Encrypted Data Keys](structures.md#encrypted-data-keys)
from the On Encrypt Response and include it
in the [encryption materials](structures.md#encryption-materials) returned.

### Decrypt Materials

If the algorithm suite contains a [signing algorithm](algorithm-suites.md#signature-algorithm),
the default CMM MUST extract the verification key
from the encryption context under the reserved `aws-crypto-public-key` key.
If this key is not present in the encryption context, the operation MUST fail
without returning any decryption materials.

If the algorithm suite does not contain a [signing algorithm](algorithm-suites.md#signature-algorithm),
but the encryption context includes the reserved `aws-crypto-public-key` key,
the operation MUST fail without returning any decryption materials.

The request MUST fail if the algorithm suite on the request is not supported by the [commitment policy](./commitment-policy.md#supported-commitment-policy-enum) on the request.

On each call to Decrypt Materials,
the default CMM MUST make a call to its [keyring's](#keyring)
[On Decrypt](keyring-interface.md#ondecrypt) operation.

The default CMM MUST obtain the Plaintext Data Key from
the On Decrypt response and include it in the decrypt
materials returned.

## Legacy Behavior : Master Key Providers

For implementations that support [master key providers](master-key-provider-interface.md),
the default CMM MUST support generating, encrypting, and decrypting data keys
using master key providers.

### Legacy

This is a legacy specification.
Master key providers SHOULD NOT be included in any additional implementations.

### Initialization

In place of requiring the caller to provide a [keyring](keyring-interface.md)
on initialization:

On default CMM initialization,
the caller MUST provide the following value:

- [Master Key Provider](master-key-provider-interface.md)

### Get Encryption Materials (master key provider)

In place of calling its keyring's [On Encrypt](keyring-interface.md#onencrypt) operation:

The default CMM MUST call its master key provider's
[Get Master Keys for Encryption](master-key-provider-interface.md#get-master-keys-for-encryption) operation
to obtain a list of master keys to use.

If the master key provider does not identify which master key MUST generate the data key,
the default CMM MUST use the first master key in the list for that purpose.
The default CMM MUST generate the data key using this master key's
[Generate Data Key](master-key-interface.md#generate-data-key) operation.

For each remaining master key,
the default CMM MUST call the master key's
[Encrypt Data Key](master-key-interface.md#encrypt-data-key) operation
with the plaintext data key.

### Decrypt Materials (master key provider)

In place of calling its keyring's [On Decrypt](keyring-interface.md#ondecrypt) operation:

The default CMM MUST call its master key provider's
[Decrypt Data Key](master-key-provider-interface.md#decrypt-data-key) operation.
