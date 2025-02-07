// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "EncryptionContext.dfy"

module HeaderTypes {
  import MPL = AwsCryptographyMaterialProvidersTypes
  import Seq
  import EncryptionContext
  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions

  datatype MessageFormatVersion =
    | V1
    | V2
  {
    function method Serialize(): (bytes: seq<uint8>) {
      match this
      case V1 => [0x01]
      case V2 => [0x02]
    }

    lemma LemmaSerializeCorrectValue()
      //= compliance/data-format/message-header.txt#2.5.1.3
      //= type=implication
      //# The version (hex) of this field
      //# MUST be a value that exists in the following table:
      ensures match this
              //= compliance/data-format/message-header.txt#2.5.1.1
              //= type=implication
              //# The value of the "Version" field MUST be "01" in the
              //# Version 1.0 header body.
              case V1 => this.Serialize() == [0x01]
              //= compliance/data-format/message-header.txt#2.5.1.2
              //= type=implication
              //# The value of the "Version" field MUST be "02" in the
              //# Version 2.0 header body.
              case V2 => this.Serialize() == [0x02]
    {}

    static function method Get(
      x: seq<uint8>
    )
      :(res: Result<MessageFormatVersion, string>)
      ensures res.Success? ==> x == res.value.Serialize()
    {
      :- Need(x == [0x01] || x == [0x02], "Unsupported Version value.");
      Success(
        match x[0]
        case 0x01 => V1
        case 0x02 => V2
      )
    }
  }

  predicate ESDKAlgorithmSuite?(a: MPL.AlgorithmSuiteInfo) {
    a.id.ESDK?
  }

  type ESDKAlgorithmSuite = a: MPL.AlgorithmSuiteInfo | ESDKAlgorithmSuite?(a)
    witness *

  datatype HeaderBody =
    | V1HeaderBody(
        nameonly messageType: MessageType,
        nameonly algorithmSuite: ESDKAlgorithmSuite, // TODO add MUST to spec to ensure this stays true
        nameonly messageId: MessageId,
        nameonly encryptionContext: EncryptionContext.ESDKCanonicalEncryptionContext,
        nameonly encryptedDataKeys: ESDKEncryptedDataKeys,
        nameonly contentType: ContentType,
        nameonly headerIvLength: nat,
        nameonly frameLength: uint32
      )
    | V2HeaderBody(
        nameonly algorithmSuite: ESDKAlgorithmSuite,
        nameonly messageId: MessageId,
        nameonly encryptionContext: EncryptionContext.ESDKCanonicalEncryptionContext,
        nameonly encryptedDataKeys: ESDKEncryptedDataKeys,
        nameonly contentType: ContentType,
        nameonly frameLength: uint32,
        nameonly suiteData: seq<uint8>
      )

  datatype HeaderAuth =
    | AESMac(
        nameonly headerIv: seq<uint8>,
        nameonly headerAuthTag: seq<uint8>
      )

  datatype MessageType =
    | TYPE_CUSTOMER_AED
  {
    function method Serialize(): (val: uint8) {
      match this
      case TYPE_CUSTOMER_AED => 0x80
    }

    lemma LemmaSerializeCorrectValue()
      //= compliance/data-format/message-header.txt#2.5.1.4
      //= type=implication
      //# The type (hex) of this field MUST be
      //# a value that exists in the following table:
      ensures this.Serialize() == 0x80
    {}

    static function method Get(
      x: uint8
    )
      :(res: Result<MessageType, string>)
      ensures res.Success? ==> x == res.value.Serialize()
    {
      :- Need(x == 0x80 , "Unsupported ContentType value.");
      Success(
        match x
        case 0x80 => TYPE_CUSTOMER_AED
      )
    }
  }

  datatype ContentType =
    | NonFramed
    | Framed
  {
    function method Serialize(): (val: uint8) {
      match this
      case NonFramed => 0x01
      case Framed => 0x02
    }

    lemma LemmaSerializeCorrectValue()
      //= compliance/data-format/message-header.txt#2.5.1.11
      //= type=implication
      //# The value
      //# (hex) of this field MUST be a value that exists in the following
      //# table:
      ensures this.Serialize() in {0x01, 0x02}
    {}

    static function method Get(
      x: uint8
    )
      :(res: Result<ContentType, string>)
      ensures res.Success? ==> x == res.value.Serialize()
    {
      :- Need(x == 0x01 || x == 0x02, "Unsupported ContentType value.");
      Success(
        match x
        case 0x01 => NonFramed
        case 0x02 => Framed
      )
    }
  }

  // TODO: push this into the `IsHeader`
  const MESSAGE_ID_LEN_V1 := 16
  const MESSAGE_ID_LEN_V2 := 32
  type MessageId = x: seq<uint8> |
      || |x| == MESSAGE_ID_LEN_V1
      || |x| == MESSAGE_ID_LEN_V2
    witness *
}
