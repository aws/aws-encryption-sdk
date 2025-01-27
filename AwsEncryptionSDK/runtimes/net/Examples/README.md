# AWS Encryption SDK for .NET Examples

This section features examples that show you
how to use the AWS Encryption SDK.
We demonstrate how to use the encryption and decryption APIs
and how to set up some common configuration patterns.

## APIs

The AWS Encryption SDK provides two high-level APIs:
one-step APIs that process the entire operation in memory
and streaming APIs.

You can find examples that demonstrate these APIs
in the [`Examples/`](./) directory.

* [How to encrypt and decrypt](./Keyring/AwsKmsKeyringExample.cs)
* [How to change the algorithm suite](./NonSigningAlgorithmSuiteExample.cs)
* [How to set the commitment policy](./CommitmentPolicy.cs)
* [How to limit the number of encrypted data keys (EDKs)](./LimitEncryptedDataKeysExample.cs)

## Configuration

To use the encryption and decryption APIs,
you need to describe how you want the library to protect your data keys.
You can do this by configuring
[keyrings](#keyrings) or [cryptographic materials managers](#cryptographic-materials-managers).
These examples will show you how to use the configuration tools that we include for you
and how to create some of your own.
We start with AWS KMS examples, then show how to use other wrapping keys.

* Using AWS Key Management Service (AWS KMS)
    * [How to use one AWS KMS key](./Keyring/AwsKmsKeyringExample.cs)
    * [How to use multiple AWS KMS keys in different regions](./Keyring/AwsKmsMrkDiscoveryMultiKeyringExample.cs)
    * [How to decrypt when you don't know the AWS KMS key](./Keyring/AwsKmsDiscoveryKeyringExample.cs)
    * [How to limit decryption to a single region](./Keyring/AwsKmsMrkDiscoveryKeyringExample.cs)
    * [How to decrypt with a preferred region but failover to others](./Keyring/AwsKmsMrkDiscoveryMultiKeyringExample.cs)
    * [How to reproduce the behavior of an AWS KMS master key provider](./Keyring/AwsKmsMultiKeyringExample.cs)
    * [How to establish a Key Hierarchy with AWS KMS and Amazon DynamoDB](./Keyring/AwsKmsHierarchical/AwsKmsHierarchicalKeyring.cs)
* Using raw wrapping keys
    * [How to use a raw AES wrapping key](./Keyring/RawAESKeyringExample.cs)
    * [How to use a raw RSA wrapping key](./Keyring/RawRSAKeyringExample.cs)
* Combining wrapping keys
    * [How to combine AWS KMS with an offline escrow key](./Keyring/MultiKeyringExample.cs)
* How to restrict algorithm suites
    * [with a custom cryptographic materials manager](./CryptographicMaterialsManager/RestrictAlgorithmSuite/SigningSuiteOnlyCMM.cs)

### Keyrings

Keyrings are the most common way for you to configure the AWS Encryption SDK.
They determine how the AWS Encryption SDK protects your data.
You can find these examples in [`Examples/Keyring`](./Keyring).

### Cryptographic Materials Managers

Keyrings define how your data keys are protected,
but there is more going on here than just protecting data keys.

Cryptographic materials managers give you higher-level controls
over how the AWS Encryption SDK protects your data.
This can include things like
enforcing the use of certain algorithm suites or encryption context settings,
reusing data keys across messages,
or changing how you interact with keyrings.
You can find these examples in
[`Examples/CryptographicMaterialsManager`](./CryptographicMaterialsManager).

### Client Supplier

The AWS Encryption SDK creates AWS KMS clients when interacting with AWS KMS.
In case the default AWS KMS client configuration doesn't suit your needs,
you can configure clients by defining a custom Client Supplier.
For example, your Client Supplier could tune
the retry and timeout settings on the client, or use different credentials
based on which region is being called. In our
[RegionalRoleClientSupplier](./ClientSupplier/RegionalRoleClientSupplier.cs)
example, we show how you can build a custom Client Supplier which
creates clients by assuming different IAM roles for different regions.

# Writing Examples

If you want to contribute a new example, that's awesome!
To make sure that your example is tested in our CI,
please make sure that it meets the following requirements:

1. The example MUST be a distinct subdirectory or file in the [`Examples/`](./) directory.
1. The example MAY be nested arbitrarily deeply.
1. Each example file MUST contain exactly one example.
1. Each example filename MUST be descriptive.
1. Each example file MUST contain a public class matching the filename, 
   with a method called `Run` that runs the example.
1. Each example MUST be exercised by a `[Fact]` test method within its class that invokes `Run`,
   providing only the results of methods from the [`ExampleUtils`](./ExampleUtils/ExampleUtils.cs) class.
