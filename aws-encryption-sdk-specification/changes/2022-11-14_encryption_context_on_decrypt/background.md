[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Customers should control where encryption context is stored

# Definitions

## Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

# Background

Encryption context is a structured form
of Additional Authenticated Data (AAD).
By structuring the AAD the AWS Encryption SDK (ESDK)
makes AAD easier to use and reason about.

The AWS Encryption SDK
requires storing all encryption context
with the encrypted message,
and validating any encryption context
after decryption.

This simplification has value
in communicating the non-secret nature of encryption context.
From [customer use cases for encryption context](encryption_context_use_cases.md)
there is customer value in a more nuanced view of encryption context.

# Requirements

1. Customers should not be forced
   to choose between encrypted message size
   and valuable authenticated encryption context.

1. Customers should not be forced
   to choose between storing non-public data
   alongside encrypted messages
   and valuable authenticated encryption context.

1. Customers should be able
   to ensure encryption context requirements are validated
   before obtaining the plaintext.

1. Customers should be able
   to ensure that critical encryption context
   authentication checks fail closed
   and do not return the plaintext.

# Success Measurements

Clear components that can be used
outside of the ESDK.

# Out of Scope

- Backwards incompatible changes
- Solutions that involve algorithm suites
- Changes to the message format
- Features other than of encryption context

# Design questions

_**Preferred options are identified like this.**_

# Question: How can customers manage encrypted message size, use non-public data, and fail closed with encryption context?

## Option 1: _**Authenticate but do not store some encryption context pairs with the message.**_

Any value not stored will not contribute to encrypted message size
and can not be disclosed by the encrypted message.
Any value that is authenticated but not stored
MUST be provided correctly on decrypt
or the authentication will not be correct.

The disadvantage is that any encryption context values
not stored with the encrypted message can be lost.
This would render the ciphertext inaccessible.

It is important to be clear, consistent, and targeted
about what values are not stored.
Storing encryption context with the message
is a [sensible default](../../tenets.md#sensible-defaults).

## Option 2: Store the keys, but not the values.

If encryption context values are lost,
it could be helpful to know something
about the shape of the missing values.
By only removing the values from the stored encryption context
customers could have a better idea about what data is missing.

This as it gives less control over size,
and it conflates an empty string with something that was removed.

The values that are not stored
should generally be the same for a given code path
e.g. `customerId` for an application that has configured this identifier globally.
Since it MUST be obvious in the code what pairs are not stored,
there is less value in seeing this reflected in the message.

# Question: How are encryption context pairs not stored with the message authenticated to the message?

## Option 1: _**Extend the Header Auth tag over the un-stored encryption context pairs.**_

The serialization of [Key Value Pairs](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/data-format/message-header.md#key-value-pairs)
MUST be empty if the encryption context is empty.
This property means that
when [constructing the header authentication](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/client-apis/encrypt.md#construct-the-header)
the serialization of any encryption context pairs
can be appended to the the [header body](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/data-format/message-header.md#header-body)
and remain backwards compatible with the existing calculation.

The signature is only over the stored encrypted message.
This way the signature can be validated without decryption.
Changing this would break backwards compatibility.

# Question: How are the encryption context values that are not stored configured?

## Option 1: _**The CMM tells the encrypt API how to split the encryption context.**_

The CMM can change the encryption context,
so any choice about this separation
before the encryption materials are returned
is based on incomplete information.

Non-public disclosure is a security concern
(Not for the individual ciphertext, but for the application as a whole.)
and security choices should be made in CMMs and keyrings.

Failing closed is on the line between
security and operational business logic.
If the encryption context
is not validated and the messages fail open,
the decryptor could become a confused deputy.
Size is clearly an operational issue.

In addition to security choices,
these kinds of operational choices,
should be made in CMMs and keyrings.

The things that should not be stored
in this way should not be different for every encrypt call.
Because they MUST be recreated,
there MUST be a clear organization.
For a given code path
the same one or two keys should be controlled in this way.

Storing the encryption context
is a [sensible default](../../tenets.md#sensible-defaults),
therefore this kind of custom behavior
is a good place for a custom CMM.

This also offers the possibilities
for additional policy hooks
in AWS KMS that do not impact
the size of the encrypted message.

## Option 2: New configuration for the encrypt call

The encrypt call could take a configuration option
to control this division.

The advantage to this option
is that binding the CMM to the storage
could make the CMM more intertwined
with the AWS Encryption SDK message format.

CMMs and Keyrings are good key distribution configuration.
They SHOULD be re-used
for other message formats or other clients.
Options that commingle storage with key distribution
muddy these waters.

However any such reusable component needs to have a serialization story.
That is that it MUST have some way to store information.
As such this stored vs not stored is not something specific to the ESDK.
Encouraging a very dynamic encryption context storage story
will make it [more difficult for customers to reason about](../../tenets.md#correct-by-construction)

## Option 3: A new component

To the extent that the CMM
is not the perfect place,
and that the encrypt API is too flexible
a new component could be a good idea.

How would this work?

1. Any dynamic API like `(encryptionContext) => [store, authenticate]`

   Simple, but a dynamic component could make security decisions.

1. Any static API like `('list', 'of', 'values') => configuration`

   How is such static configuration is different that Option 2?

1. More sweeping strategy for all serialization in addition to this.

This has some compelling ideas,
but this MUST be kept separate from the crypto.
So mixing the AAD splitting
with this makes make me very nervous.

# Question: How can customers verify encryption context before receiving plaintext?

## Option 1: _**Add encryption context option to decrypt**_

This is the dominant option.
It mirrors the standard use of AAD
in most crypto systems.

The `aws-crypt-` encryption context namespace
is reserved on encrypt.
This means that only CMMs SHOULD add these values.
But, this restriction can not be mirrored on decrypt.
Because customers MUST be able to validate on these values.
There is no clear value to
forcing customers to validate these values _only_ in CMMs.
While the encryption context keys
will be consistent for a given code path,
this is not true for the encryption context values.
Since different messages are by definition different.
Trying to distinguish between these values
that are stored vs reproduced is not [correct by construction](../../tenets.md#correct-by-construction).

## Option 2: Add an API to check before decrypt

This may be a useful API.
But splitting the check from
the decryption divides the intention
and does not help
the decrypt call be [correct by construction](../../tenets.md#correct-by-construction).

## Option 3: Do nothing

No.

# Question: When are encryption context values validated?

## Option 1: _**In the CMM before the Decryption Materials are passed to the keyring**_

Add a `Reproduced Encryption Context` option to
to the `Decryption Materials Request`.
The CMM is responsible for validating
and merges the two encryption contexts
into a single value for the `Decryption Materials`.

The CMM has the most complete information,
because it is allowed to change the encryption context.
Also, if the CMM can signal that some encryption context
values are not stored,
then it is in the best position to ensure their existence.

The CMM could add encryption context values
that are not stored with the encrypted message,
and then add these values back on decrypt.
This adds a communication chanel
that could be used to make
some CMM compositions incompatible
and therefore [hard to misuse](../../tenets.md#hard-to-misuse) together.

The CMM is also the last place _before_
the plaintext data key is unwrapped.
By failing fast we increase the security of the system.
Not only is the customer plaintext not available,
the plaintext data key to decrypt it
is also not available.
This is safe to do because the customer
has asserted to us the correct encryption context.
Even if the message could be decrypted successfully
with the stored encryption context
it is not the correct because the encryption context is incorrect.

To be clear, this means that if the customer passes
an encryption context key/value pair
for which there is a stored key/value pair
and the values do not match the CMM will fail
before attempting decryption (authentication)
of the encryption context.

## Option 2: After the CMM has unwrapped the data key?

This has nice properties but is authentication not validation.
Better to not even attempt to unwrap
the data key if the message is not what you are expecting.

## Option 3: The decrypt function before CMM

The decrypt function can not validate
pairs that were not stored
before the AAD check.
Because it does not have this information.

But, the decrypt function MUST know what
to do with stored values that don't match,
if it is going to pass a single encryption context
value on the CMM.

The decrypt function can not simply fail
if the stored values do not match
because the CMM is permitted
to change the encryption context.

# Question: How are the encryption context values that were not stored identified to be authenticated?

## Option 1: _**CMMs return the encryption context and decrypt authenticates all pairs that were not stored.**_

In most of the ESDK runtimes
the CMM does not even return
the encryption context on decrypt.
The specification now requires it
but is not clear what
the decrypt function should do with this information.

CMMs MAY change the encryption context.
This means that the new encryption context
MUST be authenticated against the encrypted message.
Returning un-authenticated encryption context
to customers is not [hard to misuse](../../tenets.md#hard-to-misuse).
It would make it easy to return
encryption context to the customer
that has not been authenticated
to this specific encrypted message.

Therefore the decrypt function MUST verify
that for all encryption context keys
that exist in both the the [header encryption context](../../data-format/message-header.md#aad)
and the [Decryption Material](../../framework/structures.md#decryption-materials)
have matching values.

This process will also identify all pairs
that exist in the [Decryption Material](../../framework/structures.md#decryption-materials)
that do not exist in the [header encryption context](../../data-format/message-header.md#aad).
This is the set of additional pairs
to be authenticated to the encrypted message.

## Option 2: The CMM communicates explicitly which values should be authenticated..

It is easy for this to get overly complicated.
If values are added to the encryption context by the CMM
but not added to be authenticated,
then information might be communicated
to the customer that was never authenticated.

# Question: Can encryption context pairs stored in the message _not_ be returned to the customer?

## Option 1: _No all pairs authenticated to the encrypted message MUST be returned._

Obfuscating such information from the caller
is just adding confusion.
If this is absolutely positively needed,
the CMM is not the proper place to configure this.
The whole decrypt function should be wrapped
for this kind of functionality.

# Question: How do customer easily use this new feature?

## Option 1: _**Provided a composable CMM.**_

A `Required Encryption Context CMM`
that takes a set of encryption context key names
and an underlying CMM.

This CMM ensures 3 things

1. Encryption context keys MUST exist before generating the unencrypted data key on encrypt.
1. Encryption context key/value pairs are authenticated but not stored with the encrypted message.
1. Encryption context keys MUST exist before unwrapping the encrypted data keys on decrypt.

Alternate names

- `Demand Encryption Context CMM`
- `Expect Encryption Context keys CMM`
- `Statutory Encryption Context CMM`
- `Compel Encryption Context CMM`
- `Necessitated Encryption Context CMM`

## Option 2: Custom CMMs built by customers.

We could provide examples only
and leave this configuration
up to customers.

This is not preferred,
because it abdicates responsibility.
If we could not create
a reasonable composable interface
we should not build the feature.

# One-Way Doors

- API changes live forever.
- Changing the way decrypt validates encryption context.
- If it turns out that not storing the EC is a bad idea,
  there will be ciphertext that will live on forever
  in an unfortunate state.

# Security Considerations

This creates a mechanisms
to enforce verifying the encryption context.
Explicit encryption context verification
is more secure than implicit encryption context verification.

The sharp edge that this introduces
is that if the required encryption context
is lost than the ciphertext is also lost.
This can be mitigated
by things like AWS KMS logging all encryption context,
but searching through this is a complicated endeavor.

# Impact/Other considerations

Explaining this subtlety of encryption context
is going to increase the complexity of our documentation.
We SHOULD build additional workshops
to help customers explore how to build
their encryption context.

Even with all this
additional questions from customers SHOULD be expected.
Since they were not REQUIRED
to verify encryption context before
they could expose subtle issues.
For example attempts to decrypt encrypted messages
with mismatched encryption context values.

# Assumptions

- All encryption context pairs MUST be authenticated.
- All encryption context pairs SHOULD be authenticated to the specific encrypted message.
- Clarity: There is never a good case
  to return encryption context values
  to the caller that have not been authenticated.

# Open Questions

- Are there better names for the recommend components?
