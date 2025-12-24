[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Encryption context values that are authenticated but not stored with the encrypted message.

## Affected Features

This serves as a reference of all features that this change affects.

| Feature                                         |
| ----------------------------------------------- |
| [Keystore](../../framework/branch-key-store.md) |
| [Structures](../../framework/structures.md)     |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                                           |
| --------------------------------------------------------------------------------------- |
| [Keystore](../../framework/branch-key-store.md)                                         |
| [Structures](../../framework/structures.md)                                             |
| [AWS KMS Hierarchical Keyring](../../framework/aws-kms/aws-kms-hierarchical-keyring.md) |

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

Inserting a record into the keystore table should correct by construction.
The current design relied too heavily on GUID for correctness of the data in the table.
A duplicate branch key id has the potential to create confusion
about what data is encrypted under what AWS KMS key.
By changing the data model we ensure that global unique guarantees
are tied to the data model and reflected in the branch key structure.

## Out of Scope

- Migration from the developer preview version
- Local policy evaluation for use of branch keys

## Motivation

Customers were very interested in defining their own branch key ids.
As they implemented their own branch key creation process
this lead them to ask
“how can I ensure that there does not already exist a branch key for this id?”
This illustrated that the design relied too heavily on GUID for correctness.

The fact that they were willing to invest in their own code
to manage branch key id creation illustrated that this is an important feature.
It is likely that they would have tried to use private function
to try and maintain forward compatibility with upstream changes.

The goal then is to simplify the structure and offer the feature.

## Operational Implications

This change will invalidate existing keystore tables.
Given that the current code is in developer preview
this is an acceptable risk.
The library is documented as not ready for production workloads.

## Reference-level Explanation

### Remove the Global Secondary Index

on [CreateKeyStore](../../framework/branch-key-store.md#createkey).

This is no longer used and does not need to exist.

### New input branch-key-id and encryption context

on [CreateKey](../../framework/branch-key-store.md#createkey)

The input needs to be able to take two new optional inputs

- branch key id
- encryption context

It needs to be able to add the new encryption context
to the branch key item.
As well as require that a custom branch key id
also requires additional encryption context for validation.

### Update the creation logic to insert 3 records

on [CreateKey](../../framework/branch-key-store.md#createkey)

The active and the version items as well as the beacon item
need to be inserted.

### New section Wrapped Branch Key Creation

Add a new section to specify how a key is created.
This can be used both on branch key creation
but also on versioning.
Since in that case a new version key is also created.

### Add ConditionExpression to writing to DDB

on both [Writing Branch Key and Beacon Key to Keystore](../../framework/branch-key-store.md#writing-branch-key-and-beacon-key-to-keystore)
and [VersionKey](../../framework/branch-key-store.md#versionkey).

When writing a new key, all 3 records MUST NOT exist.
When versioning a key, the active record MUST already exist,
and the new version MUST NOT exist.

### Update the fixed constants and prefixes for DDB attributes

Updating fixed constants to`beacon:ACTIVE` and `branch:ACTIVE`
it is very clear what they are.
This namespaces other values that may need to be associated with the key.

By updating the version prefix from `version:` to `branch:version:`
this ties the record more closely to the branch keys.

### Add a new prefix for encryption context for DDB attributes

`aws-crypto-ec:` for any custom encryption context added to the branch key on creation.

### Add example records

Examples are helpful to visualize what the data should look like.

### Update the Hierarchical Keyring to use branch material

in [AWS KMS Hierarchical Keyring](../../framework/aws-kms/aws-kms-hierarchical-keyring.md#query-branch-keystore-onencrypt)
expects the keystore to return bytes.
However the keystore specification returns branch key materials.

### New property encryption context

on [Branch Key Materials](../../framework/structures.md#branch-key-materials).
Given that the materials have not authenticated these values
they should be stored on the materials.

### Add information about optional Beacon Key.

The beacon key is optional
because it can then be removed
after being used to derive HMAC Keys.
Adding clarification to make this clear.
