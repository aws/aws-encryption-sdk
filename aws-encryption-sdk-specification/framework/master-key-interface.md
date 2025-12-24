[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Master Key Interface

## Version

0.1.0-preview

## Implementations

- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/key_providers/base.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/MasterKey.java)

## Overview

Master keys are master key providers that only provide themselves.
They also support generating and encrypting data keys.

### Legacy

This is a legacy specification.
Master keys SHOULD NOT be included in any additional implementations.
Any new implementations MUST include [Keyrings](./keyring-interface.md) instead.

### Consistency

This specification defines the common behavior between the two implementations
that determine the REQUIRED functionality.

Other specifics of behavior and API vary between the two implementations.

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted
as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Interface

### Get Master Key

Inputs and outputs are the same as for [master key providers](./master-key-provider-interface.md).

A master key MUST supply itself and MUST NOT supply any other master keys.

### Get Master Keys For Encryption

Inputs and outputs are the same as for [master key providers](./master-key-provider-interface.md).

A master key MUST supply itself and MUST NOT supply any other master keys.

### Decrypt Data Key

Inputs and outputs are the same as for [master key providers](./master-key-provider-interface.md).

A master key SHOULD attempt to decrypt a data key using itself.

A master key MUST not attempt to use any other master keys.

### Generate Data Key

This interface is used to generate and encrypt a data key.

The master key MUST generate a data key and MUST then encrypt that data key.

Inputs to this interface MUST include
the algorithm suite
and the encryption context.

The output of this interface MUST include
the plaintext data key,
the data key encrypted under the master key,
and information that can identify which master key
was used to generate and encrypt the data key.

If the master key cannot generate or encrypt the data key,
the call MUST result in an error.

### Encrypt Data Key

This interface is used to encrypt a data key.

The master key MUST encrypt the data key.

Inputs to this interface MUST include
the algorithm suite,
the encryption context,
the encrypted data key,
and information that can identify which master key
was used to encrypt the data key.

The output of this interface MUST include
a value that this master key can use to obtain
the plaintext data key.
Most commonly,
this will be the result of an encryption operation.

If the master key cannot encrypt the data key,
the call MUST result in an error.
