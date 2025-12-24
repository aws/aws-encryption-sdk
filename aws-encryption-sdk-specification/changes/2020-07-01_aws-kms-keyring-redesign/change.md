[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Interacting with AWS KMS using the AWS Encryption SDK (Change)

## Affected Features

This serves as a reference of all features that this change affects.

| Features                                                                                                                                              |
| ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| [AWS KMS Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md) |
| [AWS KMS Discovery Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/issues/84)                                                    |
| [Split AWS KMS Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/issues/83)                                                        |
| [AWS KMS Keyring Error Handling](https://github.com/awslabs/aws-encryption-sdk-specification/issues/40)                                               |
| [Client Suppliers](https://github.com/awslabs/aws-encryption-sdk-specification/issues/94)                                                             |
| [Generators](https://github.com/awslabs/aws-encryption-sdk-specification/issues/49)                                                                   |
| [Limiting network calls to specific AWS regions](https://github.com/awslabs/aws-encryption-sdk-specification/issues/90)                               |
| [AWS SDK KMS service client caching](https://github.com/awslabs/aws-encryption-sdk-specification/issues/16)                                           |
| [AWS SDK KMS service client initialization/configuration](https://github.com/awslabs/aws-encryption-sdk-specification/issues/15)                      |
| [AWS SDK KMS service client user agent string](https://github.com/awslabs/aws-encryption-sdk-specification/issues/59)                                 |
| [Passing provider info to the AWS KMS Decrypt API Call](https://github.com/awslabs/aws-encryption-sdk-specification/issues/139)                       |

## Affected Specifications

This serves as a reference of all specification documents that this change affects.

| Specification                                                 |
| ------------------------------------------------------------- |
| [AWS KMS Keyring](../../framework/aws-kms/aws-kms-keyring.md) |

## Affected Implementations

This serves as a reference for all implementations that this change affects.

| Language   | Implementation                                                                        |
| ---------- | ------------------------------------------------------------------------------------- |
| C          | [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c)                   |
| Javascript | [aws-encryption-sdk-javascript](https://github.com/aws/aws-encryption-sdk-javascript) |

This change additionally affects the unreleased AWS KMS keyring implementations for Java and Python.

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Summary

The AWS Encryption SDK’s (ESDK’s) [AWS KMS Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md) has become increasingly complex
as it [supports various customer use cases](background.md).
We are separating the functionality of the AWS KMS keyring into multiple smaller-scoped keyrings.
Each new keyring maintains a smaller API surface area.

By simplifying the AWS KMS keyring, we establish the basic building blocks for interacting with AWS KMS.
We can support additional customer use cases by writing operations to configure a multi-keyring with these smaller-scoped keyrings.
In the future, new features will be supported without refactoring code or updating customer-facing APIs.
Advanced customers can also follow a similar process for more complex customization.
This lowers the likelihood that advanced customers fork ESDK code.

## Out of Scope

1. Any customer interactions with the ESDK that are unrelated to AWS KMS
2. Keyrings other than the AWS KMS keyring
   1. We rely on the functionality of the [multi-keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/framework/multi-keyring.md),
      but the multi-keyring is out of scope/unchanged
3. [AWS KMS asymmetric CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks)

## Motivation

1. **Move logic out of the AWS KMS keyring** so a single keyring does not handle
   multiple AWS KMS [CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys)
   ([key names](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#key-names)),
   multiple AWS regions,
   or complex functionality
   ([discovery](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#is-discovery),
   limiting communication to specific AWS regions,
   AWS SDK client initialization, etc.).
   The smaller scope of the new keyrings reduces the risk of bugs during both ESDK development and ESDK usage.
   This makes them easy to use, hard to misuse.
2. **Create the basic building blocks for interacting with AWS KMS**
   by creating an AWS KMS symmetric keyring and a separate AWS KMS symmetric region discovery keyring
   (see [Reference-level Explanation](#reference-level-explanation)).
   New features are supported by writing operations that configure one or more of these smaller-scoped keyrings.
   These operations require us to use our own APIs,
   and also serve as examples to show customers how we use the basic building blocks for more complex use cases.
   This lowers the risk for one-way doors and makes the customer-facing API easy to use, hard to misuse.
3. **Move AWS SDK client initialization/configuration logic outside of the keyring API runtime**.
   This lowers the potential for keyring API runtime failures.
   There is clear intent and knowledge of _which_ keyring is using _which_ AWS SDK client
   since keyrings are explicitly configured with an AWS SDK client.
   This makes AWS SDK clients easy to use, hard to misuse.
4. **Support additional customer use cases** that cannot be supported today without significant refactors.
   For example, we now support region and key name ordering when _attempting_ decryption.
   For new supported use cases and pseudocode examples, see [Background](background.md).
   We can continue developing the ESDK with an “easy to use, hard to misuse” mindset.

## Drawbacks

We present a different development style from the existing AWS KMS keyring.
Rather than having a single keyring maintain all AWS KMS logic, logic is broken up into multiple keyrings.
This will require additional documentation
and updated usage examples.

We no longer support a _discovery_ keyring that _attempts_ decryption for any encrypted data key in any/all AWS regions
without additional configurations.
If customers need a keyring that _attempts_ decryption in all AWS regions,
they SHOULD call a service/API to get an updated list of AWS regions.
They can then use this list with the [keyring-producing operations defined below](#keyring-producing-operations).
This will prevent any ESDK or AWS SDK region-list from becoming stale over time.
In most cases, customers SHOULD simply use the keyring-producing operation for the specific AWS region(s) they want to communicate with.
This will allow customers to add additional AWS regions over time,
without allowing access to AWS regions that are not currently required.
This additionally reinforces customer intent.

This is a one-way door, but allows for increased flexibility and significantly reduces the risk of future refactors and deprecations.

## Security Implications

We reduce the API surface area of the current AWS KMS keyring.

[Client suppliers](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#client-supplier)
are removed from the AWS KMS keyring.
This includes client supplier logic for limiting communication to specific AWS regions.
This removes the security decision of _which_ AWS regions a keyring can communicate with
from the keyring’s OnEncrypt/OnDecrypt API runtime.

AWS SDK clients can be shared among different keyrings.
This was already allowed under the client supplier model, and we make this more explicit,
since any shared clients MUST be configured at keyring initialization instead of keyring API runtime.

## Operational Implications

We MUST re-implement the AWS KMS keyring in all languages the ESDK supports.
Existing ESDKs that have released AWS KMS keyrings require a migration/deprecation path.

The [ESDK for C](https://github.com/aws/aws-encryption-sdk-c/blob/aa85ca224d550cfe110e2112821a84506b9aca3e/aws-encryption-sdk-cpp/source/kms_keyring.cpp)
implements the AWS KMS keyring as _KMS keyring_ in C++ and returns generic keyring structs.
The proposal can be implemented as a new _AWS KMS keyring_ and the existing keyring implementations can be deprecated.
This allows the new implementation to satisfy the requirements
of [Issue #127](https://github.com/awslabs/aws-encryption-sdk-specification/issues/127).

The [ESDK for JavaScript](https://github.com/aws/aws-encryption-sdk-javascript/blob/75803ed4d3c8b5e86005108c173941c43e81cb56/modules/kms-keyring/src/kms_keyring.ts)
uses a _KMS keyring_ namespace for the AWS KMS keyrings.
The proposal can be implemented in a new _AWS KMS keyring_ namespace.
This now allows the ESDK for JavaScript to use the same AWS KMS naming as other ESDKs.
The previous _KMS keyring_ namespace can be deprecated.

Usage examples and documentation MUST be updated for the new keying implementations.

## Guide-level Explanation

The existing [AWS KMS Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md)
encrypts and decrypts data using AWS KMS [CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys).
It also supports the concept of an _AWS KMS discovery keyring_
that is enabled through a derived [_Is Discovery_](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#is-discovery) property.

The _AWS KMS discovery keyring_ changes the underlying behavior of the AWS KMS keyring.
It returns the unchanged encryption materials on encryption and _attempts_ to decrypt any encrypted data key,
as long as it can initialize an AWS SDK KMS service client (AWS SDK client).

To communicate with AWS KMS, the ESDK’s AWS KMS keyring needs a way to initialize AWS SDK clients.
Current implementations use a [client supplier](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#client-supplier)
that initializes AWS SDK clients.
AWS SDK clients are supplied to the AWS KMS keyring at the keyring’s API runtime.
Client suppliers maintain logic for limiting communication to specific AWS regions and caching AWS SDK clients.

To reduce the scope of the AWS KMS keyring,
we are moving the keyring’s functionality into multiple smaller-scoped keyrings.
These smaller-scoped keyrings are configured by _keyring-producing operations_.
Depending on the implementation, keyring-producing operations can be thought of as factory methods/builders.
They initialize keyrings.

Keyring-producing operations act as the customer-facing APIs.
Like client suppliers, they initialize AWS SDK clients.
However, AWS SDK clients are now initialized before a keyring is configured.
Keyring-producing operations also handle the load of manually initializing multiple, smaller-scoped keyrings.
They initialize all keyrings and AWS SDK clients required to meet a specific customer use case.
This means a keyring-producing operation will produce a multi-keyring of smaller-scoped keyrings.
We refer to the keyrings that keyring-producing operations produce as _derived keyrings_.

In summary, rather than having a single keyring maintain all AWS KMS logic,
logic is broken up into multiple smaller-scoped keyrings.
Rather than support multiple keys/AWS regions in a single AWS KMS keyring,
we use keyring-producing operations to configure a [multi-keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/framework/multi-keyring.md)
of these smaller-scoped keyrings.
The smaller-scoped AWS KMS keyrings become the basic building blocks that keyring-producing operations use to satisfy different customer use cases.

From a customer standpoint, naming will be clearer.
Most customers will not directly interact with the basic building blocks
and will only interact with the keyring-producing operations.
However, more advanced customers can directly interact with the smaller-scoped keyrings
and write their own keyring-producing operations.
This gives advanced customers more flexibility and reduces the need to fork ESDK code in order to meet a specific use case.

## Reference-level Explanation

### AWS KMS symmetric keyring

**Initialization**

On keyring initialization, an AWS KMS symmetric keyring MUST accept the following:

- [Key Name](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#key-names)
- AWS SDK KMS service client (AWS SDK client)
- Optional list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)

**Behavior**

The AWS KMS symmetric keyring MUST...

1. Be configured with a single key name
   and a single AWS SDK client for that key name.
   1. The AWS SDK client MUST be configured with credentials for an AWS principal
      that passes an [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html)
      authorization check for the encrypted data key.
2. Use the AWS SDK client provided at initialization for all AWS KMS API calls.
3. Have an AWS KMS symmetric decryption contract
   that ensures the encrypted data key’s [provider info](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/structures.md#key-provider-information)
   matches the keyring’s configured key name.
   1. Note that only key names in the key ARN format are used for decryption
      because encrypted data keys constructed by this keyring always store the identifier
      of the CMK used to encrypt it in the key ARN format.
      The OnDecrypt API checks the key name against that value before attempting decryption.
      This behavior is consistent
      with the [existing specification](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#key-names).
4. Have an AWS KMS symmetric decryption contract
   that ensures the encrypted data key’s encryption context exactly matches the encryption context used on encrypt.
5. Provide the encrypted data key’s [provider info](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/structures.md#key-provider-information)
   as part of the [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) call.

The AWS KMS symmetric keyring MAY...

1. Be configured with a list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)
   to be included in all AWS KMS API calls.

### AWS KMS symmetric region discovery keyring

Customers MUST declare their intent to use the AWS KMS symmetric region discovery keyring instead of the AWS KMS symmetric keyring.
Any keyring-producing operations that initialize this kind of keyring MUST be distinct from those that produce non-discovery keyrings.

**Initialization**

On keyring initialization, an AWS KMS symmetric region discovery keyring MUST accept the following:

- AWS SDK client (and the AWS SDK client’s region)
- Optional AWS account ID
- Optional list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)

**Behavior**

The AWS KMS symmetric region discovery keyring MUST...

1. Be configured with a single AWS SDK client.
   1. The AWS SDK client MUST be configured with credentials for an AWS principal
      that passes an [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html)
      authorization check for the encrypted data key.
   2. The keyring MUST determine the AWS region associated with the provided AWS SDK client.
      In some implementations,
      this MAY require an additional AWS region argument
      during the keyring's initialization.
      If the AWS SDK returns the AWS SDK client’s region
      directly from the AWS SDK client
      in an intuitive manner (by using a single function/method call),
      implementations MUST NOT require an additional AWS region argument.
2. Use the AWS SDK client provided at initialization for all AWS KMS API calls.
3. Be unable to write a decryption contract,
   but it MAY fulfill any AWS KMS symmetric decryption contract for the keyring's AWS region and, if provided, the configured AWS account ID.
   1. Fulfilling any AWS KMS symmetric decryption contract MUST be sufficient to succeed.
4. Provide the encrypted data key’s [provider info](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/structures.md#key-provider-information)
   as part of the [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html) call.

The AWS KMS symmetric region discovery keyring MAY...

1. Be configured with an AWS account ID.
   1. If an AWS account ID is provided,
      the AWS KMS symmetric region discovery keyring MUST only decrypt encrypted data keys
      that were encrypted using an AWS KMS CMK in that AWS account, for the keyring’s region.
   2. If no AWS account ID is provided,
      the AWS KMS symmetric region discovery keyring MUST _attempt_ to decrypt all encrypted data keys in the keyring’s region.
2. Be configured with a list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)
   to be included in all AWS KMS API calls.

### Keyring-Producing Operations

**Please note the following applies to the keyring-producing operations
that produce the [derived keyrings](#derived-keyrings) below.**

**All keyring-producing operations MUST...**

1. Produce a single [derived keyring](#derived-keyrings)
   1. Each keyring-producing operation MUST be configured with the additional arguments or parameters
      required to satisfy the specification of the derived keyring it produces.
2. Initialize the AWS SDK client(s) required for communicating with AWS KMS for the given key names/AWS regions
3. Configure each AWS KMS keyring with the AWS SDK client it needs to communicate with AWS KMS
4. Be configurable with an optional AWS SDK client configuration (client config) that includes custom AWS SDK credentials
   or a similar AWS SDK language-specific client configuration option
   (examples include:
   Java's [Client Configuration](https://docs.aws.amazon.com/AWSJavaSDK/latest/javadoc/com/amazonaws/ClientConfiguration.html)
   AND [AWS Credentials Provider](https://docs.aws.amazon.com/AWSJavaSDK/latest/javadoc/com/amazonaws/auth/AWSCredentialsProvider.html),
   Python's [botocore Config](https://boto3.amazonaws.com/v1/documentation/api/latest/guide/configuration.html)
   AND [botocore Session](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/core/session.html),
   etc.)
   1. If a client config is provided,
      the keyring-producing operation MUST configure all AWS SDK clients with the provided client config
5. Limit the initialization of new AWS SDK client(s) when possible
   1. Multiple keyrings can be configured with the same AWS SDK client if the following conditions are all met:
      1. Keyrings MUST be configured in the same keyring-producing operation call
      2. Keyrings MUST communicate with AWS KMS in the same AWS region
      3. AWS SDK clients MUST share the same AWS SDK client configuration
6. Maintain the order of all key names/AWS regions provided by the customer

**All keyring-producing operations MAY...**

1. Add an identifier to the user agent string in the AWS SDK client
   that was initialized by the keyring-producing operation to note the ESDK language and version

**All keyring-producing operations MUST NOT...**

1. Modify any customer-provided client configuration
2. Modify a customer-provided user agent string
   1. When a keyring-producing operation initializes an AWS SDK client,
      if customer overrides are not provided,
      the keyring-producing operation MAY append content to note
      the ESDK language and version to the user agent string
      as part of the AWS SDK client initialization process
3. Modify any AWS SDK client(s) initialized directly by the customer

### Derived Keyrings

**Please note the following names are not finalized.**

The ESDK MUST provide keyring-producing operations for the following derived keyrings:

1. **AWS KMS symmetric multi-CMK keyring**
   1. MUST be configured with a list of key names
   2. MUST be configured with a key name identifying the [generator](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#generator)
   3. MAY be configured with a list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)
      to be included in all AWS KMS API calls
   4. MUST be implemented as a multi-keyring of AWS KMS symmetric keyrings...
      1. with a single AWS KMS symmetric keyring for each configured key name
      2. with each AWS KMS symmetric keyring configured with the configured grant tokens, if applicable
      3. where the order of the child keyrings matches the order of the configured key names
2. **AWS KMS symmetric multi-region discovery keyring**
   1. MUST be configured with a list of AWS regions
   2. MAY be configured with an AWS account ID
   3. MAY be configured with a list of string [grant tokens](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#grant-tokens)
      to be included in all AWS KMS API calls
   4. MUST be implemented as a multi-keyring of AWS KMS symmetric region discovery keyrings...
      1. with a single AWS KMS symmetric region discovery keyring for each configured AWS region
      2. with each AWS KMS symmetric region discovery keyring configured with the configured AWS account ID, if applicable
      3. with each AWS KMS symmetric region discovery keyring configured with the configured grant tokens, if applicable
      4. where the order of the child keyrings matches the order of the configured AWS regions
