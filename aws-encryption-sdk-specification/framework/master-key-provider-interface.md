[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Master Key Provider Interface

## Version

0.1.0-preview

## Implementations

- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/key_providers/base.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/MasterKeyProvider.java)

## Overview

Master key providers are responsible for managing [master keys](./master-key-interface.md),
determining which master keys should be used for encryption,
and for decrypting data keys using available master keys.

### Legacy

This is a legacy specification.
Master key providers SHOULD NOT be included in any additional implementations.
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

### Terms

- Provider ID: A value that identifies a master key provider.
  This concept is equivalent to "key namespace" for Keyrings.
- Key ID: A value that identifies a master key
  within the context of a master key provider.
  This concept is equivalent to "key name" for Keyrings.
- Provider Info: The value that is written to a serialized encrypted data key
  that identifies a master key within the context of a master key provider.
  This MUST always be equal to the master key's key ID
  with the exception of the raw AES master key.
  For a detailed description of this exception,
  see the [Raw AES Keyring specification](./raw-aes-keyring.md).

## Interface

### Get Master Key

This interface provides a way for a master key provider to return master keys.

An implementation MUST support master key selection by key ID.

An implementation MAY support master key selection by provider info or provider ID.

The output of this interface MUST be a master key.

If the master key provider cannot locate an appropriate master key,
the call MUST result in an error.

### Get Master Keys For Encryption

This interface provides a way for a master key provider to indicate which master keys
SHOULD be used for encryption.

Inputs to this interface MUST include the encryption context.

Inputs MAY include the plaintext body and plaintext size.

The output of this interface MUST include a list of all master keys that
SHOULD be used for encryption.

The output of this interface MUST indicate which one of those master keys
MUST be used to generate the data key.

### Decrypt Data Key

This interface is used to decrypt a data key.

The master key provider SHOULD attempt to decrypt the data key
by passing the request to any master keys that it has access to
until it has either exhausted available master keys
or obtained a plaintext data key.

Inputs to this interface MUST include
the algorithm suite,
the encryption context,
and a list of encrypted data keys.

The output of this interface MUST include
the decrypted data key
and information that can identify which master key
decrypted the data key.

If the master key provider cannot decrypt the data key,
the call MUST result in an error.
