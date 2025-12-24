[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Encryption context values that are authenticated but not stored with the encrypted message.

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                                                       |
| ----------------------------------------------------------------------------- |
| [Decrypt](../../client-apis/decrypt.md)                                       |
| [Cryptographic Materials Manager Interface](../../framework/cmm-interface.md) |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                                 |
| ----------------------------------------------------------------------------- |
| [Decrypt](../../client-apis/decrypt.md)                                       |
| [Encrypt](../../client-apis/encrypt.md)                                       |
| [Cryptographic Materials Manager Interface](../../framework/cmm-interface.md) |
| [Structures](../../framework/structures.md#encryption-materials)              |
| [Structures](../../framework/structures.md#decryption-materials)              |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Repository                                                                            |
| ---------- | ------------------------------------------------------------------------------------- |
| Python     | [aws-encryption-sdk-python](https://github.com/aws/aws-encryption-sdk-python)         |
| Java       | [aws-encryption-sdk-java](https://github.com/aws/aws-encryption-sdk-java)             |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |
| Dafny      | [aws-encryption-sdk-dafny](https://github.com/aws/aws-encryption-sdk-dafny)           |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

Encryption context(EC) is a structured form
of Additional Authenticated Data (AAD).
By structuring the AAD the AWS Encryption SDK(ESDK)
makes AAD easier to use and reason about.

The AWS Encryption SDK
required storing all encryption context
with the encrypted message.
and required validating any encryption context
after decryption.
These requirements limit customer flexibility.

## Out of Scope

- Keyring interface changes

## Motivation

Change the way that the ESDK handles encryption context
to give customer flexibility in the following way.

1. Size

   Some customers are sensitive
   to the size of the encrypted message.
   These customers can use any values
   as encryption context properties
   and choose what to store to limit message size.

1. Non-public data classification

   Some customers need to
   transmit AWS Encryption SDK messages in public.
   These customers can use any values
   as encryption context properties
   but only transmit public values
   in the message.

1. When

   Customers should be able to construct
   [correct by construction](../../tenets.md#correct-by-construction) decrypt requests.
   This construction should be easy and obvious.
   It should block returning the plaintext
   so that customers know
   that the data they are working with
   is the data they expect.

1. Mechanisms

   Some encryption context values
   are critically important to validate.
   Customers should be able
   to ensure that such checks fail closed
   and do not return the plaintext.

## Drawbacks

- Data loss
- Secret vs non-public confusion
- Responsibility confusion.
  The decrypt API should ensure that all encryption context
  it gets from the CMM is validated to the message.
  In the limit, this is a breaking change.

## Operational Implications

This change adds new ways for `decrypt` to fail.
This may increase tickets.
Further messages encrypted with Encryption Context
that is not stored are not backwards compatible.

Upgrading every runtime to support this feature
is complicated by the fact that Java and Python
still support legacy interfaces.

## Reference-level Explanation

### New property `Required Encryption Context Keys`

on [Encryption Materials Request](../../framework/cmm-interface.md#encryption-materials-request)
to CMM.

This new property `Required Encryption Context Keys`
communicates to the CMM
the encryption context key-value pairs
that MUST be authenticated to the encrypted message
but SHOULD NOT be stored with the encrypted message.
This property is added to the Encryption Materials Request only for CMM composition,
it MUST NOT be used by higher level `encrypt` APIs
in [supported libraries](../../framework/algorithm-suites.md#supported-libraries).

### New property `Required Encryption Context Keys`

on [Encryption Materials](../../framework/structures.md#encryption-materials)

Communicates to the the configured keyring
what encryption context key-value pairs
will not be stored with the message.

Communicates to higher level `encrypt` APIs
in [supported libraries](../../framework/algorithm-suites.md#supported-libraries)
how to split the encryption context
into elements that are authenticated and stored
from elements that are only authenticated and not stored.
`Required Encryption Context Keys` MAY be an empty set.

### Additional CMM responsibility

on [Get Encryption Materials](../../framework/cmm-interface.md#get-encryption-materials)

CMMs can now specify
encryption context key-value pairs
that MUST be authenticated to the encrypted message
but MUST NOT be stored with the encrypted message.
The CMM MAY modify the `Required Encryption Context Keys` set.

The returned [Encryption Materials](../../framework/structures.md#encryption-materials)
MUST include `Required Encryption Context Keys`.
The value on the `Encryption Materials` SHOULD match
the value on the `Encryption Materials Request`.

### Update the Default CMM

on [Get Encryption Materials](../../framework/default-cmm.md#get-encryption-materials)

The value of `Required Encryption Context Keys`
in the [Encryption Materials Request](../../framework/cmm-interface.md#encryption-materials-request)
MUST be be included in the encryption materials returned.

### `encrypt` API can authenticate but not store

Using the property `Required Encryption Context Keys`,
on the returned [Encryption Materials](../../framework/structures.md#encryption-materials)
the encryption context on the encryption materials
is divided into 2 parts.

The authenticated and stored encryption context
is all encryption context key-value pairs where
the key does not exist in `Required Encryption Context Keys`.

The authenticated only encryption context
is all encryption context key-value pairs where
the key exists in `Required Encryption Context Keys`.
It is then serialized according to
the message header [Key Value Pairs](../../data-format/message-header.md#key-value-pairs).

The authenticated and stored encryption context
is handled as normal
in the [Header Body](../../data-format/message-header.md#header-body).

When [constructing the header](../../client-apis/encrypt.md#construct-the-header)
the authentication tag is over the message header body
and the authenticated only encryption context.
This is accomplished by appending
the serialized authenticated only encryption context
to the message header body
and using this new concatenated value as the AAD input
to the authenticated encryption algorithm specified by the algorithm suite.

### New property `Reproduced Encryption Context`

on [Decrypt Materials Request](../../framework/cmm-interface.md#decrypt-materials-request)
to CMM.

New property `Reproduced Encryption Context`
to communicate to the CMM
the encryption context key-value pairs
that are reproduced at decrypt.

### New optional input `Encryption Context`

on [decrypt](../../client-apis/decrypt.md#input)

The ESDK `decrypt` API MUST accepts an optional input `Encryption Context`.

To obtain [decryption materials](../../client-apis/decrypt.md#get-the-decryption-materials)
the call to the input CMM MUST be constructed
with `Reproduced Encryption Context` equal to
the optional input `Encryption Context`.

### New property `Required Encryption Context Keys`

on [Decryption Materials](../../framework/structures.md#decryption-materials)

Communicates to the the configured keyring
what encryption context key-value pairs
were not be stored in the encrypted message.

Communicates to higher level `decrypt` APIs
in [supported libraries](../../framework/algorithm-suites.md#supported-libraries)
how to split the encryption context
into elements that are authenticated and stored
from elements that are only authenticated
and not stored in the encrypted message.

`Required Encryption Context Keys` MAY be an empty set.

### Additional CMM responsibility

on [Decrypt Materials](../../framework/cmm-interface.md#decrypt-materials)

The CMM MUST validate customer supplied
encryption context on decrypt
by comparing the key-value pairs in
`Encryption Context` and `Reproduced Encryption Context`
on the `Decrypt Materials Request`.

The CMM MUST ensure that the value for every key
that exist in both `Encryption Context`
and `Reproduced Encryption Context` MUST be equal.

The CMM MAY fail if expected key-value pairs
do not exist in `Reproduced Encryption Context`
on the `Decrypt Materials Request`.

The CMM MAY modify the encryption context,
included in the returned `Decryption Materials`
but it SHOULD append all key-value pairs
that exist in `Reproduced Encryption Context`
but do not exist in `Encryption Context`
on the `Decrypt Materials Request`.

The decryption materials returned
MUST include `Required Encryption Context Keys`
and this set MUST include
all keys that did not exist
on the `Encryption Context`
on the `Decrypt Materials Request`.

### Update Default CMM

on [Decrypt Materials](../../framework/cmm-interface.md#decrypt-materials)

The value for every key
that exist in both `Encryption Context`
and `Reproduced Encryption Context` MUST be equal.

The [Decryption Materials](../../framework/structures.md#decryption-materials)
`Encryption Context` value
MUST include all unique key-value pairs
in both `Reproduced Encryption Context` and `Encryption Context`
from the `Decrypt Materials Request`.
The `Required Encryption Context Keys` value
MUST include all the keys
that exist in `Reproduced Encryption Context`
but do not exist in `Encryption Context`.

The `Decryption Materials` returned
MUST include the `Encryption Context`
and the `Required Encryption Context Keys`
constructed above.

### `decrypt` API can authenticate encryption context values that were not stored

Using the property `Required Encryption Context Keys`,
on the returned [Decryption Materials](../../framework/structures.md#decryption-materials)
the authenticated only encryption context is derived from
the encryption context on the decryption materials.

The authenticated only encryption context
is all encryption context key-value pairs where
the key exists in `Required Encryption Context Keys`.
It is then serialized according to
the message header [Key Value Pairs](../../data-format/message-header.md#key-value-pairs).

When [verifying the header](../../client-apis/decrypt.md#verify-the-header) the AAD input
to the authenticated encryption algorithm specified by the algorithm suite
is the message header body and the serialized authenticated only encryption context.

### New CMM to make it easy

Having created the ability authenticate but not store
some encryption context key-value pairs
customers need a simple interface to compose their requirements.

To satisfy this the `Required Encryption Context` CMM
takes an underlying CMM
and a set of encryption context keys.

If the caller provides a keyring,
then the `Required Encryption Context` CMM
MUST set its underlying CMM
to a default CMM initialized with the keyring.

On [Get Encryption Materials](../../framework/cmm-interface.md#decrypt-materials)
the `Encryption Context` of the [Encryption Materials Request](../../framework/cmm-interface.md#encryption-materials-request)
MUST have a key
for every value
in the set of encryption context keys
passed at initialization.

The `Required Encryption Context` CMM MUST
attempt to obtain the encryption materials
by making a call to the underlying CMM's Get Encryption Materials
with the set of encryption context keys
appended to `Required Encryption Context Keys`.

On [Decrypt Materials](../../framework/cmm-interface.md#decrypt-materials)
the `Reproduced Encryption Context`
on [Decrypt Materials Request](../../framework/cmm-interface.md#decrypt-materials-request)
MUST have a key
for every value
in the set of encryption context keys
passed at initialization.

The `Required Encryption Context` CMM MUST
attempt to obtain the decryption materials
by making a call to the underlying CMM's Decrypt Materials
with the unmodified `Decrypt Materials Request`.
