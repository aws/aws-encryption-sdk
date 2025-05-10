// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "./SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "EncryptionContext.dfy"
include "HeaderTypes.dfy"

module SharedHeaderFunctions {
  import MPL = AwsCryptographyMaterialProvidersTypes
  import Types = AwsCryptographyEncryptionSdkTypes
  import Seq
  import EncryptionContext
  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions
  import opened HeaderTypes
  import MaterialProviders

  function method WriteMessageFormatVersion(
    version: MessageFormatVersion
  )
    :(ret : seq<uint8>)
  {
    Write(version.Serialize())
  }

  function method ReadMessageFormatVersion(
    buffer: ReadableBuffer
  )
    :(res: ReadCorrect<MessageFormatVersion>)
    ensures CorrectlyRead(buffer, res, WriteMessageFormatVersion)
  {
    var rawVersion :- SerializeFunctions.Read(buffer, 1);

    var version :- MessageFormatVersion.Get(rawVersion.data).MapFailure(e => Error(e));
    Success(SuccessfulRead(version, rawVersion.tail))
  }

  function method WriteESDKSuiteId(
    suite: ESDKAlgorithmSuite
  ):
    (ret: seq<uint8>)
  {
    Write(suite.binaryId)
  }

  function method ReadESDKSuiteId(
    buffer: ReadableBuffer,
    mpl: MaterialProviders.MaterialProvidersClient
  )
    :(res: ReadCorrect<ESDKAlgorithmSuite>)
    ensures CorrectlyRead(buffer, res, WriteESDKSuiteId)
    ensures res.Success? ==> mpl.ValidAlgorithmSuiteInfo(res.value.data).Success?
  {
    var esdkSuiteIdBytes :- Read(buffer, 2);
    var suite :- mpl.GetAlgorithmSuiteInfo(esdkSuiteIdBytes.data).MapFailure(_ => Error("Algorithm suite ID not supported."));
    :- Need(suite.binaryId == esdkSuiteIdBytes.data, Error("Algorithm suite ID not supported."));
    :- Need(suite.id.ESDK?, Error("Algorithm suite ID not supported."));
    var esdkSuite: ESDKAlgorithmSuite := suite; // TODO Dafny has type troubles without this, explicitly use ESDKAlgorithmSuite var
    Success(SuccessfulRead(esdkSuite, esdkSuiteIdBytes.tail))
  }

  /*
   * Writes the message id as bytes, which, since the message id is already stored
   * as bytes, simply returns the message id.
   *
   * Though we have different V1 and V2 methods for the read path, since
   * they read different numbers of bytes, a single method on the write path
   * is fine since writing is identical for both.
   */
  function method WriteMessageId(
    messageId: MessageId
  ):
    (ret: seq<uint8>)
  {
    Write(messageId)
  }

  function method ReadMessageIdV1(
    buffer: ReadableBuffer
  )
    :(res: ReadBinaryCorrect<MessageId>)
    ensures CorrectlyRead(buffer, res, WriteMessageId)
  {
    var messageIdRead :- SerializeFunctions.Read(buffer, MESSAGE_ID_LEN_V1);
    var messageId: MessageId := messageIdRead.data;

    Success(SuccessfulRead(messageId, messageIdRead.tail))
  }

  function method ReadMessageIdV2(
    buffer: ReadableBuffer
  )
    :(res: ReadBinaryCorrect<MessageId>)
    ensures CorrectlyRead(buffer, res, WriteMessageId)
  {
    var messageIdRead :- SerializeFunctions.Read(buffer, MESSAGE_ID_LEN_V2);
    var messageId: MessageId := messageIdRead.data;

    Success(SuccessfulRead(messageId, messageIdRead.tail))
  }

  function method WriteContentType(
    contentType: ContentType
  ):
    (ret: seq<uint8>)
  {
    Write([contentType.Serialize()])
  }

  function method ReadContentType(
    buffer: ReadableBuffer
  )
    :(res: ReadCorrect<ContentType>)
    ensures CorrectlyRead(buffer, res, WriteContentType)
  {
    var SuccessfulRead(raw, tail) :- SerializeFunctions.Read(buffer, 1);
    var contentType :- ContentType.Get(raw[0 as uint32]).MapFailure(e => Error(e));

    assert CorrectlyReadRange(buffer, tail, WriteContentType(contentType)) by {
      reveal CorrectlyReadRange();
    }

    Success(SuccessfulRead(contentType, tail))
  }
}
