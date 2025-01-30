// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "SerializableTypes.dfy"
include "SerializeFunctions.dfy"
include "Header.dfy"
include "EncryptionContext.dfy"
include "EncryptedDataKeys.dfy"

module Frames {
  import MPL = AwsCryptographyMaterialProvidersTypes
  import Seq
  import Header
  import opened EncryptedDataKeys
  import opened EncryptionContext
  import opened SerializableTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions

  //= compliance/client-apis/encrypt.txt#2.7.1
  //= type=implication
  //# If this is the first frame sequentially, this
  //# value MUST be 1.
  const START_SEQUENCE_NUMBER: uint32 := 1
  const ENDFRAME_SEQUENCE_NUMBER: uint32 := 0xFFFF_FFFF
  const NONFRAMED_SEQUENCE_NUMBER: uint32 := 1

  type FramedHeader = h : Header.Header
    | FramedHeader?(h)
    witness *

  predicate FramedHeader?(h : Header.Header) {
    h.body.contentType.Framed?
  }

  type NonFramedHeader = h : Header.Header
    | NonFramedHeader?(h)
    witness *

  predicate NonFramedHeader?(h : Header.Header) {
    h.body.contentType.NonFramed?
  }

  datatype Frame =
    | RegularFrame(
        header: Header.Header,
        seqNum: uint32,
        iv: seq<uint8>,
        encContent: seq<uint8>,
        authTag: seq<uint8>)
    | FinalFrame (
        header: Header.Header,
        seqNum: uint32,
        iv: seq<uint8>,
        encContent: seq<uint8>,
        authTag: seq<uint8>)
    | NonFramed(
        header: Header.Header,
        //= compliance/data-format/message-body.txt#2.5.1.1
        //= type=implication
        //# The
        //# IV MUST be a unique IV within the message.
        iv: seq<uint8>,
        encContent: seq<uint8>,
        authTag: seq<uint8>
      )

  predicate IvTagLengths(frame: Frame){
    && |frame.iv| == GetIvLength(frame.header.suite) as nat
    && |frame.authTag| == GetTagLength(frame.header.suite) as nat
  }

  predicate IsRegularFrame(frame: Frame){
    && frame.RegularFrame?
       //= compliance/data-format/message-body.txt#2.5.2.1.4
       //= type=implication
       //# The authentication tag length MUST be equal to the authentication tag
       //# length of the algorithm suite specified by the Algorithm Suite ID
       //# (message-header.md#algorithm-suite-id) field.
    && IvTagLengths(frame)
    && frame.header.body.contentType.Framed?
       //= compliance/data-format/message-body.txt#2.5.2.1.3
       //= type=implication
       //# The length of the encrypted content of a Regular Frame MUST be equal
       //# to the Frame Length.
    && |frame.encContent| == frame.header.body.frameLength as nat
    && frame.seqNum < ENDFRAME_SEQUENCE_NUMBER
  }

  type RegularFrame = frame: Frame
    | IsRegularFrame(frame)
    witness *

  predicate IsFinalFrame(frame: Frame) {
    && frame.FinalFrame?
       //= compliance/data-format/message-body.txt#2.5.2.2.6
       //= type=implication
       //# The authentication tag length MUST be equal to the authentication tag
       //# length of the algorithm suite specified by the Algorithm Suite ID
       //# (message-header.md#algorithm-suite-id) field.
    && IvTagLengths(frame)
    && frame.header.body.contentType.Framed?
    && |frame.encContent| <= frame.header.body.frameLength as nat
  }

  type FinalFrame = frame: Frame
    | IsFinalFrame(frame)
    witness *

  predicate NonFramed?(frame: Frame) {
    && frame.NonFramed?
    && IvTagLengths(frame)
    && frame.header.body.contentType.NonFramed?
       //= compliance/data-format/message-body.txt#2.5.1.2
       //= type=implication
       //# The length MUST NOT be greater than "2^36 - 32", or 64 gibibytes (64
       //# GiB), due to restrictions imposed by the implemented algorithms
       //# (../framework/algorithm-suites.md).
    && |frame.encContent| < SAFE_MAX_ENCRYPT
  }

  type NonFramed = frame: Frame
    | NonFramed?(frame)
    witness *

  //= compliance/data-format/message-body.txt#2.5.2
  //= type=implication
  //# *  The total bytes allowed in a single frame MUST be less than or
  //# equal to "2^32 - 1".
  lemma LemmaRegularOrFinalFrameHasUint32ContentByteLength(frame: Frame)
    ensures IsRegularFrame(frame) || IsFinalFrame(frame)
            ==> |frame.encContent| <= 0xFFFF_FFFF
  {}

