[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Commitment Policy

## Version

0.3.0

### Changelog

- 0.3.0

  - Add commitment policy for DBE.

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                        |
| ---------- | -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0                                  | 0.1.0                     | [session.c](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/session.c)                                                 |
| NodeJS     | 0.1.0                                  | 0.1.0                     | [index.ts](https://github.com/aws/aws-encryption-sdk-javascript/blob/master/modules/client-node/src/index.ts)                         |
| Browser JS | 0.1.0                                  | 0.1.0                     | [index.ts](https://github.com/aws/aws-encryption-sdk-javascript/blob/master/modules/client-browser/src/index.ts)                      |
| Python     | 0.1.0                                  | 0.1.0                     | [\_\_init\_\_.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/__init__.py)                    |
| Java       | 0.1.0                                  | 0.1.0                     | [AwsCrypto.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/AwsCrypto.java) |

## Overview

This document describes the the Commitment Policy for the Material Providers Library.

The top level client supports configuration settings
that need to be coordinated between encrypt and decrypt.
Coordinating static settings between encrypt and decrypt across hosts is complicated.
It is important that all messages that could be sent to a host can be decrypted by that host.
A top level client makes such settings [hard to misuse](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/tenets.md#hard-to-misuse)
because anything a client encrypts can be decrypted by the same client.

## Supported Format Commitment Policy ENUM

The following tables include the commitment policies
supported by the Material Providers Library
for each [supported format](./algorithm-suites.md#supported-formats).

The Material Providers Library MUST provide
a distinct commitment policy ENUM for each format.
The `Format` Commitment Policy ENUM, where `Format`
is a value from [supported format short name](./algorithm-suites.md#supported-formats).
This ENUM can be used to configure
which Commitment Policies it supports.

| ESDK Commitment Policy ENUM     |
| ------------------------------- |
| FORBID_ENCRYPT_ALLOW_DECRYPT    |
| REQUIRE_ENCRYPT_ALLOW_DECRYPT   |
| REQUIRE_ENCRYPT_REQUIRE_DECRYPT |

| DBE Commitment Policy ENUM      |
| ------------------------------- |
| REQUIRE_ENCRYPT_REQUIRE_DECRYPT |

## Supported Commitment Policy ENUM

The Material Providers Library also MUST provide
a union ENUM for all distinct commitment policy ENUMs
called the Commitment Policy ENUM.
In this specification this union Commitment Policy ENUM
will be denoted as `Format.FormatENUM`
to uniquely identify an Commitment Policy ENUM across all supported formats.
For example `ESDK.FORBID_ENCRYPT_ALLOW_DECRYPT`
is the Commitment Policy ENUM for the ESDK Commitment Policy ENUM `FORBID_ENCRYPT_ALLOW_DECRYPT`.
This means that different formats MAY have duplicate Format Commitment Policy ENUM.

| Algorithm Suite ENUM                 |
| ------------------------------------ |
| ESDK.FORBID_ENCRYPT_ALLOW_DECRYPT    |
| ESDK.REQUIRE_ENCRYPT_ALLOW_DECRYPT   |
| ESDK.REQUIRE_ENCRYPT_REQUIRE_DECRYPT |
| DBE.REQUIRE_ENCRYPT_REQUIRE_DECRYPT  |

#### ESDK.FORBID_ENCRYPT_ALLOW_DECRYPT

When the commitment policy `ESDK.FORBID_ENCRYPT_ALLOW_DECRYPT` is configured:

- `ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF` MUST be the default algorithm suite
- [Get Encryption Materials](./cmm-interface.md#get-encryption-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of False
- [Decrypt Materials](./cmm-interface.md#decrypt-materials) MUST support all algorithm suites

#### ESDK.REQUIRE_ENCRYPT_ALLOW_DECRYPT

When the commitment policy `ESDK.REQUIRE_ENCRYPT_ALLOW_DECRYPT` is configured:

- `05 78` MUST be the default algorithm suite
- [Get Encryption Materials](./cmm-interface.md#get-encryption-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of True
- [Decrypt Materials](./cmm-interface.md#decrypt-materials) MUST support all algorithm suites

#### ESDK.REQUIRE_ENCRYPT_REQUIRE_DECRYPT

When the commitment policy `ESDK.REQUIRE_ENCRYPT_REQUIRE_DECRYPT` is configured:

- `05 78` MUST be the default algorithm suite
- [Get Encryption Materials](./cmm-interface.md#get-encryption-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of True
- [Decrypt Materials](./cmm-interface.md#decrypt-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of True

#### DBE.REQUIRE_ENCRYPT_REQUIRE_DECRYPT

The commitment policy `ESDK.REQUIRE_ENCRYPT_REQUIRE_DECRYPT` is always configured for DBE,
and results in the following requirements:

- `67 01` MUST be the default algorithm suite
- [Get Encryption Materials](./cmm-interface.md#get-encryption-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of True
- [Decrypt Materials](./cmm-interface.md#decrypt-materials) MUST only support algorithm suites that have a [Key Commitment](./algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) value of True
