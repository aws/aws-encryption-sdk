[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# How to fail with Keyrings in the AWS Encryption SDK

# Definitions

## Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

# Background

We lack a consistent definition both of what it means to fail
and how different parts of the AWS Encryption SDK (ESDK) framework handle
different failures.
Without this consistent definition,
implementations are left to
"whatever the dev thought was a good idea at the time",
creating inconsistencies across implementations.

# Requirements

In order to resolve these inconsistencies,
we need to define what kinds of failure modes we allow,
what causes them,
and how the ESDK framework components handle them.

# Success Measurements

We will know we are succeeding when we have consensus on
the definition of when keyrings fail,
what causes those failures,
and how ESDK framework components handle those failures.

# Out of Scope

- The specifics of how ESDK framework components
  communicate failure information between themselves
  is out of scope for this document.
  That will be addressed separately once we have
  consensus on this document.

- The specific nature of _how_ ESDK framework components indicate failure
  is out of scope for this document.
  That will depend on the best practices for each language.

- We need to define detailed decryption contracts
  (defined in [Issues and Alternatives](#issues-and-alternatives))
  for each keyring defined in the ESDK specification.
  This will be addressed separately once we have
  consensus on this document.

- A mechanism to interrogate a keyring to determine
  the requirements of its decryption contract
  (defined in [Issues and Alternatives](#issues-and-alternatives))
  is out of scope for this document.
  We should pursue this once decryption contracts are defined
  for each keyring.

# Issues and Alternatives

## Issue 0 : What does a keyring do?

In order to define when a keyring fails,
we must first clearly define what a keyring does.
When a keyring encrypts a data key,
it creates one or more artifacts that define
a set of requirements that MUST be met
in order to use that artifact
to obtain the data key.
We call this set of requirements the "decryption contract".
In order to obtain the data key from the artifact,
a keyring MUST "satisfy" the decryption contract.

For example, some decryption contracts for keyrings include:

- Raw RSA keyring

  - The keyring MUST have access to the private wrapping key.

- Raw AES keyring

  - The keyring MUST have access to the symmetric wrapping key.
  - The encryption context MUST exactly match
    the encryption context used on encrypt.

    - NOTE: In compliant implementations this is a MUST,
      but mathematically this is a SHOULD
      because it is possible to create a keyring
      that ignores this clause in the decryption contract
      by using AES-CTR rather than AES-GCM.

- AWS KMS keyring (single CMK)

  - The AWS KMS client MUST be configured with credentials for
    an AWS principal that will pass a `kms:Decrypt` authorization check
    for the encrypted data key.
  - The encryption context MUST exactly match
    the encryption context used on encrypt.

- Multi-keyring:

  - Satisfying any member keyring's decryption contract
    MUST be sufficient to satisfy
    the multi-keyring's decryption contract.

## Issue 1 : When MUST a keyring fail?

- On encrypt,
  if a keyring is unable or unwilling to create an artifact or artifacts
  that can be used to satisfy its decryption contract,
  the keyring MUST fail.

- On decrypt,
  if a keyring is unable or unwilling to satisfy
  the artifact's decryption contract,
  the keyring MUST fail.

## Issue 2 : What happens when a keyring fails?

When a keyring fails,
it MUST communicate
information about the cause of that failure
to the component that called the keyring.

## Issue 3 : What implications does this have to existing keyring specifications?

In addition to introducing the concept of decryption contracts
and defining each keyring's decryption contract,
this change affects the following keyring specifications.

- Keyring interface :
  The keyring interface defines no-op behavior as
  "output[ing] the encryption/decryption materials unmodified".
  This needs to be modified to frame this behavior as a failure
  and state that encryption/decryption materials MUST NOT be modified
  when a keyring fails.

  - _This also affects all keyring specifications._

- Multi-keyring :
  The multi-keyring behavior is defined in a way that assumes that
  different kinds of failure exist.
  This is in line with how we had previously thought about failure,
  but is not in line with this new model.

  - On encrypt, if _any_ child keyring fails,
    the multi-keyring MUST fail.
  - On decrypt, if _all_ child keyrings fail,
    the multi-keyring MUST fail.

# One-Way Doors

Once we define the decryption contract for a keyring,
that contract MUST NOT change.

# Security Considerations

If the decryption contract of any existing keyrings
does not match the existing published implementations,
that will have security implications for anyone using those keyrings.
