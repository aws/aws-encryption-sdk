[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Encryption Context Reserved Prefix

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------ |
| [Structures](../../framework/structures.md)                                                                        |
| [Default CMM](../../framework/default-cmm.md)                                                                      |
| [Encrypt](../../client-apis/encrypt.md)                                                                            |
| [Define encryption context reserved prefix](https://github.com/awslabs/aws-encryption-sdk-specification/issues/23) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                 |
| --------------------------------------------- |
| [Structures](../../framework/structures.md)   |
| [Default CMM](../../framework/default-cmm.md) |
| [Encrypt](../../client-apis/encrypt.md)       |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| Python     | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python)         |
| Java       | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)             |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The specification currently states that the encryption context
MUST reserve the `aws-crypto-public-key` key for use by the AWS Encryption SDK (ESDK)
and SHOULD reserve any key with the prefix `aws`.
We will replace these two requirements with a statement that the encryption context
MUST reserve any key with the prefix `aws-crypto-`.

We will also relocate this statement to apply to the encryption context passed to encrypt,
rather than on all encryption context values.

## Out of Scope

It is possible that an existing encryption context used as input to encrypt
will be broken by this change.
Designing a migration approach for affected customers
is out of scope for this document.

## Motivation

The current statement using "SHOULD" is too weak to be of any use.
Some customers could be providing encryption contexts
with keys using the `aws` prefix,
and in fact this is relatively likely
since the context of many uses of the ESDK
involves one or more relevant values related to AWS services.
This means if any future versions of CMMs packaged with the ESDK
choose to add additional internal keys to the encryption context,
they could be a breaking change.

In addition, stating that the encryption context structure itself
MUST reserve any set of keys
is questionable.
Interpreted literally, this leads to a contradiction
since the [Default CMM](../../framework/default-cmm.md) itself adds a key with the reserved prefix.
Therefore, it is clearer to specify this
as a restriction on the input to the decrypt operation.
This still allows any [CMM](../../framework/cmm-interface.md) to use the reserved scope.

## Drawbacks

Even reserving the less general `aws-crypto-` prefix
is still a breaking change,
since it is currently possible for customers
to include keys beginning with this prefix.
This is much less likely to actually break any customers
compared to any variation of the `aws` prefix, however.

## Security Implications

By itself this change SHOULD NOT have any security implications.

## Operational Implications

There is still a very slight chance
this change will break an existing use case.
This change will be released under a new major version of each implementation,
and it will at least be possible, if difficult,
for any affected customers to migrate to using a different key.

## Guide-level/Reference-level Explanation

The last two statements in the specification of the [encryption context structure](../../framework/structures.md#encryption-context)
referring to reserved keys SHALL be removed.

The encrypt operation specification will be changed to fail
if provided an encryption context containing a key
that begins with the reserved prefix `aws-crypto-`.

Finally, the default CMM Get Encryption Materials operation
will specify that if the encryption context included in the request
already contains the `aws-crypto-public-key` key
it MUST fail.
This is to ensure this key is not overwritten
in any uses of the CMM that do not go through the top-level encrypt operation.
