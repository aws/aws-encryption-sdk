[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Keystore

## Version

0.2.0

### Changelog

- 0.2.0
  - Update keystore structure and add encryption context option
- 0.1.0
  - Initial record

## Implementations

| Language | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation |
| -------- | -------------------------------------- | ------------------------- | -------------- |

## Overview

A Keystore persists hierarchical data that allows customers to call AWS KMS less often.
The Keystore persists branch keys in DynamoDb that wrap multiple data keys.
This creates a hierarchy where a branch key wraps multiple data keys and facilitates caching.
These branch keys are only generated using the [AWS KMS API GenerateDataKeyWithoutPlaintext](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyWithoutPlaintext.html).

This Keystore interface defines operations that any implementation of its specification must support and implement.

## Definitions

- [Branch Key(s)](../structures.md#branch-key): Data keys that are reused to wrap unique data keys for envelope encryption.
  For security considerations on when to rotate the branch key, refer to [Appendix B](#appendix-b-security-considerations-for-branch-key-rotation).
- [Beacon Key(s)](https://github.com/awslabs/aws-database-encryption-sdk-dynamodb-java/blob/main/specification/searchable-encryption/beacons.md#beacons):
  A root key used to then derive different beacon keys per beacon.
- [UUID](https://www.ietf.org/rfc/rfc4122.txt): a universally unique identifier that can be represented as a byte sequence or a string.

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Initialization

The following inputs MAY be specified to create a KeyStore:

- [ID](#keystore-id)
- [AWS KMS Grant Tokens](#aws-kms-grant-tokens)
- [DynamoDb Client](#dynamodb-client)
- [KMS Client](#kms-client)

The following inputs MUST be specified to create a KeyStore:

- [Table Name](#table-name)
- [AWS KMS Configuration](#aws-kms-configuration)
- [Logical KeyStore Name](#logical-keystore-name)

### Keystore ID

The Identifier for this KeyStore.
If one is not supplied, then a [version 4 UUID](https://www.ietf.org/rfc/rfc4122.txt) MUST be used.

### AWS KMS Grant Tokens

A list of AWS KMS [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).

### DynamoDb Client

The DynamoDb Client used to put and get keys from the backing DDB table.

If not provided, one will be created in the region of the supplied KMS Key ARN

### KMS Client

The KMS Client used when wrapping and unwrapping keys.

If not provided, one will be created in the region of the supplied KMS Key ARN

On initialization the KeyStore MUST append a user agent string to the AWS KMS SDK Client with the
value `aws-kms-hierarchy`.

### Table Name

The table name of the DynamoDb table that backs this Keystore.

### AWS KMS Configuration

A valid [AWS KMS Key ARN](./aws-kms/aws-kms-key-arn.md#a-valid-aws-kms-arn)
that wraps and unwraps keys stored in Amazon DynamoDB.

### Logical KeyStore Name

This name is cryptographically bound to all data stored in this table,
and logically separates data between different tables.

The logical keystore name MUST be bound to every created key.

There needs to be a one to one mapping between DynamoDB Table Names and the Logical KeyStore Name.
This value can be set to the DynamoDB table name itself, but does not need to.

Controlling this value independently enables restoring from DDB table backups
even when the table name after restoration is not exactly the same.

## Operations

The Keystore MUST support the following operations:

- [GetKeyStoreInfo](#getKeyStoreInfo)
- [CreateKeyStore](#createkeystore)
- [CreateKey](#createkey)
- [VersionKey](#versionkey)
- [GetActiveBranchKey](#getactivebranchkey)
- [GetBranchKeyVersion](#getbranchkeyversion)
- [GetBeaconKey](#beacon-key)

### GetKeyStoreInfo

This operation MUST return the keystore information in this keystore configuration.

This MUST include:

- [keystore id](#keystore-id)
- [keystore name](#table-name)
- [logical Keystore name](#logical-keystore-name)
- [AWS KMS Grant Tokens](#aws-kms-grant-tokens)
- [AWS KMS Configuration](#aws-kms-configuration)

### CreateKeyStore

This operation MUST first calls the DDB::DescribeTable API with the configured `tableName`.

If the response is successful, this operation validates that the table has the expected
[KeySchema](#keyschema) as defined below.
If the [KeySchema](#keyschema) does not match
this operation MUST yield an error.
The table MAY have additional information,
like GlobalSecondaryIndex defined.

If the client responds with a `ResourceNotFoundException`,
then this operation MUST continue and
MUST call [AWS DDB CreateTable](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_CreateTable.html)
with the following specifics:

- TableName is the configured tableName.
- [KeySchema](#keyschema) as defined below.

If the operation fails to create table, the operation MUST fail.

If the operation successfully creates a table, the operation MUST return the AWS DDB Table Arn
back to the caller.

#### KeySchema

The following KeySchema MUST be configured on the table:

| AttributeName | KeyType   | Type |
| ------------- | --------- | ---- |
| branch-key-id | Partition | S    |
| type          | Sort      | S    |

### CreateKey

The CreateKey caller MUST provide:

- An optional branch key id
- An optional encryption context

If an optional branch key id is provided
and no encryption context keys are provided this operation MUST fail.

If no branch key id is provided,
then this operation MUST create a [version 4 UUID](https://www.ietf.org/rfc/rfc4122.txt)
to be used as the branch key id.

This operation MUST create a [branch key](#branch-key) and a [beacon key](#beacon-key) according to
the [Branch Key and Beacon Key Creation](#branch-key-and-beacon-key-creation) section.

If creation of the keys are successful,
the operation MUST call Amazon DynamoDB TransactWriteItems according to the [write key material](#writing-branch-key-and-beacon-key-to-key-store) section.

If writing to the keystore succeeds,
the operation MUST return the branch-key-id that maps to both
the branch key and the beacon key.

Otherwise, this operation MUST yield an error.

#### Branch Key and Beacon Key Creation

To create a branch key, this operation MUST take the following:

- `branchKeyId`: The identifier
- `encryptionContext`: Additional encryption context to bind to the created keys

This operation needs to generate the following:

- `version`: a new guid. This guid MUST be [version 4 UUID](https://www.ietf.org/rfc/rfc4122.txt)
- `timestamp`: a timestamp for the current time.
  This timestamp MUST be in ISO 8601 format in UTC, to microsecond precision (e.g. “YYYY-MM-DDTHH:mm:ss.ssssssZ“)

The wrapped Branch Keys, DECRYPT_ONLY and ACTIVE, MUST be created according to [Wrapped Branch Key Creation](#wrapped-branch-key-creation).

To create a beacon key, this operation will continue to use the `branchKeyId` and `timestamp` as the [Branch Key](#branch-key).

The operation MUST call [AWS KMS API GenerateDataKeyWithoutPlaintext](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyWithoutPlaintext.html).
The call to AWS KMS GenerateDataKeyWithoutPlaintext MUST use the configured AWS KMS client to make the call.
The operation MUST call AWS KMS GenerateDataKeyWithoutPlaintext with a request constructed as follows:

- `KeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `NumberOfBytes` MUST be 32.
- `EncryptionContext` MUST be the [encryption context for beacon keys](#beacon-key-encryption-context).
- `GrantTokens` MUST be this keystore's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).

If the call to AWS KMS GenerateDataKeyWithoutPlaintext succeeds,
the operation MUST use the `CiphertextBlob` as the wrapped Beacon Key.

#### Wrapped Branch Key Creation

Given a `branchKeyId`, `version` and `timestamp`

The operation MUST call [AWS KMS API GenerateDataKeyWithoutPlaintext](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyWithoutPlaintext.html).
The call to AWS KMS GenerateDataKeyWithoutPlaintext MUST use the configured AWS KMS client to make the call.
The operation MUST call AWS KMS GenerateDataKeyWithoutPlaintext with a request constructed as follows:

- `KeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `NumberOfBytes` MUST be 32.
- `EncryptionContext` MUST be the [DECRYPT_ONLY encryption context for branch keys](#decrypt_only-encryption-context).
- GenerateDataKeyWithoutPlaintext `GrantTokens` MUST be this keystore's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).

If the call to AWS KMS GenerateDataKeyWithoutPlaintext succeeds,
the operation MUST use the GenerateDataKeyWithoutPlaintext result `CiphertextBlob`
as the wrapped DECRYPT_ONLY Branch Key.

The operation MUST call [AWS KMS API ReEncrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_ReEncrypt.html)
with a request constructed as follows:

- `SourceEncryptionContext` MUST be the [DECRYPT_ONLY encryption context for branch keys](#decrypt_only-encryption-context).
- `SourceKeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `CiphertextBlob` MUST be the wrapped DECRYPT_ONLY Branch Key.
- ReEncrypt `GrantTokens` MUST be this keystore's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).
- `DestinationKeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `DestinationEncryptionContext` MUST be the [ACTIVE encryption context for branch keys](#active-encryption-context).

If the call to AWS KMS ReEncrypt succeeds,
the operation MUST use the ReEncrypt result `CiphertextBlob`
as the wrapped ACTIVE Branch Key.

#### Writing Branch Key and Beacon Key to Keystore

To add the branch keys and a beacon key to the keystore the
operation MUST call [Amazon DynamoDB API TransactWriteItems](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactWriteItems.html).
The call to Amazon DynamoDB TransactWriteItems MUST use the configured Amazon DynamoDB Client to make the call.
The operation MUST call Amazon DynamoDB TransactWriteItems with a request constructed as follows:

List of TransactWriteItem:

- PUT:
  - Item:
    - “branch-key-id” (S): `branchKeyId`,
    - “type“ (S): "branch:version:" + `version`,
    - “enc” (B): the wrapped DECRYPT_ONLY Branch Key `CiphertextBlob` from the KMS operation
    - “create-time” (S): `timestamp`
    - "kms-arn" (S): configured `KMS Key ARN`
    - “hierarchy-version” (N): 1
  - ConditionExpression: `attribute_not_exists(branch-key-id)`
  - TableName: the configured Table Name
- PUT:
  - Item:
    - “branch-key-id” (S): `branchKeyId`,
    - “type“ (S): "branch:ACTIVE",
    - “enc” (B): wrapped ACTIVE Branch Key `CiphertextBlob` from the KMS operation
    - “create-time” (S): `timestamp`
    - "kms-arn" (S): configured `KMS Key ARN`
    - “hierarchy-version” (N): 1
  - ConditionExpression: `attribute_not_exists(branch-key-id)`
  - TableName: the configured Table Name
- PUT:
  - Item:
    - “branch-key-id” (S): `branchKeyId`,
    - “type“ (S): "beacon:ACTIVE",
    - “enc” (B): the wrapped Beacon Key `CiphertextBlob` from the KMS operation
    - “create-time” (S): `timestamp`
    - "kms-arn" (S): configured `KMS Key ARN`
    - “hierarchy-version” (N): 1
  - ConditionExpression: `attribute_not_exists(branch-key-id)`
  - TableName is the configured Table Name

TransactWriteItemRequest:

- TransactWriteItems: List of TransactWriteItem

If DDB TransactWriteItems is successful, this operation MUST return a successful response containing no additional data.
Otherwise, this operation MUST yield an error.

### VersionKey

On invocation, the caller:

- MUST supply a `branch-key-id`

VersionKey MUST first get the active version for the branch key from the keystore
by calling AWS DDB `GetItem`
using the `branch-key-id` as the Partition Key and `"branch:ACTIVE"` value as the Sort Key.

The values on the AWS DDB response item
MUST be authenticated according to [authenticating a keystore item](#authenticating-a-keystore-item).
If the item fails to authenticate this operation MUST fail.

The wrapped Branch Keys, DECRYPT_ONLY and ACTIVE, MUST be created according to [Wrapped Branch Key Creation](#wrapped-branch-key-creation).

To add the new branch key to the keystore,
the operation MUST call [Amazon DynamoDB API TransactWriteItems](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactWriteItems.html).
The call to Amazon DynamoDB TransactWriteItems MUST use the configured Amazon DynamoDB Client to make the call.
The operation MUST call Amazon DynamoDB TransactWriteItems with a request constructed as follows:

List of TransactWriteItem:

- PUT:
  - Item:
    - “branch-key-id” (S): `branchKeyId`,
    - “type“ (S): "branch:version:" + `version`,
    - “enc” (B): the wrapped DECRYPT_ONLY Branch Key `CiphertextBlob` from the KMS operation
    - “create-time” (S): `timestamp`
    - "kms-arn" (S): configured `KMS Key ARN`
    - “hierarchy-version” (N): 1
  - ConditionExpression: `attribute_not_exists(branch-key-id)`
  - TableName: the configured Table Name
- PUT:
  - Item:
    - “branch-key-id” (S): `branchKeyId`,
    - “type“ (S): "branch:ACTIVE",
    - “enc” (B): wrapped ACTIVE Branch Key `CiphertextBlob` from the KMS operation
    - “create-time” (S): `timestamp`
    - "kms-arn" (S): configured `KMS Key ARN`
    - “hierarchy-version” (N): 1
  - ConditionExpression: `attribute_exists(branch-key-id)`
  - TableName: the configured Table Name

TransactWriteItemRequest:

- TransactWriteItems: List of TransactWriteItem

If DDB TransactWriteItems is successful, this operation MUST return a successful response containing no additional data.
Otherwise, this operation MUST yield an error.

#### Authenticating a Keystore item

The operation MUST use the configured `KMS SDK Client` to authenticate the value of the keystore item.

Every attribute on the AWS DDB response item will be authenticated.

Every key in the constructed [encryption context](#encryption-context)
except `tableName`
MUST exist as a string attribute in the AWS DDB response item.
Every value in the constructed [encryption context](#encryption-context)
except the logical table name
MUST equal the value with the same key in the AWS DDB response item.
The key `enc` MUST NOT exist in the constructed [encryption context](#encryption-context).

The operation MUST call [AWS KMS API ReEncrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_ReEncrypt.html)
with a request constructed as follows:

- `SourceEncryptionContext` MUST be the [encryption context](#encryption-context) constructed above
- `SourceKeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `CiphertextBlob` MUST be the `enc` attribute value on the AWS DDB response item
- `GrantTokens` MUST be the configured [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).
- `DestinationKeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `DestinationEncryptionContext` MUST be the [encryption context](#encryption-context) constructed above

### GetActiveBranchKey

On invocation, the caller:

- MUST supply a `branch-key-id`

To get the active version for the branch key id from the keystore
this operation MUST call AWS DDB `GetItem`
using the `branch-key-id` as the Partition Key and `"branch:ACTIVE"` value as the Sort Key.

The AWS DDB response MUST contain the fields defined in the [branch keystore record format](#record-format).
If the record does not contain the defined fields, this operation MUST fail.

The operation MUST decrypt the branch key according to the [AWS KMS Branch Key Decryption](#aws-kms-branch-key-decryption) section.

If the branch key fails to decrypt, GetActiveBranchKey MUST fail.

This GetActiveBranchKey MUST construct [branch key materials](./structures.md#branch-key-materials)
according to [Branch Key Materials From Authenticated Encryption Context](#branch-key-materials-from-authenticated-encryption-context).

This operation MUST return the constructed [branch key materials](./structures.md#branch-key-materials).

### GetBranchKeyVersion

On invocation, the caller:

- MUST supply a `branch-key-id`
- MUST supply a `branchKeyVersion`

To get a branch key from the keystore this operation MUST call AWS DDB `GetItem`
using the `branch-key-id` as the Partition Key and "branch:version:" + `branchKeyVersion` value as the Sort Key.

The AWS DDB response MUST contain the fields defined in the [branch keystore record format](#record-format).
If the record does not contain the defined fields, this operation MUST fail.

The operation MUST decrypt the branch key according to the [AWS KMS Branch Key Decryption](#aws-kms-branch-key-decryption) section.

If the branch key fails to decrypt, this operation MUST fail.

This GetBranchKeyVersion MUST construct [branch key materials](./structures.md#branch-key-materials)
according to [Branch Key Materials From Authenticated Encryption Context](#branch-key-materials-from-authenticated-encryption-context).

This operation MUST return the constructed [branch key materials](./structures.md#branch-key-materials).

### GetBeaconKey

On invocation, the caller:

- MUST supply a `branch-key-id`

To get a branch key from the keystore this operation MUST call AWS DDB `GetItem`
using the `branch-key-id` as the Partition Key and "beacon:ACTIVE" value as the Sort Key.

The AWS DDB response MUST contain the fields defined in the [branch keystore record format](#record-format).
If the record does not contain the defined fields, this operation MUST fail.

The operation MUST decrypt the beacon key according to the [AWS KMS Branch Key Decryption](#aws-kms-branch-key-decryption) section.

If the beacon key fails to decrypt, this operation MUST fail.

This GetBeaconKey MUST construct [beacon key materials](./structures.md#beacon-key-materials) from the decrypted branch key material
and the `branchKeyId` from the returned `branch-key-id` field.

This operation MUST return the constructed [beacon key materials](./structures.md#beacon-key-materials).

## Encryption Context

This section describes how the AWS KMS encryption context is built
from the DynamoDB items that store the branch keys.

The following encryption context keys are shared:

- MUST have a `branch-key-id` attribute
- The `branch-key-id` field MUST not be an empty string
- MUST have a `type` attribute
- The `type` field MUST not be an empty string
- MUST have a `create-time` attribute
- MUST have a `tablename` attribute to store the logicalKeyStoreName
- MUST have a `kms-arn` attribute
- MUST have a `hierarchy-version`
- MUST NOT have a `enc` attribute

Any additionally attributes on the DynamoDB item
MUST be added to the encryption context.

### ACTIVE Encryption Context

The ACTIVE branch key is a copy of the DECRYPT_ONLY with the same `version`.
It is structured slightly differently so that the active version can be accessed quickly.

In addition to the [encryption context](#encryption-context):

The ACTIVE encryption context value of the `type` attribute MUST equal to `"branch:ACTIVE"`.
The ACTIVE encryption context MUST have a `version` attribute.
The `version` attribute MUST store the branch key version formatted like `"branch:version:"` + `version`.

### DECRYPT_ONLY Encryption Context

In addition to the [encryption context](#encryption-context):

The DECRYPT_ONLY encryption context MUST NOT have a `version` attribute.
The `type` attribute MUST stores the branch key version formatted like `"branch:version:"` + `version`.

### Beacon Key Encryption Context

In addition to the [encryption context](#encryption-context):

The Beacon key encryption context value of the `type` attribute MUST equal to `"beacon:ACTIVE"`.
The Beacon key encryption context MUST NOT have a `version` attribute.

### Custom Encryption Context

If custom [encryption context](./structures.md#encryption-context-3)
is associated with the branch key these values MUST be added to the AWS KMS encryption context.
To avoid name collisions each added attribute from the custom [encryption context](./structures.md#encryption-context-3)
MUST be prefixed with `aws-crypto-ec:`.
The added values MUST be equal.

## AWS KMS Branch Key Decryption

The operation MUST use the configured `KMS SDK Client` to decrypt the value of the branch key field.

Every attribute except for `enc` on the AWS DDB response item
MUST be authenticated in the decryption of `enc`

Every key in the constructed [encryption context](#encryption-context)
except `tableName`
MUST exist as a string attribute in the AWS DDB response item.
Every value in the constructed [encryption context](#encryption-context)
except the logical table name
MUST equal the value with the same key in the AWS DDB response item.
The key `enc` MUST NOT exist in the constructed [encryption context](#encryption-context).

When calling [AWS KMS Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html),
the keystore operation MUST call with a request constructed as follows:

- `KeyId` MUST be the configured `AWS KMS Key ARN` in the [AWS KMS Configuration](#aws-kms-configuration) for this keystore
- `CiphertextBlob` MUST be the `enc` attribute value on the AWS DDB response item
- `EncryptionContext` MUST be the [encryption context](#encryption-context) constructed above
- `GrantTokens` MUST be this keystore's [grant tokens](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).

## Record Format

A branch key record MUST include the following key-value pairs:

1. `branch-key-id` : Unique identifier for a branch key; represented as [AWS DDB String](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
1. `type` : One of the following; represented as [AWS DDB String](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
   - The string literal `"beacon:ACTIVE"`. Then `enc` is the wrapped beacon key.
   - The string `"branch:version:"` + `version`, where `version` is the Branch Key Version. Then `enc` is the wrapped branch key.
   - The string literal `"branch:ACTIVE"`. Then `enc` is the wrapped beacon key of the active version. Then
1. `version` : Only exists if `type` is the string literal `"branch:ACTIVE"`.
   Then it is the Branch Key Version. represented as [AWS DDB String](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
1. `enc` : Encrypted version of the key;
   represented as [AWS DDB Binary](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
1. `kms-arn`: The AWS KMS Key ARN used to generate the `enc` value.
   represented as [AWS DDB String](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
1. `create-time`: Timestamp in ISO 8601 format in UTC, to microsecond precision.
   Represented as [AWS DDB String](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)
1. `hierarchy-version`: Version of the hierarchical keyring;
   represented as [AWS DDB Number](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes)

A branch key record MAY include [custom encryption context](#custom-encryption-context) key-value pairs.
These attributes should be prefixed with `aws-crypto-ec:` the same way they are for [AWS KMS encryption context](#encryption-context).

### Branch Key Materials From Authenticated Encryption Context

The `type` attribute MUST either be equal to `"branch:ACTIVE"` or start with `"branch:version:"`.

If the `type` attribute is equal to `"branch:ACTIVE"`
then the authenticated encryption context MUST have a `version` attribute
and the version string is this value.
If the `type` attribute start with `"branch:version:"` then the version string MUST be equal to this value.

To construct [branch key materials](./structures.md#branch-key-materials) from authenticated encryption context as follows:

- [Branch Key](./structures.md#branch-key) MUST be the [decrypted branch key material](#aws-kms-branch-key-decryption)
- [Branch Key Id](./structures.md#branch-key-id) MUST be the `branch-key-id`
- [Branch Key Version](./structures.md#branch-key-version)
  The version string MUST start with `branch:version:`.
  The remaining string encoded as UTF8 bytes MUST be the Branch Key version.
- [Encryption Context](./structures.md#encryption-context-3) MUST be constructed by
  [Custom Encryption Context From Authenticated Encryption Context](#custom-encryption-context-from-authenticated-encryption-context)

### Custom Encryption Context From Authenticated Encryption Context

The custom encryption context is stored as map of UTF8 Encoded bytes.

For every key in the [encryption context](./structures.md#encryption-context-3)
the string `aws-crypto-ec:` + the UTF8 decode of this key
MUST exist as a key in the authenticated encryption context.
Also, the value in the [encryption context](./structures.md#encryption-context-3) for this key
MUST equal the value in the authenticated encryption context
for the constructed key.

### Example

Given the simplified [branch key material](./structures.md#branch-key-materials) structure

```dafny
BranchKeyMaterials(
  branchKey := [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  branchKeyId := "bbb9baf1-03e6-4716-a586-6bf29995314b",
  branchKeyVersion := "83eec007-5659-4554-bf11-699b90f41ac6",
  encryptionContext := [ "department" := "admin"]
)
```

There would be three items in the keystore table.
The DECRYPT_ONLY version, the ACTIVE version, and a beacon key.

The DECRYPT_ONLY simplified JavaScript JSON format would look like this

```
{
  "branch-key-id" : "bbb9baf1-03e6-4716-a586-6bf29995314b",
  "type" : "branch:version:83eec007-5659-4554-bf11-699b90f41ac6",
  "enc" : "NnYwxJ/oiQCLnqRh/IcrCR2mmOnO4SAVLw2pspKJKd6rpa0H8z/4hGpGxcWozdb7VByebDFWb0VTWxaOUA8=",
  "kms-arn" : "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab",
  "create-time" : "2023-06-03T19:03:29.358Z",
  "hierarchy-version" : "1",
  "aws-crypto-ec:department" : "admin",
}
```

The ACTIVE simplified JavaScript JSON format would look like this

```
{
  "branch-key-id" : "bbb9baf1-03e6-4716-a586-6bf29995314b",
  "type" : ""branch:ACTIVE",
  "version": "branch:version:83eec007-5659-4554-bf11-699b90f41ac6"
  "enc" : "BiXHTm0j27+jsgJZ7yCnvI6yvjFyStMsHiC8fnR9KzKjwwhi0gB+5CZTfXFC2ufmBtCYX/sLvKsFnEITR+k=",
  "kms-arn" : "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab",
  "create-time" : "2023-06-03T19:03:29.358Z",
  "hierarchy-version" : "1",
  "aws-crypto-ec:department" : "admin",
}
```

The BEACON simplified JavaScript JSON format would look like this

```
{
  "branch-key-id" : "bbb9baf1-03e6-4716-a586-6bf29995314b",
  "type" : "beacon:ACTIVE",
  "enc" : "hgb2RyDQinOCpzKWdi17E+t9WB9pRExQXpD/20bsu9hxr38HjQvGvihoYpL6sKuF0Ek+37B1UE9tK3SIOiE=",
  "kms-arn" : "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab",
  "create-time" : "2023-06-03T19:03:29.358Z",
  "hierarchy-version" : "1",
  "aws-crypto-ec:department" : "admin",
}
```
