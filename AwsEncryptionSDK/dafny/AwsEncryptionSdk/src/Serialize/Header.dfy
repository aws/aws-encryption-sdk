// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "EncryptionContext.dfy"

include "V1HeaderBody.dfy"
include "V2HeaderBody.dfy"
include "HeaderTypes.dfy"
include "HeaderAuth.dfy"
include "SharedHeaderFunctions.dfy"


module Header {
  import Types = AwsCryptographyEncryptionSdkTypes
  import MaterialProviders
  import Seq
  import EncryptionContext

  import V1HeaderBody
  import V2HeaderBody
  import HeaderTypes
  import HeaderAuth
  import SharedHeaderFunctions

  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions

  datatype HeaderInfo = HeaderInfo(
    nameonly body: HeaderTypes.HeaderBody,
    nameonly rawHeader: seq<uint8>,
    nameonly encryptionContext: ESDKEncryptionContext,
    nameonly suite: MPL.AlgorithmSuiteInfo,
    nameonly headerAuth: HeaderTypes.HeaderAuth
  )

  predicate IsHeader(h: HeaderInfo)
  {
    && h.suite.id.ESDK?
    && h.suite == h.body.algorithmSuite
       // TODO: Even though we're not yet supporting non-framed content,
       // this assertion about non-framed messages has ripple effects on
       // other proofs
    && (h.body.contentType.NonFramed? <==> 0 == h.body.frameLength)
    && (h.body.contentType.Framed? <==> 0 < h.body.frameLength)
    && HeaderAuth?(h.suite, h.headerAuth)
    && HeaderVersionSupportsCommitment?(h.suite, h.body)

    // The readability of the header is effectively a completeness requirement.
    // By proving the soundness of the ReadV*HeaderBody,
    // this is complicated.
    // TODO Uncomment this once the ReadV*HeaderBody functions are proved complete.
    // There are 2 readable formats, but only 1 writeable format.
    // This means that a correct header is defined by reading.
    // Less options to keep track of.
    // && CorrectlyReadHeaderBody(
    //   ReadableBuffer(h.rawHeader, 0),
    //   Success(SuccessfulRead(h.body, ReadableBuffer(h.rawHeader, |h.rawHeader|))))

    // I would like to have this relationship, but the CMM really gets to control this
    // So I'm going to push towards this distinguishing the stored vs the "complete" encryption context.
    // && h.encryptionContext == EncryptionContext.GetEncryptionContext(h.body.encryptionContext)
  }

  predicate HeaderAuth?(
    suite: MPL.AlgorithmSuiteInfo,
    headerAuth: HeaderTypes.HeaderAuth
  )
  {
    && (headerAuth.AESMac?
        ==>
          && |headerAuth.headerIv| == GetIvLength(suite) as nat
          && |headerAuth.headerAuthTag| == GetTagLength(suite) as nat)
  }

  predicate method {:opaque} HeaderVersionSupportsCommitment?(
    suite: MPL.AlgorithmSuiteInfo,
    body: HeaderTypes.HeaderBody
  )
  {
    && (suite.commitment.HKDF?
        ==>
          && body.V2HeaderBody?
          && |body.suiteData| == suite.commitment.HKDF.outputKeyLength as nat)
    && (!suite.commitment.HKDF?
        ==>
          && body.V1HeaderBody?)
  }

  type Header = h: HeaderInfo
    | IsHeader(h)
    witness *

  // ReadHeaderBody does not support streaming at this time
  //= compliance/client-apis/decrypt.txt#2.7.1
  //= type=exception
  //# This operation MUST wait if it doesn't have enough consumable
  //# encrypted message bytes to deserialize the next field of the message
  //# header until enough input bytes become consumable or the caller
  //# indicates an end to the encrypted message.

  //= compliance/client-apis/decrypt.txt#2.7.1
  //# This operation MUST attempt to deserialize all consumable encrypted
  //# message bytes until it has successfully deserialized a valid message
  //# header (../data-format/message-header.md).
  function method {:opaque} ReadHeaderBody(
    buffer: ReadableBuffer,
    maxEdks: Option<Types.CountingNumbers>,
    mpl: MaterialProviders.MaterialProvidersClient
  )
    :(res: ReadCorrect<HeaderTypes.HeaderBody>)
    ensures CorrectlyReadHeaderBody(buffer, res)
    //= compliance/data-format/message-header.txt#2.5.2.3
    //= type=implication
    //# When the content type (Section 2.5.1.11) is non-
    //# framed, the value of this field MUST be 0.
    ensures res.Success? ==>
              var h := res.value.data;
              && (h.contentType.NonFramed? <==> 0 == h.frameLength)
              && (h.contentType.Framed? <==> 0 < h.frameLength)
  {
    var version :- SharedHeaderFunctions.ReadMessageFormatVersion(buffer);

    var (body, tail) :- match version.data {
      case V1 =>
        var b :- V1HeaderBody.ReadV1HeaderBody(buffer, maxEdks, mpl);
        var body: HeaderTypes.HeaderBody := b.data;
        Success((body, b.tail))
      case V2 =>
        var b :- V2HeaderBody.ReadV2HeaderBody(buffer, maxEdks, mpl);
        var body: HeaderTypes.HeaderBody := b.data;
        Success((body, b.tail))
    };

    :- Need(body.contentType.Framed? <==> body.frameLength > 0,
            Error("Frame length must be positive if content is framed"));
    :- Need(body.contentType.NonFramed? <==> body.frameLength == 0,
            Error("Frame length must be zero if content is non-framed"));
    Success(SuccessfulRead(body, tail))
  }

  predicate CorrectlyReadHeaderBody(
    buffer: ReadableBuffer,
    res: ReadCorrect<HeaderTypes.HeaderBody>
  )
  {
    res.Success?
    ==>
      && match res.value.data
         case V1HeaderBody(_,_,_,_,_,_,_,_) =>
           V1HeaderBody.CorrectlyReadV1HeaderBody(buffer, res)
         case V2HeaderBody(_,_,_,_,_,_,_) =>
           V2HeaderBody.CorrectlyReadV2HeaderBody(buffer, res)
  }

  function method WriteHeaderBody(
    body: HeaderTypes.HeaderBody
  )
    :(ret: seq<uint8>)
  {
    match body
    case V1HeaderBody(_,_,_,_,_,_,_,_) =>
      V1HeaderBody.WriteV1HeaderBody(body)
    case V2HeaderBody(_,_,_,_,_,_,_) =>
      V2HeaderBody.WriteV2HeaderBody(body)
  }

}
