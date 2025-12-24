[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Required Encryption Context Cryptographic Materials Manager

## Version

0.1.0

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

Required Encryption Context Cryptographic Materials Manager (CMM)
is a built-in CMM to add additional controls for Encryption Context.

This CMM can be configured with a set of [encryption context](./structures.md#encryption-context) keys.
This set of keys MUST

- Exist in the encryption context of all [Encryption Materials Requests](./cmm-interface.md#encryption-materials-request).
- Exist in the [required encryption context keys](./structures.md#required-encryption-context-keys) for returned [encryption materials](./structures.md#encryption-materials)
  so that these values will not be serialized into the [supported formats](./algorithm-suites.md#supported-formats).
- Exist the reproduced encryption context of all [Decrypt Materials Requests](#decrypt-materials-request).

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

On Required Encryption Context CMM initialization,
the caller MUST provide the following values:

- [Required Encryption Context Keys](#required-encryption-context-keys)

Additionally, the caller MUST provide one of the following values:

- [Underlying Cryptographic Materials Manager (CMM)](#underlying-cryptographic-materials-manager)
- [Keyring](keyring-interface.md)

If the caller provides a keyring,
then the Required Encryption Context CMM MUST set its underlying CMM
to a [default CMM](default-cmm.md) initialized with the keyring.

### Underlying Cryptographic Materials Manager

The underlying [Cryptographic Materials Manager (CMM)](cmm-interface.md#supported-cmms)
to query for encryption/decryption materials.

### Required Encryption Context Keys

The set of encryption context keys to require
in all [behaviors](#behaviors).

## Behaviors

### Get Encryption Materials

The encryption context on the [encryption materials request](./cmm-interface.md#encryption-materials-request)
MUST contain a value for every key in the configured [required encryption context keys](#required-encryption-context-keys)
or this request MUST fail.

The Required Encryption Context CMM MUST attempt to obtain [encryption materials](./structures.md#encryption-materials)
by making a call to the [underlying CMM's](#underlying-cryptographic-materials-manager)
[Get Encryption Materials](cmm-interface.md#get-encryption-materials).
All configured [required encryption context keys](#required-encryption-context-keys)
MUST exist in the required encryption context keys
of the [encryption materials request](./cmm-interface.md#encryption-materials-request)
to the [underlying CMM's](#underlying-cryptographic-materials-manager).

The obtained [encryption materials](./structures.md#encryption-materials) MUST
have all configured [required encryption context keys](#required-encryption-context-keys)
in it's [required encryption context keys](./structures.md#required-encryption-context-keys).

### Decrypt Materials

The reproduced encryption context on the [decrypt materials request](./cmm-interface.md#decrypt-materials-request)
MUST contain a value for every key in the configured [required encryption context keys](#required-encryption-context-keys)
or this request MUST fail.

The Required Encryption Context
CMM MUST attempt to obtain [decryption materials](./structures.md#decryption-materials)
by making a call to the [underlying CMM's](#underlying-cryptographic-materials-manager)
[decrypt materials](cmm-interface.md#decrypt-materials) interface
with the unchanged [decrypt materials request](./cmm-interface.md#decrypt-materials-request).

Since the CMM can modify the encryption context
it is possible that the [underlying CMM's](#underlying-cryptographic-materials-manager)
is not the DefaultCMM.
In that case the [underlying CMM's](#underlying-cryptographic-materials-manager)
might have removed [required encryption context keys](#required-encryption-context-keys)
from the request forwarded to its keyring or underlying CMM.
This would result in these values not being authenticated
against the serialized message or even the plaintext data key.
This would violate the CMM decryption contract.
The obtained [decryption materials](./structures.md#decryption-materials) MUST
have all configured [required encryption context keys](#required-encryption-context-keys)
in it's [encryption context](./structures.md#encryption-context-2).
