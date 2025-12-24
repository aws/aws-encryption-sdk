[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# The AWS Encryption SDK (ESDK) and the keyring trace

## Background

When we designed keyrings,
we added a concept of a "keyring trace"
that the keyring uses to communicate
what actions it took.
This is an evolution of earlier indicators
in the decryption API that indicated which master key
decrypted the data key.
In both cases, we exposed the data to the caller
but did not include any guidance on what they should do with it,
how to interact with it,
or why it is important.
This is similar to how we treat encryption context
in the encryption and decryption API results.

## Goals

Our goal is to determine how, or if,
we should expose the keyring trace.

## Success Measurements

We will know we are succeeding if we can assemble
multiple known customer problems that we think keyring traces solve
and present examples that address each problem
that _either_ demonstrate why keyring traces are needed
and how they solve those problems
_or_ demonstrate why keyring traces are not needed.

## Out of Scope

Anything that requires us to add API surface area,
whether that is modifying existing APIs or interfaces,
must be treated as new features.
All new features must be
reviewed through the specification modification process.

## Issues and Alternatives

As they exist today,
keyring traces are not very usable,
but more importantly
we never explain or show why they should be used.

Each following issue is dependent on
answering the previous issue.

_Preferred options are in italics._

**New feature requirements are in bold.**

### Issue 0: Why should callers interact with the keyring trace?

If we cannot define a clear purpose for the keyring trace
that is not already met by other ESDK framework components,
we should not expose it to callers.
This needs to include not only
an explanation of what problems the keyring trace solves,
but also guidance on how to use the keyring trace
to solve those problems
and where in the framework those problems should be solved.

- _Option: They shouldn't._

  - If we cannot come up with any problems
    that the keyring trace solves in its current state,
    then we should not expose it to customers in any way
    and we should not mention it in any documentation or examples.
    It should remain an implementation detail
    until or unless we find a use for it.

- Option: Asynchronous audit log.

  - Writing the keyring trace to an audit log
    would give customers useful metrics on
    how they are using the ESDK
    throughout their systems.

    - counter: This just moves the question of "why" down the road.

- Option: Data protection controls.

  - Not all keyrings provide the same protections.
    One use of the keyring trace could be to
    validate that certain protections were applied to
    the encrypted data keys in an encrypted message.

    - ex: Require that all keyrings that encrypted the data key
      also signed the encryption context.

      - alternative: Inspect keyrings before use
        to check that they meet your requirements.

- Option: Live usage audit.

  - Because keyring behaviors can get complex,
    a live audit of keyring actions
    could be useful to enforce wrapping key requirements.

    - ex: Allow only AWS KMS wrapping keys within a specific account
      on decryption.

      - alternative: Make a keyring that filters out undesirable EDKs.

  - If a customer accepts encrypted messages from unverified sources,
    they might want to not trust encrypted messages
    that contain EDKs for unknown wrapping keys
    and use unsigned algorithm suites.

    - alternative: Make a CMM that checks these requirements
      before attempting to decrypt any EDKs.

- Option: Notification of failures and no-ops on decryption.

  - **Requires adding a new keyring trace action flag.**
  - Because CMMs and keyrings can be deeply nested
    and keyrings do not halt decryption
    if they encounter an error on decrypt,
    it can be difficult to determine
    why a decryption request failed.
    Requiring keyrings to add keyring trace entries
    that describe no-op and failure events
    would help a caller determine
    why no EDKs could be decrypted.

### Issue 1: What should callers read from the keyring trace?

The keyring trace is defined as a list of entries,
each entry composed of
one or more action flags that describe what a keyring did,
as well as information that identifies the keyring that performed those actions.

1. _Option: Both the action flag and the keyring identifier._

   - If both the action taken and the keyring that took it are important,
     the caller MUST be able to connect a trace entry
     to a keyring instance.

1. Option: Nothing.

   - If the keyring trace is intended solely for asynchronous audit,
     the caller should not interact with it at runtime.

1. Option: Only the action flag values.

   - If the primary value is in the action taken
     rather than the keyring that took that action,
     the caller should not attempt to connect a trace entry
     to a keyring instance
     or to an EDK.

1. Option: Only the keyring identifier.

   - Included for completeness.
     If the only thing that is important is which keyrings took any action,
     the keyring trace is already overly complicated.

### Issue 2: How should callers interact with the keyring trace?

More than one of these options might be necessary,
depending on the answer to **Issue 1**.

1. Option: Given an action flag, find all entries containing that flag.

   - This is straightforward and already possible
     with the current structure of the keyring trace entries.

1. Option: Given a keyring, find all entries created by that keyring.

   - **This will likely require an addition to the keyring interface.**
   - Because keyrings can have more than one key namespace and key name,
     connecting a keyring to one or more trace entries can be difficult.

### Issue 3: Where and when should callers interact with the keyring trace?

1. _Option: Within cryptographic materials managers (CMMs)._

   - All request and message values can be accessed at this level.
   - This should be sufficient for enforcing requirements
     either statically or based on the request or message metadata.

     - ex: A CMM that requires that
       all keyrings that encrypted the data key
       also signed the encryption context.
     - ex: A CMM that requires that an escrow keyring
       encrypted the data key for any messages
       whose encryption context contains a specific value.
     - ex: A CMM that writes the keyring trace to an audit log.

1. _Option: Within keyrings._

   - Not all request and message values can be accessed at this level.
   - This should be sufficient for keyrings that might choose
     to take (or not take) certain actions based on
     previous actions.

     - ex: A multi-keyring that keeps trying child keyrings
       until at least one keyring has
       verified the encryption context.

1. Option: Outside of the ESDK.

   - **Requires adding output values to the API signatures.**
   - The keyring trace must be returned from the top-level APIs.
   - This should only be necessary if the requirements
     that we expect customers to want to enforce
     vary across messages
     or depend on details outside of
     the message and request metadata.

1. Option: Within the ESDK client.

   - **Requires adding input values to the API signatures.**
   - **Requires adding a new conceptual feature.**
   - The caller providers per-request keyring trace checking requirements
     that the ESDK client performs after calling the CMM.

     - This is conceptually similar to previous ideas about
       how to give customers a way to check the encryption context
       before decrypting an encrypted message.
     - This should only be necessary if the requirements
       that we expect customers to want to enforce
       vary across messages
       or depend on details outside of
       the message and request metadata.

### Issue 4: Which actions flags should a keyring trace entry allow?

1. Option: Successful actions.

   - Any action that a keyring completes successfully.
   - This is what happens today for:
     - generate data key
     - encrypt data key
     - sign encryption context
     - decrypt data key
     - verify encryption context

1. Option: Failure.

   - **Requires adding a new keyring trace action flag.**
   - Any action that a keyring attempted but failed to complete.
   - This is useful for debugging why an encrypt or decrypt request failed.

1. Option: No-op.

   - **Requires adding a new keyring trace action flag.**
   - If a keyring chooses to do nothing.
   - This is useful for debugging why an encrypt or decrypt request failed.

## One-Way Doors

Any change that would add API surface area is a one-way door.
Any such changes must be treated as new features
and handled through the specification modification process.

1. Adding functionality to the keyring interface. (**Issue 2**)
1. Returning the keyring trace from the ESDK APIs. (**Issue 3**)
1. Adding a "message requirements" system to the ESDK APIs. (**Issue 3**)
1. Adding new keyring trace action flags. (**Issue 4**)

## Impact

1. All pending and future ESDK releases are blocked by these issues.
1. Each of the one-way doors also represents a new feature
   that must be reviewed through the specification modification process.
   This will impact all projected ESDK development and release targets.

## Open Questions

- Is it important to be able to tie
  a successful keyring trace entry to an EDK?
- Is the order of entries in the keyring trace important? If so, what order?

  - Absolute order?
  - Relative order?
  - State of materials beforehand?
  - What about concurrent actions? (ex: parallel multi-keyring)

- "[..] the requirements
  that we expect customers to want to enforce
  vary across messages
  or depend on details outside of
  the message and request metadata."

  - Do these requirements exist
    and are they requirements that
    the ESDK should support solving?
