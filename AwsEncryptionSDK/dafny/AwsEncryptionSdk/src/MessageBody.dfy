// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "Serialize/SerializableTypes.dfy"
include "../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "Serialize/Frames.dfy"

include "Serialize/Header.dfy"
include "Serialize/HeaderTypes.dfy"
include "Serialize/V1HeaderBody.dfy"
include "Serialize/HeaderAuth.dfy"
include "Serialize/SerializeFunctions.dfy"

module MessageBody {
  // export
  //   provides EncryptMessageBody, DecryptFramedMessageBody, DecryptNonFramedMessageBody,
  //     Wrappers, UInt, Streams, Client,
  //     FramesEncryptPlaintext, AESEncryption, DecryptedWithKey
  //   reveals  SeqWithGhostFrames

  import opened Wrappers
  import opened UInt = StandardLibrary.UInt
  import Types = AwsCryptographyEncryptionSdkTypes
  import MPL = AwsCryptographyMaterialProvidersTypes
  import Primitives = AtomicPrimitives
  import Streams
  import UTF8
  import SerializableTypes
  import Frames
  import Header
  import opened SerializeFunctions
  import Seq
  import StandardLibrary

  datatype BodyAADContent = AADRegularFrame | AADFinalFrame | AADSingleBlock

  const BODY_AAD_CONTENT_REGULAR_FRAME: string := "AWSKMSEncryptionClient Frame"
  const BODY_AAD_CONTENT_FINAL_FRAME: string := "AWSKMSEncryptionClient Final Frame"
  const BODY_AAD_CONTENT_SINGLE_BLOCK: string := "AWSKMSEncryptionClient Single Block"

  function method BodyAADContentTypeString(bc: BodyAADContent): string {
    match bc
    case AADRegularFrame => BODY_AAD_CONTENT_REGULAR_FRAME
    case AADFinalFrame => BODY_AAD_CONTENT_FINAL_FRAME
    case AADSingleBlock => BODY_AAD_CONTENT_SINGLE_BLOCK
  }

  const START_SEQUENCE_NUMBER: uint32 := Frames.START_SEQUENCE_NUMBER
  const ENDFRAME_SEQUENCE_NUMBER: uint32 := Frames.ENDFRAME_SEQUENCE_NUMBER
  const NONFRAMED_SEQUENCE_NUMBER: uint32 := Frames.NONFRAMED_SEQUENCE_NUMBER

  function method IVSeq(suite: MPL.AlgorithmSuiteInfo, sequenceNumber: uint32)
    :(ret: seq<uint8>)
    requires 4 <= SerializableTypes.GetIvLength(suite)
    //= compliance/data-format/message-body.txt#2.5.2.1.2
    //= type=implication
    //# The IV length MUST be equal to the IV
    //# length of the algorithm suite specified by the Algorithm Suite ID
    //# (message-header.md#algorithm-suite-id) field.
    //
    //= compliance/data-format/message-body.txt#2.5.2.2.3
    //= type=implication
    //# The IV length MUST be equal to the IV length of the algorithm suite
    //# (../framework/algorithm-suites.md) that generated the message.
    ensures |ret| == SerializableTypes.GetIvLength(suite) as nat
  {
    seq(SerializableTypes.GetIvLength(suite) as nat - 4, _ => 0) + UInt32ToSeq(sequenceNumber)
  }

  //= compliance/data-format/message-body.txt#2.5.2.1.2
  //= type=implication
  //# Each frame in the Framed Data (Section 2.5.2) MUST include an IV that
  //# is unique within the message.
  //
  //= compliance/data-format/message-body.txt#2.5.2.2.3
  //= type=implication
  //# The IV MUST be a unique IV within the message.
  lemma IVSeqDistinct(suite: MPL.AlgorithmSuiteInfo, m: uint32, n: uint32)
    requires m != n
    requires 4 <= SerializableTypes.GetIvLength(suite) 
    ensures IVSeq(suite, m) != IVSeq(suite, n)
  {
    var paddingLength :=  SerializableTypes.GetIvLength(suite) as nat - 4;
    assert IVSeq(suite, m)[paddingLength..] == UInt32ToSeq(m);
    assert IVSeq(suite, n)[paddingLength..] == UInt32ToSeq(n);
    UInt32SeqSerializeDeserialize(m);
    UInt32SeqSerializeDeserialize(n);
  }

  /**
    Says that   
  */
  function SumDecryptCalls(decryptCalls: seq<Primitives.Types.DafnyCallEvent<Primitives.Types.AESDecryptInput, Result<seq<uint8>, Primitives.Types.Error>>>)
    : seq<uint8>
    requires forall call <- decryptCalls :: call.output.Success?
  {
    if decryptCalls == [] then
      []
    else
      SumDecryptCalls(Seq.DropLast(decryptCalls)) + Seq.Last(decryptCalls).output.value
  }

  type MessageRegularFrames = regularFrames: seq<Frames.RegularFrame>
  | IsMessageRegularFrames(regularFrames)
  witness *

  predicate MessageFramesAreMonotonic(frames: seq<MessageFrame>){
    if |frames| == 0 then true
    else
      // Cardinality can be thought of as the tail index
      // of a one-based indexed array.
      // Sequence number is the one-based indexed array of frames
      && |frames| == Seq.Last(frames).seqNum as nat
      && MessageFramesAreMonotonic(Seq.DropLast(frames))
  }

  predicate MessageFramesAreForTheSameMessage(frames: seq<MessageFrame>){
    forall i
    // This excludes the First frame in the seq
    // because all headers are definitionally equal for `[]` and `[frame]`.
    | 0 < i < |frames|
    ::  Seq.First(frames).header == frames[i].header
  }

