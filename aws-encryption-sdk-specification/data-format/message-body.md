[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Message Body

## Version

See [Message Version](message.md#version).

## Implementations

- [C](https://github.com/awslabs/aws-encryption-sdk-c/blob/master/include/aws/cryptosdk/private/framefmt.h)
- [JavaScript](https://github.com/awslabs/aws-encryption-sdk-javascript/blob/master/modules/serialize/src/types.ts)
- [Python](https://github.com/aws/aws-encryption-sdk-python/blob/master/src/aws_encryption_sdk/structures.py)
- [Java](https://github.com/aws/aws-encryption-sdk-java/blob/master/src/main/java/com/amazonaws/encryptionsdk/model/ContentType.java)

## Overview

The message body is a component of the [message](message.md).  
The message body contains the encrypted data, called the [encrypted content](#encrypted-content).  
The purpose of the message body is to define the structure containing the [encrypted content](#encrypted-content).

The structure of the body depends on the content type:

- [Non-Framed Data](#non-framed-data)
- [Framed Data](#framed-data)

## Definitions

### Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Structure

The following sections describe the format of the message body for each content type.

### Non-Framed Data

Non-framed data is a sequence of encrypted bytes along with the [initialization vector (IV)](#iv)
and body [authentication tag](#authentication-tag).

The following describes the fields that form non-framed data.  
The bytes are appended in the order shown.

| Field                                                 | Length (bytes)                           | Interpreted as |
| ----------------------------------------------------- | ---------------------------------------- | -------------- |
| [IV](#iv)                                             | [IV Length](message-header.md#iv-length) | Bytes          |
| [Encrypted Content Length](#encrypted-content-length) | 8                                        | Uint64         |
| [Encrypted Content](#encrypted-content)               | Variable                                 | Bytes          |
| [Authentication Tag](#authentication-tag)             | Variable                                 | Bytes          |

#### IV

The initialization vector to use with the encryption algorithm.
The IV MUST be a unique IV within the message.

#### Encrypted Content Length

The length of the encrypted content.  
The length MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).

#### Encrypted Content

The encrypted data as returned by the [encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm).

#### Authentication Tag

The authentication value for the body.  
It is used to authenticate the message body.

### Framed Data

Framed data is a sequence of bytes divided into equal-length parts called frames.  
Each frame is encrypted separately with a unique [IV](#iv) and body [authentication tag](#authentication-tag).  
There are two kinds of frames:

- [Regular Frame](#regular-frame)
- [Final Frame](#final-frame)

Note:

- The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
- The number of frames in a single message MUST be less than or equal to `2^32 - 1`.

#### Regular Frame

All frames except the [Final Frame](#final-frame) are "Regular Frames".

The following describes the fields that form the Regular Frame Body Structure.  
The bytes are appended in the order shown.

| Field                                     | Length (bytes)                                                                                               | Interpreted as |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------ | -------------- |
| [Sequence Number](#sequence-number)       | 4                                                                                                            | UInt32         |
| [IV](#iv)                                 | [IV Length](message-header.md#iv-length)                                                                     | Bytes          |
| [Encrypted Content](#encrypted-content)   | Variable                                                                                                     | Bytes          |
| [Authentication Tag](#authentication-tag) | Algorithm suite ID's [Authentication Tag Length](../framework/algorithm-suites.md#authentication-tag-length) | Bytes          |

##### Sequence Number

The Frame sequence number.  
It is an incremental counter number for the frames.  
Framed Data MUST start at Sequence Number 1.  
Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.

##### IV

The initialization vector (IV) for the frame.  
Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
Note: This IV is different from the [Header IV](message-header.md#iv).

##### Encrypted Content

The encrypted data for each frame, as returned by the [encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm).  
The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.

##### Authentication Tag

The authentication value for the frame.  
The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.

#### Final Frame

Framed data MUST contain exactly one final frame.
The final frame MUST be the last frame.

The length of the plaintext to be encrypted in the Final Frame MUST be
greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).

- When the length of the Plaintext is not an exact multiple of the Frame Length,
  any remaining data is encrypted into the Final Frame.
- When the length of the Plaintext is an exact multiple of the Frame Length
  (including if it is equal to the frame length),
  the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
- When the length of the Plaintext is less than the Frame Length,
  the body MUST contain exactly one frame and that frame MUST be a Final Frame.

For example, in the case that the length of the Plaintext is equal to the Frame Length, there are two acceptable cases:

- The body contains exactly one frame, the final frame,
  with a content length equal to the plaintext length.
- The body contains two frames.
  The first frame is a regular frame,
  and the second frame is the final frame with a content length of 0.

The following describes the fields that form the Final Frame Body Structure.  
The bytes are appended in the order shown.

| Field                                                   | Length (bytes)                                                                                               | Interpreted as |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | -------------- |
| [Sequence Number End](#sequence-number-end)             | 4                                                                                                            | Bytes          |
| [Sequence Number](#sequence-number)                     | 4                                                                                                            | UInt32         |
| [IV](#iv)                                               | [IV Length](message-header.md#iv-length)                                                                     | Bytes          |
| [Encrypted Content Length](#encrypted-content-length-1) | 4                                                                                                            | UInt32         |
| [Encrypted Content](#encrypted-content)                 | Variable                                                                                                     | Bytes          |
| [Authentication Tag](#authentication-tag)               | Algorithm suite ID's [Authentication Tag Length](../framework/algorithm-suites.md#authentication-tag-length) | Bytes          |

##### Sequence Number End

An indicator for the Final Frame.  
The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.

##### Sequence Number

The Frame Sequence Number.  
It is an incremental counter number for the frames.
The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.

##### IV

The initialization vector for the final frame.  
The IV MUST be a unique IV within the message.  
The IV length MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
Note: This IV is different from the [Header IV](message-header.md#iv).

##### Encrypted Content Length

The length of the encrypted content.

##### Encrypted Content

The encrypted data for the final frame, as returned by the [encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm).

##### Authentication Tag

The authentication value for the final frame.  
It is used to authenticate the final frame.  
The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.

## Example Usage

The following section contains examples of the message body for [Non-Framed Data](#non-framed-data) and [Framed Data](#framed-data).

### Non-Framed Data

The following example shows the message body format for [Non-Framed Data](#non-framed-data).

#### Example Pseudo-ASN.1 Structure

```
DEFINITIONS ::= BEGIN
NonFramedBody SEQUENCE (SIZE(4)) {
     IV OCTET STRING (SIZE(IvLength)),
     EncryptedContentLength UINT64,
     EncryptedContent OCTET STRING (SIZE(EncryptedContentLength)),
     AuthTag OCTET STRING (SIZE(TagLength)),
}
```

#### Example Bytes

```
39DD3E5 915E0201 77A4AB11                  IV
00000000 0000028E                          Encrypted Content Length (654)
E8B6F955 B5F22FE4 FD890224 4E1D5155        Encrypted Content
5871BA4C 93F78436 1085E4F8 D61ECE28
59455BD8 D76479DF C28D2E0B BDB3D5D3
E4159DFE C8A944B6 685643FC EA24122B
6766ECD5 E3F54653 DF205D30 0081D2D8
55FCDA5B 9F5318BC F4265B06 2FE7C741
C7D75BCC 10F05EA5 0E2F2F40 47A60344
ECE10AA7 559AF633 9DE2C21B 12AC8087
95FE9C58 C65329D1 377C4CD7 EA103EC1
31E4F48A 9B1CC047 EE5A0719 704211E5
B48A2068 8060DF60 B492A737 21B0DB21
C9B21A10 371E6179 78FAFB0B BAAEC3F4
9D86E334 701E1442 EA5DA288 64485077
54C0C231 AD43571A B9071925 609A4E59
B8178484 7EB73A4F AAE46B26 F5B374B8
12B0000C 8429F504 936B2492 AAF47E94
A5BA804F 7F190927 5D2DF651 B59D4C2F
A15D0551 DAEBA4AF 2060D0D5 CB1DA4E6
5E2034DB 4D19E7CD EEA6CF7E 549C86AC
46B2C979 AB84EE12 202FD6DF E7E3C09F
C2394012 AF20A97E 369BCBDA 62459D3E
C6FFB914 FEFD4DE5 88F5AFE1 98488557
1BABBAE4 BE55325E 4FB7E602 C1C04BEE
F3CB6B86 71666C06 6BF74E1B 0F881F31
B731839B CF711F6A 84CA95F5 958D3B44
E3862DF6 338E02B5 C345CFF8 A31D54F3
6920AA76 0BF8E903 552C5A04 917CCD11
D4E5DF5C 491EE86B 20C33FE1 5D21F0AD
6932E67C C64B3A26 B8988B25 CFA33E2B
63490741 3AB79D60 D8AEFBE9 2F48E25A
978A019C FE49EE0A 0E96BF0D D6074DDB
66DFF333 0E10226F 0A1B219C BE54E4C2
2C15100C 6A2AA3F1 88251874 FDC94F6B
9247EF61 3E7B7E0D 29F3AD89 FA14A29C
76E08E9B 9ADCDF8C C886D4FD A69F6CB4
E24FDE26 3044C856 BF08F051 1ADAD329
C4A46A1E B5AB72FE 096041F1 F3F3571B
2EAFD9CB B9EB8B83 AE05885A 8F2D2793
1E3305D9 0C9E2294 E8AD7E3B 8E4DEC96
6276C5F1 A3B7E51E 422D365D E4C0259C
50715406 822D1682 80B0F2E5 5C94
65B2E942 24BEEA6E A513F918 CCEC1DE3      Authentication Tag
```

### Framed Data

The following example shows the message body format for [Framed Data](#framed-data).

#### Example Pseudo-ASN.1 Structure

```
DEFINITIONS ::= BEGIN
RegularFrame SEQUENCE (SIZE(4)) {
    SequenceNumber UINT32,
    IV OCTET STRING (SIZE(IvLength)),
    EncryptedContent OCTET STRING (SIZE(FrameLength)),
    AuthTag OCTET STRING (SIZE(TagLength)),
}

FinalFrame SEQUENCE (SIZE(6)) {
    SequenceNumberEnd OCTET STRING (0xFF 0xFF 0xFF 0xFF),
    SequenceNumber UINT32,
    IV OCTET STRING (SIZE(IvLength)),
    EncryptedContentLength UINT32,
    EncryptedContent OCTET STRING (SIZE(EncryptedContentLength)),
    AuthTag OCTET STRING (SIZE(TagLength)),
}
```

#### Example Bytes

```
00000001                                   Frame 1, Sequence Number (1)
6BD3FE9C ADBCB213 5B89E8F1                 Frame 1, IV
1F6471E0 A51AF310 10FA9EF6 F0C76EDF        Frame 1, Encrypted Content
F5AFA33C 7D2E8C6C 9C5D5175 A212AF8E
FBD9A0C3 C6E3FB59 C125DBF2 89AC7939
BDEE43A8 0F00F49E ACBBD8B2 1C785089
A90DB923 699A1495 C3B31B50 0A48A830
201E3AD9 1EA6DA14 7F6496DB 6BC104A4
DEB7F372 375ECB28 9BF84B6D 2863889F
CB80A167 9C361C4B 5EC07438 7A4822B4
A7D9D2CC 5150D414 AF75F509 FCE118BD
6D1E798B AEBA4CDB AD009E5F 1A571B77
0041BC78 3E5F2F41 8AF157FD 461E959A
BB732F27 D83DC36D CC9EBC05 00D87803
57F2BB80 066971C2 DEEA062F 4F36255D
E866C042 E1382369 12E9926B BA40E2FC
A820055F FB47E428 41876F14 3B6261D9
5262DB34 59F5D37E 76E46522 E8213640
04EE3CC5 379732B5 F56751FA 8E5F26AD        Frame 1, Authentication Tag
00000002                                   Frame 2, Sequence Number (2)
F1140984 FF25F943 959BE514                 Frame 2, IV
216C7C6A 2234F395 F0D2D9B9 304670BF        Frame 2, Encrypted Content
A1042608 8A8BCB3F B58CF384 D72EC004
A41455B4 9A78BAC9 36E54E68 2709B7BD
A884C1E1 705FF696 E540D297 446A8285
23DFEE28 E74B225A 732F2C0C 27C6BDA2
7597C901 65EF3502 546575D4 6D5EBF22
1FF787AB 2E38FD77 125D129C 43D44B96
778D7CEE 3C36625F FF3A985C 76F7D320
ED70B1F3 79729B47 E7D9B5FC 02FCE9F5
C8760D55 7779520A 81D54F9B EC45219D
95941F7E 5CBAEAC8 CEC13B62 1464757D
AC65B6EF 08262D74 44670624 A3657F7F
2A57F1FD E7060503 AC37E197 2F297A84
DF1172C2 FA63CF54 E6E2B9B6 A86F582B
3B16F868 1BBC5E4D 0B6919B3 08D5ABCF
FECDC4A4 8577F08B 99D766A1 E5545670
A61F0A3B A3E45A84 4D151493 63ECA38F        Frame 2, Authentication Tag
FFFFFFFF                                   Final Frame, Sequence Number End
00000003                                   Final Frame, Sequence Number (3)
35F74F11 25410F01 DD9E04BF                 Final Frame, IV
0000008E                                   Final Frame, Encrypted Content Length (142)
F7A53D37 2F467237 6FBD0B57 D1DFE830        Final Frame, Encrypted Content
B965AD1F A910AA5F 5EFFFFF4 BC7D431C
BA9FA7C4 B25AF82E 64A04E3A A0915526
88859500 7096FABB 3ACAD32A 75CFED0C
4A4E52A3 8E41484D 270B7A0F ED61810C
3A043180 DF25E5C5 3676E449 0986557F
C051AD55 A437F6BC 139E9E55 6199FD60
6ADC017D BA41CDA4 C9F17A83 3823F9EC
B66B6A5A 80FDB433 8A48D6A4 21CB
811234FD 8D589683 51F6F39A 040B3E3B        Final Frame, Authentication Tag
```
