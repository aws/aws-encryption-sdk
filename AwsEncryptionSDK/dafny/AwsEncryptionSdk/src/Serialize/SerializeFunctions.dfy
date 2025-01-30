// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"

module {:options "/functionSyntax:4" } SerializeFunctions {
  import Seq
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8

  datatype ReadProblems =
      // This _may_ be recoverable.
      // if these is more data to read,
      // then after getting this data
      // a read function _may_ be able to return a datatype.
      // If the caller is at EOS,
      // then this is not recoverable.
    | MoreNeeded(pos: nat)
      // These errors should not be recoverable.
      // The data is incorrect
      // and reading the same data again will generate the same error.
    | Error(message: string)
  type MoreNeeded = p: ReadProblems | p.MoreNeeded? witness *

  // When reading a buffer,
  // there are the bytes to read,
  // and the position to start reading withing these bytes.
  // This data structure represents this information.
  datatype ReadableBuffer = ReadableBuffer(
    bytes: seq<uint8>,
    start: nat
  )

  // This holds the data from a successful read.
  // The `data` is the read type,
  // and the `tail` is the remaining data left to be read.
  datatype SuccessfulRead<T> = SuccessfulRead(
    data: T,
    tail: ReadableBuffer
  )

  type ReadResult<T, E> = Result<SuccessfulRead<T>, E>
  type ReadCorrect<T> = ReadResult<T, ReadProblems>
  // When reading binary it can not be incorrect.
  // It may not be enough data, but since it is raw binary,
  // it can not be wrong.
  // Generally this T is `seq<uint8>`,
  // but in the a few cases like uint16, uint32, uint64
  // it may be other types.
  type ReadBinaryCorrect<T> = ReadResult<T, MoreNeeded>

  // This is the composable unit
  // that all `ReadX` functions use to prove correctness.
  // The idea is that WriteX(ReadX(ReadableBuffer)) == ReadableBuffer.
  // That a read is the inverse of a write.
  // The assertion is that the write side is correct _by construction_.
  // This means that the order of writes *is* the correct order,
  // and no additional Dafny specifications are need to define a correct write.
  // This puts the correctness on the write side,
  // and the read side is correct _because_
  // what was read is what would have been written.
  ghost predicate CorrectlyRead<T> (
    buffer: ReadableBuffer,
    res: ReadCorrect<T>,
    inversionFunction: T -> seq<uint8>
  )
  {
    res.Success?
    ==>
      CorrectlyReadRange(buffer, res.value.tail, inversionFunction(res.value.data))
  }

  // Sequence ranges are complicated in Dafny.
  // By separating out the sub sequence
  // (the bytes under consideration)
  // it is easer to prove associativity.
  opaque ghost predicate CorrectlyReadRange(
    buffer: ReadableBuffer,
    tail: ReadableBuffer,
    readRange: seq<uint8>
  )
  {
    // This predicate defines what it means to correctly read a buffer.
    // `buffer` represents the state of a buffer before a read,
    // `tail` represents the returned buffer after a read,
    // and `readRange` represents the bytes read from that buffer.
    // We are essentially moving tail's pointer further down the buffer,
    // and in order to have read `readRange` we need
    // the following to be true:
    && buffer.bytes == tail.bytes
    // buffer and tail can start at the same place (i.e the beginning)
    // as you read more, tail will grow but not larger than the size of the buffer.
    && buffer.start <= tail.start <= |buffer.bytes|
    // buffer and tail bytes are the same because when we splice them using buffer.start
    // as the start all the way to end we have the same sequence
    && buffer.bytes[buffer.start..] == tail.bytes[buffer.start..]
    // the bytes read i.e. the readRange MUST be a subset of the bytes in the buffer.
    // further it MUST be the the prefix of the bytes sliced from the buffer's start.
    && readRange <= buffer.bytes[buffer.start..]
    // the start of where we have read up to so far must be equal to where the buffer starts
    // and the length of the sequence of what we have read.
    && tail.start == buffer.start + |readRange|
  }

  // Sequence ranges are complicated in Dafny.
  // To accumulate a bunch of reads together
  // you need to move from the bytes to the ranges.
  // e.g. buffer.bytes[buffer.start..tail.start]
  lemma CorrectlyReadByteRange(
    buffer: ReadableBuffer,
    tail: ReadableBuffer,
    readRange: seq<uint8>
  )
    requires CorrectlyReadRange(buffer, tail, readRange)
    ensures buffer.start <= tail.start <= |buffer.bytes|
    // we ensure that from buffer.start to tail.start we have correctly read that segment
    ensures CorrectlyReadRange(buffer, tail, buffer.bytes[buffer.start..tail.start])
    // once we know we have correctly read, we assert that the splice of buffer.start to tail.start
    // of the buffer is equal to the seq range which is what we have read so far.
    ensures buffer.bytes[buffer.start..tail.start] == readRange
  {
    // This helper lemma allows you to use it elsewhere in the project by revealing the
    // opaque predicate just here instead of everywhere in the project which lets
    // you control the resource count.
    reveal CorrectlyReadRange();
  }

  // Given a range of bytes,
  // This lets you pack on additionally correctly read bytes.
  // By starting with CorrectlyReadByteRange
  // you can compose several reads together.
  // e.g.
  // CorrectlyReadByteRange(buffer, firstRead.tail, Write(firstRead.data));
  // AppendToCorrectlyReadByteRange(buffer, firstRead.tail, secondRead.tail Write(secondRead.data));
  lemma AppendToCorrectlyReadByteRange(
    buffer: ReadableBuffer,
    verifiedTail: ReadableBuffer,
    tail:ReadableBuffer,
    readRange: seq<uint8>
  )
    requires buffer.start <= verifiedTail.start <= |buffer.bytes|
    // We require that we have correctly read up to the point where we have verified we have read to
    requires CorrectlyReadRange(buffer, verifiedTail, buffer.bytes[buffer.start..verifiedTail.start])
    // By moving the pointer to now start at verifiedTail we want to require that packing on the size of
    // readRange we stay within the bounds of the buffer.
    requires CorrectlyReadRange(verifiedTail, tail, readRange)
    // Make sure we have not read past the length of total available byte
    ensures buffer.start <= tail.start <= |buffer.bytes|
    // Tail is the point of where we have read to; this point is past the verifiedTail
    // but less than the total buffer.
    ensures CorrectlyReadRange(buffer, tail, buffer.bytes[buffer.start..tail.start])
    // We ensure that the chunk from the start of the buffer to the start of the tail
    // is equal to what we have previously proved we have read up to, to the new chunk
    // we are packing of correctly read bytes.
    ensures buffer.bytes[buffer.start..tail.start]
         == buffer.bytes[buffer.start..verifiedTail.start] + readRange
  {
    reveal CorrectlyReadRange();
    CorrectlyReadByteRange(verifiedTail, tail, readRange);
  }

  // This function is trivial,
  // but it lets `Read` have a `Write` function
  // for its `CorrectlyRead` ensures clause.
  // This facilitates proof
  // because both sides use exactly the same function.
  function Write(
    data: seq<uint8>
  )
    :(res: seq<uint8>)
    ensures data == res
  {
    data
  }

  opaque function Read(
    buffer: ReadableBuffer,
    length: nat
  ):
    (res: ReadBinaryCorrect<seq<uint8>>)
    // Optional elements in a data structure can be represented with a length of 0 bytes.
    ensures
      && buffer.start + length <= |buffer.bytes|
      <==>
      && res.Success?
      && |res.value.data| == length
    ensures
      && |buffer.bytes| < buffer.start + length
      <==>
      && res.Failure?
      && res.error.MoreNeeded?
      && res.error.pos == buffer.start + length
    ensures CorrectlyRead(buffer, res, Write)
  {
    reveal CorrectlyReadRange();

    var end := buffer.start + length;
    :- Need(|buffer.bytes| >= end, MoreNeeded(end));

    Success(SuccessfulRead(
              buffer.bytes[buffer.start..end],
              buffer.(start := end)))
  }

  function WriteUint16(
    number: uint16
  )
    :(ret: seq<uint8>)
  {
    Write(UInt16ToSeq(number))
  }

  opaque function ReadUInt16(
    buffer: ReadableBuffer
  )
    :(res: ReadBinaryCorrect<uint16>)
    ensures buffer.start + 2 <= |buffer.bytes| <==> res.Success?
    ensures CorrectlyRead(buffer, res, WriteUint16)
  {
    var SuccessfulRead(uint16Bytes, tail) :- Read(buffer, 2);
    Success(SuccessfulRead(SeqToUInt16(uint16Bytes), tail))
  }

  function WriteUint32(
    number: uint32
  )
    :(ret: seq<uint8>)
  {
    Write(UInt32ToSeq(number))
  }

  opaque function ReadUInt32(
    buffer: ReadableBuffer
  ):
    (res: ReadBinaryCorrect<uint32>)
    ensures buffer.start + 4 <= |buffer.bytes| <==> res.Success?
    ensures CorrectlyRead(buffer, res, WriteUint32)
  {
    var SuccessfulRead(uint32Bytes, tail) :- Read(buffer, 4);
    Success(SuccessfulRead(SeqToUInt32(uint32Bytes), tail))
  }

  function WriteUint64(
    number: uint64
  )
    :(ret: seq<uint8>)
  {
    Write(UInt64ToSeq(number))
  }

  opaque function ReadUInt64(
    buffer: ReadableBuffer
  ):
    (res: ReadBinaryCorrect<uint64>)
    ensures buffer.start + 8 <= |buffer.bytes| <==> res.Success?
    ensures CorrectlyRead(buffer, res, UInt64ToSeq)
  {
    var SuccessfulRead(uint64Bytes, tail) :- Read(buffer, 8);
    Success(SuccessfulRead(SeqToUInt64(uint64Bytes), tail))
  }

  function WriteShortLengthSeq(
    d: Uint8Seq16
  )
    :(res: seq<uint8>)
  {
    WriteUint16((|d| as uint16))
    + Write(d)
  }

  opaque function ReadShortLengthSeq(
    buffer: ReadableBuffer
  ):
    (res: ReadCorrect<Uint8Seq16>)
    ensures CorrectlyRead(buffer, res, WriteShortLengthSeq)
    ensures res.Success? ==>
              && ReadUInt16(buffer).Success?
              && |res.value.data| == ReadUInt16(buffer).value.data as nat
  {
    var length: SuccessfulRead<uint16> :- ReadUInt16(buffer);
    var d: SuccessfulRead<Uint8Seq16> :- Read(length.tail, length.data as nat);

    assert CorrectlyReadRange(buffer, d.tail, WriteShortLengthSeq(d.data)) by {
      reveal CorrectlyReadRange();
    }
    Success(d)
  }

  function WriteUint32Seq(
    d: Uint8Seq32
  )
    :(res: seq<uint8>)
  {
    WriteUint32(|d| as uint32)
    + Write(d)
  }

  opaque function ReadUint32Seq(
    buffer: ReadableBuffer
  ):
    (res: ReadCorrect<Uint8Seq32>)
    ensures CorrectlyRead(buffer, res, WriteUint32Seq)
    ensures res.Success? ==>
              && ReadUInt32(buffer).Success?
              && |res.value.data| == ReadUInt32(buffer).value.data as nat
  {
    var length :- ReadUInt32(buffer);
    var d: SuccessfulRead<Uint8Seq32> :- Read(length.tail, length.data as nat);

    assert CorrectlyReadRange(buffer, d.tail, WriteUint32Seq(d.data)) by {
      reveal CorrectlyReadRange();
    }

    Success(d)
  }

  function WriteUint64Seq(
    d: Uint8Seq64
  )
    :(res: seq<uint8>)
  {
    WriteUint64(|d| as uint64)
    + Write(d)
  }

  opaque function ReadUint64Seq(
    buffer: ReadableBuffer
  ):
    (res: ReadCorrect<Uint8Seq64>)
    ensures CorrectlyRead(buffer, res, WriteUint64Seq)
    ensures res.Success? ==>
              && ReadUInt64(buffer).Success?
              && |res.value.data| == ReadUInt64(buffer).value.data as nat
  {
    var length :- ReadUInt64(buffer);
    var d: SuccessfulRead<Uint8Seq64> :- Read(length.tail, length.data as nat);

    assert CorrectlyReadRange(buffer, d.tail, WriteUint64Seq(d.data)) by {
      reveal CorrectlyReadRange();
    }

    Success(d)
  }

  // Completeness Lemmas to prove that ReadX/WriteX are both sound and complete

  // Helper for completeness proofs.
  // The serialization soundness is defined by the writer.
  // This is why `CorrectlyRead` takes a writing function.
  // However this only proves soundness and not completeness.
  // Something that can be written but never read is "valid".
  // Proving that if the bytes were written,
  // then they MUST be readable is completeness.
  ghost predicate CorrectlyReadableByteRange?(
    buffer: ReadableBuffer,
    bytes: seq<uint8>
  )
    ensures CorrectlyReadableByteRange?(buffer, bytes)
            ==>
              && buffer.start <= buffer.start + |bytes| <= |buffer.bytes|
              && CorrectlyReadRange(buffer, buffer, buffer.bytes[buffer.start..buffer.start])
  {
    reveal CorrectlyReadRange();
    CorrectlyReadRange(buffer, MoveStart(buffer, |bytes|), bytes)
  }

  ghost function MoveStart(
    buffer: ReadableBuffer,
    length: nat
  )
    :(ret: ReadableBuffer)
  {
    buffer.(start := buffer.start + length)
  }

  // Useful lemma in order to prove the completeness of advancing the read buffer
  // of the chunk that you have written.
  lemma AdvanceCorrectlyReadableByteRange?(
    buffer: ReadableBuffer,
    bytes: seq<uint8>,
    verifiedTail: ReadableBuffer,
    readRange?: seq<uint8>
  )
    requires CorrectlyReadableByteRange?(buffer, bytes)
    // verfified tail is what we have read and verified we have correctly read
    // require that it is not longer than the length of bytes available
    requires buffer.start <= verifiedTail.start <= |buffer.bytes|
    // In order to pack on readRange? we MUST require that we have correctly read up to
    // verifiedTail. We could get the completeness that packing on verifiedTail succeeded from another
    // lemma, but in order to pack on we MUST be able to read up until the verifiedTail.
    requires CorrectlyReadRange(buffer, verifiedTail, buffer.bytes[buffer.start..verifiedTail.start])
    // The total number of bytes MUST be less than or equal to the sequence you have verified to have read
    // plus the additional sequence you are packing on
    requires buffer.bytes[buffer.start..verifiedTail.start] + readRange? <= bytes
    // Assert that we can pack on readRange? onto what we have verified we have read up to
    ensures CorrectlyReadableByteRange?(verifiedTail, readRange?)
  {
    reveal CorrectlyReadRange();
  }

  // This lemma is helpful because you can prove that what you have
  // written can be read. The following `ReadXIsComplete` lemmas
  // are more tailored to being able to read specific lengths which
  // are also useful
  lemma ReadIsComplete(
    data: seq<uint8>,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<seq<uint8>>)
    requires
      // Links data to the buffer so we know these bytes are the same
      && Write(data) == bytes
      // Here we have the buffer and we require that we can correctly read
      // up to the length of bytes or what we have written,
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == Read(buffer, |bytes|)
  {
    reveal CorrectlyReadRange();
    // After we have written data and gotten its length we can read its length from
    // the buffer and we know we can do this from our precondition.
    ret := Read(buffer, |Write(data)|).value;
    // After we have read what we have writen we need to prove
    // that we can read the length of the what we wrote
    // In order to satisfy the postcondition
    CorrectlyReadByteRange(buffer, ret.tail, bytes);
  }

  lemma ReadUInt16IsComplete(
    data: uint16,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<uint16>)
    requires
      && WriteUint16(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadUInt16(buffer)
  {
    reveal CorrectlyReadRange();
    ret := ReadUInt16(buffer).value;
    CorrectlyReadByteRange(buffer, ret.tail, bytes);
  }

  lemma ReadUInt32IsComplete(
    data: uint32,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<uint32>)
    requires
      && WriteUint32(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadUInt32(buffer)
  {
    reveal CorrectlyReadRange();
    ret := ReadUInt32(buffer).value;
    CorrectlyReadByteRange(buffer, ret.tail, bytes);
  }

  lemma ReadUInt64IsComplete(
    data: uint64,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<uint64>)
    requires
      && WriteUint64(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadUInt64(buffer)
  {
    reveal CorrectlyReadRange();
    ret := ReadUInt64(buffer).value;
    CorrectlyReadByteRange(buffer, ret.tail, bytes);
  }

  lemma ReadShortLengthSeqIsComplete(
    data: Uint8Seq16,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<Uint8Seq16>)
    requires
      && WriteShortLengthSeq(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadShortLengthSeq(buffer)
  {
    assert bytes == WriteUint16(|data| as uint16) + Write(data);
    assert bytes[..|WriteUint16(|data| as uint16)|] == WriteUint16(|data| as uint16);
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, buffer, WriteUint16(|data| as uint16));
    var continuation := ReadUInt16IsComplete(|data| as uint16, WriteUint16(|data| as uint16), buffer);
    CorrectlyReadByteRange(buffer, continuation.tail, WriteUint16(continuation.data));
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, continuation.tail, Write(data));
    var tail := ReadIsComplete(data, Write(data), continuation.tail);
    assert tail.data == data;

    reveal ReadShortLengthSeq();
    ret := ReadShortLengthSeq(buffer).value;
  }

  lemma ReadUint32SeqIsComplete(
    data: Uint8Seq32,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<Uint8Seq32>)
    requires
      && WriteUint32Seq(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadUint32Seq(buffer)
  {

    assert bytes == WriteUint32(|data| as uint32) + Write(data);
    assert bytes[..|WriteUint32(|data| as uint32)|] == WriteUint32(|data| as uint32);
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, buffer, WriteUint32(|data| as uint32));
    var continuation := ReadUInt32IsComplete(|data| as uint32, WriteUint32(|data| as uint32), buffer);
    CorrectlyReadByteRange(buffer, continuation.tail, WriteUint32(continuation.data));
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, continuation.tail, Write(data));
    var tail := ReadIsComplete(data, Write(data), continuation.tail);
    assert tail.data == data;

    reveal ReadUint32Seq();
    ret := ReadUint32Seq(buffer).value;
  }

  lemma ReadUint64SeqIsComplete(
    data: Uint8Seq64,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<Uint8Seq64>)
    requires
      && WriteUint64Seq(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ReadUint64Seq(buffer).Success?
      && ReadUint64Seq(buffer).value.data == data
      && ReadUint64Seq(buffer).value.tail.start == buffer.start + |bytes|
    ensures
      && ret.data == data
      && Success(ret) == ReadUint64Seq(buffer)
  {
    reveal CorrectlyReadRange();
    assert bytes == WriteUint64(|data| as uint64) + Write(data);
    assert bytes[..|WriteUint64(|data| as uint64)|] == WriteUint64(|data| as uint64);
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, buffer, WriteUint64(|data| as uint64));
    var continuation := ReadUInt64IsComplete(|data| as uint64, WriteUint64(|data| as uint64), buffer);
    CorrectlyReadByteRange(buffer, continuation.tail, WriteUint64(continuation.data));
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, continuation.tail, Write(data));
    var tail := ReadIsComplete(data, Write(data), continuation.tail);
    assert tail.data == data;

    reveal ReadUint64Seq();
    ret := ReadUint64Seq(buffer).value;
  }

}