  predicate IsMessageRegularFrames(regularFrames: seq<Frames.RegularFrame>) {
    // The total number of frames MUST be at most UINT32_MAX.
    // And a RegularFrame can not have a sequence number
    // equal to the ENDFRAME_SEQUENCE_NUMBER.
    //
    //= compliance/data-format/message-body.txt#2.5.2
    //= type=implication
    //# *  The number of frames in a single message MUST be less than or
    //# equal to "2^32 - 1".
    && 0 <= |regularFrames| < ENDFRAME_SEQUENCE_NUMBER as nat
    // The sequence number MUST be monotonic
    //
    //= compliance/data-format/message-body.txt#2.5.2.1.1
    //= type=implication
    //# Framed Data MUST start at Sequence Number 1.
    //
    //= compliance/data-format/message-body.txt#2.5.2.1.1
    //= type=implication
    //# Subsequent frames MUST be in order and MUST contain an increment of 1
    //# from the previous frame.
    && MessageFramesAreMonotonic(regularFrames)
    // All frames MUST all be from the same messages i.e. the same header
    && MessageFramesAreForTheSameMessage(regularFrames)
  }

  type NonFramedMessage = Frames.NonFramed

  datatype FramedMessageBody = FramedMessageBody(
    regularFrames: MessageRegularFrames,
    //= compliance/data-format/message-body.txt#2.5.2.2
    //= type=implication
    //# Framed data MUST contain exactly one final frame.
    finalFrame: Frames.FinalFrame
  )

  type FramedMessage = body: FramedMessageBody
  |
  //= compliance/data-format/message-body.txt#2.5.2.2.2
  //= type=implication
  //# The Final Frame Sequence number MUST be equal to the total number of
  //# frames in the Framed Data.
  && MessageFramesAreMonotonic(body.regularFrames + [body.finalFrame])
  && MessageFramesAreForTheSameMessage(body.regularFrames + [body.finalFrame])
  witness *

  type MessageFrame = frame: Frames.Frame
  |
  || Frames.IsFinalFrame(frame)
  || Frames.IsRegularFrame(frame)
  witness *

  type Frame = frame: Frames.Frame
  |
  || Frames.IsFinalFrame(frame)
  || Frames.IsRegularFrame(frame)
  || Frames.NonFramed?(frame)
  witness *

  lemma LemmaAddingNextRegularFrame(
    regularFrames: MessageRegularFrames,
    nextRegularFrame: Frames.RegularFrame
  )
    requires |regularFrames| + 1 < ENDFRAME_SEQUENCE_NUMBER as nat
    requires nextRegularFrame.seqNum as nat == |regularFrames| + 1 as nat
    requires 0 < |regularFrames| ==> Seq.First(regularFrames).header == nextRegularFrame.header
    ensures IsMessageRegularFrames(regularFrames + [nextRegularFrame])
  {}

