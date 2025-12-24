[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Remove Keyring Trace

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Keyring Trace](https://github.com/awslabs/aws-encryption-sdk-specification/blob/61f9edd7c4adf8e8ff9af77bbe9eaf3015099a88/framework/structures.md#keyring-trace-2) |
| [Keyring Failure Communication](https://github.com/awslabs/aws-encryption-sdk-specification/issues/40)                                                             |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                                 |
| ----------------------------------------------------------------------------- |
| [Structures](../../framework/structures.md)                                   |
| [Keyring Interface](../../framework/keyring-interface.md)                     |
| [AWS KMS Keyring](../../framework/aws-kms/aws-kms-keyring.md)                 |
| [Raw AES Keyring](../../framework/raw-aes-keyring.md)                         |
| [Raw RSA Keyring](../../framework/raw-rsa-keyring.md)                         |
| [Cryptographic Materials Manager Interface](../../framework/cmm-interface.md) |
| [Default Cryptographic Materials Manager](../../framework/default-cmm.md)     |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Version Introduced | Version Removed | Repository                                                                            |
| ---------- | ------------------ | --------------- | ------------------------------------------------------------------------------------- |
| C          | 0.1.0              | n/a             | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | 0.1.0              | n/a             | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

We will remove the keyring trace from the AWS Encryption SDK specification
and affected implementations
because we have determined that existing and better-defined parts of
the AWS Encryption SDK framework provide better solutions
to the problems that we intended the keyring trace to solve.

## Out of Scope

The design for keyring failure communication is out of scope.
That feature [is tracked separately](https://github.com/awslabs/aws-encryption-sdk-specification/issues/40).

## Motivation

We added the keyring trace with the anticipation that it would be a useful tool
to make assertions about what keyrings did to encryption and decryption materials.
However, we never defined how callers should interact with the keyring trace.
Before adding keyrings to additional implementations beyond C and Javascript,
[we re-evaluated how callers should interact with the keyring trace](background.md)
and came to the conclusion that they should not.
We determined that the keyring trace is unnecessary
because all expected use-cases are better solved either
by making keyrings that are correct by construction
or by proactively checking requirements before invoking keyrings.
We had considered adding failure information to the keyring trace,
but upon reviewing the capabilities that we would want in
a tool to communicate failure information,
we came to the conclusion that the keyring trace does not meet those requirements
and that a purpose-built solution will solve that problem better
than retrofitting failure information onto the keyring trace.

## Drawbacks

We will lose the reactive trace of keyring actions.
However, [upon review](background.md) we concluded that
any reactive checks that a caller might want to make
are better addressed by either proactive checks before the keyring(s) take such actions
or by ensuring that the keyring is correct by construction
so that the only thing it _can_ do is what the caller wants to happen.

When we [add keyring failure communication](https://github.com/awslabs/aws-encryption-sdk-specification/issues/40),
we will need a new mechanism to convey that information.
However, [upon review](background.md) we concluded that
the keyring trace is the wrong tool for that.

One thing that callers could have used the keyring trace to do is to
retain an audit log of all actions that keyrings performed
during an encryption or decryption attempt.
However, audit logging really just pushes the question of "why" further down the road.
If you want to control what is happening,
the correct way to do that is to construct your
keyring(s) and cryptographic materials manager(s)
such that the only thing that they _can_ do is what you want them to do.

## Security Implications

The main security implication of this change is the loss of the ability
to have an audit log of all (successful) actions that keyrings took.
However, enforcing requirements beforehand
gives users more control over the actions and results than
checking for requirements after the fact.

## Operational Implications

Removing the keyring trace is a breaking change in
the [C and Javascript implementations of the AWS Encryption SDK](#affected-implementations).

Neither implementation exposes the keyring trace outside of the AWS Encryption SDK framework.
The only way that a caller can interact with the keyring trace is
through custom keyrings or cryptographic materials managers.

## Guide-level Explanation

The keyring trace is a record of successful actions taken by keyrings
that can be used to make decisions in reaction to those actions.
After [reviewing how callers should interact with the keyring trace](background.md),
we have come to the conclusion that anything that a caller might want to do
in reaction to a successful keyring action
is better done either proactively _before_ the keyring would take an action
or by ensuring that the keyring is correct by construction
and can only do what the caller wants it to do.
Based on this conclusion,
we will remove the keyring trace from this specification
and from [all published keyring implementations](#affected-implementations).

This change will not impact anyone who is not using a custom
keyring or cryptographic materials manager.

This change will impact anyone who is using a custom
keyring or cryptographic materials manager
that reads the keyring trace.

In addition to removing the keyring trace,
we will provide a list of problems that could have been addressed using the keyring trace
along with examples that demonstrate how to solve those problems
using other components of the AWS Encryption SDK framework.

## Reference-level Explanation

### Code Change

Code changes required to make this change include:

- Stop writing to the trace:
  All provided keyrings write entries to the keyring trace.
  Remove this.
- Remove trace from materials:
  The keyring trace is exposed in the encryption materials and decryption materials structures.
  Remove this.
- Remove trace flags and types:
  Remove any data types, structures, etc that are only used as part of the keyring trace.
  This includes the keyring trace flags.

### Examples

Examples that MUST be added include:

- Use AWS KMS Discovery Keyring to decrypt,
  but require that a CMK in a specific account
  was used to decrypt the data key.

  - Use a filtering keyring to only pass through encrypted data keys
    that were encrypted by AWS KMS using a CMK in a specific account.

- Require that all encrypted data keys are encrypted by keyrings
  that sign the encryption context.

  - Only include keyrings that sign the encryption context in your keyring.

- Require that the keyring that decrypts the data key
  also verify the encryption context.

  - Only include keyrings that verify the encryption context in your keyring.

- Reject encrypted messages that contain an encrypted data key
  that was encrypted under an unknown wrapping key.
  (This scenario could impact the security of the message signature.)

  - Use a CMM that inspects the encrypted data keys'
    wrapping key identifiers before attempting to decrypt them.

- Require that the data key is always generated by a specific keyring.

  - Use the multi-keyring to enforce that a specific keyring MUST always generate the data key.
