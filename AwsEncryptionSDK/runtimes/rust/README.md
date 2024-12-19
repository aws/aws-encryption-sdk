# AWS Encryption SDK for Rust

[![build status](https://github.com/aws/aws-encryption-sdk-dafny/actions/workflows/daily_ci.yml/badge.svg?branch=mainline)](https://github.com/aws/aws-encryption-sdk-dafny/actions/workflows/daily_ci.yml)
[![crates.io](https://img.shields.io/crates/v/aws-esdk.svg)](https://crates.io/crates/aws-esdk)
[![docs](https://docs.rs/aws-esdk/badge.svg)](https://docs.rs/aws-esdk)
[![rustc](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://img.shields.io/badge/rust-1.81%2B-orange.svg)

This is the official [AWS Encryption SDK for Rust](https://crates.io/crates/aws-esdk).

## [CHANGELOG](https://github.com/aws/aws-encryption-sdk-dafny/blob/mainline/AwsEncryptionSDK/runtimes/rust/CHANGELOG.md)

## Overview

The AWS Encryption SDK enables secure client-side encryption. It uses cryptography best practices to protect your data and protect the encryption keys that protect your data. Each data object is protected with a unique data encryption key, and the data encryption key is protected with a key encryption key called a wrapping key. The encryption method returns a single, portable [encrypted message](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/message-format.html) that contains the encrypted data and the encrypted data key, so you don't need to keep track of the data encryption keys for your data. You can use KMS keys in [AWS Key Management Service](https://aws.amazon.com/kms/) (AWS KMS) as wrapping keys. The AWS Encryption SDK also provides APIs to define and use encryption keys from other key providers.

For more details about the design and architecture of the AWS Encryption SDK, see the [AWS Encryption SDK Developer Guide](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/introduction.html).

## Examples for AWS Encryption SDK in Rust

Please look at the Examples on how to use the Encryption SDK in Rust [here](https://github.com/aws/aws-encryption-sdk-dafny/tree/mainline/releases/rust/esdk/examples).

Please note that some examples MAY require internet access and valid AWS credentials, since calls to KMS are made.

## License

This library is licensed under the Apache 2.0 License.