  method {:vcs_split_on_every_assert} EncryptMessageBody(
    plaintext: seq<uint8>,
    header : Header.Header,
    key: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (result: Result<FramedMessage, Types.Error>)
    requires |key| == SerializableTypes.GetEncryptKeyLength(header.suite) as nat
    requires 4 <= SerializableTypes.GetIvLength(header.suite)
    requires 0 < header.body.frameLength as nat <= UINT32_LIMIT
    requires header.body.contentType.Framed?

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    ensures match result
      case Failure(e) => true
      case Success(body) =>
      // add final
        && (forall i, j, frame: Frames.Frame, callEvent
          |
            && 0 <= i < |body.regularFrames|
            && |old(crypto.History.AESEncrypt)| < j < |crypto.History.AESEncrypt| - 1
            && i == j - |old(crypto.History.AESEncrypt)|
            && frame == body.regularFrames[i]
            && callEvent == crypto.History.AESEncrypt[j]
          ::
            && callEvent.input.key == key
            && callEvent.output.Success?
            && frame.header == header
            && frame.seqNum as nat == i + 1
            && frame.iv == callEvent.input.iv
            && frame.encContent == callEvent.output.value.cipherText
            && frame.authTag == callEvent.output.value.authTag
          )
  {
    var n : int, sequenceNumber := 0, START_SEQUENCE_NUMBER;
    var regularFrames: MessageRegularFrames := [];

    //= compliance/client-apis/encrypt.txt#2.7
    //# Before the end of the input is indicated, this operation MUST process
    //# as much of the consumable bytes as possible by constructing regular
    //# frames (Section 2.7.1).

    //= compliance/client-apis/encrypt.txt#2.7
    //# If an end to the input has been indicated, there are no more
    //# consumable plaintext bytes to process, and a final frame has not yet
    //# been constructed, this operation MUST construct an empty final frame
    //# (Section 2.7.1).
    // This one is true because of our while loop construction below, where we check that
    // adding another frame puts us at < |plaintext|. This means we will never
    // consume the entire plaintext in this while loop, and will always construct
    // a final frame after exiting it.
    while n + header.body.frameLength as nat < |plaintext|
      invariant |plaintext| != 0 ==> 0 <= n < |plaintext|
      invariant |plaintext| == 0 ==> 0 == n
      invariant START_SEQUENCE_NUMBER <= sequenceNumber <= ENDFRAME_SEQUENCE_NUMBER
      invariant |regularFrames| == (sequenceNumber - START_SEQUENCE_NUMBER) as nat
      invariant |crypto.History.AESEncrypt| - |old(crypto.History.AESEncrypt)|
        == |regularFrames|
      invariant 0 < |regularFrames| ==> Seq.First(regularFrames).header == header;
      invariant IsMessageRegularFrames(regularFrames)

      invariant forall i: nat, j: nat, frame: Frames.Frame, callEvent
      |
        && 0 <= i < |regularFrames|
        && |old(crypto.History.AESEncrypt)| <= j < |crypto.History.AESEncrypt|
        && i == j - |old(crypto.History.AESEncrypt)|
        && frame == regularFrames[i]
        && callEvent == crypto.History.AESEncrypt[j]
      ::
        && callEvent.input.key == key
        && callEvent.output.Success?
        && frame.header == header
        && frame.seqNum as nat == i + 1
        && frame.iv == callEvent.input.iv
        && frame.encContent == callEvent.output.value.cipherText
        && frame.authTag == callEvent.output.value.authTag

    {
      :- Need(sequenceNumber < ENDFRAME_SEQUENCE_NUMBER, Types.AwsEncryptionSdkException(
          message := "too many frames"));
      var plaintextFrame := plaintext[n..n + header.body.frameLength as nat];

      //= compliance/client-apis/encrypt.txt#2.7
      //# *  If there are enough input plaintext bytes consumable to create a
      //# new regular frame, such that creating a regular frame does not
      //# processes all consumable bytes, then this operation MUST construct
      //# a regular frame (Section 2.7.1) using the consumable plaintext
      //# bytes.
      var regularFrame :- EncryptRegularFrame(
        key,
        header,
        plaintextFrame,
        sequenceNumber,
        crypto
      );

      assert regularFrame.seqNum as nat == |regularFrames| + 1;
      LemmaAddingNextRegularFrame(regularFrames, regularFrame);
      regularFrames := regularFrames + [regularFrame];

      n := n + header.body.frameLength as nat;

      //= compliance/client-apis/encrypt.txt#2.7.1
      //# Otherwise, this value MUST be 1 greater than
      //# the value of the sequence number of the previous frame.
      sequenceNumber := sequenceNumber + 1;
    }

    //= compliance/client-apis/encrypt.txt#2.7
    //# *  If there are not enough input consumable plaintext bytes to create
    //# a new regular frame, then this operation MUST construct a final
    //# frame (Section 2.7.1)
    //
    //= compliance/data-format/message-body.txt#2.5.2.2
    //# *  When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a
    //# Final Frame.

    //= compliance/client-apis/encrypt.txt#2.7
    //# *  If there are exactly enough consumable plaintext bytes to create
    //# one regular frame, such that creating a regular frame processes
    //# all consumable bytes, then this operation MUST construct either a
    //# final frame or regular frame (Section 2.7.1) with the remaining
    //# plaintext.
    //
    //= compliance/data-format/message-body.txt#2.5.2.2
    //# *  When the length of the Plaintext is an exact multiple of the Frame
    //# Length (including if it is equal to the frame length), the Final
    //# Frame encrypted content length SHOULD be equal to the frame length
    //# but MAY be 0.
    var finalFrame :- EncryptFinalFrame(
      key,
      header,
      plaintext[n..],
      sequenceNumber,
      crypto
    );

    result := Success(FramedMessageBody(
      regularFrames := regularFrames,
      finalFrame := finalFrame
    ));
  }

  method EncryptRegularFrame(
    key: seq<uint8>,
    header: Header.Header,
    plaintext: seq<uint8>,
    sequenceNumber: uint32,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<Frames.RegularFrame, Types.Error>)
    requires |key| == SerializableTypes.GetEncryptKeyLength(header.suite) as nat
    requires 4 <= SerializableTypes.GetIvLength(header.suite)
    requires 0 < header.body.frameLength as nat <= UINT32_LIMIT && START_SEQUENCE_NUMBER <= sequenceNumber < ENDFRAME_SEQUENCE_NUMBER
    requires header.body.contentType.Framed?
    requires |plaintext| <= UINT32_LIMIT

    //= compliance/client-apis/encrypt.txt#2.7.1
    //= type=implication
    //# o  For a regular frame the length of this plaintext MUST equal
    //# the frame length.

    //= compliance/client-apis/encrypt.txt#2.7.1
    //= type=implication
    //# -  For a regular frame the length of this plaintext subsequence
    //# MUST equal the frame length.
    requires |plaintext| == header.body.frameLength as nat && sequenceNumber != ENDFRAME_SEQUENCE_NUMBER

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()
    ensures old(crypto.History.AESEncrypt) < crypto.History.AESEncrypt
    ensures crypto.History.AESEncrypt == old(crypto.History.AESEncrypt) + [Seq.Last(crypto.History.AESEncrypt)]

    ensures match res
      case Failure(e) => true
      case Success(frame) =>
        && Seq.Last(crypto.History.AESEncrypt).output.Success?
        && var encryptionOutput := Seq.Last(crypto.History.AESEncrypt).output.value;
        && var encryptionInput := Seq.Last(crypto.History.AESEncrypt).input;

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# To construct a regular or final frame that represents the next frame
        //# in the encrypted message's body, this operation MUST calculate the
        //# encrypted content and an authentication tag using the authenticated
        //# encryption algorithm (../framework/algorithm-suites.md#encryption-
        //# algorithm) specified by the algorithm suite (../framework/algorithm-
        //# suites.md), with the following inputs:
        && encryptionInput.encAlg == header.suite.encrypt.AES_GCM
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The AAD is the serialized message body AAD (../data-format/
        //#    message-body-aad.md), constructed as follows:
        && encryptionInput.aad == BodyAAD(
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The message ID (../data-format/message-body-aad.md#message-id)
          //#    is the same as the message ID (../data-frame/message-
          //#    header.md#message-id) serialized in the header of this message.
          header.body.messageId,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The Body AAD Content (../data-format/message-body-aad.md#body-
          //# aad-content) depends on whether the thing being encrypted is a
          //# regular frame or final frame.  Refer to Message Body AAD
          //# (../data-format/message-body-aad.md) specification for more
          //# information.
          AADRegularFrame,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The sequence number (../data-format/message-body-
          //# aad.md#sequence-number) is the sequence number of the frame
          //# being encrypted.
          sequenceNumber,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The content length (../data-format/message-body-aad.md#content-
          //# length) MUST have a value equal to the length of the plaintext
          //# being encrypted.
          |plaintext| as uint64
        )
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The IV is the sequence number (../data-format/message-body-
        //#   aad.md#sequence-number) used in the message body AAD above, padded
        //#   to the IV length (../data-format/message-header.md#iv-length).
        && encryptionInput.iv == IVSeq(header.suite, sequenceNumber)

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The cipherkey is the derived data key
        && encryptionInput.key == key
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The plaintext is the next subsequence of consumable plaintext
        //# bytes that have not yet been encrypted.
        && encryptionInput.msg == plaintext

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# This operation MUST serialize a regular frame or final frame with the
        //# following specifics:
        && frame.header == header
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Sequence Number (../data-format/message-body.md#sequence-number):
        //# MUST be the sequence number of this frame, as determined above.
        && frame.seqNum == sequenceNumber
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  IV (../data-format/message-body.md#iv): MUST be the IV used when
        //# calculating the encrypted content above
        && frame.iv == encryptionInput.iv
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Encrypted Content (../data-format/message-body.md#encrypted-
        //# content): MUST be the encrypted content calculated above.
        && frame.encContent == encryptionOutput.cipherText
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Authentication Tag (../data-format/message-body.md#authentication-
        //# tag): MUST be the authentication tag output when calculating the
        //# encrypted content above.
        && frame.authTag == encryptionOutput.authTag
  {
    var iv := IVSeq(header.suite, sequenceNumber);
    var aad := BodyAAD(
      header.body.messageId,
      AADRegularFrame,
      sequenceNumber,
      |plaintext| as uint64
    );

    var aesEncryptInput := Primitives.Types.AESEncryptInput(
      encAlg := header.suite.encrypt.AES_GCM,
      iv := iv,
      key := key,
      msg := plaintext,
      aad := aad
    );

    var maybeEncryptionOutput := crypto.AESEncrypt(aesEncryptInput);
    var encryptionOutput :- maybeEncryptionOutput
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    var frame: Frames.RegularFrame := Frames.Frame.RegularFrame(
      header,
      sequenceNumber,
      iv,
      encryptionOutput.cipherText,
      encryptionOutput.authTag
    );

    return Success(frame);
  }

  method EncryptFinalFrame(
    key: seq<uint8>,
    header: Header.Header,
    plaintext: seq<uint8>,
    sequenceNumber: uint32,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<Frames.FinalFrame, Types.Error>)
    requires |key| == SerializableTypes.GetEncryptKeyLength(header.suite) as nat
    requires 4 <= SerializableTypes.GetIvLength(header.suite)
    requires START_SEQUENCE_NUMBER <= sequenceNumber <= ENDFRAME_SEQUENCE_NUMBER
    requires 0 <= |plaintext| <= UINT32_LIMIT
    requires header.body.contentType.Framed?
    //= compliance/client-apis/encrypt.txt#2.7.1
    //= type=implication
    //# -  For a final frame this MUST be the remaining plaintext bytes
    //# which have not yet been encrypted, whose length MUST be equal
    //# to or less than the frame length.
    //
    //= compliance/data-format/message-body.txt#2.5.2.2
    //= type=implication
    //# The length of the plaintext to be encrypted in the Final Frame MUST
    //# be greater than or equal to 0 and less than or equal to the Frame
    //# Length (message-header.md#frame-length).
    requires |plaintext| <= header.body.frameLength as nat

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()
    ensures old(crypto.History.AESEncrypt) < crypto.History.AESEncrypt
    ensures crypto.History.AESEncrypt == old(crypto.History.AESEncrypt) + [Seq.Last(crypto.History.AESEncrypt)]

    ensures match res
      case Failure(e) => true
      case Success(frame) =>
        && Seq.Last(crypto.History.AESEncrypt).output.Success?
        && var encryptionOutput := Seq.Last(crypto.History.AESEncrypt).output.value;
        && var encryptionInput := Seq.Last(crypto.History.AESEncrypt).input;

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# To construct a regular or final frame that represents the next frame
        //# in the encrypted message's body, this operation MUST calculate the
        //# encrypted content and an authentication tag using the authenticated
        //# encryption algorithm (../framework/algorithm-suites.md#encryption-
        //# algorithm) specified by the algorithm suite (../framework/algorithm-
        //# suites.md), with the following inputs:
        && encryptionInput.encAlg == header.suite.encrypt.AES_GCM
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The AAD is the serialized message body AAD (../data-format/
        //#    message-body-aad.md), constructed as follows:
        && encryptionInput.aad == BodyAAD(
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The message ID (../data-format/message-body-aad.md#message-id)
          //#    is the same as the message ID (../data-frame/message-
          //#    header.md#message-id) serialized in the header of this message.
          header.body.messageId,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The Body AAD Content (../data-format/message-body-aad.md#body-
          //# aad-content) depends on whether the thing being encrypted is a
          //# regular frame or final frame.  Refer to Message Body AAD
          //# (../data-format/message-body-aad.md) specification for more
          //# information.
          AADFinalFrame,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The sequence number (../data-format/message-body-
          //# aad.md#sequence-number) is the sequence number of the frame
          //# being encrypted.
          sequenceNumber,
          //= compliance/client-apis/encrypt.txt#2.7.1
          //= type=implication
          //# -  The content length (../data-format/message-body-aad.md#content-
          //# length) MUST have a value equal to the length of the plaintext
          //# being encrypted.
          |plaintext| as uint64
        )
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The IV is the sequence number (../data-format/message-body-
        //#   aad.md#sequence-number) used in the message body AAD above, padded
        //#   to the IV length (../data-format/message-header.md#iv-length).
        && encryptionInput.iv == IVSeq(header.suite, sequenceNumber)

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The cipherkey is the derived data key
        && encryptionInput.key == key
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  The plaintext is the next subsequence of consumable plaintext
        //# bytes that have not yet been encrypted.
        && encryptionInput.msg == plaintext

        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# This operation MUST serialize a regular frame or final frame with the
        //# following specifics:
        && frame.header == header
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Sequence Number (../data-format/message-body.md#sequence-number):
        //# MUST be the sequence number of this frame, as determined above.
        && frame.seqNum == sequenceNumber
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  IV (../data-format/message-body.md#iv): MUST be the IV used when
        //# calculating the encrypted content above
        && frame.iv == encryptionInput.iv
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Encrypted Content (../data-format/message-body.md#encrypted-
        //# content): MUST be the encrypted content calculated above.
        && frame.encContent == encryptionOutput.cipherText
        //= compliance/client-apis/encrypt.txt#2.7.1
        //= type=implication
        //# *  Authentication Tag (../data-format/message-body.md#authentication-
        //# tag): MUST be the authentication tag output when calculating the
        //# encrypted content above.
        && frame.authTag == encryptionOutput.authTag
  {

    var iv := IVSeq(header.suite, sequenceNumber);

    var aad := BodyAAD(
      header.body.messageId,
      AADFinalFrame,
      sequenceNumber,
      //= compliance/client-apis/encrypt.txt#2.7.1
      //# o  For a final frame this MUST be the length of the remaining
      //# plaintext bytes which have not yet been encrypted, whose
      //# length MUST be equal to or less than the frame length.
      |plaintext| as uint64
    );

    var aesEncryptInput := Primitives.Types.AESEncryptInput(
      encAlg := header.suite.encrypt.AES_GCM,
      iv := iv,
      key := key,
      msg := plaintext,
      aad := aad
    );

    var maybeEncryptionOutput := crypto.AESEncrypt(aesEncryptInput);

    var encryptionOutput :- maybeEncryptionOutput
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    var finalFrame: Frames.FinalFrame := Frames.Frame.FinalFrame(
      header,
      sequenceNumber,
      iv,
      encryptionOutput.cipherText,
      encryptionOutput.authTag
    );

    return Success(finalFrame);
  }

  method {:vcs_split_on_every_assert} DecryptFramedMessageBody(
    body: FramedMessage,
    key: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<seq<uint8>, Types.Error>)
    requires |key| == SerializableTypes.GetEncryptKeyLength(body.finalFrame.header.suite) as nat

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()
    ensures old(crypto.History.AESDecrypt) < crypto.History.AESDecrypt

    ensures match res
      case Failure(_) => true
      case Success(plaintext) => //Exists a sequence of frames which encrypts the plaintext and is serialized in the read section of the stream
        var decryptCalls := crypto.History.AESDecrypt[|old(crypto.History.AESDecrypt)|..];
        && (forall call <- decryptCalls :: call.output.Success?)
        && SumDecryptCalls(decryptCalls) == plaintext
        && (forall cryptoCall <- decryptCalls
          ::
            && cryptoCall.input.key == key
            && exists frame <- body.regularFrames + [body.finalFrame]
              ::
                && frame.encContent == cryptoCall.input.cipherTxt
                && frame.authTag == cryptoCall.input.authTag)

  {
    var plaintext := [];

    // Dafny has trouble joining this seq with `body.regularFrames` and `plaintextSeg`
    // It seems simpler to create a sub seq that I can prove is equivalent
    var AESDecryptHistory := crypto.History.AESDecrypt[|old(crypto.History.AESDecrypt)|..];
    assert |AESDecryptHistory| == 0;
    assert SumDecryptCalls(AESDecryptHistory) == plaintext;

    for i := 0 to |body.regularFrames|
      // // The goal is to assert FramesEncryptPlaintext.
      // // But this requires the final frame e.g. a FramedMessage.
      // // So I decompose this into parts
      invariant |body.regularFrames[..i]| == |AESDecryptHistory|
      invariant crypto.History.AESDecrypt == old(crypto.History.AESDecrypt) + AESDecryptHistory
      // So far: All decrypted frames decrypt to the list of plaintext chunks
      invariant forall j
      | 0 <= j < i
      ::
        && AESDecryptHistory[j].output.Success?
        && AESDecryptHistory[j].input.key == key
        && AESDecryptHistory[j].input.cipherTxt == body.regularFrames[j].encContent
        && AESDecryptHistory[j].input.authTag == body.regularFrames[j].authTag
        && AESDecryptHistory[j].input.iv == body.regularFrames[j].iv
        // && AESDecryptHistory[j].input.aad == BodyAADByFrameType(body.regularFrames[j])
      invariant SumDecryptCalls(AESDecryptHistory) == plaintext
    {

      var plaintextSegment :- DecryptFrame(body.regularFrames[i], key, crypto);
      plaintext := plaintext + plaintextSegment;

      AESDecryptHistory := AESDecryptHistory + [Seq.Last(crypto.History.AESDecrypt)];
      assert Seq.Last(AESDecryptHistory) == Seq.Last(crypto.History.AESDecrypt);
      assert crypto.History.AESDecrypt[i + |old(crypto.History.AESDecrypt)|].input.iv == body.regularFrames[i].iv;
    }

    var finalPlaintextSegment :- DecryptFrame(body.finalFrame, key, crypto);
    plaintext := plaintext + finalPlaintextSegment;

    res := Success(plaintext);

    AESDecryptHistory := AESDecryptHistory + [Seq.Last(crypto.History.AESDecrypt)];
    assert SumDecryptCalls(AESDecryptHistory) == plaintext;
    assert AESDecryptHistory == crypto.History.AESDecrypt[|old(crypto.History.AESDecrypt)|..];
  }

  method {:vcs_split_on_every_assert} DecryptFrame(
    frame: Frame,
    key: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<seq<uint8>, Types.Error>)
    requires |key| == SerializableTypes.GetEncryptKeyLength(frame.header.suite) as nat

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()
    ensures old(crypto.History.AESDecrypt) < crypto.History.AESDecrypt
    ensures crypto.History.AESDecrypt == old(crypto.History.AESDecrypt) + [Seq.Last(crypto.History.AESDecrypt)]

    ensures match res
      case Success(plaintextSegment) => ( // Decrypting the frame encoded in the stream is the returned ghost frame
        && Seq.Last(crypto.History.AESDecrypt).output.Success?
        && var decryptInput := Seq.Last(crypto.History.AESDecrypt).input;
        && decryptInput.encAlg == frame.header.suite.encrypt.AES_GCM
        && decryptInput.key == key
        && decryptInput.cipherTxt == frame.encContent
        && decryptInput.authTag == frame.authTag
        && decryptInput.iv == frame.iv
        && decryptInput.aad == BodyAADByFrameType(frame)
        && res.value == Seq.Last(crypto.History.AESDecrypt).output.value
        && |res.value| == |decryptInput.cipherTxt|

        && (frame.RegularFrame? || frame.FinalFrame? ==> |plaintextSegment| <= UINT32_LIMIT)
      )
      case Failure(_) => true
  {
    var aad := BodyAADByFrameType(frame);

    var maybePlaintextSegment :=
      // This is here as a citation for the decryption.
      // The manner in which it is currently called does not allow for a
      // single frame to be decrypted. We are ok with this as there is no
      // streaming.
      //= compliance/client-apis/decrypt.txt#2.7.4
      //= type=exception
      //# Once at least a single frame is deserialized (or the entire body in
      //# the un-framed case), this operation MUST decrypt and authenticate the
      //# frame (or body) using the authenticated encryption algorithm
      //# (../framework/algorithm-suites.md#encryption-algorithm) specified by
      //# the algorithm suite (../framework/algorithm-suites.md), with the
      //# following inputs:
      crypto.AESDecrypt(
        Primitives.Types.AESDecryptInput(
          encAlg := frame.header.suite.encrypt.AES_GCM,
            //#*  The cipherkey is the derived data key
          key := key,
            //#*  The ciphertext is the encrypted content (../data-format/message-
            //#   body.md#encrypted-content).
          cipherTxt := frame.encContent,
            //#*  the tag is the value serialized in the authentication tag field
            //#   (../data-format/message-body.md#authentication-tag) in the message
            //#   body or frame.
          authTag := frame.authTag,
            //#*  The IV is the sequence number (../data-format/message-body-
            //#   aad.md#sequence-number) used in the message body AAD above, padded
            //#   to the IV length (../data-format/message-header.md#iv-length) with
            //#   0.
          iv := frame.iv,
            //#*  The AAD is the serialized message body AAD (../data-format/
            //#   message-body-aad.md)
          aad := aad
        ));

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# If this decryption fails, this operation MUST immediately halt and
    //# fail.
    var plaintextSegment :- maybePlaintextSegment
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    return Success(plaintextSegment);
  }

  /*
   * Extracts the Body Additional Authenticated Data as per the
   * AWS Encryption SDK Specification for Message Body AAD.
   */
  function method BodyAADByFrameType(
    frame: Frame
  )
    : (aad: seq<uint8>)
  {
    var (sequenceNumber, bc, length) := match frame
      case RegularFrame(header,seqNum,_,_,_) => (
        //= compliance/data-format/message-body-aad.txt#2.4.3
        //# For framed data (message-body.md#framed-data), the value of this
        //# field MUST be the frame sequence number (message-body.md#sequence-
        //# number).
        seqNum,

        //= compliance/data-format/message-body-aad.txt#2.4.2
        //# *  The regular frames (message-body.md#regular-frame) in framed data
        //# (message-body.md#framed-data) MUST use the value
        //# "AWSKMSEncryptionClient Frame".
        AADRegularFrame,

        //= compliance/data-format/message-body-aad.txt#2.4.4
        //# *  For framed data (message-body.md#framed-data), this value MUST
        //# equal the length, in bytes, of the plaintext being encrypted in
        //# this frame.

        //= compliance/data-format/message-body-aad.txt#2.4.4
        //# -  For regular frames (message-body.md#regular-frame), this value
        //# MUST equal the value of the frame length (message-
        //# header.md#frame-length) field in the message header.
        header.body.frameLength as uint64
      )
      case FinalFrame(_,seqNum,_, encContent,_) => (
        //= compliance/data-format/message-body-aad.txt#2.4.3
        //# For framed data (message-body.md#framed-data), the value of this
        //# field MUST be the frame sequence number (message-body.md#sequence-
        //# number).
        seqNum,

        //= compliance/data-format/message-body-aad.txt#2.4.2
        //# *  The final frame (message-body.md#final-frame) in framed data
        //# (message-body.md#framed-data) MUST use the value
        //# "AWSKMSEncryptionClient Final Frame".
        AADFinalFrame,

        //= compliance/data-format/message-body-aad.txt#2.4.4
        //# *  For framed data (message-body.md#framed-data), this value MUST
        //# equal the length, in bytes, of the plaintext being encrypted in
        //# this frame.

        //= compliance/data-format/message-body-aad.txt#2.4.4
        //# -  For the final frame (message-body.md#final-frame), this value
        //# MUST be greater than or equal to 0 and less than or equal to
        //# the value of the frame length (message-header.md#frame-length)
        //# field in the message header.
        |encContent| as uint64
      )
      case NonFramed(_,_,encContent,_) => (
        //= compliance/client-apis/decrypt.txt#2.7.4
        //# If this is un-framed data,
        //# this value MUST be 1.

        //= compliance/data-format/message-body-aad.txt#2.4.3
        //# For non-framed data (message-body.md#non-framed-data), the
        //# value of this field MUST be "1".
        NONFRAMED_SEQUENCE_NUMBER,
        
        //= compliance/data-format/message-body-aad.txt#2.4.2
        //# *  Non-framed data (message-body.md#non-framed-data) MUST use the
        //# value "AWSKMSEncryptionClient Single Block".
        AADSingleBlock,

        //= compliance/data-format/message-body-aad.txt#2.4.4
        //# *  For non-framed data (message-body.md#non-framed-data), this value
        //# MUST equal the length, in bytes, of the plaintext data provided to
        //# the algorithm for encryption.
        |encContent| as uint64
      );

    BodyAAD(frame.header.body.messageId, bc, sequenceNumber, length)
  }

  /*
   * Serializes the Message Body ADD
   */
  function method BodyAAD(
    messageID: seq<uint8>,
    bc: BodyAADContent,
    sequenceNumber: uint32,
    length: uint64
  ) 
    : (aad: seq<uint8>)
  {
    var contentAAD := UTF8.Encode(BodyAADContentTypeString(bc));
    //= compliance/client-apis/decrypt.txt#2.7.4
    //#*  The AAD is the serialized message body AAD (../data-format/
    //#   message-body-aad.md), constructed as follows:

      //# -  The message ID (../data-format/message-body-aad.md#message-id)
      //#    is the same as the message ID (../data-frame/message-
      //#    header.md#message-id) deserialized from the header of this
      //#    message.
      messageID
      //# -  The Body AAD Content (../data-format/message-body-aad.md#body-
      //#    aad-content) depends on whether the thing being decrypted is a
      //#    regular frame, final frame, or un-framed data.  Refer to
      //#    Message Body AAD (../data-format/message-body-aad.md)
      //#    specification for more information.
      + contentAAD.value
      //# -  The sequence number (../data-format/message-body-
      //#    aad.md#sequence-number) is the sequence number deserialized
      //#    from the frame being decrypted. 
      + UInt32ToSeq(sequenceNumber)
      //= compliance/client-apis/decrypt.txt#2.7.4
      //# -  The content length (../data-format/message-body-aad.md#content-
      //# length) MUST have a value equal to the length of the plaintext
      //# that was encrypted.
      + UInt64ToSeq(length)
  }

  function method WriteFramedMessageBody(
    body: FramedMessage
  )
    :(ret: seq<uint8>)
    //= compliance/data-format/message-body.txt#2.5.2.2
    //= type=implication
    //# The final frame
    //# MUST be the last frame.
    ensures ret == WriteMessageRegularFrames(body.regularFrames)
      + Frames.WriteFinalFrame(body.finalFrame)
  {
    WriteMessageRegularFrames(body.regularFrames) + Frames.WriteFinalFrame(body.finalFrame)
  }

  function method WriteMessageRegularFrames(
    frames: MessageRegularFrames
  )
    :(ret: seq<uint8>)
    ensures if |frames| == 0 then
      ret == []
    else
      ret == WriteMessageRegularFrames(Seq.DropLast(frames))
      + Frames.WriteRegularFrame(Seq.Last(frames))
  {
    if |frames| == 0 then []
    else
      WriteMessageRegularFrames(Seq.DropLast(frames))
      + Frames.WriteRegularFrame(Seq.Last(frames))
  }

  function method {:recursive} {:vcs_split_on_every_assert} ReadFramedMessageBody(
    buffer: ReadableBuffer,
    header: Frames.FramedHeader,
    regularFrames: MessageRegularFrames,
    continuation: ReadableBuffer
  )
    :(res: ReadCorrect<FramedMessage>)
    requires forall frame: Frames.Frame | frame in regularFrames :: frame.header == header
    requires buffer.bytes == continuation.bytes
    requires buffer.start <= continuation.start 
    requires 0 <= continuation.start <= |buffer.bytes|
    requires CorrectlyReadRange(buffer, continuation, buffer.bytes[buffer.start..continuation.start])
    requires CorrectlyRead(buffer, Success(SuccessfulRead(regularFrames, continuation)), WriteMessageRegularFrames)
    decreases ENDFRAME_SEQUENCE_NUMBER as nat - |regularFrames|
    ensures CorrectlyRead(buffer, res, WriteFramedMessageBody)
    ensures res.Success?
    ==>
      && res.value.data.finalFrame.header == header
  {
    reveal CorrectlyReadRange();
    var sequenceNumber :- ReadUInt32(continuation);

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# If deserializing framed data (../data-format/message-body.md#framed-
    //# data), this operation MUST use the first 4 bytes of a frame to
    //# determine if the frame MUST be deserialized as a final frame
    //# (../data-format/message-body.md#final-frame) or regular frame
    //# (../fata-format/message-body/md#regular-frame).
    if (sequenceNumber.data != ENDFRAME_SEQUENCE_NUMBER) then
    
      assert {:split_here} true;

      //= compliance/client-apis/decrypt.txt#2.7.4
      //# Otherwise, this MUST
      //# be deserialized as the sequence number (../data-format/message-
      //# header.md#sequence-number) and the following bytes according to the
      //# regular frame spec (../data-format/message-body.md#regular-frame).
      var regularFrame :- Frames.ReadRegularFrame(continuation, header);
      assert buffer.bytes == continuation.bytes;
      assert buffer.bytes == regularFrame.tail.bytes by {
        reveal CorrectlyReadRange();
      }
      //= compliance/client-apis/decrypt.txt#2.7.4
      //# If this is framed data and the first
      //# frame sequentially, this value MUST be 1.
      // Where this is the seqNum
      // Imagine that |regularFrames| is 0, than seqNum is 1
      //= compliance/client-apis/decrypt.txt#2.7.4
      //# Otherwise, this
      //# value MUST be 1 greater than the value of the sequence number
      //# of the previous frame.
      :- Need(regularFrame.data.seqNum as nat == |regularFrames| + 1, Error("Sequence number out of order."));

      assert {:split_here} true;
      LemmaAddingNextRegularFrame(regularFrames, regularFrame.data);

      var nextRegularFrames: MessageRegularFrames := regularFrames + [regularFrame.data];

      assert {:split_here} true;
      assert CorrectlyRead(continuation, Success(regularFrame), Frames.WriteRegularFrame);
      assert continuation.bytes == regularFrame.tail.bytes by {
        reveal CorrectlyReadRange();
      }

      assert {:split_here} true;

      // This method recursively reads all the frames in the buffer,
      // instead of reading one frame a time, so this requirement cannot be met
      //= compliance/client-apis/decrypt.txt#2.7.4
      //= type=exception
      //# Once at least a single frame is deserialized (or the entire body in
      //# the un-framed case), this operation MUST decrypt and authenticate the
      //# frame (or body) using the authenticated encryption algorithm
      //# (../framework/algorithm-suites.md#encryption-algorithm) specified by
      //# the algorithm suite (../framework/algorithm-suites.md), with the
      //# following inputs:
      var res := Success(SuccessfulRead(nextRegularFrames, regularFrame.tail));
      assert CorrectlyRead(buffer, res, WriteMessageRegularFrames) by {
        reveal CorrectlyReadRange();

        var tail := res.value.tail;
        var readRange := WriteMessageRegularFrames(res.value.data);
        assert buffer.bytes == tail.bytes;
        assert buffer.start <= tail.start <= |buffer.bytes|;
        assert buffer.bytes[buffer.start..] == tail.bytes[buffer.start..];
        assert readRange <= buffer.bytes[buffer.start..];
        assert tail.start == buffer.start + |readRange|;
      }

      ReadFramedMessageBody(
        buffer,
        header,
        nextRegularFrames,
        regularFrame.tail
      )
    else
      //= compliance/client-apis/decrypt.txt#2.7.4
      //# If the first 4 bytes
      //# have a value of 0xFFFF, then this MUST be deserialized as the
      //# sequence number end (../data-format/message-header.md#sequence-
      //# number-end) and the following bytes according to the final frame spec
      //# (../data-format/message-body.md#final-frame).
      assert sequenceNumber.data == ENDFRAME_SEQUENCE_NUMBER;
      
      assert {:split_here} true;
      var finalFrame :- Frames.ReadFinalFrame(continuation, header);
      :- Need(
        finalFrame.data.seqNum as nat == |regularFrames| + 1,
        Error("Sequence number out of order.")
      );

      assert {:split_here} true;
      assert MessageFramesAreMonotonic(regularFrames + [finalFrame.data]);
      assert MessageFramesAreForTheSameMessage(regularFrames + [finalFrame.data]);

      var body: FramedMessage := FramedMessageBody(
        regularFrames := regularFrames,
        finalFrame := finalFrame.data
      );

      assert {:split_here} true;
      assert CorrectlyRead(continuation, Success(finalFrame), Frames.WriteFinalFrame);
      assert {:split_here} true;
      Success(SuccessfulRead(body, finalFrame.tail))
  }

  function WriteNonFramedMessageBody(
    body: NonFramedMessage
  )
    :(ret: seq<uint8>)
    ensures ret == Frames.WriteNonFramed(body)
  {
    Frames.WriteNonFramed(body)
  }

  function method {:vcs_split_on_every_assert} ReadNonFramedMessageBody(
    buffer: ReadableBuffer,
    header: Frames.NonFramedHeader
  )
    :(res: ReadCorrect<NonFramedMessage>)
    ensures CorrectlyRead(buffer, res, WriteNonFramedMessageBody)
    ensures res.Success?
    ==>
      && res.value.data.header == header
  {
    var block :- Frames.ReadNonFrame(buffer, header);
    Success(SuccessfulRead(block.data, block.tail))
  }
}
