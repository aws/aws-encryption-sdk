# AWS Encryption SDK for Rust

AWS Encryption SDK for Rust

## Using the AWS Encryption SDK for Rust

The AWS Encryption SDK is available on [Crates.io](https://www.crates.io/).

For more details about the design and architecture of the AWS Encryption SDK, see the [AWS Encryption SDK Developer Guide](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/introduction.html).

## Building the AWS Encryption SDK for Rust

To build, the AWS Encryption SDK requires the most up to date version of [Dafny](https://github.com/dafny-lang/dafny) on your PATH.

You will also need to ensure that you fetch all submodules using either `git clone --recursive ...` when cloning the repository or `git submodule update --init` on an existing clone.

To setup your project to use the AWS Encryption SDK in Rust, run:

```
cd AwsEncryptionSDK
# Polymorph smithy to Rust
make polymorph_rust
# Transpile Dafny to Rust
make transpile_rust
```

### (Optional) Set up the AWS Encryption SDK to work with AWS KMS

If you set up the AWS Encryption SDK to use the AWS KMS Keyring,
the AWS Encryption SDK will make calls to AWS KMS on your behalf,
using the appropriate AWS SDK.

However, you must first set up AWS credentials for use with the AWS SDK.

## Testing the AWS Encryption SDK for Rust

### Configure AWS credentials

To run the test suite you must first set up AWS credentials for use with the AWS SDK.
This is required in order to run the integration tests, which use a KMS Keyring against a publicly accessible KMS CMK.

### Run the tests

Run the test suite with:

```
cd AwsEncryptionSDK
make test_rust
```

Run tests on examples, to ensure they are up to date:

```
cd AwsEncryptionSDK/runtimes/rust/
cargo test --examples
```

Please look at the Examples on how to use the Encryption SDK in Rust [here](examples).

Please note that tests and test vectors require internet access and valid AWS credentials, since calls to KMS are made as part of the test workflow.

## License

This library is licensed under the Apache 2.0 License.
