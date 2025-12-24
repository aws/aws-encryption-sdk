[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS Encryption SDK Specification's tenets

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

# Background

The AWS Encryption SDK (ESDK) specification serves
as a large living design document.
These tenets are the "motivating force"
from which we derive the ESDK design.

We're always open to new ones
if you can think of better ones
and make a case for them.

# Priorities

When weighing difficult trade-offs
our ordered set of priorities are:

1. Security
1. Readability
1. Ease of use
1. Performance

# Principles

When making design decisions
these principles influence our choices.
New features SHOULD look to principles
to support adoption.

## The specification first

### Summary

We write changes in the specification
before they are developed in an implementation.

If we can not write down what a feature should do,
we can't have any confidence that it will do what we want.

Examples of recording
the requirements before development
include test-driven development
and [specification by example](https://en.wikipedia.org/wiki/Specification_by_example).

Other projects that have
informed how we approached this,
in no particular order:

- https://www.ietf.org/standards/rfcs/
- https://github.com/rust-lang/rfcs
- https://www.python.org/dev/peps/

### Motivation

The purpose of having a single specification
is to create alignment and accountability.
We describe what a feature is,
why such a feature is important,
and how such a feature should be used.
This is like Test-Driven Development
where we create the requirement
before we create the implementation.

We MUST communicate our requirements
in order to implement them.
No matter how careful we are,
there will be some ambiguity.
We resolve this ambiguity by
including why we made specific choices.

This drives us towards alignment
to keep the implementations the same.
As implementors find ambiguity
we use the written specification,
background, and intention
to keep us accountable.

This alignment goes beyond just
the inter-operability of the message format.
We document the AWS Encryption SDK (ESDK) as a single library.
We want to be able to describe
how the ESDK works independently of the implementation.
Clever or unique features in an implementation
are an opportunity for bugs.
They also work against inter-operability
because a unique behavior in one implementation
may be relied on by customers.
If this behavior does not exist
in other implementations,
these implementations are not inter-operable.

Well understood features
trend towards consistent ideas
as they change over time.
This benefits customers
because it produces better
and more stable implementations and documentation.

## Easy to use/Hard to misuse

### Summary

This principle is often referenced as one
but is two parts.
Discussing them individually helps
clarify what each one means
and how they reinforce each other.

## Easy to use

### Summary

Tools should be easy to use.
The form should lend itself to the function.
The way that these tools work together
should be obvious.

An example is furniture assembly.
Pre-drilled holes with nuts and bolts
make it clear both how and where
parts should be joined.
The shape of the joints
can also resolve left and right
or front and back.

The AWS Encryption SDK (ESDK) is a primitive that
is used to build larger software systems.
It is composed of parts and options.

Customers use these parts together
to accomplish their goals.
The ESDK MUST facilitate the customer goal,
not frustrate it.

### Motivation

Our priority is to
design parts that intuitively compose together.
[Design for assembly](https://en.wikipedia.org/wiki/Design_for_assembly)
emphasizes creating a functioning product from parts.

The ESDK is a part inside our customer's application.
Our APIs SHOULD explain to our customers
how they fit together.

The time our customers spend
trying to figure out how to use the ESDK
is just a transaction cost
that takes away from their goal.

## Hard to misuse

### Summary

Tools should be safe.
It should be clear to customers
if there are sharp edges and where.

An example of "hard to misuse"
is gas and water pipes.
In the US they have opposite threading.
This makes it very difficult to connect
a gas line to a water line accidentally.

The AWS Encryption SDK (ESDK) SHOULD be [easy to use](#easy-to-use)
but SHOULD NOT easily go together unsafely.

Customers should not be surprised
by unexpected or unsafe behavior.

### Motivation

Not every configuration is safe to use.
Expressiveness and power
can be at odds with safety.
Therefore, we actively discourage unsafe use.
This is another property you get with [design for assembly](https://en.wikipedia.org/wiki/Design_for_assembly).

This means we will decline to build features
if the danger of misuse is too high.
This is not simply a case of cost vs. benefit.
The downside risk of misuse SHOULD be evaluated
independent of the value of new features.

## Correct by construction

### Summary

Correctness can be evaluated
at different stages of construction.

1. Development

   During development the code
   as written may not satisfy the requirements.
   It is rarely the case that
   the code satisfies none of the requirements.
   The problem is more often
   that the code is only _mostly_ correct
   as opposed to **all** correct.

1. Initialization

   At runtime when components are configured,
   initialized, or created
   not all combinations are correct.

1. Use

   At runtime when the configured components
   are used dynamically
   during steady state operation,
   not all inputs are correct
   for every operation.

The AWS Encryption SDK (ESDK) is a component
that customers use to achieve their security requirements.
If the ESDK is not correct,
it is more difficult for customers
to reason about their security requirements.

Given a component that operates correctly,
customers want the configuration that is correct for them.
The above stages also apply to the use of the ESDK.

Again furniture assembly is an example.
By providing accurate pictures
of each step and the finished product
it is obvious after all the parts
have been assembled that the table is complete.

It is easier to verify
that a system is correct
when you're creating it,
because systems tend toward complexity
over time.
As systems evolve, small, seemingly minor, changes
can have unwanted emergent behavior.
It is unlikely that your system
will be _less_ complicated as time goes on.
This is true both for the ESDK
and for our customers using the ESDK.

### Motivation

The ESDK's priority is to
create interfaces that are correct by construction.
Our priority in evaluating correctness
and communicating incorrectness MUST be

1. Development
1. Initialization
1. Use

Early detection of errors
helps us deliver correct code to customers
but it also helps customers build
systems that have steady state stability.

Our customers MUST be able to
clearly express their requirements
in the construction of their calls to our APIs.

As a developer is integrating the ESDK into a project
they MUST be able to [easily](#easy-to-use) transform
a set of requirements into a implementation
that is obviously correct.

Similarly, reviewers or maintainers
SHOULD be able to understand
how the implementation satisfies the requirements.

Being able to easily compose the ESDK parts
to satisfy all of your requirements is not enough.
We expect a complete list of requirements
to have conflicting security needs for different types of data.
A single component that satisfies
multiple conflicting requirements
may be complete and expressive,
but it is also confusing
and is [easy to misuse](#hard-to-misuse).

Assembled components
should do one thing well.
Leaving the determination of
what requirement is being satisfied
to dynamic runtime behavior is suboptimal.
It is more difficult for customers
to reason about correct behavior
that can change between calls.

## Clear Inputs

### Summary

Functions take arguments.
What these arguments are
and how they are intended to be used
SHOULD be clear at the call site.
This is a specific use of
[correct by construction](#correct-by-construction)
and [easy to use](#easy-to-use).

Code is read much more often
than it is written
so we prioritize the expression of intent
for those who come after us.
By structuring clarity at the call site
we maintain the readability of our codebase.
By structuring our API
to encourage this same kind of clarity
we help customers express their intent
when using our tools.

Examples of features used to express clear inputs
at the call site are
named parameters, enums, and options mappings.

### Motivation

Although expressing intent at the call site
makes the AWS Encryption SDK (ESDK) more verbose,
it also makes function calls obviously correct by construction.

As a developer gets more experience
with a library they will remember
what parameters do what and how they are passed.
But this requires time and effort.

Due to the critical nature
of the data handled by the AWS Encryption SDK,
this extensive experience MUST NOT
be a requirement for review.
The reviewer, auditor,
or a later developer performing maintenance
SHOULD be able to easily understand
the intent of the parameters that are passed.

This is important because
while the complexity of systems generally increase,
there is no requirement that
the experience and understanding
of the people maintaining these system must increase.
Just because something is crystal clear to you today
does not mean it will be equally clear to you later,
let alone someone without your experience and understanding.

We express this in the specification
by naming options and encouraging enums.
But also our examples MUST use
expressive variable names
to help drive home the intention.

## Sensible Defaults

### Summary

Functions need parameters to know what actions to take.
Sometimes these parameters should have defaults.
A parameter that has a default
will take a specific value if no value is passed.

This simplifies using the function
because callers do not need
to select a value.

Sensible Defaults are defaults
that are optimal in the majority of use cases.
If a default is optimal in all use cases,
then it dominates all other options.
These kinds of dominant options
SHOULD NOT be configurable at all.

An example is the principle of least surprise.
Customers MUST have the freedom to configure
but SHOULD NOT need extensive domain understanding
to get started.

Defaults should guide customers towards
the best in class configurations that we know of.
As customers need to consider tradeoffs,
they can do so in an informed manner.
In this way another example might be
[nudge theory](https://en.wikipedia.org/wiki/Nudge_theory) from behavioral economics:
defaults can always be overridden,
but help to gently encourage improvements over time.

### Motivation

The AWS Encryption SDK (ESDK) aims
to be [easy to use](#easy-to-use).
Limiting the number of options
that are needed lowers the barrier to entry.
Sensible Defaults should be selected
where possible.

Given the relationship between encrypt and decrypt,
options are preferred on encrypt.
Once a message has been encrypted,
decryption of that message
SHOULD be gated by access control and correctness,
not configuration options.

Therefore since access control and correctness options
_must_ agree between encrypt and decrypt,
they SHOULD NOT have defaults
and SHOULD be set at encryption time.
Because if these defaults need to change
the ESDK would no longer be backwards compatible.
Sensible defaults MAY change
but MUST NOT change in ways
that break backwards compatibility.

A example of a correctness option
that MUST NOT have a default value
is the [padding scheme](framework/raw-rsa-keyring.md#padding-scheme)
for wrapping the data key
in the raw RSA keyring.
The padding scheme is not stored
and MUST be configured to the same value for decrypt.
There will generally be an optimal security answer
at any given point in time.

As time progresses however,
this answer will change.
If we provided such a default,
changing it
will break existing customers.
Because the padding value is not stored
with the wrapped data key.
This break will be not be obvious,
because the customer never set this value before.

This burns trust
and forces customers to explicitly set a value,
which defeats the purpose of having a default.
Therefore, we do not provide defaults
for such configurations.