  const SAFE_MAX_ENCRYPT := 0xFFFFFFFE0 // 2^36 - 32

  function method WriteRegularFrame(
    regularFrame: RegularFrame
  )
    :(ret: seq<uint8>)
    ensures
      && ReadUInt32(ReadableBuffer(ret, 0)).Success?
      && ReadUInt32(ReadableBuffer(ret, 0)).value.data != ENDFRAME_SEQUENCE_NUMBER
  {
    reveal ReadUInt32();
    reveal CorrectlyReadRange();

    WriteUint32(regularFrame.seqNum)
    + Write(regularFrame.iv)
    + Write(regularFrame.encContent)
    + Write(regularFrame.authTag)
  }

  function method {:vcs_split_on_every_assert} ReadRegularFrame(
    buffer: ReadableBuffer,
    header: FramedHeader
  )
    :(res: ReadCorrect<RegularFrame>)
    ensures res.Success?
            ==> res.value.data.header == header && res.value.tail.start <= |buffer.bytes|
    ensures CorrectlyRead(buffer, res, WriteRegularFrame)
  {

    var sequenceNumber :- ReadUInt32(buffer);
    assert CorrectlyReadRange(buffer, sequenceNumber.tail, WriteUint32(sequenceNumber.data));
    :- Need(sequenceNumber.data < ENDFRAME_SEQUENCE_NUMBER, Error("Regular frame sequence number can not equal or exceed the final frame."));
    assert buffer.bytes == sequenceNumber.tail.bytes by {
      reveal CorrectlyReadRange();
    }
    var iv :- Read(sequenceNumber.tail, GetIvLength(header.suite) as nat);
    var encContent :- Read(iv.tail, header.body.frameLength as nat);
    var authTag :- Read(encContent.tail, GetTagLength(header.suite) as nat);
    assert
      && authTag.tail.start <= |buffer.bytes|
      && buffer.start <= authTag.tail.start
      && authTag.tail.start == buffer.start + |buffer.bytes[buffer.start..authTag.tail.start]|
    by {
      reveal CorrectlyReadRange();
    }

    var regularFrame: RegularFrame := Frame.RegularFrame(
                                        header,
                                        sequenceNumber.data,
                                        iv.data,
                                        encContent.data,
                                        authTag.data
                                      );

    assert CorrectlyReadRange(buffer, authTag.tail, WriteRegularFrame(regularFrame)) by {
      CorrectlyReadByteRange(buffer, sequenceNumber.tail, WriteUint32(sequenceNumber.data));
      AppendToCorrectlyReadByteRange(buffer, sequenceNumber.tail, iv.tail, Write(iv.data));
      AppendToCorrectlyReadByteRange(buffer, iv.tail, encContent.tail, Write(encContent.data));
      AppendToCorrectlyReadByteRange(buffer, encContent.tail, authTag.tail, Write(authTag.data));
    }
    Success(SuccessfulRead(regularFrame, authTag.tail))
  }

  function method WriteFinalFrame(
    finalFrame: FinalFrame
  )
    :(ret: seq<uint8>)
    ensures
      && ReadUInt32(ReadableBuffer(ret, 0)).Success?
         //= compliance/data-format/message-body.txt#2.5.2.2.1
         //= type=implication
         //# The value MUST be encoded as the 4 bytes "FF FF FF FF" in hexadecimal
         //# notation.
      && ReadUInt32(ReadableBuffer(ret, 0)).value.data == ENDFRAME_SEQUENCE_NUMBER
  {
    reveal ReadUInt32();
    reveal CorrectlyReadRange();

    WriteUint32(ENDFRAME_SEQUENCE_NUMBER)
    + WriteUint32(finalFrame.seqNum)
    + Write(finalFrame.iv)
    + WriteUint32Seq(finalFrame.encContent)
    + Write(finalFrame.authTag)
  }

  function method {:vcs_split_on_every_assert} ReadFinalFrame(
    buffer: ReadableBuffer,
    header: FramedHeader
  )
    :(res: ReadCorrect<FinalFrame>)
    ensures res.Success?
            ==> res.value.data.header == header
    ensures CorrectlyRead(buffer, res, WriteFinalFrame)

