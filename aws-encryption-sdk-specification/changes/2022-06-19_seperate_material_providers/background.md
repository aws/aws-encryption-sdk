[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Material Providers Library (MPL) and the AWS Encryption SDK (ESDK)

## Background

When we designed keyrings
and Cryptographic Materials Managers (CMMs),
our goal was to simplify
the key distribution configuration for the ESDK.
These components have been the culmination
of design from the S3 Encryption Client (S3EC),
the DynamoDB Encryption Client (DDBEC),
the original Master Key interfaces,
and the introduction of CMMs to support caching.

All of these various interfaces intend to do the same thing.

## Goals

Our goal is to break out keyrings and CMMs
out from the ESDK specification
so that they can be used more abstractly
in other libraries.

## Success Measurements

We will know we are succeeding if we can
reuse keyrings and CMMs in other our other libraries.
Like the S3 Encryption Client
and the DynamoDB Encryption Client.

## Out of Scope

The specific cryptographic requirements
for Algorithm Suites in each library.
That is how every library implements
the security requirements for a given suite.

## Issues and Alternatives

Today, we have a variety of ways to configure key distribution.
Master Keys/Master Key Providers in the ESDK Java and Python,
Cryptographic Material Providers in the DDBEC,
and Encryption Materials in S3EC.
This adds complexity for customers attempting to configure our libraries.
This added complexity does not add distinguishing value for each library.
For example to use an AWS KMS Key the minium requirement should be the ARN.

Each following issue is dependent on
answering the previous issue.

_Preferred options are in italics._

**New feature requirements are in bold.**

### Issue 1: Should we consolidate on CMM/Keyrings or something new?

- _Option: We should use CMM/Keyrings_

  - It is not that we expect them to be complete,
    but the benefit to customers
    in a single documented configuration is the dominant value.
    By having a single interface we can also focus
    our efforts on new libraries or features.
    Having consolidated all libraries to a single interface,
    we could then improve them as needed.

- Option: Build purpose built interfaces

  - This is what we have done.
    To date, each new implementation
    takes ideas from all others.
    As of this writhing there are no known
    specific features of any key distribution configuration
    that are seen as exclusive to that library.
    Purpose built interfaces then do not add value
    in simply obtaining materials.
    They may add value by tightly coupling the specific security properties
    required in these materials use to the implementation.
    But this detail is needed regardless of the interface.

- Option: Build a new shared interface

  - Given the general improvements from S3EC to DDBEC to the ESDK
    I see no reason that this final redesign
    would not have room for improvement.
    Such a design takes work and would delay other feature development.
    Having consolidated all libraries to a single interface,
    we could then improve or even embark on such a redesign.

### Issue 2: Should Algorithm Suites used by multiple libraries be duplicated?

- _Option: Yes_

  - Each library will have specific implementation details for each Algorithm Suites.
    This alone makes these Algorithm Suites different.
    For example just because 2 suites use HKDF and have a encryption key of 32 bytes
    does not make any claim about how the `HKDF:info` is stored or obtained for these 2 implementations.
    Trying to consolidate this distinction is optimizing for the wrong thing.

    It is critically important that we can audit the security properties for a suite.
    Creating ambiguity in this dimension serves no one.

- Option: No

  - This makes for a slightly smaller binary.
    As above, this is optimizing for the wrong thing.

### Issue 3: Should Algorithm Suites ID used by multiple libraries be duplicated?

- _Option: No_

  - If you need to have a cryptographic binding between all options of an algorithm suites
    there should be no way to confuse these options.
    Now that there are multiple supported libraries,
    this makes it impossible to take values serialized in one library
    and reformat these values into another library
    and have things like HKDF return the same value.

    By having different hex values there can be no collision.

### Issue 4: Should the [Message Format Version](../../framework/algorithm-suites.md#message-format-version) property be moved into the implementing library?

- _Option: No_

  - The coupling between the MPL
    and its dependent library is strong.
    It is not intended to be a generic key distribution system.
    This is because for the suite to offer
    the its specific security properties
    there are certain implementation requirements that must be met.

    For any dependent library these details need to be checked.

- Option: Yes

  - This makes the MPL easier to use,
    but it also makes it easier to misuse.

### Issue 5: Should the [Message Format Version](../../framework/algorithm-suites.md#message-format-version) property be expanded to include multiple values?

- _Option: No_

  - This creates needless ambiguity.
    Auditing the implementation requirements is difficult enough.
    Adding additional complexity to this process
    is optimizing for the wrong thing.

- Option: Yes

  - Saving space here just makes MPL easier to misuse.
    Any implementing library will already need
    some kind of branching logic to handle such a serialization distinction.
    Placing this logic on the algorithm suite
    is the right level of abstraction.
    Again, the security properties are related to the serialization
    because this is how the implementation requirements are met.

    There should only be one way to serialize a message.
    This makes the connection between the algorithm suite
    and the serialization format correct by construction.
    Because there is no branching.

### Issue 6: How should the list of Algorithm Suite IDs be a super set?

- _Option: There should be an individual list of Algorithm Suite ID enums per library that is also grouped by a union._

  - By having a list of separate enums it is easy to express
    what suites are supported by what library.
    The enum for the ID is not the same
    as the algorithm suite info: e.g. all the constants.
    This duplication can be handled by Dafny subset types.
    So while it is complicated,
    the result is a call that is correct by construction.

    While having a list of separate enums is great for the libraries,
    there should also be a unique single list that is a union of all enums.
    This list is the list of suites that the CMM should take.

- Option: The list of Algorithm Suites should include all needed suites.

  - Having a single list of all Algorithm Suites
    will help us reason about their security properties.
    Mapping each of these IDs will create duplicates
    that only differ by suite ID.
    (In the case of multiple libraries supporting a suite.)

### Issue 7: Should we handle Commitment Policy differently than Algorithm Suites?

- _Option: We should handle them the same way._

  - Once we have an enum for every supported library,
    there is a pattern for how to deal with this.
    Commitment is thorny since if there is any way to bypass it,
    then you can never say that you have it.
    This may be an option that is not needed globally
    because new libraries must support commitment
    by default and would not have a non committing suite option.
    But treating such options all the same way
    makes for a simple easy to understand patter.
    It increases the complexity of the MPL
    does not increase the complexity for
    customers using supported libraries.

- Option: Move this out to the supported library.

  - Commitment Polices also handle the default algorithm.
    Given that the CMM is expected to be hold all configuration
    and be passed into encrypt/decrypt calls
    I don't know how this information could be recovered.
    If the library and not the CMM sets the default suite,
    this is a large breaking change.
    Also, the proper place to select the default suite,
    is in the CMM.

### Issue 8: How should we implement these keyrings and CMM interfaces?

- _Option: As an independent shared library._

  - By providing a single implementation
    customers get a single interface.
    But also, we have a mechanism
    to keep the various interfaces in sync.
    We are already investing in Dafny to verify our implementations.
    By using Dafny's portability we can compile the keyrings and CMMs
    into every required runtime.

- Option: Each library implements the specification

  - We need to maintain an implementation per library.
    Given that we _could_ include a single library,
    what value does this add?
    Also, any customer who needs to write a custom keyring
    must also port that implementation for each library.

### Issue 9: Where doe the Encryption Context serialization for Raw AES Keyring and Caching CMM logic live?

- _Option: Leave the specification as pointing to the Encryption SDK._

  - At this time there already exists a specification.
    Continuing to use the existing specification
    does not preclude a duplicate function.
    In fact, regardless there will be a duplicate function,
    at least in spirit since both the ESDK and MPL
    functions will exist and do the same thing.

    This is a two way door.
    At a later date another specification change
    could pull this function into the MPL.
    In the event of a new message format
    for the ESDK, this would likely be the case.

    This is also the case for the serialization of Encrypted Data Keys
    in the Caching CMM.

- Option: Add serialization function to MPL.

  - This function is needed for both the Caching CMM
    and the RawAES Keyring.
    So adding such a function is a requirement.
    Since the constraints for MPL is just ordering
    this function can be used by each supported library.

### Issue 10: How can we track transitive requirements for cryptographic implementations?

- Option: Incorporate the logic into MPL.

  - This simplifies the logic for supported libraries.
    But it may complicate back porting MPL
    into existing libraries.
    This is because they are already doing this.
    Having the function does not mean
    that they need to be used.

- Option: Implement them in the supported libraries
  - Keep these requirement separate,
    and use duvet to track their completion
    in the supported libraries.
    This is how they are currently handled.

## One-Way Doors

Separating the materials providers into a separate library
is a one-way door.
Once we publish this stand alone library
it will be possible to simply deprecate it.
This tight coupling of the material providers
with their consumers make the specification more complicated
and makes the material providers less composable.
However given the requirements of a given algorithm suite
and the security properties that it provides
this decreased compensability is a feature.
It makes the the Material Providers Library
hard to misuse.

## Impact

1. This tight coupling of MPL with its consumers
   make it more complicated to add or change features.
