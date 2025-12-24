[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Message Header

## Version

0.1.1

### Changelog

- 0.1.2

  - Add requirements to specify that Algorithm Suite be ESDK supported

- 0.1.1

  - Include `aws-crypto-public-key` encryption context key.

- 0.1.0-preview

  - Initial record

## Implementations

- [C](https://github.com/aws/aws-encryption-sdk-c/blob/master/source/header.c)
- [JavaScript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/serialize/src/types.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/structures.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/model/CiphertextHeaders.java)

## Overview

The message header is a component of the [message](message.md).

The purpose of the message header is to define the authenticated metadata required for decryption of the [message body](message-body.md#encrypted-content), including:

- The format of the [message](message.md) and [message body](message-body.md)
- The encrypted data keys needed for decryption of the [message body encrypted content](message-body.md#encrypted-content)
- The [encryption context](../framework/structures.md#encryption-context)

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Structure

The message header is a sequence of bytes that MUST be in big-endian format.
The following table describes the fields that form the header.
The bytes are appended in the order shown.

| Field                                           | Length (bytes) | Interpreted as                                  |
| ----------------------------------------------- | -------------- | ----------------------------------------------- |
| [Header Body](#header-body)                     | Variable       | [Header Body](#header-body)                     |
| [Header Authentication](#header-authentication) | Variable       | [Header Authentication](#header-authentication) |

### Header Body

#### Header Body Version 1.0

The following table describes the fields that form the Version 1.0 header body.
The value of the `Version` field MUST be `01` in the Version 1.0 header body.
The bytes are appended in the order shown.

| Field                                       | Length (bytes)                                                                    | Interpreted as                                                                                |
| ------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| [Version](#version)                         | 1                                                                                 | See [Supported Versions](#supported-versions)                                                 |
| [Type](#type)                               | 1                                                                                 | See [Supported Types](#supported-types)                                                       |
| [Algorithm Suite ID](#algorithm-suite-id)   | 2                                                                                 | See [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) |
| [Message ID](#message-id)                   | 16                                                                                | Bytes                                                                                         |
| [AAD](#aad)                                 | Variable. [Self-describing](#aad).                                                | [AAD](#aad)                                                                                   |
| [Encrypted Data Keys](#encrypted-data-keys) | Variable. Determined by the number of encrypted data keys and the length of each. | [Encrypted Data Keys](#encrypted-data-keys)                                                   |
| [Content Type](#content-type)               | 1                                                                                 | See [Supported Types](#supported-content-types)                                               |
| [Reserved](#reserved)                       | 4                                                                                 | See [Reserved](#reserved)                                                                     |
| [IV Length](#iv-length)                     | 1                                                                                 | UInt8                                                                                         |
| [Frame Length](#frame-length)               | 4                                                                                 | UInt32                                                                                        |

#### Header Body Version 2.0

The following table describes the fields that form the Version 2.0 header body.
The value of the `Version` field MUST be `02` in the Version 2.0 header body.
The bytes are appended in the order shown.

| Field                                         | Length (bytes)                                                                    | Interpreted as                                                                                |
| --------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| [Version](#version)                           | 1                                                                                 | See [Supported Versions](#supported-versions)                                                 |
| [Algorithm Suite ID](#algorithm-suite-id)     | 2                                                                                 | See [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) |
| [Message ID](#message-id)                     | 32                                                                                | Bytes                                                                                         |
| [AAD](#aad)                                   | Variable. [Self-describing](#aad).                                                | [AAD](#aad)                                                                                   |
| [Encrypted Data Keys](#encrypted-data-keys)   | Variable. Determined by the number of encrypted data keys and the length of each. | [Encrypted Data Keys](#encrypted-data-keys)                                                   |
| [Content Type](#content-type)                 | 1                                                                                 | See [Supported Types](#supported-content-types)                                               |
| [Frame Length](#frame-length)                 | 4                                                                                 | UInt32                                                                                        |
| [Algorithm Suite Data](#algorithm-suite-data) | Variable. Determined by the Algorithm Suite ID                                    | See [Algorithm Suite Data](../framework/algorithm-suites.md#algorithm-suite-data)             |

#### Version

The version of the message format.
The version (hex) of this field MUST be a value that exists in the following table:

##### Supported Versions

| Value (hex) | Version |
| ----------- | ------- |
| 01          | 1.0     |
| 02          | 2.0     |

#### Type

The type of the message format.
The type (hex) of this field MUST be a value that exists in the following table:

##### Supported Types

| Value (hex) | Type                                  |
| ----------- | ------------------------------------- |
| 80          | Customer Authenticated Encrypted Data |

#### Algorithm Suite ID

The identifier for the algorithm suite used when generating the message.
The value (hex) of this field MUST be a value that exists in the
[Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).

#### Message ID

A Message ID MUST uniquely identify the [message](message.md).
While implementations cannot guarantee complete uniqueness,
implementations MUST use a good source of randomness when generating messages IDs in order to make
the chance of duplicate IDs negligible.

The purpose of the message ID is to:

- Protect against accidental reuse of a derived data key or the wearing out of derived data keys in
  the AWS Encryption SDK
- Uniquely identify the [message](message.md)
- Bind the message header to the [message body](message-body.md)
- Provide a mechanism to securely reuse a data key with multiple messages

#### AAD

The Additional Authenticated Data (AAD) for the header.

This AAD is an encoding of the [encryption context](../framework/structures.md#encryption-context).
The bytes are appended in the order shown:

| Field                                             | Length (bytes)                                                                          | Interpreted as                      |
| ------------------------------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------- |
| [Key Value Pairs Length](#key-value-pairs-length) | 2                                                                                       | UInt16                              |
| [Key Value Pairs](#key-value-pairs)               | Variable. Determined by the value of [Key Value Pairs Length](#key-value-pairs-length). | [Key Value Pairs](#key-value-pairs) |

##### Key Value Pairs Length

The length of the [Key Value Pairs](#key-value-pairs) field in bytes.

When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.

##### Key Value Pairs

The encoding of the key-value pairs of the [encryption context](../framework/structures.md#encryption-context),
serialized according to it's [specification for serialization](../framework/structures.md#serialization).

When the [encryption context](../framework/structures.md#encryption-context) is empty,
this field MUST NOT be included in the [AAD](#aad).

#### Encrypted Data Keys

The encoding of the [encrypted data keys](../framework/structures.md#encrypted-data-key).

The following table describes the fields that form the encrypted data keys.
The bytes are appended in the order shown.

| Field                                                     | Length, in bytes                                                     | Interpreted as                                            |
| --------------------------------------------------------- | -------------------------------------------------------------------- | --------------------------------------------------------- |
| [Encrypted Data Key Count](#encrypted-data-key-count)     | 2                                                                    | UInt16                                                    |
| [Encrypted Data Key Entries](#encrypted-data-key-entries) | Variable. Determined by the count and length of each key-value pair. | [Encrypted Data Key Entries](#encrypted-data-key-entries) |

#### Encrypted Data Key Count

The number of encrypted data keys.
This value MUST be greater than 0.

#### Encrypted Data Key Entries

A sequence of one or more encrypted data key entries.

The following table describes the fields that form each encrypted data key entry.
The bytes are appended in the order shown.

| Field                                                               | Length, in bytes                                                                                  | Interpreted as      |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------------------- |
| [Key Provider ID Length](#key-provider-id-length)                   | 2                                                                                                 | UInt16              |
| [Key Provider ID](#key-provider-id)                                 | Variable. Equal to the value specified in the previous 2 bytes (Key Provider ID Length).          | UTF-8 encoded bytes |
| [Key Provider Information Length](#key-provider-information-length) | 2                                                                                                 | UInt16              |
| [Key Provider Information](#key-provider-information)               | Variable. Equal to the value specified in the previous 2 bytes (Key Provider Information Length). | Bytes               |
| [Encrypted Data Key Length](#encrypted-data-key-length)             | 2                                                                                                 | UInt16              |
| [Encrypted Data Key](#encrypted-data-key)                           | Variable. Equal to the value specified in the previous 2 bytes (Encrypted Data Key Length).       | Bytes               |

##### Key Provider ID Length

The length of the key provider ID.

##### Key Provider ID

The key provider ID.
See [the specification for encrypted data keys](../framework/structures.md#encrypted-data-key).

##### Key Provider Information Length

The length of the key provider information.

##### Key Provider Information

The key provider information.
See [the specification for encrypted data keys](../framework/structures.md#encrypted-data-key).

##### Encrypted Data Key Length

The length of the encrypted data key.

##### Encrypted Data Key

The encrypted data key.
It is the data key encrypted by the key provider.

#### Content Type

The content type of the [message body](message-body.md).
The value (hex) of this field MUST be a value that exists in the following table:

##### Supported Content Types

| Value (hex) | Content Type                                  |
| ----------- | --------------------------------------------- |
| 01          | [Non-Framed](message-body.md#non-framed-data) |
| 02          | [Framed](message-body.md#framed-data)         |

### Algorithm Suite Data

A variable length byte sequence interpreted according to the algorithm suite.
The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.

#### Reserved

A reserved sequence of 4 bytes
that MUST have the value (hex) of `00 00 00 00`.

#### IV Length

The length of the initialization vector (IV).
This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
[algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.

#### Frame Length

The length of the [encrypted content](message-body.md#encrypted-content) within each [regular frame](message-body.md#regular-frame) of framed content.
When the [content type](#content-type) is non-framed, the value of this field MUST be 0.

### Header Authentication

The header authentication contains fields used for authentication of the [header body](#header-body).

#### Header Authentication Version 1.0

The following table describes the fields that form the header authentication.
The bytes are appended in the order shown.

| Field                                     | Length, in bytes                                                                                                                                                                              | Interpreted as |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| [IV](#iv)                                 | Variable. Determined by the IV length value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.                      | Bytes          |
| [Authentication Tag](#authentication-tag) | Variable. Determined by the byte value of the authentication tag of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field. | Bytes          |

#### Header Authentication Version 2.0

The following table describes the fields that form the header authentication.
The bytes are appended in the order shown.

| Field                                     | Length, in bytes                                                                                                                                                                              | Interpreted as |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| [Authentication Tag](#authentication-tag) | Variable. Determined by the byte value of the authentication tag of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field. | Bytes          |

#### IV

The initialization vector (IV) used as input to calculate the [authentication tag](#authentication-tag).

#### Authentication Tag

The authentication value for the header.
The [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field
[determines how the value of this field is calculated](../client-apis/encrypt.md),
and uses this value to [authenticate the contents of the header during decryption](../client-apis/decrypt.md).

## Appendix

### Example Structure Definition

Below is an example definition of the message header format described in this document,
described in a "ASN.1-like" format.

```
DEFINITIONS ::= BEGIN

Version             OCTET STRING (0x01)

Type                OCTET STRING (0x80)

AlgorithmId         OCTET STRING (
    0x00 0x14, 0x00 0x46, 0x00 0x78,
    0x01 0x14, 0x01 0x46, 0x01 0x78,
    0x02 0x14, 0x02 0x46, 0x02 0x78,
)

MessageId           OCTET STRING (SIZE(16))

KeyValuePair        SEQUENCE (SIZE(4)) {
    KeyLength   UINT16,
    Key         UTF8String (SIZE(0..2^16-1)),
    ValueLength UINT16,
    Value       UTF8String (SIZE(0..2^16-1)),
}

AAD                 SEQUENCE (SIZE(1..2)) {
    KeyValuePairsLength UINT16,
    KeyValuePairs       SEQUENCE (SIZE(2)) {
        KeyValuePairCount   UINT16,
        KeyValuePairEntries SEQUENCE (SIZE(1..2^16-1)) of KeyValuePair
    }
}

EncryptedDataKey    SEQUENCE (SIZE(6)) {
    ProviderIdLength        UINT16,
    ProviderId              UTF8String (SIZE(0..2^16-1)),
    KeyInfoLength           UINT16,
    KeyInfo                 OCTET STRING (SIZE(0..2^16-1)),
    EncryptedDataKeyLength  UINT16,
    EncryptedDataKey        OCTET STRING (SIZE(0..2^16-1)),
}

EncryptedDataKeys   SEQUENCE (SIZE(2)) {
    EncryptedDataKeyCount   UINT16,
    EncryptedDataKeyEntries SEQUENCE (SIZE(1..2^16-1)) of EncryptedDataKey
}

ContentType         OCTET STRING (0x01, 0x02)

Reserved            OCTET STRING (0x00 0x00 0x00 0x00)

IvLength            UINT8

FrameLength         UINT32

HeaderBody          SEQUENCE (SIZE(10)) {
    Version             Version,
    Type                Type,
    AlgorithmId         AlgorithmId,
    MessageId           MessageId,
    AAD                 AAD,
    EncryptedDataKeys   EncryptedDataKeys,
    ContentType         ContentType,
    Reserved            Reserved,
    IvLength            IvLength,
    FrameLength         FrameLength,
}

HeaderAuth          SEQUENCE (SIZE(2)) {
    Iv                  OCTET STRING,
    AuthTag             OCTET STRING,
}

Header          SEQUENCE (SIZE(2)) {
    HeaderBody          HeaderBody,
    HeaderAuth          HeaderAuth,
}
```

### Example Bytes

The example below shows the raw bytes of an example header, in hexadecimal notation,
followed by a description of what those bytes represent.

```
01                                         Version (1.0)
80                                         Type (128, customer authenticated encrypted data)
0378                                       Algorithm Suite ID (see Algorithm Suites Reference)
B8929B01 753D4A45 C0217F39 404F70FF        Message ID (random 128-bit value)
008E                                       AAD Length (142)
0004                                       AAD Key-Value Pair Count (4)
0005                                       AAD Key-Value Pair 1, Key Length (5)
30746869 73                                AAD Key-Value Pair 1, Key ("0This")
0002                                       AAD Key-Value Pair 1, Value Length (2)
6973                                       AAD Key-Value Pair 1, Value ("is")
0003                                       AAD Key-Value Pair 2, Key Length (3)
31616E                                     AAD Key-Value Pair 2, Key ("1an")
000A                                       AAD Key-Value Pair 2, Value Length (10)
656E6372 79774690 6F6E                     AAD Key-Value Pair 2, Value ("encryption")
0008                                       AAD Key-Value Pair 3, Key Length (8)
32636F6E 74657874                          AAD Key-Value Pair 3, Key ("2context")
0007                                       AAD Key-Value Pair 3, Value Length (7)
6578616D 706C65                            AAD Key-Value Pair 3, Value ("example")
0015                                       AAD Key-Value Pair 4, Key Length (21)
6177732D 63727970 746F2D70 75626C69        AAD Key-Value Pair 4, Key ("aws-crypto-public-key")
632D6B65 79
0044                                       AAD Key-Value Pair 4, Value Length (68)
41734738 67473949 6E4C5075 3136594B        AAD Key-Value Pair 4, Value ("AsG8gG9InLPu16YKlqXTOD+nykG8YqHAhqecj8aXfD2e5B4gtVE73dZkyClA+rAMOQ==")
6C715854 4F442B6E 796B4738 59714841
68716563 6A386158 66443265 35423467
74564537 33645A6B 79436C41 2B72414D
4F513D3D
0002                                       Encrypted Data Key Count (2)
0007                                       Encrypted Data Key 1, Key Provider ID Length (7)
6177732D 6B6D73                            Encrypted Data Key 1, Key Provider ID ("aws-kms")
004B                                       Encrypted Data Key 1, Key Provider Information Length (75)
61726E3A 6177733A 6B6D733A 75732D77        Encrypted Data Key 1, Key Provider Information ("arn:aws:kms:us-west-2:111122223333:key/715c0818-5825-4245-a755-138a6d9a11e6")
6573742D 323A3131 31313232 32323333
33333A6B 65792F37 31356330 3831382D
35383235 2D343234 352D6137 35352D31
33386136 64396131 316536
00A7                                       Encrypted Data Key 1, Encrypted Data Key Length (167)
01010200 7857A1C1 F7370545 4ECA7C83        Encrypted Data Key 1, Encrypted Data Key
956C4702 23DCE8D7 16C59679 973E3CED
02A4EF29 7F000000 7E307C06 092A8648
86F70D01 0706A06F 306D0201 00306806
092A8648 86F70D01 0701301E 06096086
48016503 04012E30 11040C28 4116449A
0F2A0383 659EF802 0110803B B23A8133
3A33605C 48840656 C38BCB1F 9CCE7369
E9A33EBE 33F46461 0591FECA 947262F3
418E1151 21311A75 E575ECC5 61A286E0
3E2DEBD5 CB005D
0007                                       Encrypted Data Key 2, Key Provider ID Length (7)
6177732D 6B6D73                            Encrypted Data Key 2, Key Provider ID ("aws-kms")
004E                                       Encrypted Data Key 2, Key Provider Information Length (78)
61726E3A 6177733A 6B6D733A 63612D63        Encrypted Data Key 2, Key Provider Information ("arn:aws:kms:ca-central-1:111122223333:key/9b13ca4b-afcc-46a8-aa47-be3435b423ff")
656E7472 616C2D31 3A313131 31323232
32333333 333A6B65 792F3962 31336361
34622D61 6663632D 34366138 2D616134
372D6265 33343335 62343233 6666
00A7                                       Encrypted Data Key 2, Encrypted Data Key Length (167)
01010200 78FAFFFB D6DE06AF AC72F79B        Encrypted Data Key 2, Encrypted Data Key
0E57BD87 3F60F4E6 FD196144 5A002C94
AF787150 69000000 7E307C06 092A8648
86F70D01 0706A06F 306D0201 00306806
092A8648 86F70D01 0701301E 06096086
48016503 04012E30 11040CB2 A820D0CC
76616EF2 A6B30D02 0110803B 8073D0F1
FDD01BD9 B0979082 099FDBFC F7B13548
3CC686D7 F3CF7C7A CCC52639 122A1495
71F18A46 80E2C43F A34C0E58 11D05114
2A363C2A E11397
01                                         Content Type (1, non-framed data)
00000000                                   Reserved
0C                                         IV Length (12)
00000000                                   Frame Length (0, non-framed data)
734C1BBE 032F7025 84CDA9D0                 IV
2C82BB23 4CBF4AAB 8F5C6002 622E886C        Authentication Tag
```
