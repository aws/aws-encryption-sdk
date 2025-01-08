# AWS Encryption SDK for Go Examples

This section features examples that show you
how to use the AWS Encryption SDK.
We demonstrate how to use the encryption and decryption APIs
and how to set up some common configuration patterns.

## APIs

The AWS Encryption SDK provides two high-level APIs:
one-step APIs that process the entire operation in memory
and streaming APIs.

You can find examples that demonstrate these APIs
in the [`examples/`](./) directory.

* [How to encrypt and decrypt](./keyring/awskmskeyring/awskmskeyring.go)
* [How to change the algorithm suite](./misc/setencryptionalgorithmsuite.go)
* [How to set the commitment policy](./misc/commitmentpolicy.go)
* [How to limit the number of encrypted data keys (EDKs)](./misc/limitencrypteddatakeysexample.go)

## Configuration

To use the encryption and decryption APIs,
you need to describe how you want the library to protect your data keys.
You can do this by configuring
[keyrings](#keyrings) or [cryptographic materials managers](#cryptographic-materials-managers).
These examples will show you how to use the configuration tools that we include for you
and how to create some of your own.
We start with AWS KMS examples, then show how to use other wrapping keys.

* Using AWS Key Management Service (AWS KMS)
    * [How to use one AWS KMS key](./keyring/awskmskeyring/awskmskeyring.go)
    * [How to use multiple AWS KMS keys in different regions](./keyring/awskmsmrkmultikeyring/awskmsmrkmultikeyring.go)
    * [How to decrypt when you don't know the AWS KMS key](./keyring/awskmsdiscoverykeyring/awskmsdiscoverykeyring.go)
    * [How to limit decryption to a single region](./keyring/awskmsmrkdiscoverykeyring/awskmsmrkdiscoverykeyring.go)
    * [How to decrypt with a preferred region but failover to others](./keyring/awskmsmrkdiscoverykeyring/awskmsmrkdiscoverykeyring.go)
    * [How to reproduce the behavior of an AWS KMS master key provider](./keyring/awskmsmultikeyring/awskmsmultikeyring.go)
* Using raw wrapping keys
    * [How to use a raw AES wrapping key](./keyring/rawaeskeyring/rawaeskeyring.go)
    * [How to use a raw RSA wrapping key](./keyring/rawrsakeyring/rawrasakeyring.go)
* Combining wrapping keys
    * [How to combine AWS KMS with an offline escrow key](./keyring/multikeyring/multikeyring.go)
* How to restrict algorithm suites
    * [with a custom cryptographic materials manager](./cryptographicmaterialsmanager/restrictalgorithmsuite/signingsuiteonlycmm.go)

### Keyrings

Keyrings are the most common way for you to configure the AWS Encryption SDK.
They determine how the AWS Encryption SDK protects your data.
You can find these examples in [`examples/keyring`](./keyring).

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
[`examples/cryptographic_materials_manager`](./cryptographicmaterialsmanager).

### Client Supplier

The AWS Encryption SDK creates AWS KMS clients when interacting with AWS KMS.
In case the default AWS KMS client configuration doesn't suit your needs,
you can configure clients by defining a custom Client Supplier.
For example, your Client Supplier could tune
the retry and timeout settings on the client, or use different credentials
based on which region is being called. In our
[regional_role_client_supplier](./clientsupplier/regionalroleclientsupplier.go)
example, we show how you can build a custom Client Supplier which
creates clients by assuming different IAM roles for different regions.

# Writing Examples

If you want to contribute a new example, that's awesome!
To make sure that your example runs in our CI,
please make sure that it meets the following requirements:

1. The example MUST be a distinct subdirectory or file in the [`examples/`](./) directory.
1. The example MAY be nested arbitrarily deeply. 
1. Each example file MUST contain exactly one example.
1. Each example filename MUST be descriptive.
1. Each example file MUST contain validation checks to check for expected returned values and MUST panic is the returned value is no expected.
1. Each example MUST also be called inside the `main` function of [main.go](./main.go).