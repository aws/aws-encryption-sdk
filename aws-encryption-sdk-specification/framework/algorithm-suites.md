[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Algorithm Suites

## Version

0.4.0

### Changelog

- 0.4.0

  - Reframe "supported libraries" as "supported formats"
  - Add suites with symmetric signing
  - Add DBE as supported format

- 0.3.0
  - [Material Providers Library (MPL)](../changes/2022-06-19_seperate_material_providers/change.md)

## Implementations

| Language   | Confirmed Compatible with Spec Version | Minimum Version Confirmed | Implementation                                                                                                                                                      |
| ---------- | -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C          | 0.1.0                                  | n/a                       | [cipher.c](https://github.com/awslabs/aws-encryption-sdk-c/blob/master/source/cipher.c)                                                                             |
| NodeJS     | 0.1.0                                  | n/a                       | [node_algorithms.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/node_algorithms.ts)                       |
| Browser JS | 0.1.0                                  | n/a                       | [web_crypto_algorithms.ts](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/material-management/src/web_crypto_algorithms.ts)           |
| Python     | 0.1.0                                  | n/a                       | [identifiers.py](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/identifiers.py)                                                |
| Java       | 0.1.0                                  | n/a                       | [CryptoAlgorithm.java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/CryptoAlgorithm.java)                   |
| Dafny      | 0.1.0                                  | n/a                       | [AlgorithmSuites.dfy](https://github.com/aws/aws-encryption-sdk-dafny/blob/mainline/src/AwsCryptographicMaterialProviders/AlgorithmSuites.dfy)                      |
| Java       | 0.3.0                                  | 0.3.0                     | [AlgorithmSuite.java](https://github.com/aws/amazon-s3-encryption-client-java/blob/main/src/main/java/software/amazon/encryption/s3/algorithms/AlgorithmSuite.java) |

## Overview

An algorithm suite is a collection of cryptographic algorithms and related values.
The algorithm suite defines the behaviors [supported formats](#supported-formats) MUST follow for cryptographic operations.

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### AES

Specification: [NIST FIPS 297](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)

The Advanced Encryption Standard (AES) is a symmetric block cipher encryption algorithm.

### GCM

Specification: [NIST Special Publication 800-38D](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf)

Galois/Counter Mode is a mode of operation for block ciphers that provides authenticated encryption with additional data (AEAD).

If specified to use GCM, the AWS Encryption SDK MUST use GCM with the following specifics:

- The internal block cipher is the encryption algorithm specified by the algorithm suite.

### CBC

Specification: [NIST Special Publication 800-38A](https://nvlpubs.nist.gov/nistpubs/legacy/sp/nistspecialpublication800-38a.pdf)

Cipher Block Chaining is a mode of operation for symmetric key block ciphers that provides data confidentiality but does not provide authenticated encryption.

If specified to use CBC, the S3 Encryption Client MUST use CBC with the following specifics:

- CBC MUST only be used to decrypt existing ciphertexts.
- CBC MUST NOT be used to encrypt new ciphertexts.

### CTR

Specification: [NIST Special Publication 800-38A](https://nvlpubs.nist.gov/nistpubs/legacy/sp/nistspecialpublication800-38a.pdf)

Counter is a mode of operation for symmetric key block ciphers that provides data confidentiality but does not provide authenticated encryption.

If specified to use CTR, the S3 Encryption Client MUST use CTR with the following specifics:

- CTR MUST only be used to decrypt existing ciphertexts.
- CTR MUST NOT be used to encrypt new ciphertexts.

### Identity KDF

The identity key derivation function (Identity KDF) is a key derivation algorithm.

The Identity KDF MUST take a byte sequence as input,
and MUST return the input, unchanged, as output.

If included in the algorithm suite,
the algorithm suite's encryption key length MUST equal the algorithm suite's [key derivation input length](#key-derivation-input-length).

### HKDF

Specification: [RFC 5869](https://tools.ietf.org/html/rfc5869)

The HMAC-based extract-and-expand key derivation function (HKDF) is a key derivation algorithm.

## Supported Formats

The following table inclues the cryptographic formats supported by the Material Providers Library.
Both short and long name MUST be unique.

| Cryptographic Format (long)                  | Cryptographic Format (short) |
| -------------------------------------------- | ---------------------------- |
| AWS Encryption SDK Message Format            | ESDK                         |
| S3 Encryption Client Cryptographic Format    | S3EC                         |
| AWS Database Encryption Cryptographic Format | DBE                          |

## Supported Format Algorithm Suites ENUM

The following tables includes the algorithm suites
supported by the Material Providers Library
for each [supported format](#supported-formats).
The Material Providers Library MUST provide
a set of algorithm suite ENUM for each [supported format](#supported-format).

| ESDK Algorithm Suite ENUM                         |
| ------------------------------------------------- |
| ALG_AES_128_GCM_IV12_TAG16_NO_KDF                 |
| ALG_AES_192_GCM_IV12_TAG16_NO_KDF                 |
| ALG_AES_256_GCM_IV12_TAG16_NO_KDF                 |
| ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256            |
| ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256            |
| ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256            |
| ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256 |
| ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 |
| ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 |
| ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY            |
| ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384 |

| S3 EC Algorithm Suite ENUM        |
| --------------------------------- |
| ALG_AES_256_CBC_IV16_NO_KDF       |
| ALG_AES_256_CTR_IV16_TAG16_NO_KDF |
| ALG_AES_256_GCM_IV12_TAG16_NO_KDF |

| DBE Algorithm Suite ENUM                                             |
| -------------------------------------------------------------------- |
| ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            |
| ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 |

## Supported Algorithm Suites ENUM

The Material Providers Library MUST provide
an ENUM that is the super set of all the [supported format algorithm suites enum](#supported-format-algorithm-suites-enum)
called the Algorithm Suite ENUM.
In this specification this Algorithm Suite ENUM
will be denoted as `Format.FormatENUM`
to uniquely identify an Algorithm Suite ENUM across all supported formats.
For example `ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF`
is the Algorithm Suite ENUM for the ESDK Algorithm Suite ENUM `ALG_AES_128_GCM_IV12_TAG16_NO_KDF`.
This means that different formats MAY have duplicate Format Algorithm Suite ENUM.

| Algorithm Suite ENUM                                                     |
| ------------------------------------------------------------------------ |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                   |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                              |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                   |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                   |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                   |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                              |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                              |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                              |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                   |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                   |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                   |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                         |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                   |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                   |
| DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            |
| DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 |

## Supported Algorithm Suites

The following table includes all supported algorithm suites.
The value `00 00` is reserved
and MUST NOT be used
as an Algorithm Suite ID in the future.
Algorithm Suite ID MUST be a unique hex value across all supported algorithm suites.

| Algorithm Suite ENUM                                                     | Algorithm Suite ID (hex) | Message Format Version | Algorithm Suite Data Length (bytes) |
| ------------------------------------------------------------------------ | ------------------------ | ---------------------- | ----------------------------------- |
| DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | 67 00                    | 1.0                    | N/A                                 |
| DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | 67 01                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                   | 05 78                    | 2.0                    | 32                                  |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                              | 04 78                    | 2.0                    | 32                                  |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                   | 03 78                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                   | 03 46                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                   | 02 14                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                              | 01 78                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                              | 01 46                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                              | 01 14                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                   | 00 78                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                   | 00 46                    | 1.0                    | N/A                                 |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                   | 00 14                    | 1.0                    | N/A                                 |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                         | 00 70                    | 1.0                    | N/A                                 |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                   | 00 71                    | 1.0                    | N/A                                 |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                   | 00 72                    | 1.0                    | N/A                                 |

## Algorithm Suites Encryption Key Derivation Settings

The following table includes key derivation information for supported algorithm suites.

| Algorithm Suite ENUM                                                       | Key Derivation Input Length (bits) | Algorithm    | Hash Function | Salt Length (bits) | Key Commitment |
| -------------------------------------------------------------------------- | ---------------------------------- | ------------ | ------------- | ------------------ | -------------- |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | 256                                | HKDF         | SHA-512       | N/A                | True           |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | 256                                | HKDF         | SHA-512       | N/A                | True           |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                     | 256                                | HKDF         | SHA-512       | 256                | True           |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                                | 256                                | HKDF         | SHA-512       | 256                | True           |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | 256                                | HKDF         | SHA-384       | 0                  | False          |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | 192                                | HKDF         | SHA-384       | 0                  | False          |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                     | 128                                | HKDF         | SHA-256       | 0                  | False          |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                                | 256                                | HKDF         | SHA-256       | 0                  | False          |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                                | 192                                | HKDF         | SHA-256       | 0                  | False          |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                                | 128                                | HKDF         | SHA-256       | 0                  | False          |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | 256                                | Identity KDF | N/A           | 0                  | False          |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                     | 192                                | Identity KDF | N/A           | 0                  | False          |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                     | 128                                | Identity KDF | N/A           | 0                  | False          |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                           | 256                                | Identity KDF | N/A           | 0                  | False          |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                     | 256                                | Identity KDF | N/A           | 0                  | False          |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | 256                                | Identity KDF | N/A           | 0                  | False          |

## Algorithm Suites Encryption Settings

The following table includes the encryption settings for supported algorithm suites.

| Algorithm Suite ENUM                                                       | Encryption Algorithm | Encryption Algorithm Mode | Encryption Key Length (bits) | IV Length (bytes) | Authentication Tag Length (bytes) |
| -------------------------------------------------------------------------- | -------------------- | ------------------------- | ---------------------------- | ----------------- | --------------------------------- |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | AES                  | GCM                       | 256                          | 12                | 16                                |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                     | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                                | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | AES                  | GCM                       | 192                          | 12                | 16                                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                     | AES                  | GCM                       | 128                          | 12                | 16                                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                                | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                                | AES                  | GCM                       | 192                          | 12                | 16                                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                                | AES                  | GCM                       | 128                          | 12                | 16                                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | AES                  | GCM                       | 256                          | 12                | 16                                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                     | AES                  | GCM                       | 192                          | 12                | 16                                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                     | AES                  | GCM                       | 128                          | 12                | 16                                |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                           | AES                  | CBC                       | 256                          | 16                | N/A                               |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                     | AES                  | CTR                       | 256                          | 16                | N/A                               |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | AES                  | GCM                       | 256                          | 12                | 16                                |

## Algorithm Suites Commit Key Derivation Settings

The following table includes commitment information for supported algorithm suites.
These values are only relevant to algorithm suites that support [key commitment](#key-commitment).

| Algorithm Suite ENUM                                                       | Key Derivation Input Length (bits) | Algorithm | Hash Function | Salt Length (bits) |
| -------------------------------------------------------------------------- | ---------------------------------- | --------- | ------------- | ------------------ |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | 256                                | HKDF      | SHA-512       | N/A                |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | 256                                | HKDF      | SHA-512       | N/A                |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                     | 256                                | HKDF      | SHA-512       | 256                |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                                | 256                                | HKDF      | SHA-512       | 256                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                     | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                                | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                                | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                                | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                     | N/A                                | N/A       | N/A           | N/A                |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                     | N/A                                | N/A       | N/A           | N/A                |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                           | N/A                                | N/A       | N/A           | N/A                |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                     | N/A                                | N/A       | N/A           | N/A                |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | N/A                                | N/A       | N/A           | N/A                |

## Algorithm Suites Signature Settings

The following table includes signature information for supported algorithm suites.

An algorithm suite with a symmetric signature algorithm MUST use [intermediate key wrapping](#intermediate-key-wrapping).

| Algorithm Suite ENUM                                                       | Asymmetric Signature Algorithm | Symmetric Signature Algorithm |
| -------------------------------------------------------------------------- | ------------------------------ | ----------------------------- |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | Not applicable                 | HMAC with SHA-384             |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | ECDSA with P-384 and SHA-384   | HMAC with SHA-384             |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                     | ECDSA with P-384 and SHA-384   | Not appliccable               |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                                | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | ECDSA with P-384 and SHA-384   | Not appliccable               |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | ECDSA with P-384 and SHA-384   | Not appliccable               |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                     | ECDSA with P-256 and SHA-256   | Not appliccable               |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                                | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                                | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                                | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                     | Not applicable                 | Not appliccable               |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                     | Not applicable                 | Not appliccable               |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                           | Not applicable                 | Not appliccable               |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                     | Not applicable                 | Not appliccable               |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | Not applicable                 | Not appliccable               |

## Algorithm Suitees EDK Wrapping Settings

Based on the algorithm suite, there may be additional requirements to the wrapping and serialization of [encrypted data keys (EDKs)](./structures.md#encrypted-data-key).

The following table includes EDK wrapping information for supported algorithm suites.

| Algorithm Suite ENUM                                                       | EDK Wrapping Algorithm    |
| -------------------------------------------------------------------------- | ------------------------- |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384            | Intermediate Key Wrapping |
| DDBEC.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 | Intermediate Key Wrapping |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384                     | Direct Key Wrapping       |
| ESDK.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY                                | Direct Key Wrapping       |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | Direct Key Wrapping       |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384                     | Direct Key Wrapping       |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256                     | Direct Key Wrapping       |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256                                | Direct Key Wrapping       |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256                                | Direct Key Wrapping       |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256                                | Direct Key Wrapping       |
| ESDK.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | Direct Key Wrapping       |
| ESDK.ALG_AES_192_GCM_IV12_TAG16_NO_KDF                                     | Direct Key Wrapping       |
| ESDK.ALG_AES_128_GCM_IV12_TAG16_NO_KDF                                     | Direct Key Wrapping       |
| S3EC.ALG_AES_256_CBC_IV16_NO_KDF                                           | Direct Key Wrapping       |
| S3EC.ALG_AES_256_CTR_IV16_TAG16_NO_KDF                                     | Direct Key Wrapping       |
| S3EC.ALG_AES_256_GCM_IV12_TAG16_NO_KDF                                     | Direct Key Wrapping       |

### Supported EDK Wrapping Algorithms

- [Direct Key Wrapping](#direct-key-wrapping)
- [Intermediate Key Wrapping](#intermediate-key-wrapping)

#### Direct Key Wrapping

[Encrypted data keys](./structures.md#encrypted-data-key) produced under algorithms suites
using Direct Key Wrapping have no additional requirements,
other than those already perscribed by the [EDK Ciphertext specification](./structures.md#encrypted-data-key).

#### Intermediate Key Wrapping

For algorithm suites including symmetric signing, additional steps are needed in order to create material
unique for each encrypted data key that can be used for symmetric signing.
Through the creation of intermediate material during the key wrapping process,
those with access to unwrap a particular encrypted data key also have access to symmetric signing material associated
with that particular encrypted data key.
With this property, a message can be created that is decryptable by several parties,
but also signed by each party such that party A can be sure that the message was not
updated by party B.
Only those with the ability to wrap data keys for all parties are capable of
creating messages that will be authenticated by all parties.

[Encrypted data keys](./structures.md#encrypted-data-key) produced under algorithm suites using Intermediate Key Wrapping,
have the following requirements:

- For each encrypted data key, a distinct `intermediate key` MUST be generated using cryptographically secure random bytes.
  This intermediate key MUST have length equal to the encryption key length of the algorithm suite.
- For each encrypted data key, a [symmetric signing key](./structures.md#symmetric-signing-key) MUST be derived from the `intermediate key`
  using the key derivation algorithm in the algorithm suite, with the following specifics:
  - The input key material is the `intermediate key`
  - The salt is empty
  - The info is "AWS_MPL_INTERMEDIATE_KEYWRAP_MAC" as UTF8 bytes.
- For each encrypted data key, a `key encryption key` MUST be derived from the `intermediate key`
  using the key derivation algorithm in the algorithm suite, with the following specifics:
  - The input key material is the `intermediate key`
  - The salt is empty
  - The info is "AWS_MPL_INTERMEDIATE_KEYWRAP_ENC" as UTF8 bytes.
- The [EDK ciphertext](./structures.md#ciphertext) MUST be the following serialization:

| Field                      | Length (bytes)                                     | Interpreted as |
| -------------------------- | -------------------------------------------------- | -------------- |
| Wrapped Plaintext Data Key | The algorithm suite's encryption key length + 12   | Bytes          |
| Wrapped Intermediate Key   | Determined by the keyring responsible for wrapping | Bytes          |

##### Wrapped Plaintext Data Key

The wrapped plaintext data key MUST be the result of the following AES GCM 256 Encrypt operation:

- Plaintext: the [plaintext data key](./structures.md#plaintext-data-key) in the related encryption or decryption materials.
- Encryption key: The `key encryption key` derived above.
- AAD: The [encryption context](./structures.md#encryption-context) in the related encryption or decryption materials,
  [serialized according to it's specification](structures.md#serialization).
- IV: The IV is 0.

This value MUST be equal to the algorithm suite's encryption key length + 16.

##### Wrapped Intermediate Key

The wrapped intermediate key has the same requirements for wrapping as the
[EDK Ciphertext](./structures.md#encrypted-data-key) normally has under [Direct Key Wrapping](#direct-key-wrapping).

## Structure

The fields described below are REQUIRED to be specified by algorithm suites, unless otherwise specified.

### Algorithm Suite ENUM

A value that uniquely identifies an algorithm suite.

### Algorithm Suite ID

A 2-byte hex value that MUST uniquely identify an algorithm suite.
This 2-byte value SHOULD be used to bind algorithm suites to ciphertext.

### Encryption Algorithm

The block cipher encryption algorithm.

The length of the input encryption key MUST equal the [encryption key length](#encryption-key-length) specified by the algorithm suite.

#### Supported Encryption Algorithms

- [AES](#aes)

### Encryption Algorithm Mode

The AEAD operation mode used with the encryption algorithm.

The length of the input IV MUST equal the IV length specified by the algorithm suite.
The length of the authentication tag MUST equal the authentication tag length specified by the algorithm suite.

#### Supported Encryption Algorithm Modes

- [GCM](#gcm)
- [CBC](#cbc) - legacy, decrypt-only
- [CTR](#ctr) - legacy, decrypt-only

### Encryption Key Length

The length of the encryption key used as input to the encryption algorithm.

### IV Length

The length of the initialization vector (IV) used with the encryption algorithm.

### Authentication Tag Length

The length of the authentication tag used with AEAD.

### Encryption Key Derivation Algorithm

This key derivation algorithm defines what key derivation function (KDF) to use for encryption key generation.
The specified KDF algorithm MUST be used to generate the encryption algorithm encryption key input.

#### Supported Encryption Key Derivation Algorithms

- [Identity KDF](#identity-kdf)
- [HKDF](./transitive-requirements.md#hkdf-encryption-key)

### Key Derivation Input Length

The length of the input to the Key Derivation Algorithm.

### Key Commitment

AES-GCM is not key committing by default.
Key commitment is a property,
which ensures that exactly one data key can be used to decrypt a ciphertext.
However, not all algorithm suites provide this property.

#### Supported Key Commitment Values

- True
- False

### Commit Key

A key that is used to provide [key commitment](#key-commitment) to AES-GCM.

### Commit Key Derivation Algorithm

This key derivation algorithm defines what key derivation function (KDF) to use for commitment key generation.
The specified KDF algorithm MUST be used to generate the [commit key](#commit-key).

#### Supported Commit Key Derivation Algorithm

- [HKDF](./transitive-requirements.md#hkdf-commit-key)

### Commit Key Length

The length of the commit key used to verify [key commitment](#key-commitment).

### Asymmetric Signature Algorithm

This field is OPTIONAL.

The asymmetric signature algorithm defines what asymmetric algorithm to use for asymmetric signature generation and verification.

If the algorithm suite includes an asymmetric signature algorithm:

- Asymmetric signatures MUST be generated using the specified asymmetric signature algorithm.
- Asymmetric signatures MUST be verified using the specified asymmetric signature algorithm.

If the algorithm suite does not include a asymmetric signature algorithm:

- Asymmetric signatures MUST NOT be generated.
- Asymmetric signatures MUST NOT be verified.

#### Supported Asymmetric Signature Algorithms

- [ECDSA with P256 and SHA256](./transitive-requirements.md#ecdsa)
- [ECDSA with P384 and SHA384](./transitive-requirements.md#ecdsa)

### Symmetric Signature Algorithm

This field is OPTIONAL.

The symmetric signature algorithm defines what symmetric algorithm to use for symmetric signature generation and verification.

If the algorithm suite includes an symmetric signature algorithm:

- Symmetric signatures MUST be generated using the specified symmetric signature algorithm.
- Symmetric signatures MUST be verified using the specified symmetric signature algorithm.
- The algorithm suite MUST also use [Intermediate Key Wrapping](#intermediate-key-wrapping).

If the algorithm suite does not include a symmetric signature algorithm:

- Symmetric signatures MUST NOT be generated.
- Symmetric signatures MUST NOT be verified.

#### Supported Symmetric Signature Algorithms

- [HMAC](https://datatracker.ietf.org/doc/html/rfc2104) with [SHA384](https://www.rfc-editor.org/rfc/rfc6234)

### Message Format Version

Indicates the serialization or message format version for the supported algorithm suite.
This value can be duplicated across [supported formats](#supported-formats).
This MUST be used to branch any serialization/deserialization logic in [supported formats](#supported-formats).

#### Supported Message Format Version

**ESDK**

- 1.0
- 2.0

**S3EC**

- 1.0

**DDBEC**

- 1.0

### Algorithm Suite Data

Algorithm suites may capture a variable-per-algorithm-suite length of data
relevant to that algorithm suite’s mode of operation.

#### Supported Algorithm Suite Data

- Algorithm Suite 05 78 MUST store the commit key in the suite data
- Algorithm Suite 04 78 MUST store the commit key in the suite data

### Algorithm Suite Data Length

Then length of the [algorithm suite data](#algorithm-suite-data).
The field MAY be length 0.
Its length and how the contents are used
are determined by the algorithm suite.
Where the contents are stored is determined by the message format.

#### Supported Algorithm Suite Data Lengths

- 32

## Security Considerations

### Which algorithm suite should I use?

You should use the default algorithm suite.

You should use an algorithm suite that supports key commitment.

You may use the non-default AES-GCM with key derivation and signing key algorithm suites
if key derivation input lengths of other sizes are required.

If the users who encrypt and the users who decrypt are equally trusted,
you may use AES-GCM with only key derivation algorithm suites.

You should not use AES-GCM with only key derivation algorithm suites
if the users who encrypt and the users who decrypt are not equally trusted.

You should not use AES-GCM without key Derivation or signing,
except for backwards compatibility.

You should not use AES-CBC unless you need to decrypt legacy ciphertexts encrypted using AES-CBC.

You should not use AES-CTR unless you need to perform partial decryption of a ciphertext object encrypted using AES-GCM in a client that does not support a safe mode of partial ciphertext decryption.
