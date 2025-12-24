[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Interacting with AWS KMS using the AWS Encryption SDK (Background)

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Background

The AWS Encryption SDK’s (ESDK’s) AWS KMS keyring has become increasingly complex as it supports various customer use cases.
The existing [AWS KMS Keyring](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md)
encrypts and decrypts data using AWS KMS [CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys).
It also supports the concept of an _AWS KMS discovery keyring_ that is enabled
through a derived [_Is Discovery_](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#is-discovery) property.

The _AWS KMS discovery keyring_ changes the underlying behavior of the AWS KMS keyring.
It returns the unchanged encryption materials on encryption and _attempts_ to decrypt any encrypted data key,
as long as it can initialize an AWS SDK KMS service client (AWS SDK client).

To communicate with AWS KMS, the ESDK’s AWS KMS keyring needs a way to initialize AWS SDK clients.
Current implementations use a [client supplier](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#client-supplier)
that initializes AWS SDK clients.
AWS SDK clients are supplied to the AWS KMS keyring at the keyring’s API runtime.
Client suppliers can fail to supply clients, resulting in API runtime failures.
Client suppliers can also maintain additional logic,
such as limiting communication to specific AWS regions and caching AWS SDK clients.
Client supplier implementations are not currently consistent among different ESDKs,
though the ESDKs allow customers to implement their own client suppliers.

While the current features the AWS KMS keyring supports are important for our customers,
the existing AWS KMS keyring surface area has been overloaded to support these features.
This has led to a keyring that is increasingly difficult for customers to use in their intended ways
because of the keyring’s complex logic.
Furthermore, the AWS KMS keyring requires additional development and maintenance time
because its increasing API surface area.
One can argue that the current AWS KMS keyring has remained fairly easy to use,
but it has become increasingly easy to misuse and difficult to maintain.

## Goals

The goal of this document is to determine _how_, or _if_, the existing AWS KMS keyring design should change.

## Success Measurements

We know we are successful if we can note different ways customers want to interact with AWS KMS using the ESDK
and _either_ present examples for how the existing AWS KMS keyring currently addresses these use cases
in an “easy to use, hard to misuse” manner,
_or_ demonstrate how a different approach better addresses these behaviors
in an “easy to use, hard to misuse” manner.

We do not currently support some of the use cases we would like to support.
For example, we want to enable customers to define a list of AWS regions
and ensure that the ESDK _attempts_ to decrypt the encrypted data keys only in the specified regions
and tries these regions in the order that the customer specified
(both region limiting and region ordering).
We can gauge success if we can support new features.

For a list of the customer behaviors we are addressing,
as well as pseudocode usage examples,
see [_Customer Use Cases_](#customer-use-cases).

Long-term, we will also measure success based on the number of additional refactors
and code changes the AWS KMS keyrings require to meet new customer use cases.
We want to add support for new use cases without making existing implementations more complex.
We do not want new features to affect the code paths of existing features.
This can be tracked via a GitHub issue that requires us to have an AWS KMS keyring retrospective
after the proposed design is implemented, released, and customer feedback is obtained.

## Out of Scope

Any customer behaviors that are not related to AWS KMS interactions in the ESDK are out of scope for this document.

Keyrings other than the AWS KMS keyring are out of scope for this document.
While the document mentions and relies on the functionalities of the multi-keyring,
this proposal does not change any multi-keyring logic.
All existing handling of the multi-keyring generator and child keyrings is unchanged and considered out of scope.

[AWS KMS asymmetric CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks)
are out of scope for this document.
All AWS KMS keyrings in this document use [AWS KMS symmetric CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#symmetric-cmks).

The following AWS KMS interactions, which are not currently supported by the AWS KMS keyring, are considered out of scope for this design.

1. Optionally limit and order the CMKs to _attempt_ decryption with
   based on their [AWS partition](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
2. Ensure the ESDK encrypts my data in a parallel manner, using every AWS KMS CMK I identify in parallel
   to independently call AWS KMS and encrypt the same piece of data
3. Ensure the ESDK _attempts_ to decrypt my data in a parallel manner,
   _attempting_ to decrypt every encrypted data key in parallel
4. Ensure it is possible to efficiently limit decryption to specific CMKs
   by passing the specific CMKs I want to use as part of my decryption call

## Proposed Design

The high level approach is to separate the existing AWS KMS keyring functionality into separate keyrings,
with each keyring maintaining a relatively small surface area.
To handle customer use cases,
the proposal uses _keyring-producing operations_
that produce a multi-keyring of smaller-scoped child AWS KMS keyrings.
Each child AWS KMS keyring is tied to a single AWS SDK KMS service client (AWS SDK client)
that is initialized before child keyring initialization.
We refer to the keyrings that keyring-producing operations produce as _derived keyrings_.

By simplifying the AWS KMS keyring,
the basic building blocks for long-term maintainability and extensibility are established.
ESDK developers can support additional customer use cases by writing new keyring-producing operations
without needing to refactor existing code or update customer-facing APIs.
Customers are given the same building blocks,
which allows for additional customization and reduces the need for forked code.
The smaller components reinforce an "easy to use, hard to misuse" mindset.

### AWS KMS symmetric keyring

For the purpose of this document,
the proposed design’s _AWS KMS symmetric keyring_ will be referred to as the _AWS KMS keyring_
for brevity.

**Initialization**

On keyring initialization, an AWS KMS symmetric keyring MUST accept the following:

- String identifying an AWS KMS CMK
  ([Key Name](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#key-names))
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

For the purpose of this document,
the proposed design’s _AWS KMS symmetric region discovery keyring_ will be referred to as the _AWS KMS discovery keyring_
for brevity.

The derived _Is Discovery_ property MUST be removed from the AWS KMS keyring.
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

Depending on the implementation, keyring-producing operations can be thought of as factory methods/builders.
They initialize keyrings.

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

1. Modify any customer-provided client config
2. Modify the user agent string if the customer has already provided one
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

### Examples

Examples MUST be written in each ESDK implementation for all customer use cases noted below.

## One-Way Doors

Any changes to the API are considered one-way doors -
these include changing the AWS KMS keyring’s implementation and APIs,
as well as developing additional types of keyrings.
These changes MUST be updated through the specification modification process
and all updates MUST include examples and clear documentation.

The proposal presents a different development style from the existing AWS KMS keyring.
Rather than having a single keyring maintain all AWS KMS logic,
logic is broken up into multiple keyrings.
Rather than support multiple keys/AWS regions in a single AWS KMS keyring,
use a multi-keyring of child AWS KMS keyrings.
The AWS KMS keyring and the AWS KMS discovery keyring become basic building blocks
that keyring-producing operations use to satisfy different customer use cases.
This is a one-way door, but allows for increased flexibility and significantly reduces the risk of future refactors and deprecations.

We are also committing to supporting the use cases under [_Customer Use Cases_](#customer-use-cases).
New use cases include:

- Issue 4a: Know that the ESDK _attempts_ to decrypt my data using the AWS KMS CMKs in the order that I provided them
- Issue 6: Ensure the ESDK optionally allows me to provide a specific AWS SDK client for a specific AWS KMS CMK
- Issue 8a: Optionally limit and order the CMKs to attempt decryption with based on their AWS region or AWS account ID
- Issue 8b: Ensure it is possible to efficiently limit decryption to specific CMKs,
  even if the decryption operation has a large number of CMKs that my credentials have access to
- Issue 12a: Have failures that must occur at runtime occur during startup instead of during encrypt/decrypt, whenever possible

## Security Considerations

This proposal reduces the API surface area of the current AWS KMS keyring.

[Client suppliers](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#client-supplier)
are removed from the AWS KMS keyring.
This includes client supplier logic for limiting communication to specific AWS regions.
This removes the security decision of _which_ AWS regions a keyring can communicate with
from the keyring’s OnEncrypt/OnDecrypt API runtime.

AWS SDK clients can be shared among different keyrings.
This was already allowed under the client supplier model, and we make this more explicit,
since any shared clients MUST be configured at keyring initialization instead of keyring API runtime.

## Impact

1. All pending and future ESDK releases are blocked by these issues
2. Any API or behavioral changes require implementation and an ESDK update
3. Any API or behavioral changes require documentation and example updates
4. Existing ESDKs that implement keyrings require a migration/deprecation path
   1. The [ESDK for C](https://github.com/aws/aws-encryption-sdk-c/blob/aa85ca224d550cfe110e2112821a84506b9aca3e/aws-encryption-sdk-cpp/source/kms_keyring.cpp)
      implements the AWS KMS keyring as _KMS keyring_ in C++ and returns generic keyring structs.
      The proposal can be implemented as a new _AWS KMS keyring_ and the existing keyring implementations can be deprecated.
      This allows the new implementation to satisfy the requirements
      of [Issue #127](https://github.com/awslabs/aws-encryption-sdk-specification/issues/127).
   2. The [ESDK for JavaScript](https://github.com/aws/aws-encryption-sdk-javascript/blob/75803ed4d3c8b5e86005108c173941c43e81cb56/modules/kms-keyring/src/kms_keyring.ts)
      uses a _KMS keyring_ namespace for the AWS KMS keyrings.
      The proposal can be implemented in a new _AWS KMS keyring_ namespace.
      This now allows the ESDK for JavaScript to use the same AWS KMS naming as other ESDKs.
      The previous _KMS keyring_ namespace can be deprecated.

## Assumptions

1. We are assuming [_Customer Use Cases_](#customer-use-cases) represents valid use cases
2. We are assuming there is sufficient education and documentation for any changes
3. We are assuming the AWS SDK allows customers to customize the AWS SDK client at initialization
   using an AWS SDK client configuration or a similar AWS SDK language-specific configuration option

## Justification

Before showing how [_Customer Use Cases_](#customer-use-cases) are handled by the proposal,
we give a high-level justification for the approach.

1. **Move logic out of the AWS KMS keyring** so a single keyring does not handle multiple AWS KMS [CMKs](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys)
   ([key names](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#key-names)),
   multiple AWS regions,
   or complex functionality
   ([discovery](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md#is-discovery),
   limiting communication to specific AWS regions,
   AWS SDK client initialization, etc.).
   The AWS KMS keyring is configured with a single AWS SDK client and key name.
   The AWS KMS discovery keyring is configured with a single AWS SDK client and an optional AWS account ID.
   The smaller scope of the new keyrings reduces the risk of bugs during both ESDK development and ESDK usage.
   This makes them easy to use, hard to misuse.
2. **Create the basic building blocks for interacting with AWS KMS**
   by creating an AWS KMS keyring and a separate AWS KMS discovery keyring.
   New features are supported by writing operations that configure one or more of these smaller-scoped keyrings.
   These operations require us to use our own APIs,
   and also serve as examples to show customers how we use the basic building blocks for more complex use cases.
   This lowers the risk for one-way doors and makes the customer-facing API easy to use, hard to misuse.
3. **Move AWS SDK client initialization/configuration outside of the keyring API runtime**.
   This lowers the potential for keyring API runtime failures.
   There is clear intent and knowledge of _which_ keyring is using _which_ AWS SDK client
   since keyrings are explicitly configured with an AWS SDK client.
   This makes AWS SDK clients easy to use, hard to misuse.
4. **Support additional customer use cases** that cannot be supported today without significant refactors.
   For example, we now support region and key name ordering when _attempting_ decryption.
   We can continue developing the ESDK with an “easy to use, hard to misuse” mindset.

## Customer Use Cases

Each customer use case is defined as a separate issue,
and we show how the proposed design addresses each issue.
For alternatives, see [_Appendices_](#appendices).

All ESDK language implementations MUST provide examples for each issue.

**Although they are not explicitly stated in the examples below,
grant tokens MUST be optional inputs to all keyring-producing operations
and AWS KMS keyrings.**

**All examples below are represented as pseudocode
and are not indicative of final naming
or specific language implementations.**

**As a customer, I want to...**

### Issue 1: Identify any AWS KMS CMK in the ESDK by using a key ARN, key ID, alias ARN, and/or alias name

This use case is separate from the proposed approach, but is still required.
It defines the ways customer can identify an AWS KMS CMK.

Additional restrictions still exist.
For example, only key names in the key ARN format are used for decryption.

**Proposal: Support all key names that AWS KMS supports
by passing all customer-identified key names directly to the AWS SDK client.**

1. Customer identified key names are passed directly to AWS KMS without validation
   1. No need to perform potentially complex validation or parse customer intent
   2. No need to update the ESDK if formats identifying AWS KMS CMKs ever change
   3. Matches current implementation

### Issue 2: Identify an AWS KMS CMK that the ESDK uses to generate a plaintext data key by calling AWS KMS

Customers need a way to configure which key name identifies the generator. This is required at keyring initialization.

**Proposal: The generator is only configured for the multi-keyring.**

1. The proposed design handles multiple CMKs by using a multi-keyring of AWS KMS keyrings.
   A multi-keyring is configured with a generator keyring and a list of child keyrings at initialization.
   A keyring-producing operation initializes the generator keyring, child keyrings, and multi-keyring.
   The multi-keyring calls the generator’s OnEncrypt API first,
   followed by the child keyrings’ OnEncrypt APIs.
   This multi-keyring behavior is unchanged from the current multi-keyring implementation.
   The generator is not handled by the AWS KMS keyring as a separate argument.
   1. Generators are inherently multi-keys concepts
   2. Multi-keyring behavior is unchanged
   3. Keyring-producing operation makes initializing
      a multi-keyring,
      the generator keyring,
      and the child keyrings simple

```
// Without a keyring-producing operation, the proposal is represented as
generatorClient = new AwsSdk.KmsServiceClient()
generatorKeyring = new AwsKmsSymmetricKeyring(generatorKeyName, generatorClient)
client1 = new AwsSdk.KmsServiceClient()
kmsKeyring1 = new AwsKmsSymmetricKeyring(keyName1, client1)
...
multiKeyring = new MultiKeyring(generatorKeyring, [kmsKeyring1, ...])

// The required keyring-producing operation initializes AWS SDK clients
// and one AWS KMS keyring per key name using the required AWS SDK client.
// The generator keyring and the child keyrings are configured for the multi-keyring.
multiKeyring = MakeAwsKmsSymmetricMultiCMKKeyring(generatorKeyName, [keyName1, ...])
```

### Issue 3: Provide the ESDK with one or more AWS KMS CMKs so it encrypts my data by calling AWS KMS

See [_Issue 3a_](#issue-3a-know-that-the-esdk-uses-every-aws-kms-cmk-in-the-order-i-provide-them-to-encrypt-my-data).

### Issue 3a: Know that the ESDK uses every AWS KMS CMK in the order I provide them to encrypt my data

Customers need a way to identify the key names they want to use to encrypt their data

**Proposal: Each AWS KMS keyring is configured with a single AWS KMS CMK.
A multi-keyring maintains the order of its child keyrings,
therefore maintaining the order of the AWS KMS CMKs used to encrypt.**

1. A keyring-producing operation initializes a multi-keyring configured with multiple child AWS KMS keyrings.
   The multi-keyring calls the generator keyring’s OnEncrypt API
   followed by each child AWS KMS keyring’s OnEncrypt API.
   This ordering MUST match the order of the key names provided to the keyring-producing operation.
   1. Multi-keyring behavior is unchanged
   2. Keyring-producing operation initializes AWS KMS keyrings
      in the order of the provided key names
      and passes this list to the multi-keyring,
      which maintains the list’s order

```
// Assuming the multi-keyring has been initialized (see Issue 2)
multiKeyring.OnEncrypt(...)
```

### Issue 4: Provide the ESDK with one or more AWS KMS CMKs so it attempts to decrypt my data by calling AWS KMS

See [_Issue 4a_](#issue-4a-know-that-the-esdk-attempts-to-decrypt-my-data-using-the-aws-kms-cmks-in-the-order-that-i-provided-them).

### Issue 4a: Know that the ESDK attempts to decrypt my data using the AWS KMS CMKs in the order that I provided them

Customers need a way to identify the key names they want to use to _attempt_ to decrypt their data.

**Proposal: Each AWS KMS keyring is configured with a single AWS KMS CMK.
A multi-keyring maintains the order of its child keyrings,
therefore maintaining the order of the AWS KMS CMKs used to decrypt.**

1. A keyring-producing operation initializes a multi-keyring configured with multiple child AWS KMS keyrings.
   The multi-keyring calls the OnDecrypt API for the keyrings it was initialized with in the order they were configured.
   This ordering MUST match the order of the key names provided to the keyring-producing operation.
   1. Multi-keyring behavior is unchanged
   2. Keyring-producing operation initializes AWS KMS keyrings
      in the order of the provided AWS KMS CMKs
      and passes this list to the multi-keyring,
      which maintains the list’s order

```
// Assuming the multi-keyring has been initialized (see Issue 2)
multiKeyring.OnDecrypt(...)
```

### Issue 5: Have the ESDK automatically set up AWS SDK clients for communicating with AWS KMS in my AWS KMS CMKs’ region(s)

Customers want the ESDK to initialize AWS SDK clients
when AWS SDK client customization is not required.

**Proposal: Keyring-producing operations initialize AWS SDK clients
as well as the keyrings that are configured with them.**

1. Every AWS KMS keyring is configured with an AWS SDK client at keyring initialization.
   The AWS SDK client is initialized by the keyring-producing operation
   with the default AWS SDK client configuration
   so customers do not manually initialize AWS SDK clients using the AWS SDK.
   1. Keyring-producing operation handles AWS SDK client initialization and configuration
   2. AWS SDK client initialization and configuration logic is moved
      outside of the AWS KMS keyring runtime,
      limiting the keyring’s scope
   3. Failures are more likely to occur at keyring initialization
      instead of keyring API runtime
   4. During a single keyring-producing operation call,
      AWS SDK clients can be reused by other key names in the same region
   5. AWS SDK clients are not cached
      between different keyring-producing operation calls
2. Every AWS KMS discovery keyring requires an AWS SDK client to be configured at keyring initialization.
   AWS SDK clients are initialized by the keyring-producing operation
   (see [_Issue 8_](#issue-8-be-able-to-attempt-decryption-without-needing-to-specify-the-cmks-to-use-for-decryption)).

```
// Without a keyring-producing operation, the proposal is represented as
client1 = new AwsSdk.KmsServiceClient("us-west-2")
kmsKeyring1 = new AwsKmsSymmetricKeyring(keyName1, client1)
client2 = new AwsSdk.KmsServiceClient("us-east-1")
kmsKeyring2 = new AwsKmsSymmetricKeyring(keyName2, client2)
...
multiKeyring = new MultiKeyring(generatorKeyring, [kmsKeyring1, kmsKeyring2, ...])

// The required keyring-producing operation initializes AWS SDK clients
// by determining the region of all key names.
// The ARN format specifies the key name's AWS region
// while non-ARN formatted key names use the unspecified (local) region.
//
// The keyring-producing operation uses each region to initialize an AWS SDK client
// for that region using the AWS SDK.
// AWS SDK clients are regionalized and a single client cannot be used
// in multiple AWS regions.
//
// It is likely the keyring-producing operation will have an internal mapping
// from region to AWS SDK client,
// so that key names in the same region can use the same AWS SDK client.
// The mapping is not cached between different keyring-producing operation calls.
//
// Once all clients are initialized,
// AWS KMS keyrings are configured for each key name
// with the AWS SDK client for that key name's region.
//
// Finally, the keyring-producing operation initializes
// a multi-keyring with child AWS KMS keyrings
// in the same order as the key names provided to the keyring-producing operation.
multiKeyring = MakeAwsKmsSymmetricMultiCMKKeyring(generator, [keyName1, ...])
// Configuring the keyring will fail if an AWS SDK client fails to initialize.
//
// If an AWS SDK client is successfully initialized, it can still fail network calls
// at the keyring's OnEncrypt/OnDecrypt API runtime
```

### Issue 6: Ensure the ESDK optionally allows me to provide a specific AWS SDK client for a specific AWS KMS CMK

Customers want to be able to customize their AWS SDK clients,
at a per-key name level.
Under the current AWS KMS keyring specification,
customization is supported at a per-region level using client suppliers,
but not at a per-key name level without using multiple AWS KMS keyrings and multiple client suppliers.

**Proposal: Keyring-producing operations allow customers to customize AWS SDK clients
by allowing customers to configure an AWS SDK client configuration (client config).
The client config allows customers to change properties like the AWS SDK timeout/credentials/etc.**

**Customers can manually initialize their own AWS KMS keyrings or multi-keyrings for additional customization.**

1. The AWS KMS keyring is clearly configured with a single key name and AWS SDK client.
   Keyring-producing operations can be configured with a client config
   that is used to configure all AWS SDK clients the operation initializes.
   1. All AWS SDK clients initialized in a single keyring-producing operation call
      are configured with the same client config
   2. Keyring-producing operations can be written for additional customization
      1. Configure a mapping from key name to client config
      2. Configure a mapping from AWS region to client config
      3. New features can be written without affecting existing code
2. Every AWS KMS discovery keyring requires an AWS SDK client to be configured at keyring initialization.
   AWS SDK clients are initialized by the keyring-producing operation
   (see [_Issue 8_](#issue-8-be-able-to-attempt-decryption-without-needing-to-specify-the-cmks-to-use-for-decryption)).

```
// Without a keyring-producing operation, the proposal is represented as
customUsWest2Client = new AwsSdk.KmsServiceClient("us-west-2", AwsConfig)
kmsKeyring1 = new AwsKmsSymmetricKeyring(keyName1, customUsWest2Client)
...

// Keyring-producing operations optionally accept an AWS SDK client configuration
// (AwsConfig)
// that allows AWS SDK clients to be configured
// with custom timeouts, credentials, etc.
// The keyring-producing operation follows the same process as Issue 5,
// but uses the AWS SDK client config to configure AWS SDK clients it initializes.
multiKeyring = MakeAwsKmsSymmetricMultiCMKKeyring(generator, [keyName1, ...], AwsConfig)

// Keyring-producing operations SHOULD be configurable with mappings.
// For example, we could have keyName1's AWS SDK client
// configured with a custom AWS SDK client config
// MakeAwsKmsSymmetricMultiCMKKeyring(generator, [keyName1, ..], {keyName1: AwsConfig})

// For even more complex customization,
// which is outside of the scope of the AWS SDK client config,
// customers manually initialize an AWS SDK client and AWS KMS keyring
myVeryCustomSDKClient = new MyVeryCustomAwsSdkClient()
kmsKeyring1 = new AwsKmsSymmetricKeyring(keyName1, myVeryCustomSDKClient)
```

### Issue 7: Optionally specify the regions the ESDK communicates with

See [_Issue 7a_](#issue-7a-optionally-specify-the-esdk-only-communicates-with-the-local-region).

### Issue 7a: Optionally specify the ESDK only communicates with the local region

Customers want to limit the regions the ESDK communicates with.

The actual AWS region the _local region_ represents is determined at initialization.
There is not a separate identifier for the _local region_.
If the _local region_ is actually _us-west-2_,
it is represented as _us-west-2_
and it is not represented as a unique _local region_ identifier.

**Proposal: The AWS KMS keyring limits communication to the single key name it is configured with.
The AWS KMS discovery keyring limits communication to the single AWS region it is configured with.**

1. Each AWS KMS keyring is configured with a single key name and AWS SDK client.
   No additional AWS SDK clients are initialized after the keyring has been configured.
   Customers can configure AWS KMS keyrings with key names in the local region.
   1. See [_Issue 2_](#issue-2-identify-an-aws-kms-cmk-that-the-esdk-uses-to-generate-a-plaintext-data-key-by-calling-aws-kms),
      [_Issue 3a_](#issue-3a-know-that-the-esdk-uses-every-aws-kms-cmk-in-the-order-i-provide-them-to-encrypt-my-data),
      or [_Issue 4a_](#issue-4a-know-that-the-esdk-attempts-to-decrypt-my-data-using-the-aws-kms-cmks-in-the-order-that-i-provided-them)
2. Each AWS KMS discovery keyring is configured with a single AWS region and AWS SDK client.
   No additional AWS SDK clients are initialized after the keyring has been configured.
   Customers can configure the AWS KMS discovery keyring with the local region.
   1. AWS SDK clients can be configured without specifying a region (which implies the local region)
   2. A local-only AWS KMS discovery keyring-producing operation can be written
   3. See [_Issue 8_](#issue-8-be-able-to-attempt-decryption-without-needing-to-specify-the-cmks-to-use-for-decryption)

### Issue 8: Be able to attempt decryption without needing to specify the CMKs to use for decryption

See [_Issue 8a_](#issue-8a-optionally-limit-and-order-the-cmks-to-attempt-decryption-with-based-on-their-aws-region-or-aws-account-id).

### Issue 8a: Optionally limit and order the CMKs to attempt decryption with based on their AWS region or AWS account ID

Customers want to maintain the existing [AWS KMS regional discovery keyring](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#kms-keyring-regional)
use case,
where a keyring can _attempt_ decryption for any encrypted data key in a specific region,
regardless of the AWS KMS CMK used to encrypt it.
We also want to enable customers to have ordered region limiting,
where customers provide an ordered list of AWS regions
and decryption is _attempted_ in the provided order.
We also want to enable customers to limit decryption to encrypted data keys for a specific AWS account ID.

**Proposal: The AWS KMS discovery keyring is configured with a single AWS region,
a single AWS SDK client,
and an optional AWS account ID.**

**A keyring-producing operation is configured with a list of AWS regions
and an optional AWS account ID.
The keyring-producing operation initializes one AWS KMS discovery keyring per-region
and configures it with the optional AWS account ID.
Each AWS KMS discovery keyring is a child in a multi-keyring,
in the same order as the regions configured by the customer.**

**If customers need a keyring that _attempts_ decryption in all AWS regions,
they SHOULD call a service/API
to get an updated list of AWS regions.
This will prevent any ESDK or AWS SDK region-list from becoming stale over time.
In most cases, customers SHOULD simply use the keyring-producing operation
for the specific regions they need.
This will provide flexibility for adding more regions over time,
without allowing access to regions that are not currently required.**

1. By configuring the keyring-producing operation,
   customers configure the region(s) and AWS account ID they want to communicate with.
   The multi-keyring maintains the order of its child keyrings.
   The keyring-producing operation ensures the order of the multi-keyring’s child keyrings
   matches the order of the customer-configured regions.
   1. Multi-keyring behavior is unchanged
   2. Supports new ordered region filtering use case
   3. Supports new AWS account ID filtering use case

```
// Without a keyring-producing operation, the proposal is represented as
awsAccountId = "1234..."
usWest2Client = new AwsSdk.KmsServiceClient("us-west-2")
usWest2DiscoveryKeyring = new AwsKmsSymmetricRegionDiscoveryKeyring(usWest2Client, awsAccountId)
usEast1Client = new AwsSdk.KmsServiceClient("us-east-1")
usEast1DiscoveryKeyring = new AwsKmsSymmetricRegionDiscoveryKeyring(usEast1Client, awsAccountId)
...
multiKeyring = new MultiKeyring([usWest2DiscoveryKeyring, usEast1DiscoveryKeyring, ...])

// The required keyring-producing operation initializes AWS SDK clients
// for each configured region.
// The keyring-producing operation configures one AWS KMS discovery keyring
// per configured region, with the associated AWS SDK client.
// Customers can optionally configure an AWS account ID.
multiKeyring = MakeAwsKmsSymmetricMultiRegionDiscoveryKeyring(["us-west-2", "us-east-1", ...], awsAccountId)

// If customers do not need to limit the AWS account ID...
multiKeyring = MakeAwsKmsSymmetricMultiRegionDiscoveryKeyring(["us-west-2", "us-east-1", ...])
```

### Issue 8b: Ensure it is possible to efficiently limit decryption to specific CMKs, even if the decryption operation has a large number of CMKs that my credentials have access to

Customers that have access to a large number of key names want a way to limit decryption to specific key names.
Unlike [_Issue 4_](#issue-4-provide-the-esdk-with-one-or-more-aws-kms-cmks-so-it-attempts-to-decrypt-my-data-by-calling-aws-kms)
and [_Issue 8a_](#issue-8a-optionally-limit-and-order-the-cmks-to-attempt-decryption-with-based-on-their-aws-region-or-aws-account-id),
it is not always clear what this filtering criteria is in advanced,
at application/service start time.
Customers need a way to generate AWS KMS keyrings with minimal performance impacts.

**Proposal: Each AWS KMS keyring and each AWS KMS discovery keyring is associated with a single AWS SDK client.
AWS SDK clients can be re-used.
Keyrings that limit communication to specific AWS regions
and/or a specific AWS account ID exist.
Keyrings supporting an explicit list of key names also exist.**

1. Customers configure AWS KMS keyrings and AWS KMS discovery keyrings with specific AWS SDK clients.
   Keyring-producing operations configure AWS KMS keyrings
   and initialize AWS SDK clients before keyring configuration.
   AWS SDK clients are initialized before a keyring is configured.
   Customers can (manually) re-use a keyring or AWS KMS client.
   Keyring-producing operations allow customers to quickly configure keyrings
   that only support specific key names or AWS regions.
   1. SDK clients are initialized before keyring API runtime
   2. Configuring keyrings is computationally cheap compared to initializing AWS SDK clients
   3. Customers can (manually) re-use AWS SDK clients or keyrings
   4. Keyring-producing operations allow customers to configure keyrings that only support
      the AWS regions,
      key names,
      or AWS account ID they care about

### Issue 8c: Understand the security implications of attempting decryption without needing to specify the CMKs to use for decryption

Regardless of the option, clear documentation is critical.

At a minimum, we can RECOMMEND that customers configure the region(s)/AWS account
they want to communicate with when configuring AWS KMS discovery keyrings.
This is RECOMMENDED over configuring a keyring that _attempts_ decryption
for all AWS regions without any AWS account filtering.
Naming MUST also be clear, so customers understand what each keyring-producing operation does.

### Issue 9: Have clear documentation on how the ESDK interacts with AWS KMS, including what information the ESDK sends about me to AWS KMS

Regardless of the option,
clear documentation is critical.
A specification change is required to note any user agent strings
configured by the ESDK when initializing AWS SDK clients.

Customer use case examples are required for each language-specific implementation.

### Issue 10: Ensure the ESDK’s process for communicating with AWS KMS is both memory and network efficient

Customers want to ensure that we are considering memory and network implications in the ESDK.

**Proposal: Each AWS KMS keyring and AWS KMS discovery keyring is associated with a single AWS SDK client.
Network calls are only made to the appropriate AWS KMS API(s).**

1. Keyring-producing operations initialize AWS SDK clients before keyring configuration.
   AWS KMS keyrings that are configured
   as part of the same keyring-producing operation call MAY share AWS SDK clients.
   1. AWS SDK clients are shared within the scope of a single keyring-producing operation call
      1. Only applies if multiple keyrings are configured for the same AWS region
      2. Only applies if the same AWS config can be shared by multiple keyrings’ AWS SDK clients
   2. Network calls are...
      1. Limited to OnEncrypt/OnDecrypt keyring API runtime
      2. Limited to the [AWS KMS Encrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html),
         [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html),
         and [AWS KMS GenerateDataKey API](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey.html)

### Issue 11: Receive clear messaging from the ESDK when an operation fails

Regardless of the option,
clear messaging on failures is critical.
This proposal MUST adhere to any specification guidance on error/failure messaging.

### Issue 12: Have failures occur at compile time instead of runtime, whenever possible

See [_Issue 12a_](#issue-12a-have-failures-that-must-occur-at-runtime-occur-during-startup-instead-of-during-encryptdecrypt-whenever-possible).

### Issue 12a: Have failures that must occur at runtime occur during startup instead of during encrypt/decrypt, whenever possible

**Proposal: AWS SDK client initialization MUST occur before keyring configuration.
Failures resulting from AWS SDK client initialization occur before keyring API runtime.**

1. Errors that occur during AWS SDK client initialization occur before keyring initialization
2. Keyring cannot be configured without an initialized AWS SDK client
3. AWS SDK client initialization debugging is more reproducible, since it occurs before keyring runtime

# Appendices

## Alternatives

_"Current Implementation"_ refer to the existing AWS KMS keyring,
as it is defined in the ESDK Specification
([AWS KMS keyring version 0.2.2](https://github.com/awslabs/aws-encryption-sdk-specification/blob/dbc17f93100667e28dc54e64d05a625db3e5bac2/framework/kms-keyring.md)).

We do not currently support all customer use cases listed below.
**"Code change"** is bolded under _Current implementation_ options
were a code change is required to support the use case.

### Issue 1: Identify any AWS KMS CMK in the ESDK by using a key ARN, key ID, alias ARN, and/or alias name

**Alternative 1: Support all key names that AWS KMS supports
by performing validation on key names prior to usage.**

1. Customers identify key names,
   which pass through multiple forms of validation.
   Each validation varies in complexity.
   For example, we can determine if a string is in the [ARN format](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
   by checking its prefix
   and then determine if it is well-formed with valid characters/length.
   1. CON: Potentially complex validation introduces a wide surface area for bugs
      1. Especially if any regex validation is performed
   2. CON: Maintenance to ensure key names remain up-to-date with accepted AWS KMS CMK identifier formats
   3. CON: Customers MAY use older ESDK versions that do not support newer AWS KMS CMK identifier formats

### Issue 2: Identify an AWS KMS CMK that the ESDK uses to generate a plaintext data key by calling AWS KMS

**Alternative 1: _Current Implementation_ -
Identify the generator directly as an optional argument at AWS KMS keyring initialization.**

1. The AWS KMS keyring has an optional generator field, which allows it to distinguish between the generator and the other key names.
   1. PRO: Clear separation between the generator and other key names
   2. CON: Wide API surface area, as multiple key names and a generator need to be supported by the same AWS KMS keyring

```
// The current implementation is either configured with key names
// and a generator separately
clientSupplier = new AwsKmsDefaultClientSupplier()
keyring = new AwsKmsKeyring(generator, keyNames, clientSupplier)

// Or a keyring-producing operations accept a list of key names
// and assumes the first key name MUST be the generator
// (or some similar variant)
clientSupplier = new AwsKmsDefaultClientSupplier()
keyring = new AwsKmsKeyring([generator, keyName1, keyName2...], clientSupplier)

// Keyring-producing operations can remove the need to specify a client supplier
keyring = MakeAwsKmsKeyring(generator, keyNames)
```

**Alternative 2: Each AWS KMS keyring, as defined in the proposal,
has an optional _Is Generator_ boolean argument.**

1. The AWS KMS keyring is configured with a single key name,
   an AWS SDK client,
   and an optional _Is Generator_ boolean.
   The keyring-producing operation configures a multi-keyring
   with child AWS KMS keyrings and a generator keyring.
   Only the generator keyring has the _Is Generator_ boolean set as true.
   All child keyrings have the _Is Generator_ boolean set as false.
   1. PRO: Keyring-producing operation ensures only a single AWS KMS keyring has _Is Generator_ set as true
   2. PRO: Additional clarity around whether a keyring MUST call
      the [AWS KMS GenerateDataKey API](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey.html)
   3. CON: Additional API surface area for the AWS KMS keyring
   4. CON: Generators are an inherently a multi-key concept

**Alternative 3: In addition to being configured with a required key name,
each AWS KMS keyring is configured with an optional generator key name.**

1. This represents _Alternative 2_,
   but the AWS KMS keyring is configured with an optional generator key name,
   instead of an _Is Generator_ boolean.
   1. CON: No guarantee the required key name and generator are in the same AWS region
      1. Would require an additional, optional, AWS SDK client for the generator
      2. Risk of confusion between the generator’s AWS SDK client
         and the required key name’s AWS SDK client
   2. CON: Additional API surface area
      1. Complexity begins to approach existing AWS KMS keyring implementation

### Issue 3a: Know that the ESDK uses every AWS KMS CMK in the order I provide them to encrypt my data

**Alternative 1: _Current Implementation_ -
Use a single AWS KMS keyring, which is configured with multiple key names.**

1. The keyring is directly configured with a list of key names.
   The keyring calls the [AWS KMS Encrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html)
   based on the order of the provided key names.
   1. PRO: Implementations guarantee the order of the key names is maintained
   2. CON: Single keyring’s scope includes multiple key names

### Issue 4a: Know that the ESDK attempts to decrypt my data using the AWS KMS CMKs in the order that I provided them

**Alternative 1: _Current Implementation_ -
Use a single AWS KMS keyring, which is configured with multiple key names.**

1. The keyring is directly configured with a list of key names.
   The keyring calls [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html)
   based on the order of the encrypted data keys provided to the OnDecrypt API.
   1. **CON: Code change required
      to switch from _attempting_ decryption
      based on the order of the encrypted data keys
      to _attempting_ decryption
      based on the order of the key names
      provided at keyring initialization.**

### Issue 5: Have the ESDK automatically set up AWS SDK clients for communicating with AWS KMS in my AWS KMS CMKs’ region(s)

**Alternative 1: _Current Implementation_ -
AWS KMS client suppliers supply AWS SDK clients at AWS KMS keyring API runtime.**

1. Default client suppliers initialize AWS SDK clients.
   Implementations cache AWS SDK clients.
   1. CON: AWS SDK client initialization logic is run inside the AWS KMS keyring
      1. Occurs during API runtime
         1. Initialization failures occur during API runtime
   2. CON: Very large surface area for supplying clients
      1. Initializing AWS SDK clients
      2. _When_/_how_ to cache
      3. _Which_ AWS regions to communicate with
2. The _Is Discovery_ AWS KMS keyring variant uses client suppliers as well
   (see [_Issue 8a_](#issue-8a-optionally-limit-and-order-the-cmks-to-attempt-decryption-with-based-on-their-aws-region-or-aws-account-id-1)).

```
// Without a keyring-producing operation,
// the AWS KMS keyring implementation is represented as
clientSupplier = new AwsKmsDefaultClientSupplier()
kmsKeyring = new AwsKmsKeyring(generator, keyNames, clientSupplier)

// With a keyring-producing operation,
// manual client supplier initialization is not required
kmsKeyring = MakeAwsKmsKeyring(generator, keyNames)
// Configuring an AWS KMS keyring will not error
// for AWS SDK client-related reasons.
// Errors instead occur at the keyring's OnEncrypt/OnDecrypt API runtime.

// If an AWS SDK client is successfully initialized, it can still fail network calls
// at the keyring's OnEncrypt/OnDecrypt API runtime.
```

**Alternative 2: Continue using client suppliers,
but have client suppliers supply AWS SDK clients at keyring initialization
instead of keyring API runtime.**

1. The client supplier supplies AWS SDK clients
   for all key names at the keyring’s initialization,
   rather than the first time the key name is used in the OnEncrypt/OnDecrypt API.
   1. PRO: Moves logic for obtaining AWS SDK clients from API runtime to keyring initialization
      1. Better for error handling/debugging
   2. CON: Client suppliers still maintain a wide surface area
      1. Initializing AWS SDK clients
      2. _When_/_how_ to cache
      3. _Which_ AWS regions to communicate with
2. The _Is Discovery_ AWS KMS keyring variant initializes AWS SDK clients at runtime
   (see [_Issue 8a_](#issue-8a-optionally-limit-and-order-the-cmks-to-attempt-decryption-with-based-on-their-aws-region-or-aws-account-id-1)).

**Alternative 3: Client suppliers are replaced with a static mapping
of key name to AWS SDK client.
The mapping is part of the keyring’s configuration.**

1. The keyring is configured with a mapping from key name to AWS SDK client.
   Keyring-producing operations initialize this mapping for customers.
   The initialization process ensures an AWS SDK client exists for all key names.
   1. PRO: Logic for AWS SDK client initialization occurs outside of the keyring
   2. CON: Maintains the complexity of configuring a single AWS KMS keyring
      with multiple key names
   3. Note: an alternative is a static mapping of AWS region to AWS SDK client 1. Reduces some of the available customization options
      (see [_Issue 6_](#issue-6-ensure-the-esdk-optionally-allows-me-to-provide-a-specific-aws-sdk-client-for-a-specific-aws-kms-cmk-1)).

```
// Without a keyring-producing operation, initializing maps is time consuming
// for the developer.
// The map represents {key = key name : value = AWS SDK client}
clientMap = {}
clientMap.put(keyName1, new AwsSdk.KmsServiceClient())
// If the key names are in the same region, the same AWS SDK client can be reused,
// instead of initializing multiple new AWS SDK clients
clientMap.put(keyName2, new AwsSdk.KmsServiceClient())
...
kmsKeyring = new AwsKmsKeyring(generator, keyNames, clientMap)
// The AWS KMS keyring ensures every key name in the list of key names
// has an entry in the client map

// With a keyring-producing operation this is simplified
// Here, assume AWS SDK clients can be initialized for any AWS region
kmsKeyring = MakeKmsKeyring(generator, keyNames)
```

### Issue 6: Ensure the ESDK optionally allows me to provide a specific AWS SDK client for a specific AWS KMS CMK

**Alternative 1: _Current Implementation_ -
Customers can write their own client suppliers,
which allow per-region customization.
To allow per-AWS KMS CMK customization,
customers need to configure different AWS KMS keyrings with different client suppliers.**

1. The current implementation has a client supplier accept an AWS region
   and return an AWS SDK client for that region.
   1. CON: Client suppliers support region inputs, not key name inputs
   2. CON: Client suppliers maintain a wide surface area
      1. Initializing AWS SDK clients
      2. _When_/_how_ to cache
      3. _Which_ AWS regions to communicate with

```
// Without a keyring-producing operation,
// the AWS KMS keyring implementation is represented as
clientSupplier = new AwsKmsCustomClientConfigSupplier(AwsConfig)
kmsKeyring = new AwsKmsKeyring(generator, keyNames, clientSupplier)

// Customers can implement their own client suppliers,
// but these are limited to region customization
MyCustomClientSupplier1
    GetClient(region) returns SdkClient
        if (GetRegion(region) == "us-west-2"):
            return new AwsSdk.KmsServiceClient("us-west-2", someCustomProperty)
        else:
            return ...
MyCustomClientSupplier2
    GetClient(region) returns SdkClient
        if (GetRegion(region) == "us-west-2"):
            return new AwsSdk.KmsServiceClient("us-west-2", someOtherCustomProperty)
        else:
            return ...

// Manually initialize separate AWS KMS keyrings with different client suppliers.
clientSupplier1 = new MyCustomClientSupplier1()
kmsKeyring1 = new AwsKmsKeyring(keyNamesForClientSupplier1, clientSupplier1)
clientSupplier2 = new MyCustomClientSupplier2()
kmsKeyring2 = new AwsKmsKeyring(keyNamesForClientSupplier2, clientSupplier2)
...
multiKeyring = new MultiKeyring(generatorKeyring, [kmsKeyring1, kmsKeyring2, ...])

// Some implementations MAY potentially provide a keyring-producing operation
// to handle this customization at a higher level.
// This adds additional complexity.
```

**Alternative 2: Client suppliers are updated
to accept a key name input instead of an AWS region input.**

1. The client supplier behaves similarly to _Alternative 1_.
   The client supplier accepts a key name.
   1. PRO: Allows additional customization
   2. **CON: A code change is required
      to have client suppliers natively support AWS KMS CMK inputs**
   3. CON: Client suppliers maintain a wide surface area
      1. Initializing AWS SDK clients
      2. _When_/_how_ to cache
      3. _Which_ AWS regions to communicate with

```
// Without a keyring-producing operation, this looks similar to
clientSupplier = new AwsKmsCustomClientConfigSupplier(AwsConfig)
kmsKeyring = new AwsKmsKeyring(generator, keyNames, clientSupplier)

// Customers can implement their own client suppliers
MyCustomClientSupplier
    GetClient(keyName) returns SdkClient
        if (GetRegion(keyName) == "us-west-2"):
            return new AwsSdk.KmsServiceClient("us-west-2", someCustomProperty)
        else if (keyName == "somethingVerySpecific"):
            return new ...
        else:
            return ...

kmsKeyring = new AwsKmsKeyring(generator, keyNames, new MyCustomClientSupplier())
```

**Alternative 3: Client suppliers are replaced
with a static mapping of key name to AWS SDK client.
The mapping is part of the keyring’s configuration.**

1. The keyring is configured with a mapping from key name to AWS SDK client.
   Keyring-producing operations initialize this mapping for customers.
   The initialization process ensures an AWS SDK client exists for all key names.
   1. PRO: Logic for AWS SDK client initialization occurs outside of the keyring
   2. CON: Maintains the complexity of configuring a single AWS KMS keyring with multiple key names

```
// Without a keyring-producing operation, initializing maps is time consuming
// for the developer.
// The map represents {key = key name : value = AWS SDK client}
clientMap = {}
clientMap.put(keyName1, new AwsSdk.KmsServiceClient("us-west-2", AwsConfig1))
clientMap.put(keyName2, new AwsSdk.KmsServiceClient("us-west-2", AwsConfig2))
...
kmsKeyring = new AwsKmsKeyring(generator, keyNames, clientMap)

// With a keyring-producing operation, this is made simpler
kmsKeyring = MakeAwsKmsKeyring(generator, keyNames, AwsConfig)

// Alternatively, keyring-producing operations MAY be configured with
// a mapping from key name to AWS SDK client config.
//
// Behind the scenes, an AWS KMS keyring is still configured with a client map,
// but the keyring-producing operation constructs this map
// by configuring each key name's AWS SDK client
// with the appropriate AWS SDK client config.
//
// The config map represents {key = key name : value = AWS SDK client config}
configMap = {}
configMap.put(keyName1, AwsConfig1)
configMap.put(keyName2, AwsConfig2)
...
kmsKeyring = MakeAwsKmsKeyring(generator, keyNames, configMap)
```

### Issue 7a: Optionally specify the ESDK only communicates with the local region

**Alternative 1: _Current Implementation_ -
Customers use client suppliers for restricting communications to specific AWS regions.**

1. Implementations of an _include region client supplier_ exist.
   Customers configure the client supplier
   with the regions they want to communicate with.
   The client supplier only supplies/initializes clients for the configured regions.
   1. CON: Failures MAY occur when the AWS SDK client is being supplied
      1. AWS KMS keyring API runtime
   2. CON: Security decision for _which_ regions to communicate with occurs at API runtime
      1. Does not occur at AWS KMS keyring initialization

```
// Initialize an Include Regions Client Supplier
// and configure the AWS KMS keyring to use it
regionClientSupplier = new IncludeRegionsClientSupplier([us-west-2, us-east-1, ...])
kmsKeyring = new AwsKmsKeyring(generator, keyNames, regionClientSupplier)

// When the keyring's OnEncrypt/OnDecrypt API is called,
// the Include Regions Client Supplier checks if the region
// is part of the provided list.
// If not, it fails to supply an AWS SDK client.
```

**Alternative 2: An AWS KMS keyring is configured
with a client mapping from AWS region to AWS SDK client.
The mapping restricts what regions AWS SDK clients can be obtained for.**

1. A keyring-producing operation is configured with a list of regions.
   For each region,
   the keyring-producing operation adds an entry to a map,
   with the region as the key
   and an initialized AWS SDK client for that region as a value.
   The AWS KMS keyring is configured with this mapping.
   The AWS KMS keyring verifies that each key name’s region has an entry in the map
   during keyring configuration.
   1. PRO: Failures MAY occur when the keyring is being configured
      instead of when the keyring is being used
   2. PRO: If the AWS KMS keyring is successfully configured,
      it can communicate with all key names it was configured with
   3. CON: The AWS KMS keyring is still configured with multiple key names
      1. Wider surface area

```
// Without a keyring-producing operation, initializing maps is time consuming
// for the developer
// The map represents {key = AWS region : value = AWS SDK client}
regionClientMap = {}
regionClientMap.put("us-west-2", new AwsSdk.KmsServiceClient("us-west-2"))
regionClientMap.put("us-east-1", new AwsSdk.KmsServiceClient("us-east-1"))
...
kmsKeyring = new AwsKmsKeyring(generator, keyNames, regionClientMap)

// With a keyring-producing operation, this is simplified
kmsKeyring = MakeAwsKmsKeyring(generator, keyNames, ["us-west-2", "us-east-1", ...])
```

### Issue 8a: Optionally limit and order the CMKs to attempt decryption with based on their AWS region or AWS account ID

**Alternative 1: _Current Implementation_ -
Use an _include regions client supplier_/_limit regions client supplier_
with an AWS KMS keyring that has the derived _Is Discovery_ property.**

1. Customers use a client supplier
   so AWS SDK clients are only supplied in specific regions.
   This limits region discovery from any region to specific AWS regions.
   1. CON: Client suppliers initialize AWS SDK clients at keyring API runtime
   2. **CON: Code change required to _attempt_ decryption in a specific order**
   3. **CON: Code change required to only supply clients for a specific AWS account**
   4. Alternatively...
      1. Keyring-producing operation is configured
         with a list of AWS regions
      2. Keyring-producing operation initializes one AWS KMS keyring
         with the derived _Is Discovery_ property for each configured region
         1. Each AWS KMS keyring uses a different _include regions client supplier_
         2. Each _include regions client supplier_ is configured with a single allowed region
      3. Keyring-producing operation configures a multi-keyring with these keyrings
         1. Each AWS KMS keyring with the derived _Is Discovery_ property is a child
            of the multi-keyring

```
// Initialize an Include Regions Client Supplier and pass it into the keyring
regionClientSupplier = new IncludeRegionsClientSupplier([us-west-2, us-east-1, ...])
kmsKeyring = new AwsKmsKeyring(regionClientSupplier)

// This keyring does not maintain ordering
// and does not allow for limiting calls to a single AWS account
```

**Alternative 2: Implement a composable filtering keyring.
The filtering keyring is configured with a list of AWS regions,
an optional AWS account ID,
and an AWS KMS keyring with the _Is Discovery_ property.**

1. A filtering keyring MUST be configured
   with an AWS KMS keyring with the derived _Is Discovery_ property
   and MUST be configured with a list of AWS regions.
   A filtering keying MAY be configured with an AWS account ID.
   When the OnDecrypt keyring API is called,
   the filtering keyring takes the list of encrypted data keys
   and determines the AWS region and AWS account ID
   for the AWS KMS CMK used to encrypt the data key.
   If the AWS KMS CMK’s AWS region and AWS account ID are part of the keyring’s configuration,
   the encrypted data key is copied to a new list.
   The new list is ordered the same as the keyring’s configured AWS regions.
   The new list is provided to the AWS KMS keyring with the derived _Is Discovery_ property
   (example below).
   1. PRO/CON: Only allows filtering for AWS KMS encrypted data keys
      1. Data keys MUST be encrypted with an AWS KMS CMK
   2. CON: Encrypted data keys MUST be filtered and ordered
      before they are provided to the underlying keyring’s OnDecrypt API
   3. CON: The AWS KMS keyring with the derived _Is Discovery_ property allows communication
      with any AWS account ID or region
      1. Filtering keyring ensures it is only called with encrypted data keys
         that meet the filter requirements
         1. This could potentially be misused
   4. CON: Another keyring
      1. Increases complexity
      2. Potential for customer confusion

```
// Configure the filtering keyring
// with a list of allowed regions, an optional AWS account ID,
// and an AWS KMS keyring with the derived 'Is Discovery' property.
awsAccountId = "1234..."
discoveryKeyring = MakeAwsKmsKeyring()
regions = ["us-west-2", "us-east-1", ...]
filteringKeying = new FilteringKeyring(discoveryKeyring, regions, awsAccountId)

// A keyring-producing operation simplifies this
filteringKeying = MakeFilteringKeyring(regions, awsAccountId)

// The filter process is implemented inside the filtering keyring's OnDecrypt API.
// For the purpose of this example, only consider encrypted data keys
OnDecrypt(edks)
  // Initialize a separate list for the encrypted data keys
  // that the underlying keyring will *attempt* to decrypt.
  // Do not mutate the provided list of encrypted data keys (edks) directly.
  allowedEdks = []

  // Intentionally making this O(n^2)
  // so the filter/order behavior is clear
  forall region in regions:
    forall edk in edks:
      if (GetRegion(edk) == region):
        if (awsAccountId == null) || (GetAwsAccountId(edk) == awsAccountId):
          allowedEdks.append(edk)

  // Now, call the underlying keyring with the ordered, filtered list
  // of encrypted data keys
  return discoveryKeyring.OnDecrypt(allowedEdks)
```

**Alternative 3: Implement a composable filtering keyring.
The filtering keyring is configured with any keyring
and an operation to filter encrypted data keys.**

1. A filtering keyring MUST be configured
   with another keyring
   and MUST be configured with an operation
   that accepts a list of encrypted data keys
   and return an ordered, filtered list
   of encrypted data keys
   that the underlying keyring MUST process
   (example below).
   1. CON: Encrypted data keys MUST be filtered and ordered
      before they are provided to the underlying keyring’s OnDecrypt API
   2. CON: The AWS KMS keyring with the derived _Is Discovery_ property allows communication
      with any AWS account ID or region
      1. Filtering keyring ensures it is only called with encrypted data keys
         that meet the filter requirements
         1. This could potentially be misused
   3. CON: Another keyring
      1. Increases complexity
      2. Potential for customer confusion
   4. CON: Very wide keyring API surface area
      1. _Any_ encrypted data keys MAY be provided
         1. Data keys MAY be encrypted with an AWS KMS CMK
         2. Data keys MAY be encrypted with something else
   5. CON: Allows any custom filtering code execution
   6. CON: Keyring-producing operations MUST be written to make this simpler
   7. CON: Unclear if this has additional value compared to _Alternative 2_

```
// Write an operation to filter and order encrypted data keys
FilterOrder(edks) returns EDK[]
  // Follow a process similar to Alternative 2,
  // but add additional checks to ensure the encrypted data keys were encrypted
  // with an AWS KMS CMK.
  //
  // Assume there is a way to maintain the requested AWS regions
  // and/or AWS account ID.
  // For example, in object oriented languages, one could write a class
  // that is configured with a list of AWS regions and an AWS account ID.
  //
  // Use the configured list of regions and AWS account ID for filtering/sorting,
  // and return the encrypted data keys
  // that the underlying keyring should *attempt* to decrypt.

// Initialize the filtering keyring with a keyring
// and the filter operation. This is going to be language dependent.
//
// Here, we assume FilterAndOrderAwsKmsCmkEncryptedDataKeys is a class
// that has implemented the FilterOrder(edks) operation
// and is initialized with a list of AWS regions and an AWS account ID.
regions = ["us-west-2", "us-east-1", ...]
awsAccountId = "1234..."
filter = new FilterAndOrderAwsKmsCmkEncryptedDataKeys(regions, awsAccountId)
discoveryKeyring = MakeAwsKmsKeyring()
filteringKeying = new FilteringKeyring(discoveryKeyring, filter)

// A keyring-producing operation MAY simplify this
filteringKeying = MakeAwsKmsDiscoveryFilteringKeyring(regions, awsAccountId)
```

### Issue 8b: Ensure it is possible to efficiently limit decryption to specific CMKs, even if the decryption operation has a large number of CMKs that my credentials have access to

**Alternative 1: _Current Implementation_ -
Each AWS KMS keyring is configured with a single client supplier.
A single client supplier MAY maintain a cache of one or more AWS SDK clients.
Client suppliers can be re-used.
Client suppliers that limit communication to specific AWS regions exist.
The AWS KMS keyring can be configured with a specific list of key names,
but the client supplier the AWS KMS keyring is using needs to be configured
to supply AWS SDK clients for all of the configured key names’ regions.**

1. Customers configure AWS KMS keyrings with a specific client supplier.
   AWS SDK clients are initialized at the keyring’s API runtime,
   but MAY be cached by the client supplier.
   Customers can (manually) re-use a keyring or client supplier.
   Customers limit communication to specific AWS regions
   by using a client supplier.
   Customers can configure an AWS KMS keyring with specific key names,
   but the client supplier could be misconfigured
   and AWS KMS clients MAY not be provided for all key names’ regions.
   1. PRO/CON: Customers can (manually) re-use client suppliers or keyrings
   2. CON: SDK clients are initialized at keyring API runtime
   3. CON: Calling an AWS KMS keyring’s OnEncrypt/OnDecrypt API MAY be computationally expensive
      if an AWS SDK client has not yet been initialized or cached
   4. CON: Multiple client suppliers required for customizing multiple AWS SDK clients
      within a single region
      1. **Code change required for client supplier to filter/customize behavior on AWS KMS CMK inputs**
   5. CON: Client supplier configuration MAY not align with keyring configuration
      1. Keyring is configured with specific key names,
         but client supplier cannot supply clients for all key names’ regions
   6. CON: AWS KMS keyring cannot be configured to only communicate with a specific AWS account

### Issue 10: Ensure the ESDK’s process for communicating with AWS KMS is both memory and network efficient

**Alternative 1: _Current Implementation_ -
An AWS KMS keyring is configured with one or more key names.
AWS SDK clients are initialized/supplied by client suppliers
for each key name’s AWS region.**

1. A client supplier MAY have an AWS SDK client cache.
   If the client supplier has a cache
   and the requested region’s AWS SDK client is in the cache,
   the cached AWS SDK client will be returned.
   If the requested region’s AWS SDK client is not in the cache,
   an AWS SDK client will be initialized.
   Keyrings sharing the same client supplier share the same cache.
   1. PRO: If an AWS KMS keyring is configured with a key name,
      but the key name’s region is hypothetically never requested,
      an AWS SDK client MAY not be initialized for the region
   2. PRO: Network calls are...
      1. Limited to OnEncrypt/OnDecrypt keyring API runtime
      2. Limited to the [AWS KMS Encrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html),
         [AWS KMS Decrypt API](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html),
         and [AWS KMS GenerateDataKey API](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey.html)
   3. PRO/CON: Client suppliers cache AWS SDK clients

### Issue 12a: Have failures that must occur at runtime occur during startup instead of during encrypt/decrypt, whenever possible

**Alternative 1: _Current Implementation_ -
AWS SDK client initialization failures occur during the AWS KMS keyring’s API runtime.**

1. Client suppliers MAY fail to supply/initialize AWS SDK clients.
   These failures occur during the AWS KMS keyring’s API runtime.
   1. **CON: Code change required for client suppliers to initialize AWS SDK clients at keyring configuration**
