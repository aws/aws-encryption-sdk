[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Definitions

## Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Specification Document

A single document that defines a single feature.
Ex: [Keyring interface](./framework/keyring-interface.md).

## Specification

The collection of all specification documents contained within this repository.

# Versioning Policy

We follow [Semantic Versioning](https://semver.org/).

Given a version number MAJOR.MINOR.PATCH, increment the:

- MAJOR version when you make incompatible API changes,
- MINOR version when you add functionality in a backwards compatible manner, and
- PATCH version when you make backwards compatible bug fixes.

## Beta Releases

Versions with MAJOR version 0 (0.MINOR.PATCH) are beta releases.
Beta releases MAY introduce MAJOR changes in a MINOR version increment.

# Overall

The overall specification,
defined as the collection of all specification documents,
is not versioned.

# Specification Documents

Each specification document MUST define its version.
This version only applies to the contents of that document.

## Changelogs

Each specification document MUST contain a changelog
that records the changes made to each version of the document.

## Implementations

Each specification document SHOULD maintain a listing of
implementations that have been known to comply with
a version of that specification document.
Each implementation listing MUST identify the language
as well as provide a link to a file in the implementation
that states what version of the specification document
it implements.

NOTE:
The presence of a reference to an implementation does not imply that
every listed implementation complies with the current version of
the specification document that contains that reference.
