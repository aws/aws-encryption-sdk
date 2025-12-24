[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# How to fail with Keyrings in the AWS Encryption SDK

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                                                |
| ------------------------------------------------------------------------------------------------------ |
| [Keyring Failure Communication](https://github.com/awslabs/aws-encryption-sdk-specification/issues/40) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                             |
| --------------------------------------------------------- |
| [Keyring Interface](../../framework/keyring-interface.md) |
| [Raw AES Keyring](../../framework/raw-aes-keyring.md)     |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md)     |
| [Multi-Keyring](../../framework/multi-keyring.md)         |

## Affected Implementations

The scope of this change only affects the specification.
Follow-up changes that define how this change
affects specific keyring specifications
will affect implementations.

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

In order to define when keyrings MUST fail,
we define a new concept of the "decryption contract",
how it applies to keyrings generally,
and how it is used to determine when keyrings MUST fail.

## Out of Scope

- The specifics of how ESDK components
  communicate failure information between themselves
  is out of scope.
  We will address this in future specification changes.

- The specific nature of _how_ ESDK framework components indicate failure
  is out of scope.
  That will depend on the best practices for each language.

- Defining the decryption contract for each keyring
  is out of scope.
  We will address this in future specification changes.

- A mechanism to interrogate a keyring
  to determine the properties of its decryption contract
  is out of scope.
  We SHOULD address this in future changes.

- This change only applies to keyrings.
  We SHOULD define analogous concepts for
  cryptographic materials managers
  and the AWS Encryption SDK client
  in future changes.

## Motivation

We lack a consistent definition both of
what it means for a keyring to fail
and what causes a keyring to fail.
Without this consistent definition,
different keyrings and implementations
do slightly different things,
leading to implementation drift
and confusion for users and implementors.

In order to clarify this situation,
we MUST first define what it means for a keyring to fail,
and that means defining what it means for a keyring to succeed.
To do this,
we define a new concept of the
"decryption contract".

## Drawbacks

This change SHOULD not introduce any drawbacks.
It is simplifying and clarifying
a relationship that already exists
in the specification
and clearly defining
how that relationship affects behavior.

## Security Implications

The decryption contract
simplifies the definition of
the security properties of a keyring
and provides a consistent way to describe
the security properties of each keyring.

## Operational Implications

Clearly defining what causes keyrings to fail
makes it simpler for users to predict failing behavior
in different environments and implementations.

## Guide-level Explanation

On encryption,
a keyring creates one or more "decryption artifacts",
sometimes called "encrypted data keys",
that are included in the resulting encrypted message.
On decryption,
a keyring consumes decryption artifacts
and fulfills a set of requirements
by using these decryption artifacts
to obtain the plaintext data key.
This set of requirements is what we call the "decryption contract."

## Reference-level Explanation

### Definitions and Terms

- **decryption artifact** :
  A value that is consumed on decryption
  to obtain decryption materials.

- **decryption contract** :
  A set of requirements that MUST be met
  to obtain decryption materials
  using one or more decryption artifacts.

- **creating the decryption artifact** :
  The process of creating one or more decryption artifacts
  intended for a component with a specific decryption contract.

- **fulfilling the decryption contract** :
  The process of taking one or more decryption artifacts
  and using it to meet the requirements of
  a decryption contract
  in order to obtain the plaintext data key.

### How to Succeed

On encryption,
a keyring creates one or more decryption artifacts.

On decrypt,
a keyring fulfills its decryption contract
using one or more decryption artifacts.

### How to Fail

On encryption,
if a keyring is unable or unwilling to
create all of its required decryption artifacts,
that keyring MUST fail.
If a keyring is unable or unwilling to
create any decryption artifacts,
that keyring MUST fail.

On decryption,
if a keyring is unable or unwilling to
fulfill its decryption contract
using the decryption artifacts that it receives,
that keyring MUST fail.

### Application to Keyring Specifications

Each keyring specification MUST define
what decryption artifacts it MUST create on encryption
and what keyrings it intends to receive
those decryption artifacts.
This MAY be itself.

Each keyring specification MUST also define
that keyring's decryption contract.

The specifications for each keyring
will be defined in separate changes,
but as a descriptive example,
they might look something like this:

- Raw RSA keyring

  - MUST create one decryption artifact,
    intended for a raw RSA keyring.
  - decryption contract:

    - The decrypting keyring MUST have access to decrypt
      using the private wrapping key.

- Raw AES keyring

  - MUST create one decryption artifact,
    intended for a raw AES keyring.
  - decryption contract:

    - Create one decryption artifact,
      intended for a raw RSA keyring.
    - The decrypting keyring MUST have access to decrypt
      using the symmetric wrapping key.
    - The encryption context MUST exactly match
      the encryption context used on encrypt.

      - NOTE: In compliant implementations this is a MUST,
        but mathematically this is a SHOULD
        because it is possible to create a keyring
        that ignores this clause in the decryption contract
        by using AES-CTR rather than AES-GCM.

- Multi-keyring

  - MUST create all decryption artifacts for all child keyrings.
  - decryption contract:

    - Fulfilling any member keyring's decryption contract
      MUST be sufficient to fulfill
      the multi-keyring's decryption contract.

- AWS KMS keyring (single CMK)

  - MUST create one decryption artifact,
    intended for an AWS KMS keyring.
  - decryption contract:

    - The decrypting keyring MUST have access to
      an AWS KMS client that is configured with credentials for
      an AWS principal that passes a `kms:Decrypt` authorization check
      for the encrypted data key.
    - The encryption context MUST exactly match
      the encryption context used on encrypt.
    - NOTE: These properties are supplied by AWS KMS.
      The AWS KMS keyring is simply a conduit to AWS KMS.

- AWS KMS discovery keyring

  - MUST NOT create any decryption contract.
  - decryption contract:

    - The AWS KMS discovery keyring is unable to write a decryption contract.
    - The AWS KMS discovery keyring MAY fulfill any AWS KMS decryption contract
      within its configured AWS region.

      - NOTE: Successfully fulfilling an AWS KMS decryption contract
        depends on access to credentials for an AWS principal
        with the necessary permission
        to pass a `kms:Decrypt` authorization check
        for the encrypted data key.

    - Fulfilling any AWS KMS decryption contract
      MUST be sufficient to fulfill
      the AWS KMS discovery keyring's decryption contract.
