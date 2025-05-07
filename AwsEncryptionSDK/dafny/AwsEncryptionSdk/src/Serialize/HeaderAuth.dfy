// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "HeaderTypes.dfy"
include "EncryptionContext.dfy"
include "EncryptedDataKeys.dfy"

module HeaderAuth {
  import Types = AwsCryptographyEncryptionSdkTypes
  import MPL = AwsCryptographyMaterialProvidersTypes
  import Seq
  import HeaderTypes
  import opened EncryptedDataKeys
  import opened EncryptionContext
  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions

  type AESMac = a: HeaderTypes.HeaderAuth
    | a.AESMac?
    witness *

  function method WriteHeaderAuthTagV2(
    headerAuth: AESMac
  )
    :(ret: seq<uint8>)
  {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# With the authentication tag calculated, if the message format version
    //# associated with the algorithm suite (../framework/algorithm-
    //# suites.md#supported-algorithm-suites) is 2.0, this operation MUST
    //# serialize the message header authentication (../data-format/message-
    //# header.md#header-authentication-version-2-0) with the following
    //# specifics:

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# ./data-format/message-
    //# header.md#authentication-tag): MUST have the value of the
    //# authentication tag calculated above.
    Write(headerAuth.headerAuthTag)
  }

  function method WriteHeaderAuthTagV1(
    headerAuth: AESMac
  )
    :(ret: seq<uint8>)
  {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# If the message format version associated with the algorithm suite
    //# (../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# this operation MUST serialize the message header authentication
    //# (../data-format/message-header.md#header-authentication-version-1-0)
    //# with the following specifics:

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  IV (../data-format/message-header.md#iv): MUST have the value of
    //# the IV used in the calculation above, padded to the IV length
    //# (../data-format/message-header.md#iv-length) with 0.
    Write(headerAuth.headerIv)
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Authentication Tag (../data-format/message-
    //# header.md#authentication-tag): MUST have the value of the
    //# authentication tag calculated above.
    + Write(headerAuth.headerAuthTag)
  }

  function method WriteHeaderAuthTag(
    headerAuth: AESMac,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(ret: Result<seq<uint8>, Types.Error>)
  {
    match suite.messageVersion
    case 1 => Success(WriteHeaderAuthTagV1(headerAuth))
    case 2 => Success(WriteHeaderAuthTagV2(headerAuth))
    case _ => Failure(Types.AwsEncryptionSdkException( message := "Unexpected message version"))
  }

  function method ReadHeaderAuthTagV1(
    buffer: ReadableBuffer,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(res: ReadCorrect<AESMac>)
    ensures CorrectlyRead(buffer, res, WriteHeaderAuthTagV1)
    ensures res.Success?
            ==>
              && |res.value.data.headerIv| == GetIvLength(suite) as nat
              && |res.value.data.headerAuthTag| == GetTagLength(suite) as nat
  {
    var headerIv :- Read(buffer, GetIvLength(suite) as uint64);
    var headerAuthTag :- Read(headerIv.tail, GetTagLength(suite) as uint64);

    var auth: AESMac := HeaderTypes.HeaderAuth.AESMac(
                          headerIv := headerIv.data,
                          headerAuthTag := headerAuthTag.data
                        );

    assert CorrectlyReadRange(buffer, headerAuthTag.tail, WriteHeaderAuthTagV1(auth)) by {
      reveal CorrectlyReadRange();
    }

    Success(SuccessfulRead(auth, headerAuthTag.tail))
  }

  function method ReadHeaderAuthTagV2(
    buffer: ReadableBuffer,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(res: ReadCorrect<AESMac>)
    ensures CorrectlyRead(buffer, res, WriteHeaderAuthTagV2)
    ensures res.Success?
            ==>
              && |res.value.data.headerIv| == GetIvLength(suite) as nat
              && |res.value.data.headerAuthTag| == GetTagLength(suite) as nat
  {
    // TODO: probably this hardcoded iv of all 0s will go into alg suite
    var headerIv := GetIvLengthZeros(suite);
    var headerAuthTag :- Read(buffer, GetTagLength(suite) as uint64);

    var auth: AESMac := HeaderTypes.HeaderAuth.AESMac(
                          headerIv := headerIv,
                          headerAuthTag := headerAuthTag.data
                        );

    Success(SuccessfulRead(auth, headerAuthTag.tail))
  }


  function method {:opaque} ReadHeaderAuthTag(
    buffer: ReadableBuffer,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(res: ReadCorrect<AESMac>)
    ensures suite.messageVersion == 1 ==> CorrectlyRead(buffer, res, WriteHeaderAuthTagV1)
    ensures suite.messageVersion == 2 ==> CorrectlyRead(buffer, res, WriteHeaderAuthTagV2)
    ensures res.Success?
            ==>
              && |res.value.data.headerIv| == GetIvLength(suite) as nat
              && |res.value.data.headerAuthTag| == GetTagLength(suite) as nat
    ensures suite.messageVersion != 1 && suite.messageVersion != 2 ==> res.Failure?
  {
    match suite.messageVersion
    case 1 => ReadHeaderAuthTagV1(buffer, suite)
    case 2 => ReadHeaderAuthTagV2(buffer, suite)
    case _ => Failure(Error("Unexpected message version"))
  }
}