    //= compliance/client-apis/decrypt.txt#2.7.4
    //= type=implication
    //# If deserializing a final frame (../data-format/message-body.md#final-
    //# frame), this operation MUST ensure that the length of the encrypted
    //# content field is less than or equal to the frame length deserialized
    //# in the message header.
    ensures
      res.Success?
      ==>
        && var finalFrameSignalRes := ReadUInt32(buffer);
        && finalFrameSignalRes.Success?
        && var sequenceNumberRes := ReadUInt32(finalFrameSignalRes.value.tail);
        && sequenceNumberRes.Success?
        && var ivRes := Read(sequenceNumberRes.value.tail, GetIvLength(header.suite) as nat);
        && ivRes.Success?
        && var encContentRes := ReadUint32Seq(ivRes.value.tail);
        && encContentRes.Success?
        && |encContentRes.value.data| as uint32 <= header.body.frameLength

  {
    reveal ReadUInt32();
    var finalFrameSignal :- ReadUInt32(buffer);
    :- Need(finalFrameSignal.data == ENDFRAME_SEQUENCE_NUMBER, Error("Final frame sequence number MUST be the end-frame sequence number."));

    var sequenceNumber :- ReadUInt32(finalFrameSignal.tail);
    var iv :- Read(sequenceNumber.tail, GetIvLength(header.suite) as nat);

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# If deserializing a final frame (../data-format/message-body.md#final-
    //# frame), this operation MUST ensure that the length of the encrypted
    //# content field is less than or equal to the frame length deserialized
    //# in the message header.
    var contentLength :- ReadUInt32(iv.tail);
    :- Need(contentLength.data <= header.body.frameLength, Error("Content length MUST NOT exceed the frame length."));

    var encContent :- ReadUint32Seq(iv.tail);
    var authTag :- Read(encContent.tail, GetTagLength(header.suite) as nat);
    var finalFrame: FinalFrame := Frame.FinalFrame(
                                    header,
                                    sequenceNumber.data,
                                    iv.data,
                                    encContent.data,
                                    authTag.data
                                  );

    assert CorrectlyReadRange(buffer, authTag.tail, WriteFinalFrame(finalFrame)) by {
      reveal CorrectlyReadRange();
    // It seems that Dafny can find a solution pretty fast.
    // But I leave this here in case there is some problem later.
    // CorrectlyReadByteRange(buffer, finalFrameSignal.tail, WriteUint32(finalFrameSignal.data));
    // AppendToCorrectlyReadByteRange(buffer, finalFrameSignal.tail, sequenceNumber.tail, WriteUint32(sequenceNumber.data));
    // AppendToCorrectlyReadByteRange(buffer, sequenceNumber.tail, iv.tail, Write(iv.data));
    // AppendToCorrectlyReadByteRange(buffer, iv.tail, encContent.tail, WriteUint32Seq(encContent.data));
    // AppendToCorrectlyReadByteRange(buffer, encContent.tail, authTag.tail, Write(authTag.data));
    }

    Success(SuccessfulRead(finalFrame, authTag.tail))
  }

  function method ReadNonFrame(
    buffer: ReadableBuffer,
    header: NonFramedHeader
  )
    :(res: ReadCorrect<NonFramed>)
    ensures res.Success?
            ==> res.value.data.header == header
    ensures CorrectlyRead(buffer, res, WriteNonFramed)
  {
    var iv :- Read(buffer, GetIvLength(header.suite) as nat);
    // Checking only the content length _before_ reading it into memory
    // is just a nice thing to do given the sizes involved.
    var contentLength :- ReadUInt64(iv.tail);
    :- Need(contentLength.data as nat < SAFE_MAX_ENCRYPT, Error("Frame exceeds AES-GCM cryptographic safety for a single key/iv."));
    var encContent :- ReadUint64Seq(iv.tail);
    var authTag :- Read(encContent.tail, GetTagLength(header.suite) as nat);

    var nonFramed: NonFramed := Frame.NonFramed(
                                  header,
                                  iv.data,
                                  encContent.data,
                                  authTag.data
                                );

    assert CorrectlyReadRange(buffer, authTag.tail, WriteNonFramed(nonFramed)) by {
      reveal CorrectlyReadRange();
      CorrectlyReadByteRange(buffer, iv.tail, Write(iv.data));
      AppendToCorrectlyReadByteRange(buffer, iv.tail, encContent.tail, WriteUint64Seq(encContent.data));
      AppendToCorrectlyReadByteRange(buffer, encContent.tail, authTag.tail, Write(authTag.data));
    }

    Success(SuccessfulRead(nonFramed, authTag.tail))
  }

  // The Encryption SDK no longer supports writing Non-Framed Data.
  // That is why this is a function and not a function method.
  // This is here to support correct reading of Non-Framed Data.
  function WriteNonFramed(
    nonFramed: NonFramed
  )
    :(ret: seq<uint8>)
  {
    Write(nonFramed.iv)
    + WriteUint64Seq(nonFramed.encContent)
    + Write(nonFramed.authTag)
  }

}
