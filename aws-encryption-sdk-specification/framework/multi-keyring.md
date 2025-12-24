[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Multi-Keyring

## Version

0.1.1

## Changelog

- 0.1.1

  - Clarify [keyring failure on decrypt](../changes/2020-06-04_how-to-fail-with-keyrings/change.md)

- 0.1.0-preview

  - Initial record

## Implementations

- [C](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/multi_keyring.c)
- [Javascript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/multi_keyring.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/keyrings/multi.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/keyrings/MultiKeyring.java)

## Overview

A keyring which combines other keyrings, allowing one [OnEncrypt](#onencrypt) or [OnDecrypt](#ondecrypt)
call to modify the [encryption](structures.md#encryption-materials) or [decryption](structures.md#decryption-materials)
materials using more than one keyring.

A multi-keyring is capable of producing encrypted data keys that can be decrypted by multiple keyrings.
A multi-keyring can decrypt an encrypted data key as long as the multi-keyring contains at least one keyring capable
of decrypting that encrypted data key.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Inputs

On keyring initialization, a keyring MUST define at least one of the following:

- [Generator Keyring](#generator-keyring)
- [Child Keyrings](#child-keyrings)

### Generator Keyring

A [keyring](keyring-interface.md) that can [generate data keys](keyring-interface.md#generate-data-key).

This keyring MUST implement the [Generate Data Key](keyring-interface.md#generate-data-key) behavior
during [OnEncrypt](keyring-interface.md#onencrypt).
This means that this keyring MUST return [encryption materials](structures.md#encryption-materials) containing
a plaintext data key on [OnEncrypt](keyring-interface.md#onencrypt).

If the list of [child keyrings](#child-keyrings) is empty,
a [generator keyring](#generator-keyring) MUST be defined for the keyring.

### Child Keyrings

A list of [keyrings](keyring-interface.md) to be used to modify the [encryption](structures.md#encryption-materials)
or [decryption materials](structures.md#decryption-materials).

If this keyring does not have a [generator keyring](#generator-keyring), this list MUST NOT be empty.

## Operation

### OnEncrypt

If this keyring has a generator keyring,
this keyring MUST first generate a plaintext data key using the generator keyring:

- If the input encryption materials already include a plaintext data key,
  OnEncrypt MUST fail.
- This keyring MUST first call the generator keyring's OnEncrypt
  using the input encryption materials as input.
- If the generator keyring fails OnEncrypt,
  this OnEncrypt MUST also fail.
- If the generator keyring returns encryption materials missing a plaintext data key,
  OnEncrypt MUST fail.

If this keyring does not have a [generator keyring](#generator-keyring),
and the input [encryption materials](structures.md#encryption-materials)
does not include a plaintext data key, OnEncrypt MUST fail.

Next, for each [keyring](keyring-interface.md) in this keyring's list of [child keyrings](#child-keyrings),
the keyring MUST call [OnEncrypt](keyring-interface.md#onencrypt).
The [encryption materials](structures.md#encryption-materials) input into OnEncrypt are the
input encryption materials if this is the first OnEncrypt call.
If this is not the first OnEncrypt call, the encryption materials input into OnEncrypt are the encryption materials
output by the previous OnEncrypt call.
If the child keyring's [OnEncrypt](keyring-interface.md#onencrypt) fails, this OnEncrypt MUST also fail.

If all previous [OnEncrypt](keyring-interface.md#onencrypt) calls succeeded, this keyring MUST return
the [encryption materials](structures.md#encryption-materials) returned by the last OnEncrypt call.

### OnDecrypt

If the decryption materials already contain a plaintext data key,
the keyring MUST fail
and MUST NOT modify the [decryption materials](structures.md#decryption-materials).

Otherwise, OnDecrypt MUST first attempt to decrypt the [encrypted data keys](structures.md#encrypted-data-keys-1)
in the input [decryption materials](structures.md#decryption-materials) using its
[generator keyring](#generator-keyring). If the generator keyring is unable to decrypt
the materials, the multi-keyring MUST attempt to decrypt using its child keyrings,
until one either succeeds in decryption or all have failed.

For each [keyring](keyring-interface.md) to be used for decryption,
the multi-keyring MUST call that keyring's [OnDecrypt](keyring-interface.md#ondecrypt) using
the unmodified [decryption materials](structures.md#decryption-materials) and the input
[encrypted data key](structures.md#encrypted-data-key) list.
If [OnDecrypt](keyring-interface.md#ondecrypt) returns [decryption materials](structures.md#decryption-materials)
containing a plaintext data key, the multi-keyring MUST immediately return the modified decryption materials.
If the child keyring's OnDecrypt call fails, the multi-keyring MUST collect the error and continue
to the next keyring, if any.

If, after calling [OnDecrypt](keyring-interface.md#ondecrypt) on every [child keyring](#child-keyrings)
(and possibly the [generator keyring](#generator-keyring)), the [decryption materials](structures.md#decryption-materials)
still do not contain a plaintext data key, OnDecrypt MUST return a failure message containing the
collected failure messages from the child keyrings.

## Security Considerations

Users SHOULD examine the [keyrings](keyring-interface.md) they include in a multi-keyring to ensure
that they understand what set of keyrings will be capable of obtaining the plaintext data key from
the returned set of encrypted data keys.

In more detail:

Multi-keyrings will produce a set of [encrypted data keys](structures.md#encrypted-data-key) on
[OnEncrypt](keyring-interface.md#onencrypt) that includes the encrypted data keys of every sub-keyring
(a keyring which is either the [generator keyring](#generator-keyring) or a member of [child keyrings](#child-keyrings))
that is capable of producing encrypted data keys.

As such, any [keyring](keyring-interface.md) that is capable of obtaining the plaintext data key from
[encrypted data keys](structures.md#encrypted-data-key) produced by one of the sub-keyrings,
by definition, is capable of obtaining the plaintext data key for the set of encrypted data keys
the multi-keyring produces on [OnEncrypt](keyring-interface.md#onencrypt).

In typical cases, most [keyrings](keyring-interface.md) are defined such that they are capable of
decrypting the encrypted data keys they produce.
As such, when including such [keyrings](keyring-interface.md),
the multi-keyring will produce a set of [encrypted data keys](structures.md#encrypted-data-keys)
such that any one of the sub-keyrings is capable of obtaining the plaintext data key.
