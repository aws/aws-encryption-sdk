[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Raw RSA Keyring V2

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                               |
| ----------------------------------------------------- |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                  |
| -------------------------------------------------------------- |
| [Raw RSA Keyring](../../framework/aws-kms/aws-kms-rsa-keyring) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The ESDK message format includes a digital signature over all of body of the message.
This signature is verified on decrypt with a verification key that is
persisted in the Encryption Context.
With an AWS KMS Keyring
the Encryption Context is included in the
Encrypt and Decrypt invocations as encryption context.
This encryption context is immutable
because it is enforce by AWS KMS
and it is bound to the encrypted data key.

But RSA is not an Authenticated Encryption with Additional Data (AEAD) cipher;
as such, it's invocation cannot include encryption context.
This means that an by default an AWS KMS RSA keyring
would not have these properties.

To solve this,
we propose storing a representation of the Encryption Context in
the Encrypted Data Key's ciphertext of AWS KMS RSA Keyrings.

On Decrypt,
this representation would be compared to the given Encryption Context.
If this comparison fails, the decrypt would fail.

## Out of Scope

This proposal does not address:

- Integrating the Raw RSA Keyring

## Motivation

Digital Signatures are a prominent feature of the ESDK's message format.
It is a miss that Verification Key is not protected in the RSA Keyring.
These changes make the Encryption Context bound to the encrypted data key
and the message more tightly bound together.
