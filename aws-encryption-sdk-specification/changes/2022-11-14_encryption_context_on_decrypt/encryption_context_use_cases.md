[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Customer use cases for encryption context

# Definitions

**Conventions used in this document**

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Must

In cryptographic applications MUST
can refer to a cryptographic property
that is ensured by math.
In ths document MUST refers to business logic.
It is therefore _possible_ to create
an application that may not honor these MUSTs.
A goal is to make these MUST
as hard as possible to violate.

## Verification

In the context of this document verification means
satisfying the customer's requirements on decryption.
Therefore from the perspective of the customer,
if _no_ elements of the encryption context are ever verified,
this is functionally equivalent to a message with no encryption context.

# Background

The AWS Encryption SDK (ESDK) treats
the ciphertext and the encryption context as equally sensitive.
This is reasonable position to take
because for most customers the encrypter, decrypter, transport and storage systems
all existed as secure systems.
This simplification has value
in communicating the non-secret nature of encryption context.

To reevaluate this choice here are
several ways that customers
may want to use encryption context.
These requirements are intended to conflict,
and we may not choose to satisfy all of them.

By highlighting the conflicts that exist
in handling the encryption context
we will be able decide if we want to change this.
Also, if we do we will have a guide
to structure any future design document.

# Encryption Context

Encryption context is set by the encrypter
for use by the decrypter.
It is a structured form of Additional Authenticated Data(AAD).
The decryption process ensures
that these values have not been altered.
This means that the encrypter MUST
set the super-set of all values
relevant to any decrypter.
Therefore encryption context values
that are set but **never** used are an anti-pattern.

The ESDK does not offer any freedom
about what values of the encryption context
are stored with the message or when and how
the validation of the encryption context occurs.

## Why customers care?

To avoid an overly abstract conversation
here are concrete examples.
These examples lead to
precise formulations of customer requirements.

1. The confused deputy

   A common pattern is
   to have a decrypter
   that can decrypt data for many callers.

   Possession of the ciphertext
   by any given caller
   does not grant the authority
   to decrypt that ciphertext.

   Caller α MUST only
   receive data intended for caller α.

   A possible solution is to add
   the authN information for every caller
   to the encryption context
   for ciphertext that is intended for that caller.
   If the decrypter verifies this,
   then our overly simplicity system is "safe".

   **As an encrypter I want to ensure
   that decrypters MUST verify
   specific values in the encryption context.**

1. Complicated validation requirements

   Validation requirements can be more complicated than equality.
   Perhaps there are multiple valid data formats (JSON, XML),
   date range requirements (less than 1 week old),
   a key must exist regardless of value (any 'invoice').

   These are all examples where the decrypter knows something
   about the shape of the encryption context,
   but not the exact values.

   **As a decrypter I want to validate
   parts of the encryption context
   based on conditions other than equality.**

1. Logging, debugging and auditing

   Encryption context is a wrapper of AAD.
   As such it only admits of equality.
   Some values are useful,
   but are not practical for the decrypter
   to be required to validate.
   DateTimes fall into this category.
   It is useful to know exactly when
   something was created or encrypted
   but coordinating time in a distributed system is complicated.
   If the authenticity of this data
   can be ensured then it can be relied on
   to evaluate the provenance of the data.

   To decrypt the ciphertext the decrypter MUST
   have access to the encryption context
   and the encryption context MUST NOT be secret.
   Therefore the encryption context SHOULD be safe to log locally.

   **As a decrypter I want additional information
   to aid in metrics, logs, and debugging
   that I am not required to validate.**

1. Non-public information

   Because my data is encrypted
   I may want to store or transport it publicly.
   To protect myself from confused deputy problems
   I want to use information to identify
   the customer who owns this ciphertext.
   This information is non-public.
   While it is not so sensitive that it
   must be encrypted in transit,
   it is also not appropriate to
   transmit this data publicly in plaintext.

   For example, using a customer ID
   or email address would result
   in always sending that data
   in plaintext as part of the
   encrypted messages.

   **As an encrypter I want
   to have non-public information
   in my encryption context,
   but still store or transit
   my ciphertext without
   additionally storing or transmitting
   this non-public information
   in plaintext.**

1. Non-public is not the same as secret

   A customer might look at encryption context values
   that are _not_ stored with the message
   and conclude that this value
   can be used as a secret.
   This is absolutely **not** the case.

   Encryption context SHOULD be safe to log.
   AWS KMS logs all encryption context
   to AWS CloudTrail!

   Creating confusion about
   what is secret is dangerous.
   Customers might be surprised
   and in the case of secrets
   this surprise will burn trust.

   **As a customer I do not want to
   accidentally leak secrets
   because of a subtle misunderstanding.**

1. Size

   There are use cases where size is at a premium.
   Cookies, continuation tokens, database entries,
   IoT/embedded devices.
   As the overall size of the plaintext gets smaller,
   and the number of relationships grows
   the impact of duplicated encryption context grows.

   **As an encrypter I want greater control
   over the size of the ESDK message
   by not storing encryption context
   that the decrypter MUST be able to be recreate.**

1. Data loss

   If the encryption context is not stored
   with the message than it can be lost.
   If it is lost the ESDK MUST NOT decrypt the message.
   While there may be technical ways
   to get around this they are complicated, expensive, and dangerous.

   **As a customer I do not want to lose data
   because of missing encryption context
   from an operational event or misconfiguration.**

1. Side channels, error message, and crash dumps

   Once the ciphertext has been decrypted
   it now exits as plaintext.
   If it turns out that this
   is not the correct plaintext
   it SHOULD be discarded.

   The best thing to do is to
   fail as soon as the ESDK knows
   that the encryption context
   is not valid.
   It is even better if the ESDK can avoid
   unwrapping the data key.
   This makes it impossible
   for a customer process to accidentally
   include the plaintext in an error message.

   **As a decrypter I want to ensure
   I MUST NOT have plaintext
   for an invalid encryption context.**

1. Signing and authenticity

   In addition to confidential messages,
   there are times when a customer may
   want to take action on metadata.
   Or is not interested in confidentiality at all.

   In these cases the authenticity _is_ the value.
   For example keeping count of different types of things.
   The type is stored in the encryption context
   and for each item seen,
   the appropriate counter is incremented.

   This process MUST happen _after_ the authenticity
   has been evaluated.

   **As a decrypter I want to take action
   based on authenticated data.**
