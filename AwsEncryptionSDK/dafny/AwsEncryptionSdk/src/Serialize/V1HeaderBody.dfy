// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "HeaderTypes.dfy"
include "SharedHeaderFunctions.dfy"
include "EncryptionContext.dfy"
include "EncryptedDataKeys.dfy"

module {:options "/functionSyntax:4" } V1HeaderBody {
  import Types = AwsCryptographyEncryptionSdkTypes
  import MaterialProviders
  import Seq
  import HeaderTypes
  import opened SharedHeaderFunctions
  import opened EncryptedDataKeys
  import opened EncryptionContext
  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions

  type V1HeaderBody = h: HeaderTypes.HeaderBody
    | h.V1HeaderBody?
    witness *

  //= compliance/data-format/message-header.txt#2.5.2.1
  //= type=implication
  //# A reserved sequence of 4 bytes that MUST have the value (hex) of "00
  //# 00 00 00".
  const RESERVED_BYTES: seq<uint8> := [0x00, 0x00, 0x00, 0x00]
  type ReservedBytes = s: seq<uint8>
    | s == RESERVED_BYTES
    witness RESERVED_BYTES

  function WriteV1HeaderBody(
    body: V1HeaderBody
  )
    :(ret: seq<uint8>)
  {

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# If the message format version associated with the algorithm suite
    //# (../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# then the message header body (../data-format/message-
    //# header.md#header-body-version-1-0) MUST be serialized with the
    //# following specifics:

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Version (../data-format/message-header.md#version-1): MUST have a
    //# value corresponding to 1.0 (../data-format/message-
    //# header.md#supported-versions)
    WriteMessageFormatVersion(HeaderTypes.MessageFormatVersion.V1)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Type (../data-format/message-header.md#type): MUST have a value
    //# corresponding to Customer Authenticated Encrypted Data (../data-
    //# format/message-header.md#supported-types)
    + WriteV1MessageType(body.messageType)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Algorithm Suite ID (../data-format/message-header.md#algorithm-
    //# suite-id): MUST correspond to the algorithm suite (../framework/
    //# algorithm-suites.md) used in this behavior
    + WriteESDKSuiteId(body.algorithmSuite)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Message ID (../data-format/message-header.md#message-id): The
    //# process used to generate this identifier MUST use a good source of
    //# randomness to make the chance of duplicate identifiers negligible.
    + WriteMessageId(body.messageId)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  AAD (../data-format/message-header.md#aad): MUST be the
    //# serialization of the encryption context (../framework/
    //# structures.md#encryption-context) in the encryption materials
    //# (../framework/structures.md#encryption-materials)
    + WriteAADSection(body.encryptionContext)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Encrypted Data Keys (../data-format/message-header.md#encrypted-
    //# data-key-entries): MUST be the serialization of the encrypted data
    //# keys (../framework/structures.md#encrypted-data-keys) in the
    //# encryption materials (../framework/structures.md#encryption-
    //# materials)
    + WriteEncryptedDataKeysSection(body.encryptedDataKeys)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Content Type (../data-format/message-header.md#content-type): MUST
    //# be 02 (../data-format/message-header.md#supported-content-types)
    + WriteContentType(body.contentType)
    + WriteV1ReservedBytes(RESERVED_BYTES)
    //= compliance/data-format/message-header.txt#2.5.2.2
    //# This value MUST be
    //# equal to the IV length (../framework/algorithm-suites.md#iv-length)
    //# value of the algorithm suite (../framework/algorithm-suites.md)
    //# specified by the Algorithm Suite ID (Section 2.5.1.5) field.
    //
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  IV Length (../data-format/message-header.md#iv-length): MUST match
    //# the IV length (../framework/algorithm-suites.md#iv-length)
    //# specified by the algorithm suite (../framework/algorithm-
    //# suites.md)
    + WriteV1HeaderIvLength(GetIvLength(body.algorithmSuite))
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Frame Length (../data-format/message-header.md#frame-length): MUST
    //# be the value of the frame size determined above.
    + WriteUint32(body.frameLength)
  }

  function {:vcs_split_on_every_assert} ReadV1HeaderBody(
    buffer: ReadableBuffer,
    maxEdks: Option<Types.CountingNumbers>,
    mpl: MaterialProviders.MaterialProvidersClient
  )
    :(res: ReadCorrect<V1HeaderBody>)
    ensures CorrectlyReadV1HeaderBody(buffer, res)
  {
    var version :- ReadMessageFormatVersion(buffer);
    :- Need(version.data.V1?, Error("Message version must be version 1."));

    var messageType :- ReadV1MessageType(version.tail);

    var suite :- ReadESDKSuiteId(messageType.tail, mpl);
    :- Need(suite.data.commitment.None?, Error("Algorithm suite must not support commitment."));

    var messageId :- ReadMessageIdV1(suite.tail);

    var encryptionContext :- EncryptionContext.ReadAADSection(messageId.tail);

    var encryptedDataKeys :- EncryptedDataKeys.ReadEncryptedDataKeysSection(encryptionContext.tail, maxEdks);

    var contentType :- ReadContentType(encryptedDataKeys.tail);

    var reservedBytes :- ReadV1ReservedBytes(contentType.tail);

    var headerIvLength :- ReadV1HeaderIvLength(
                            reservedBytes.tail,
                            suite.data
                          );

    var frameLength :- ReadUInt32(headerIvLength.tail);

    var body:V1HeaderBody := HeaderTypes.V1HeaderBody(
                               messageType := messageType.data,
                               algorithmSuite := suite.data,
                               messageId := messageId.data,
                               encryptionContext := encryptionContext.data,
                               encryptedDataKeys := encryptedDataKeys.data,
                               contentType := contentType.data,
                               headerIvLength := headerIvLength.data as uint64,
                               frameLength := frameLength.data
                             );
    // To prove that we can correctly read what we just wrote we have to break down the pieces
    // of what we wrote and that they were correctly stitched together.
    assert CorrectlyReadV1HeaderBody(buffer, Success(SuccessfulRead(body, frameLength.tail))) by {

      assert RESERVED_BYTES == reservedBytes.data;
      assert body.algorithmSuite == suite.data;
      assert GetIvLength(body.algorithmSuite) == headerIvLength.data;
      // Messages are different if there is an expanded AAD Section
      if IsV1ExpandedAADSection(buffer) {
        // In order to prove we have the correct construction we throw in the inverse of what we read; which
        // is the write. In the following asserts we are basically saying, "to prove I can read this part of
        // the message I'll show you that I can write it and that I can read it, up to its correct range AND that it is
        // the same as the buffer". This way of proving the construction of the message stabilizes the verfication.
        assert CorrectlyReadRange(buffer, frameLength.tail, WriteV1ExpandedAADSectionHeaderBody(body)) by {
          CorrectlyReadByteRange(buffer, version.tail, WriteMessageFormatVersion(version.data));
          AppendToCorrectlyReadByteRange(buffer, version.tail, messageType.tail, WriteV1MessageType(messageType.data));
          AppendToCorrectlyReadByteRange(buffer, messageType.tail, suite.tail, WriteESDKSuiteId(suite.data));
          AppendToCorrectlyReadByteRange(buffer, suite.tail, messageId.tail, WriteMessageId(messageId.data));
          assert IsExpandedAADSection(messageId.tail) by {
            assert |WriteMessageFormatVersion(version.data)| == 1;
            assert |WriteV1MessageType(messageType.data)| == 1;
            assert |WriteESDKSuiteId(suite.data)| == 2;
            assert |WriteMessageId(messageId.data)| == HeaderTypes.MESSAGE_ID_LEN_V1 as nat;
            reveal CorrectlyReadRange();
          }
          AppendToCorrectlyReadByteRange(buffer, messageId.tail, encryptionContext.tail, WriteExpandedAADSection(encryptionContext.data));
          AppendToCorrectlyReadByteRange(buffer, encryptionContext.tail, encryptedDataKeys.tail, WriteEncryptedDataKeysSection(encryptedDataKeys.data));
          AppendToCorrectlyReadByteRange(buffer, encryptedDataKeys.tail, contentType.tail, WriteContentType(contentType.data));
          AppendToCorrectlyReadByteRange(buffer, contentType.tail, reservedBytes.tail, WriteV1ReservedBytes(reservedBytes.data));
          AppendToCorrectlyReadByteRange(buffer, reservedBytes.tail, headerIvLength.tail, WriteV1HeaderIvLength(headerIvLength.data));
          AppendToCorrectlyReadByteRange(buffer, headerIvLength.tail, frameLength.tail, WriteUint32(frameLength.data));
        }
      } else {
        assert CorrectlyReadRange(buffer, frameLength.tail, WriteV1HeaderBody(body)) by {
          CorrectlyReadByteRange(buffer, version.tail, WriteMessageFormatVersion(version.data));
          AppendToCorrectlyReadByteRange(buffer, version.tail, messageType.tail, WriteV1MessageType(messageType.data));
          AppendToCorrectlyReadByteRange(buffer, messageType.tail, suite.tail, WriteESDKSuiteId(suite.data));
          AppendToCorrectlyReadByteRange(buffer, suite.tail, messageId.tail, WriteMessageId(messageId.data));
          assert !IsExpandedAADSection(messageId.tail) by {
            assert |WriteMessageFormatVersion(version.data)| == 1;
            assert |WriteV1MessageType(messageType.data)| == 1;
            assert |WriteESDKSuiteId(suite.data)| == 2;
            assert |WriteMessageId(messageId.data)| == HeaderTypes.MESSAGE_ID_LEN_V1 as nat;
            reveal CorrectlyReadRange();
          }
          AppendToCorrectlyReadByteRange(buffer, messageId.tail, encryptionContext.tail, WriteAADSection(encryptionContext.data));
          AppendToCorrectlyReadByteRange(buffer, encryptionContext.tail, encryptedDataKeys.tail, WriteEncryptedDataKeysSection(encryptedDataKeys.data));
          AppendToCorrectlyReadByteRange(buffer, encryptedDataKeys.tail, contentType.tail, WriteContentType(contentType.data));
          AppendToCorrectlyReadByteRange(buffer, contentType.tail, reservedBytes.tail, WriteV1ReservedBytes(reservedBytes.data));
          AppendToCorrectlyReadByteRange(buffer, reservedBytes.tail, headerIvLength.tail, WriteV1HeaderIvLength(headerIvLength.data));
          AppendToCorrectlyReadByteRange(buffer, headerIvLength.tail, frameLength.tail, WriteUint32(frameLength.data));
        }
      }
    }

    Success(SuccessfulRead(body, frameLength.tail))
  }

  ghost predicate CorrectlyReadV1HeaderBody(
    buffer: ReadableBuffer,
    res: ReadCorrect<V1HeaderBody>
  )
  {
    // To prove that we can read we pass in the inverse of the read (the write) in
    // each branch.
    if IsV1ExpandedAADSection(buffer) then
      CorrectlyRead(buffer, res, WriteV1ExpandedAADSectionHeaderBody)
    else
      CorrectlyRead(buffer, res, WriteV1HeaderBody)
  }

  ghost predicate IsV1ExpandedAADSection(
    buffer: ReadableBuffer
  )
  {
    // version + messageType + suiteId + messageId
    var headerBytesToAADStart := 1 + 1 + 2 + 16;
    IsExpandedAADSection(MoveStart(buffer, headerBytesToAADStart))
  }

  function WriteV1MessageType(
    messageType: HeaderTypes.MessageType
  ):
    (ret: seq<uint8>)
  {
    [messageType.Serialize()]
  }

  opaque function ReadV1MessageType(
    buffer: ReadableBuffer
  )
    :(res: ReadCorrect<HeaderTypes.MessageType>)
    ensures CorrectlyRead(buffer, res, WriteV1MessageType)
  {
    var SuccessfulRead(raw, tail) :- SerializeFunctions.Read(buffer, 1);
    var messageType :- HeaderTypes.MessageType.Get(raw[0 as uint32]).MapFailure(e => Error(e));

    assert CorrectlyReadRange(buffer, tail, WriteV1MessageType(messageType)) by {
      reveal CorrectlyReadRange();
    }
    Success(SuccessfulRead(messageType, tail))
  }

  function WriteV1ReservedBytes(
    reservedBytes: ReservedBytes
  ):
    (ret: seq<uint8>)
  {
    reservedBytes
  }

  opaque function ReadV1ReservedBytes(
    buffer: ReadableBuffer
  )
    :(res: ReadCorrect<ReservedBytes>)
    ensures CorrectlyRead(buffer, res, WriteV1ReservedBytes)
  {
    var SuccessfulRead(raw, tail) :- SerializeFunctions.Read(buffer, |RESERVED_BYTES| as uint64);
    :- Need(raw == RESERVED_BYTES, Error("Incorrect reserved bytes."));
    var reservedBytes: ReservedBytes := raw;
    Success(SuccessfulRead(reservedBytes, tail))
  }

  function WriteV1HeaderIvLength(
    ivLength: uint8
  )
    :(ret: seq<uint8>)
  {
    [ivLength]
  }

  opaque function ReadV1HeaderIvLength(
    buffer: ReadableBuffer,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(res: ReadCorrect<uint8>)
    ensures CorrectlyRead(buffer, res, WriteV1HeaderIvLength)
    ensures res.Success? ==> GetIvLength(suite) == res.value.data
  {
    var SuccessfulRead(raw, tail) :- SerializeFunctions.Read(buffer, 1);
    :- Need(raw[0 as uint32] == GetIvLength(suite), Error("HeaderIv Length does not match Algorithm Suite."));

    assert CorrectlyReadRange(buffer, tail, WriteV1HeaderIvLength(raw[0 as uint32])) by {
      reveal CorrectlyReadRange();
    }

    Success(SuccessfulRead(raw[0 as uint32], tail))
  }

  // This is *not* a function,
  // because it is *only* used for correctness.
  // This represents the sub-optimal encoding of AAD
  // where empty AAD is encoded with an extra 2 bytes.
  ghost function WriteV1ExpandedAADSectionHeaderBody(
    body: V1HeaderBody
  )
    :(ret: seq<uint8>)
  {

    WriteMessageFormatVersion(HeaderTypes.MessageFormatVersion.V1)
    + WriteV1MessageType(body.messageType)
    + WriteESDKSuiteId(body.algorithmSuite)
    + WriteMessageId(body.messageId)
    + WriteExpandedAADSection(body.encryptionContext)
    + WriteEncryptedDataKeysSection(body.encryptedDataKeys)
    + WriteContentType(body.contentType)
    + WriteV1ReservedBytes(RESERVED_BYTES)
    + WriteV1HeaderIvLength(GetIvLength(body.algorithmSuite))
    + WriteUint32(body.frameLength)
  }

  lemma HeadersAreTheSameWhenThereIsEncryptionContext(body: V1HeaderBody)
    requires 0 < |body.encryptionContext|
    ensures WriteV1ExpandedAADSectionHeaderBody(body) == WriteV1HeaderBody(body)
  {}

}
