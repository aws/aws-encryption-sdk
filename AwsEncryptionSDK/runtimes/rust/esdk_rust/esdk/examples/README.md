# AWS Encryption SDK for Rust Examples

This section features examples that show you
how to use the AWS Encryption SDK.
We demonstrate how to use the encryption and decryption APIs
and how to set up some common configuration patterns.

## APIs

The AWS Encryption SDK provides a high-level,
one-step API that processes encryption
and decryption operations entirely in memory.

You can find examples that demonstrate these APIs
in the [`examples/`](./) directory.

- [How to encrypt and decrypt](./keyring/aws_kms_keyring_example.rs)
- [How to change the algorithm suite](./set_encryption_algorithm_suite_example.rs)
- [How to set the commitment policy](./set_commitment_policy_example.rs)
- [How to limit the number of encrypted data keys (EDKs)](./limit_encrypted_data_keys_example.rs)

## Configuration

To use the encryption and decryption APIs,
you need to describe how you want the library to protect your data keys.
You can do this by configuring
[keyrings](#keyrings) or [cryptographic materials managers](#cryptographic-materials-managers).
These examples will show you how to use the configuration tools that we include for you
and how to create some of your own.
We start with AWS KMS examples, then show how to use other wrapping keys.

- Using AWS Key Management Service (AWS KMS)
  - [How to use one AWS KMS key](./keyring/aws_kms_keyring_example.rs)
  - [How to use multiple AWS KMS keys in different regions](./keyring/aws_kms_mrk_discovery_multi_keyring_example.rs)
  - [How to decrypt when you don't know the AWS KMS key](./keyring/aws_kms_discovery_keyring_example.rs)
  - [How to limit decryption to a single region](./keyring/aws_kms_mrk_discovery_keyring_example.rs)
  - [How to decrypt with a preferred region but failover to others](./keyring/aws_kms_mrk_discovery_multi_keyring_example.rs)
  - [How to reproduce the behavior of an AWS KMS master key provider](./keyring/aws_kms_multi_keyring_example.rs)
- Using raw wrapping keys
  - [How to use a raw AES wrapping key](./keyring/raw_aes_keyring_example.rs)
  - [How to use a raw RSA wrapping key](./keyring/raw_rsa_keyring_example.rs)
- Combining wrapping keys
  - [How to combine AWS KMS with an offline escrow key](./keyring/multi_keyring_example.rs)
- How to restrict algorithm suites
  - [with a custom cryptographic materials manager](./cryptographic_materials_manager/restrict_algorithm_suite/signing_suite_only_cmm.rs)

### Keyrings

Keyrings are the most common way for you to configure the AWS Encryption SDK.
They determine how the AWS Encryption SDK protects your data.
For more information about keyrings, see the [AWS Developer Guide on using Keyrings](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html).
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
For more information about cryptographic materials managers, see the [AWS Developer Guide on cryptographic materials managers](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#crypt-materials-manager).
You can find these examples in
[`examples/cryptographic_materials_manager`](./cryptographic_materials_manager).

### Client Supplier

The AWS Encryption SDK creates AWS KMS clients when interacting with AWS KMS.
In case the default AWS KMS client configuration doesn't suit your needs,
you can configure clients by defining a custom Client Supplier.
For example, your Client Supplier could tune
the retry and timeout settings on the client, or use different credentials
based on which region is being called. In our
[regional_role_client_supplier](./client_supplier/regional_role_client_supplier.rs)
example, we show how you can build a custom Client Supplier which
creates clients by assuming different IAM roles for different regions.

## Writing Examples

If you want to contribute a new example, that's awesome!
To make sure that your example is tested in our CI,
please make sure that it meets the following requirements:

1. The example MUST be a distinct subdirectory or file in the [`examples/`](./) directory.
1. The example MAY be nested arbitrarily deeply. However, each example file MUST be added to the `mod.rs` files appropriately according to the directory structure. If the example is in the root directory [`examples/`](./), you MUST also add the module to the [`main.rs`](./main.rs) file. For instance, `pub mod set_commitment_policy_example;`.
1. Each example file MUST contain exactly one example.
1. Each example filename MUST be descriptive.
1. Each example file MUST contain a testing function with the attribute `#[tokio::test(flavor = "multi_thread")]` just like the one at the end of the [KMS Keyring](./keyring/aws_kms_keyring_example.rs).
1. Each example MUST also be called inside the `main` function of [main.rs](./main.rs).
