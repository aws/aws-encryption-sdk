// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "./SerializableTypes.dfy"
include "SerializeFunctions.dfy"

module {:options "/functionSyntax:4" } EncryptionContext {
  import Seq
  import StandardLibrary
  import opened SerializableTypes
  import MPL = AwsCryptographyMaterialProvidersTypes
  import opened StandardLibrary.UInt
  import opened Wrappers
  import opened UTF8
  import opened SerializeFunctions
  import opened StandardLibrary.MemoryMath

  // There needs to be a canonical representation of the encryption context.
  // Once the encryption context has been serialized,
  // then the bytes in the message can provide this canonical representation.
  // However, it is best if a canonical representation exists.
  // For reading and writing, we use the arbitrary given order,
  // and separate the ordering problem from serialization.
  type ESDKEncryptionContextPair = p: Pair<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
    |
      && HasUint16Len(p.key) && ValidUTF8Seq(p.key)
      && HasUint16Len(p.value) && ValidUTF8Seq(p.value)
    witness *

  type ESDKCanonicalEncryptionContext = pairs: seq<ESDKEncryptionContextPair>
    | ESDKCanonicalEncryptionContext?(pairs)
    witness *

  ghost predicate ESDKCanonicalEncryptionContext?(pairs: seq<ESDKEncryptionContextPair>) {
    && HasUint16Len(pairs)
    && LinearLength(pairs) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH
       //= compliance/data-format/message-header.txt#2.5.1.7.2.2
       //= type=implication
       //# This sequence MUST NOT contain duplicate entries.
    && KeysAreUnique(pairs)
  }

  lemma {:vcs_split_on_every_assert} ESDKEncryptionContextMapImpliesESDKCanonicalEncryptionContext(encryptionContext: MPL.EncryptionContext)
    ensures IsESDKEncryptionContext(encryptionContext)
            ==> ESDKCanonicalEncryptionContext?(GetCanonicalLinearPairs(encryptionContext))
  {
    if IsESDKEncryptionContext(encryptionContext) {
      assert HasUint16Len(GetCanonicalLinearPairs(encryptionContext));
      assert LinearLength(GetCanonicalLinearPairs(encryptionContext)) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH;
      assert KeysAreUnique(GetCanonicalLinearPairs(encryptionContext));
    }
  }

  function GetCanonicalEncryptionContext(
    encryptionContext: ESDKEncryptionContext
  )
    :(ret: ESDKCanonicalEncryptionContext)
    ensures |encryptionContext| == |ret|
  {
    ESDKEncryptionContextMapImpliesESDKCanonicalEncryptionContext(encryptionContext);
    GetCanonicalLinearPairs(encryptionContext)
  }

  function GetEncryptionContext(
    canonicalEncryptionContext: ESDKCanonicalEncryptionContext
  )
    :(ret: MPL.EncryptionContext)
    ensures |canonicalEncryptionContext| == 0 ==> |ret| == 0
  {
    // This is needed because Dafny can not reveal the subset type by default?
    assert ESDKCanonicalEncryptionContext?(canonicalEncryptionContext);
    assert KeysAreUnique(canonicalEncryptionContext);
    // Can't make this uint64, because Dafny explodes
    map i
      | 0 <= i < |canonicalEncryptionContext|
      :: canonicalEncryptionContext[i].key := canonicalEncryptionContext[i].value
  }

  lemma {:vcs_split_on_every_assert} LemmaCardinalityOfEncryptionContextEqualsPairs(
    pairs: ESDKCanonicalEncryptionContext,
    ec: MPL.EncryptionContext
  )
    requires ec == GetEncryptionContext(pairs)
    ensures |ec| == |pairs|
    ensures forall k: ValidUTF8Bytes <- ec :: Pair(k, ec[k]) in pairs
    ensures forall p <- pairs
              :: p.key in ec && p.value == ec[p.key]
  {
    if |pairs| == 0 {
    } else {
      var front := Seq.DropLast(pairs);
      var tail := Seq.Last(pairs);
      var ecOfFront := GetEncryptionContext(front);
      assert pairs == front + [tail];
      assert ec.Keys == ecOfFront.Keys + {tail.key};

      assert |ecOfFront.Keys| == |ecOfFront|;
      LemmaCardinalityOfEncryptionContextEqualsPairs(front, ecOfFront);
    }
  }

  lemma {:vcs_split_on_every_assert} LinearLengthOfUniquePairsIsOrderIndependent(
    pairs1: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>,
    pairs2: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  )
    requires forall p <- pairs1 :: p in pairs2
    requires forall p <- pairs2 :: p in pairs1
    requires |pairs1| == |pairs2|
    requires KeysAreUnique(pairs1) && KeysAreUnique(pairs2)
    ensures LinearLength(pairs1) == LinearLength(pairs2)
  {
    if |pairs1| == 0 || |pairs2| == 0{
      assert LinearLength(pairs1) == 0;
    } else {
      var tail := Seq.Last(pairs1);
      assert tail in pairs1;
      var i := Seq.IndexOf(pairs2, tail);

      var pairs1WithoutTail := RemoveValue<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>(pairs1, tail);
      var pairs2WithoutTail := RemoveValue<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>(pairs2, tail);

      assert pairs1WithoutTail == Seq.DropLast(pairs1);
      assert pairs2WithoutTail == pairs2[..i] + Seq.DropFirst(pairs2[i..]);

      LinearLengthOfUniquePairsIsOrderIndependent(pairs1WithoutTail, pairs2WithoutTail);

      calc {
        LinearLength(pairs1);
      ==
        LinearLength(pairs1WithoutTail + [tail]);
      == {LinearLengthIsDistributive(pairs1WithoutTail, [tail]);}
        LinearLength(pairs1WithoutTail) + LinearLength([tail]);
      == // From inductive hypothesis
        LinearLength(pairs2WithoutTail) + LinearLength([tail]);
      == {assert pairs2WithoutTail == pairs2[..i] + Seq.DropFirst(pairs2[i..]);}
        LinearLength(pairs2[..i] + Seq.DropFirst(pairs2[i..])) + LinearLength([tail]);
      == {LinearLengthIsDistributive(pairs2[..i], Seq.DropFirst(pairs2[i..]));}
        LinearLength(pairs2[..i]) + LinearLength(Seq.DropFirst(pairs2[i..])) + LinearLength([tail]);
      == // Reorder for clarity
        LinearLength(pairs2[..i]) + LinearLength([tail]) + LinearLength(Seq.DropFirst(pairs2[i..]));
      == {LinearLengthIsDistributive([tail], Seq.DropFirst(pairs2[i..]));}
        LinearLength(pairs2[..i]) + LinearLength([tail] + Seq.DropFirst(pairs2[i..]));
      == {assert pairs2[i..] == [pairs2[i]] + Seq.DropFirst(pairs2[i..]);}
        LinearLength(pairs2[..i]) + LinearLength(pairs2[i..]);
      == {LinearLengthIsDistributive(pairs2[..i], pairs2[i..]);}
        LinearLength(pairs2[..i] + pairs2[i..]);
      == {Seq.LemmaSplitAt(pairs2, i);}
        LinearLength(pairs2);
      }
    }
  }

  lemma {:vcs_split_on_every_assert} LemmaLengthOfPairsEqualsEncryptionContext(
    pairs: ESDKCanonicalEncryptionContext,
    ec: MPL.EncryptionContext
  )
    requires ec == GetEncryptionContext(pairs)
    ensures LinearLength(pairs) == Length(ec)
  {
    var canonicalPairs := GetCanonicalLinearPairs(ec);
    AllKeysAreInCanonicalLinearPairs(ec, canonicalPairs);
    LemmaCardinalityOfEncryptionContextEqualsPairs(pairs, ec);
    LinearLengthOfUniquePairsIsOrderIndependent(pairs, canonicalPairs);
  }

  lemma AllKeysAreInCanonicalLinearPairs(
    encryptionContext: MPL.EncryptionContext,
    pairs: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  )
    requires pairs == GetCanonicalLinearPairs(encryptionContext)
    ensures forall k: UTF8.ValidUTF8Bytes <- encryptionContext
              :: Pair(k, encryptionContext[k]) in pairs
    ensures forall p <- pairs
              ::
                && p.key in encryptionContext
                && p.value == encryptionContext[p.key]
    ensures KeysAreUnique(pairs)
  {
    if exists p ::
        && p !in pairs
        && p.key in encryptionContext
        && p.value == encryptionContext[p.key]
    {
      var p :|
        && p !in pairs
        && p.key in encryptionContext
        && p.value == encryptionContext[p.key];
      var keys := SortedSets.ComputeSetToOrderedSequence2(
        encryptionContext.Keys,
        UInt.UInt8Less
      );
      var i :| 0 <= i < |keys| && keys[i] == p.key;
      assert pairs[i] == p;
    }
  }

  lemma {:vcs_split_on_every_assert} LemmaESDKCanonicalEncryptionContextIsESDKEncryptionContext(
    pairs: ESDKCanonicalEncryptionContext,
    ec: MPL.EncryptionContext
  )
    requires ec == GetEncryptionContext(pairs)
    ensures IsESDKEncryptionContext(ec)
  {
    LemmaCardinalityOfEncryptionContextEqualsPairs(pairs, ec);
    LemmaLengthOfPairsEqualsEncryptionContext(pairs, ec);
    assert IsESDKEncryptionContext(ec) by {
      assert |ec| == |pairs| < UINT16_LIMIT by {
        assert HasUint16Len(pairs);
      }
      assert Length(ec) == LinearLength(pairs) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH;
      forall element <- (ec.Keys + ec.Values)
        ensures
          && HasUint16Len(element)
          && ValidUTF8Seq(element)
      {
        assert element in (set p <- pairs :: p.key) + (set p <- pairs :: p.value);
      }
    }

  }

  lemma {:vcs_split_on_every_assert} SubsetOfESDKEncryptionContextIsESDKEncryptionContext(ec: MPL.EncryptionContext, subEC: MPL.EncryptionContext)
    requires IsESDKEncryptionContext(ec)
    requires subEC.Keys <= ec.Keys
    requires forall k <- subEC.Keys :: ec[k] == subEC[k]
    ensures IsESDKEncryptionContext(subEC)
  {
    var complement := Complement(ec, subEC);

    calc {
      Add(Length(complement), Length(subEC));
    ==
      Add(LinearLength(GetCanonicalLinearPairs(complement)), LinearLength(GetCanonicalLinearPairs(subEC)));
    == {LinearLengthIsDistributive(GetCanonicalLinearPairs(complement), GetCanonicalLinearPairs(subEC));}
      LinearLength(GetCanonicalLinearPairs(complement) + GetCanonicalLinearPairs(subEC));
    == {
         var pairs1 := GetCanonicalLinearPairs(complement + subEC);
         var pairs2 := GetCanonicalLinearPairs(complement) + GetCanonicalLinearPairs(subEC);

         GetCanonicalLinearPairsIsBijective(complement + subEC, pairs1);
         GetCanonicalLinearPairsIsBijective(complement, GetCanonicalLinearPairs(complement));
         GetCanonicalLinearPairsIsBijective(subEC, GetCanonicalLinearPairs(subEC));
         assert forall p <- pairs1 :: p in pairs2;
         assert forall p <- pairs2 :: p in pairs1 by {
           forall p <- pairs2
             ensures p in pairs1
           {
             calc ==>
             {
               p in pairs2;
             ==>
               p in GetCanonicalLinearPairs(complement) + GetCanonicalLinearPairs(subEC);
             ==> {
                 assert (forall p' <- GetCanonicalLinearPairs(complement) :: p' in GetCanonicalLinearPairs(complement + subEC));
                 assert (forall p' <- GetCanonicalLinearPairs(subEC) :: p' in GetCanonicalLinearPairs(complement + subEC));
               }
               p in GetCanonicalLinearPairs(complement + subEC);
             ==>
               p in pairs1;
             }
           }
         }
         LinearLengthOfUniquePairsIsOrderIndependent(pairs1, pairs2);
       }
      LinearLength(GetCanonicalLinearPairs(complement + subEC));
    ==
      Length(complement + subEC);
    == {assert ec == complement + subEC;}
      Length(ec);
    }

    assert Length(subEC) <= Length(ec);
  }

  //= compliance/data-format/message-header.txt#2.5.1.7
  //# The following table describes the fields that form the AAD.  The
  //# bytes are appended in the order shown.
  //#
  //#  +=================+==============================+================+
  //#  | Field           | Length (bytes)               | Interpreted as |
  //#  +=================+==============================+================+
  //#  | Key Value Pairs | 2                            | UInt16         |
  //#  | Length (Section |                              |                |
  //#  | 2.5.1.7.1)      |                              |                |
  //#  +-----------------+------------------------------+----------------+
  //#  | Key Value Pairs | Variable.  Determined by the | Key Value      |
  //#  | (Section        | value of Key Value Pairs     | Pairs (Section |
  //#  | 2.5.1.7.2)      | Length (Section 2.5.1.7.1).  | 2.5.1.7.2)     |
  //#  +-----------------+------------------------------+----------------+
  function WriteAADSection(
    ec: ESDKCanonicalEncryptionContext
  ):
    (ret: seq<uint8>)
    ensures
      if |ec| == 0 then
        // These all deal with |ec| == 0

        //= compliance/data-format/message-header.txt#2.5.1.7.1
        //= type=implication
        //# When the encryption context (../framework/structures.md#encryption-
        //# context) is empty, the value of this field MUST be 0.

        //= compliance/data-format/message-header.txt#2.5.1.7.2
        //= type=implication
        //# When the encryption
        //# context (../framework/structures.md#encryption-context) is empty,
        //# this field MUST NOT be included in the Section 2.5.1.7.

        ret == WriteUint16(|ec| as uint16)
      else
        //= compliance/data-format/message-header.txt#2.5.1.7.2.1
        //= type=implication
        //# The value of this field MUST be greater
        //# than 0.
        && var aad := WriteAAD(ec);
        && ret == WriteUint16(|aad| as uint16) + aad
  {
    SequenceIsSafeBecauseItIsInMemory(ec);
    if |ec| as uint64 == 0 then WriteUint16(0)
    else
      var aad := WriteAAD(ec);
      WriteUint16(|aad| as uint16) + aad
  }

  // To Calculate the Authentication Only AAD for the Encryption Context,
  // the Encryption Context MUST be:
  // - filtered,
  // - Canonicalized
  // Finally, if the result of the above is an empty Encryption Context,
  // it is serialized as [].
  // Otherwise, it is serialized following the spec's
  // compliance/data-format/message-header.txt#2.5.1.7.2
  // This method ONLY handles the last portion.
  //
  // On Decrypt, the Decryption Materials' filtering is done
  // via buildEncryptionContextToOnlyAuthenticate,
  // which returns the pairs that exist in requiredEncryptionContextKeys.
  // On Encrypt, the filtering is done inside of `BuildHeaderForEncrypt`,
  // (currently) on lines 472 to 474.
  // Both Decrypt & Encrypt Canonicalize via GetCanonicalEncryptionContext.
  function WriteEmptyEcOrWriteAAD(
    ec: ESDKCanonicalEncryptionContext
  ):
    (ret: seq<uint8>)
    ensures HasUint16Len(ret)
  {
    // The Serialization of No Encryption Context is NOT `[0, [0, 0]]`,
    // but `[]`.
    SequenceIsSafeBecauseItIsInMemory(ec);
    if |ec| as uint64 == 0 then
      []
    else
      WriteAAD(ec)
  }

  lemma WriteEmptyEcOrWriteAADIsCorrect(ec: ESDKCanonicalEncryptionContext)
    ensures WriteEmptyEcOrWriteAAD(ec) == WriteAADSection(ec)[2..]
  {}

  //= compliance/data-format/message-header.txt#2.5.1.7.2
  //#The following table describes the fields that form the Key Value
  //#Pairs.  The bytes are appended in the order shown.
  //#
  //#  +==================+=========================+==================+
  //#  | Field            | Length (bytes)          | Interpreted as   |
  //#  +==================+=========================+==================+
  //#  | Key Value Pair   | 2                       | UInt16           |
  //#  | Count (Section   |                         |                  |
  //#  | 2.5.1.7.2.1)     |                         |                  |
  //#  +------------------+-------------------------+------------------+
  //#  | Key Value Pair   | Variable.  Determined   | Key Value Pair   |
  //#  | Entries (Section | by the count and length | Entries (Section |
  //#  | 2.5.1.7.2.2)     | of each key-value pair. | 2.5.1.7.2.2)     |
  //#  +------------------+-------------------------+------------------+
  function WriteAAD(
    ec: ESDKCanonicalEncryptionContext
  ):
    (ret: seq<uint8>)
    ensures HasUint16Len(ret)
    // To support older versions of the ESDK
    // |ec| == 0 is encoded as 0 count.
    // However,
    // this |ec| == 0 behavior is never invoked,
    // as this method is protected by
    // WriteAADSection and WriteEmptyEcOrWriteAAD,
    // which both handle |ec| == 0 independently
    // of this method.
    ensures |ec| == 0 ==> ret == WriteUint16(0)
  {
    WriteUint16(|ec| as uint16) + WriteAADPairs(ec)
  }

  function WriteAADPairs(
    ec: ESDKCanonicalEncryptionContext
  ):
    (ret: seq<uint8>)
    ensures |ec| == 0
            ==>
              && LinearLength(ec) as nat == |ret|
              && ret == []
    ensures |ec| != 0
            ==>
              && LinearLength(Seq.DropLast(ec)) as nat + PairLength(Seq.Last(ec)) as nat == |ret|
              && WriteAADPairs(Seq.DropLast(ec)) + WriteAADPair(Seq.Last(ec)) == ret
    ensures |ret| < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH as nat
  {
    if |ec| == 0 then []
    else
      assert LinearLength(Seq.DropLast(ec)) < LinearLength(ec);
      WriteAADPairs(Seq.DropLast(ec)) + WriteAADPair(Seq.Last(ec))
  }
  by method { // because Seq.DropLast makes a full copy
    var result : seq<uint8> := [];
    SequenceIsSafeBecauseItIsInMemory(ec);
    for i : uint64 := 0 to |ec| as uint64
      invariant ESDKCanonicalEncryptionContext?(ec)
      invariant ESDKCanonicalEncryptionContext?(ec[..i])
      invariant result == WriteAADPairs(ec[..i])
    {
      result := result + WriteAADPair(ec[i]);
      ESDKCanonicalEncryptionContextCanBeSplit(ec);
      assert result == WriteAADPairs(ec[..i]) + WriteAADPair(ec[i]);
      assert Seq.DropLast(ec[..i+1]) == ec[..i];
      assert result == WriteAADPairs(Seq.DropLast(ec[..i+1])) + WriteAADPair(Seq.Last(ec[..i+1]));
      assert ESDKCanonicalEncryptionContext?(ec[..i]);
      assert ESDKCanonicalEncryptionContext?(ec[..i+1]);
      assert result == WriteAADPairs(ec[..i+1]);

    }
    assert result == WriteAADPairs(ec[..|ec|]);
    assert ec == ec[..|ec|];
    assert result == WriteAADPairs(ec);
    return result;
  }

  //= compliance/data-format/message-header.txt#2.5.1.7.2.2
  //# The following table describes the fields that form each key value
  //# pair entry.  The bytes are appended in the order shown.
  //#
  //# +========+=========================================+================+
  //# | Field  | Length (bytes)                          | Interpreted    |
  //# |        |                                         | as             |
  //# +========+=========================================+================+
  //# | Key    | 2                                       | UInt16         |
  //# | Length |                                         |                |
  //# +--------+-----------------------------------------+----------------+
  //# | Key    | Variable.  Equal to the value specified | UTF-8 encoded  |
  //# |        | in the previous 2 bytes (Key Length).   | bytes          |
  //# +--------+-----------------------------------------+----------------+
  //# | Value  | 2                                       | UInt16         |
  //# | Length |                                         |                |
  //# +--------+-----------------------------------------+----------------+
  //# | Value  | Variable.  Equal to the value specified | UTF-8 encoded  |
  //# |        | in the previous 2 bytes (Value Length). | bytes          |
  //# +--------+-----------------------------------------+----------------+
  function WriteAADPair(
    pair: ESDKEncryptionContextPair
  ):
    (ret: seq<uint8>)
    ensures PairLength(pair) as nat == |ret|
  {
    WriteShortLengthSeq(pair.key) + WriteShortLengthSeq(pair.value)
  }

  function ReadAADPair(
    buffer: ReadableBuffer
  )
    :(res: ReadCorrect<ESDKEncryptionContextPair>)
    ensures CorrectlyRead(buffer, res, WriteAADPair)
    ensures res.Success? ==> (res.value.tail.start >= buffer.start) && PairLength(res.value.data) == res.value.tail.start - buffer.start
  {
    var SuccessfulRead(key, keyEnd) :- ReadShortLengthSeq(buffer);
    :- Need(ValidUTF8Seq(key), Error("Invalid Encryption Context key"));
    var SuccessfulRead(value, tail) :- ReadShortLengthSeq(keyEnd);
    :- Need(ValidUTF8Seq(value), Error("Invalid Encryption Context value"));
    var pair:ESDKEncryptionContextPair := Pair(key, value);

    assert CorrectlyReadRange(buffer, tail, WriteAADPair(pair)) by {
      reveal CorrectlyReadRange();
    }
    assert PairLength(pair) == tail.start - buffer.start by {
      reveal CorrectlyReadRange();
    }

    Success(SuccessfulRead(pair, tail))
  }

  function {:tailrecursion} {:vcs_split_on_every_assert} ReadAADPairs(
    buffer: ReadableBuffer,
    accumulator: ESDKCanonicalEncryptionContext,
    keys: set<UTF8.ValidUTF8Bytes>,
    count: uint16,
    nextPair: ReadableBuffer
  ):
    (res: ReadCorrect<ESDKCanonicalEncryptionContext>)
    requires 0 <= |accumulator| <= count as nat < UINT16_LIMIT
    requires CorrectlyReadRange(buffer, nextPair, WriteAADPairs(accumulator))
    requires KeysToSet(accumulator) == keys
    decreases count as int - |accumulator|
    ensures res.Success? ==> count as nat == |res.value.data|
    ensures CorrectlyRead(buffer, res, WriteAADPairs)
  {
    if count > |accumulator| as uint16 then
      var SuccessfulRead(pair, newPos) :- ReadAADPair(nextPair);
      :- Need(pair.key !in keys, Error("Duplicate Encryption Context key value."));
      :- Need((newPos.start >= buffer.start) && newPos.start - buffer.start < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH, Error("Encryption Context exceeds maximum length."));

      var nextAcc := accumulator + [pair];
      // Calling `KeysToSet` once per pair
      // will be faster than calling it on `nextAcc` every time.
      assert KeysToSet(nextAcc) == keys + KeysToSet([pair]) by {
        reveal KeysToSet();
      }
      var nextKeys := keys + KeysToSet([pair]);

      assert LinearLength(nextAcc) == LinearLength(accumulator) + PairLength(pair);
      assert KeysAreUnique(nextAcc) by {
        reveal KeysToSet();
      }
      assert ESDKCanonicalEncryptionContext?(nextAcc) by {
        assert LinearLength(accumulator) as nat == |WriteAADPairs(accumulator)|;
        assert nextPair.start as nat == buffer.start as nat + |WriteAADPairs(accumulator)| by {
          reveal CorrectlyReadRange();
        }
        assert LinearLength(accumulator) == nextPair.start - buffer.start;
      }
      CorrectlyReadByteRange(buffer, nextPair, WriteAADPairs(accumulator));
      AppendToCorrectlyReadByteRange(buffer, nextPair, newPos, WriteAADPair(pair));

      ReadAADPairs(buffer, nextAcc, nextKeys, count, newPos)
    else
      assert CorrectlyRead(buffer, Success(SuccessfulRead(accumulator, nextPair)), WriteAADPairs);
      Success(SuccessfulRead(accumulator, nextPair))
  }

  function ReadAAD(
    buffer: ReadableBuffer
  ):
    (res: ReadCorrect<ESDKCanonicalEncryptionContext>)
    ensures CorrectlyRead(buffer, res, WriteAAD)
  {
    reveal CorrectlyReadRange();
    var SuccessfulRead(count, ecPos) :- ReadUInt16(buffer);
    if count == 0 then
      var edks: ESDKCanonicalEncryptionContext := [];
      assert CorrectlyRead(buffer, Success(SuccessfulRead(edks, ecPos)), WriteAAD);
      Success(SuccessfulRead(edks, ecPos))
    else
      var accumulator := [];
      var keys := KeysToSet(accumulator);
      var SuccessfulRead(pairs, tail) :- ReadAADPairs(ecPos, accumulator, keys, count, ecPos);
      Success(SuccessfulRead(pairs, tail))
  }

  opaque function {:vcs_split_on_every_assert} ReadAADSection(
    buffer: ReadableBuffer
  ):
    (res: ReadCorrect<ESDKCanonicalEncryptionContext>)
    ensures if IsExpandedAADSection(buffer) then
              CorrectlyRead(buffer, res, WriteExpandedAADSection)
            else
              CorrectlyRead(buffer, res, WriteAADSection)
  {
    var length :- ReadUInt16(buffer);
    CorrectlyReadByteRange(buffer, length.tail, WriteUint16(length.data));
    if length.data == 0 then
      var empty: ESDKCanonicalEncryptionContext := [];

      assert CorrectlyReadRange(buffer, length.tail, WriteAADSection(empty));
      assert !IsExpandedAADSection(buffer);
      assert CorrectlyReadRange(buffer, length.tail, WriteAADSection(empty));

      Success(SuccessfulRead(empty, length.tail))
    else
      SequenceIsSafeBecauseItIsInMemory(length.tail.bytes);
      :- Need(Add(length.tail.start, length.data as uint64) <= |length.tail.bytes| as uint64, MoreNeeded(Add(length.tail.start, length.data as uint64)));

      var verifyCount :- ReadUInt16(length.tail);
      AppendToCorrectlyReadByteRange(buffer, length.tail, verifyCount.tail, WriteUint16(verifyCount.data));
      if length.data == 2 then
        // This is the expanded case referred to in IsStandardCompressedAADSection.
        // It is not a canonically correct message,
        // but it should still be parsed.
        :- Need(verifyCount.data == 0, Error("Encryption Context pairs count can not exceed byte length"));
        var empty: ESDKCanonicalEncryptionContext := [];
        assert IsExpandedAADSection(buffer) by {
          reveal CorrectlyReadRange();
          reveal ReadUInt16();
        }

        Success(SuccessfulRead(empty, verifyCount.tail))
      else
        // This count MUST be greater than 0,
        // because the length of the AAD is greater than 2.
        :- Need(0 < verifyCount.data, Error("Encryption Context byte length exceeds pairs count."));

        var aad :- ReadAAD(length.tail);
        :- Need((aad.tail.start >= length.tail.start) && aad.tail.start - length.tail.start == length.data as uint64, Error("AAD Length did not match stored length."));

        assert !IsExpandedAADSection(buffer);
        AppendToCorrectlyReadByteRange(buffer, length.tail, aad.tail, WriteAAD(aad.data));
        assert |WriteAAD(aad.data)| as uint16 == length.data;
        assert WriteAADSection(aad.data) == WriteUint16(|WriteAAD(aad.data)| as uint16) + WriteAAD(aad.data);
        CorrectlyReadByteRange(buffer, aad.tail, WriteAADSection(aad.data));

        Success(aad)
  }

  opaque function KeysToSet<K(==), V(==)>(pairs: Linear<K, V>): set<K>
  {
    set p: Pair<K,V> | p in pairs :: p.key
  }

  // Makes sure that the AAD Section is optimally encoded.
  // The AAD section is supposed to encode
  // an empty encryption context, {}, as a length of 0
  // because the additional 2 bytes to encode 0 pairs are redundant.
  // However, since there did once exists ESDKs
  // that incorrectly wrote empty encryption contexts as
  // `[0,2,0,0]` this read path MUST be supported.
  ghost predicate IsExpandedAADSection(
    buffer: ReadableBuffer
  )
  {
    reveal CorrectlyReadRange();
    reveal ReadUInt16();
    && buffer.start as nat + 2 <= |buffer.bytes|
    && var sectionLength := ReadUInt16(buffer).value;
    && sectionLength.data == 2
    && sectionLength.tail.start as nat + 2 <= |buffer.bytes|
    && ReadUInt16(sectionLength.tail).value.data == 0
  }

  // This is *not* a function,
  // because it is *only* used for correctness.
  // This represents the sub-optimal encoding of AAD
  // with an extra 2 bytes.
  ghost function WriteExpandedAADSection(
    ec: ESDKCanonicalEncryptionContext
  ):
    (ret: seq<uint8>)
    ensures if |ec| == 0 then
              ret == [0, 2, 0, 0]
            else
              && var aad := WriteAAD(ec);
              && ret == UInt16ToSeq(|aad| as uint16) + aad
  {
    var aad := WriteAAD(ec);
    UInt16ToSeq(|aad| as uint16) + aad
  }

  // Completeness Lemmas to prove that ReadX/WriteX are both sound and complete

  // This lemma is to prove that a buffer with bytes from `WriteAADPair`
  // MUST be `ReadAADPair` Success?
  lemma ReadAADPairIsComplete(
    data: ESDKEncryptionContextPair,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<ESDKEncryptionContextPair>)
    requires
      && WriteAADPair(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadAADPair(buffer)
  {
    reveal CorrectlyReadRange();
    assert bytes == WriteShortLengthSeq(data.key) + WriteShortLengthSeq(data.value);
    assert bytes[..|WriteShortLengthSeq(data.key)|] == WriteShortLengthSeq(data.key);

    var key := ReadShortLengthSeqIsComplete(data.key, WriteShortLengthSeq(data.key), buffer);
    assert key.data == data.key;

    var value := ReadShortLengthSeqIsComplete(data.value, WriteShortLengthSeq(data.value), key.tail);
    assert value.data == data.value;

    return ReadAADPair(buffer).value;
  }

  // When dealing with a seq<Pairs> that is `ESDKCanonicalEncryptionContext?`
  // we need to be able to break up this single `seq` into parts.
  // This is slightly complicated because the subSeqs
  // MUST still satisfy `ESDKCanonicalEncryptionContext?`.
  // This is true to a human,
  // Dafny complains a little about the uniqueness constraint,
  // but mostly about the length.
  // This is done by breaking up a single seq
  // because the requirements made proof about
  // 2 parts difficult to put back together.
  lemma ESDKCanonicalEncryptionContextCanBeSplit(
    data: ESDKCanonicalEncryptionContext
  )
    ensures forall accumulator
              | accumulator <= data
              ::
                && ESDKCanonicalEncryptionContext?(accumulator)
                && ESDKCanonicalEncryptionContext?(data[|accumulator|..])
  {
    forall accumulator
      | accumulator <= data
      ensures
        && ESDKCanonicalEncryptionContext?(accumulator)
        && ESDKCanonicalEncryptionContext?(data[|accumulator|..])
    {
      assert |accumulator| <= |data|;
      assert |data[|accumulator|..]| <= |data|;
      assert KeysAreUnique(accumulator);
      assert KeysAreUnique(data[|accumulator|..]);
      assert data == accumulator + data[|accumulator|..];
      LinearLengthIsDistributive(accumulator, data[|accumulator|..]);
      // Each part MUST be less than `ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH`
      // Since the whole MUST be less than this does the trick.
      assert LinearLength(data) == LinearLength(accumulator) + LinearLength(data[|accumulator|..]);
    }
  }

  // What it says on the can.
  // Need to be able to reason about subSeq of a given ESDKCanonicalEncryptionContext.
  lemma {:vcs_split_on_every_assert} WriteAADPairsIsDistributive(
    a: ESDKCanonicalEncryptionContext,
    b: ESDKCanonicalEncryptionContext
  )
    requires ESDKCanonicalEncryptionContext?(a + b)
    ensures WriteAADPairs(a + b) == WriteAADPairs(a) + WriteAADPairs(b)
  {
    if b == [] {
      assert a + b == a;
    } else {
      calc {
        WriteAADPairs(a + b);
      == // Definition of WriteAADPairs
        if |a+b| == 0 then []
        else WriteAADPairs(Seq.DropLast(a + b)) + WriteAADPair(Seq.Last(a+b));
      == {assert |a+b| > 0;} // Because of the above `if` block
        WriteAADPairs(Seq.DropLast(a + b)) + WriteAADPair(Seq.Last(a+b));
      == {assert Seq.Last(a+b) == Seq.Last(b) && Seq.DropLast(a+b) == a + Seq.DropLast(b);} // Breaking apart (a+b)
        WriteAADPairs(a + Seq.DropLast(b)) + WriteAADPair(Seq.Last(b));
      == {WriteAADPairsIsDistributive(a, Seq.DropLast(b));} // This lets us break apart WriteAADPairs(a + Seq.DropLast(b))
        (WriteAADPairs(a) + WriteAADPairs(Seq.DropLast(b))) + WriteAADPair(Seq.Last(b));
      == // Move () to prove associativity of +
        WriteAADPairs(a) + (WriteAADPairs(Seq.DropLast(b)) + WriteAADPair(Seq.Last(b)));
      == {assert WriteAADPairs(Seq.DropLast(b)) + WriteAADPair(Seq.Last(b)) == WriteAADPairs(b);} // join the 2 parts of b back together
        WriteAADPairs(a) + WriteAADPairs(b);
      }
    }
  }

  lemma {:vcs_split_on_every_assert} ReadAADPairsIsComplete(
    data: ESDKCanonicalEncryptionContext,
    accumulator: ESDKCanonicalEncryptionContext,
    keys: set<UTF8.ValidUTF8Bytes>,
    bytes: seq<uint8>,
    buffer: ReadableBuffer,
    accumulatorPairsRead: ReadableBuffer
  )
    returns (ret: SuccessfulRead<ESDKCanonicalEncryptionContext>)
    decreases |data| - |accumulator|

    requires accumulator <= data
    requires KeysToSet(accumulator) == keys
    requires HasUint16Len(data)

    requires
      && WriteAADPairs(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
      && CorrectlyReadRange(buffer, accumulatorPairsRead, WriteAADPairs(accumulator))
    ensures
      && ret.data == data
      && Success(ret) == ReadAADPairs(buffer, accumulator, keys, |data| as uint16, accumulatorPairsRead)
  {
    // This will match _both_ the case where |data| == 0 && the terminal case.
    // It is expressed this way because we are,
    // building up to the terminal case (see the inductive call below)
    if data == accumulator {
      ret := ReadAADPairs(
        buffer,
        accumulator,
        keys,
        |data| as uint16,
        accumulatorPairsRead)
      .value;
    } else {

      assert accumulator < data;
      // assert |accumulator| + 1 <= |data|;

      assert CorrectlyReadableByteRange?(accumulatorPairsRead, WriteAADPair(data[|accumulator|])) by {

        ESDKCanonicalEncryptionContextCanBeSplit(data);
        var nextAccumulator := data[..|accumulator| + 1]; //   accumulator +  [data[|accumulator|]];
        assert ESDKCanonicalEncryptionContext?(nextAccumulator);
        assert ESDKCanonicalEncryptionContext?(data[|nextAccumulator|..]);
        calc {
          bytes;
        == // From requires clause
          WriteAADPairs(data);
        == {LemmaSplitAtInclusive(data, |accumulator| + 1);} // Split data apart
          WriteAADPairs(nextAccumulator + data[|nextAccumulator|..]);
        == {WriteAADPairsIsDistributive(nextAccumulator, data[|nextAccumulator|..]);}
          WriteAADPairs(nextAccumulator) + WriteAADPairs(data[|nextAccumulator|..]);
        == {assert WriteAADPairs(Seq.DropLast(nextAccumulator)) + WriteAADPair(Seq.Last(nextAccumulator)) == WriteAADPairs(nextAccumulator);}
          (WriteAADPairs(Seq.DropLast(nextAccumulator)) + WriteAADPair(Seq.Last(nextAccumulator))) + WriteAADPairs(data[|nextAccumulator|..]);
        == {assert Seq.DropLast(nextAccumulator) == accumulator && WriteAADPairs(Seq.DropLast(nextAccumulator)) == WriteAADPairs(accumulator);}
          WriteAADPairs(accumulator) + WriteAADPair(Seq.Last(nextAccumulator)) + WriteAADPairs(data[|nextAccumulator|..]);
        == {assert Seq.Last(nextAccumulator) == data[|accumulator|];}
          WriteAADPairs(accumulator) + WriteAADPair(data[|accumulator|]) + WriteAADPairs(data[|nextAccumulator|..]);
        }

        CorrectlyReadByteRange(buffer, accumulatorPairsRead, WriteAADPairs(accumulator));
        AdvanceCorrectlyReadableByteRange?(buffer, bytes, accumulatorPairsRead, WriteAADPair(data[|accumulator|]));
      }

      // Since we know that the bytes here at `nextPair`
      // are valid WriteAADPair bytes,
      // we can prove that this part is complete
      var pair := ReadAADPairIsComplete(
        data[|accumulator|],
        WriteAADPair(data[|accumulator|]),
        accumulatorPairsRead
      );

      assert
        && KeysToSet(accumulator + [pair.data]) == keys + KeysToSet([pair.data])
        && pair.data.key !in keys
        && accumulator + [pair.data] <= data
      by {
        reveal KeysToSet();
        assert pair.data == data[|accumulator|];
        assert pair.data.key !in keys;
        assert pair.data in data;
        assert accumulator + [data[|accumulator|]] + data[|accumulator| + 1..] == data;
        assert (accumulator + [data[|accumulator|]]) + data[|accumulator| + 1..] == data;
        assert (accumulator + [data[|accumulator|]]) <= data;
      }
      assert ESDKCanonicalEncryptionContext?(accumulator + [pair.data]) by {
        assert accumulator + [pair.data] <= data by {
          assert accumulator < data;
          assert pair.data == data[|accumulator|];
        }

        // The length constraint is a little complicated.
        // Dafny wants to know the length of bytes is in bounds.
        // This is obviously true,
        // because Dafny believes that the `|bytes| < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH`.
        // Dafny also believes that `LinearLength(accumulator) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH`.
        // Adding `[pair.data]` to accumulator is the part that it is unsure about.
        assert LinearLength(accumulator + [pair.data]) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH by {
          assert LinearLength(data) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH;
          calc {
            LinearLength(data);
          == {assert data == (accumulator + [pair.data]) + data[|(accumulator + [pair.data])|..];}
            LinearLength((accumulator + [pair.data]) + data[|(accumulator + [pair.data])|..]);
          == {LinearLengthIsDistributive((accumulator + [pair.data]), data[|(accumulator + [pair.data])|..]);}
            LinearLength((accumulator + [pair.data])) + LinearLength(data[|(accumulator + [pair.data])|..]);
          }
        }

        assert HasUint16Len(accumulator + [pair.data]) by {
          assert HasUint16Len(data);
          assert accumulator + [pair.data] <= data;
        }
      }

      assert CorrectlyReadRange(buffer, pair.tail, WriteAADPairs(accumulator + [pair.data])) by {

        assert WriteAADPairs(accumulator + [pair.data]) == WriteAADPairs(accumulator) + WriteAADPair(pair.data);
        CorrectlyReadByteRange(buffer, accumulatorPairsRead, WriteAADPairs(accumulator));
        AppendToCorrectlyReadByteRange(buffer, accumulatorPairsRead, pair.tail, WriteAADPair(pair.data));
      }

      // Invoking the inductive hypothesis
      // This will recurse *forward* to the final case where data == accumulator.
      // Along the way, we prove ReadAADPairIsComplete
      // for each encryption context pair "along the way".
      ret := ReadAADPairsIsComplete(
        data,
        accumulator + [pair.data],
        keys + KeysToSet([pair.data]),
        bytes,
        buffer,
        pair.tail
      );

      assert Success(ret) == ReadAADPairs(buffer, accumulator, keys, |data| as uint16, accumulatorPairsRead) by {
        reveal CorrectlyReadRange();
        assert ret.data == data;
      }

    }
  }

  lemma {:vcs_split_on_every_assert} ReadAADIsComplete(
    data: ESDKCanonicalEncryptionContext,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<ESDKCanonicalEncryptionContext>)
    requires
      && WriteAAD(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadAAD(buffer)
  {
    assert WriteUint16(|data| as uint16) + WriteAADPairs(data) == bytes;
    assert bytes[|WriteUint16(|data| as uint16)|..] == WriteAADPairs(data);
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, buffer, WriteUint16(|data| as uint16));

    var count := ReadUInt16IsComplete(
      |data| as uint16,
      WriteUint16(|data| as uint16),
      buffer
    );
    assert count.data as nat == |data|;
    var accumulator:ESDKCanonicalEncryptionContext := [];
    CorrectlyReadByteRange(buffer, count.tail, WriteUint16(count.data));
    AdvanceCorrectlyReadableByteRange?(buffer, bytes, count.tail, WriteAADPairs(data));
    assert CorrectlyReadRange(count.tail, count.tail, WriteAADPairs(accumulator)) by {
      assert count.tail.bytes[count.tail.start..count.tail.start] == WriteAADPairs(accumulator);
    }

    var pairs := ReadAADPairsIsComplete(
      data,
      accumulator,
      KeysToSet(accumulator),
      WriteAADPairs(data),
      count.tail,
      count.tail
    );
    assert pairs.data == data;

    return ReadAAD(buffer).value;
  }

  lemma {:vcs_split_on_every_assert} ReadAADSectionIsComplete(
    data: ESDKCanonicalEncryptionContext,
    bytes: seq<uint8>,
    buffer: ReadableBuffer
  )
    returns (ret: SuccessfulRead<ESDKCanonicalEncryptionContext>)
    requires
      && WriteAADSection(data) == bytes
      && CorrectlyReadableByteRange?(buffer, bytes)
    ensures
      && ret.data == data
      && Success(ret) == ReadAADSection(buffer)
  {
    assert 2 <= |bytes|;

    if 0 == |data| {
      assert WriteAADSection(data) == WriteUint16(0);
      var _ := ReadUInt16IsComplete(0, bytes, buffer);
      reveal ReadAADSection();
      ret := ReadAADSection(buffer).value;
    } else {
      assert WriteUint16(|WriteAAD(data)| as uint16) + WriteAAD(data) == bytes;
      assert 0 < |data|;

      AdvanceCorrectlyReadableByteRange?(buffer, bytes, buffer, WriteUint16(|WriteAAD(data)| as uint16));
      var length := ReadUInt16IsComplete(
        |WriteAAD(data)| as uint16,
        WriteUint16(|WriteAAD(data)| as uint16),
        buffer
      );

      CorrectlyReadByteRange(buffer, length.tail, WriteUint16(length.data));
      AdvanceCorrectlyReadableByteRange?(buffer, bytes, length.tail, WriteAAD(data));

      assert length.data == |WriteAAD(data)| as uint16;
      assert length.tail.start as nat + length.data as nat <= |length.tail.bytes|;
      assert !IsExpandedAADSection(buffer);

      var aad := ReadAADIsComplete(
        data,
        WriteAAD(data),
        length.tail
      );

      reveal ReadAADSection();
      reveal CorrectlyReadRange();
      ret := ReadAADSection(buffer).value;

      assert aad.data == data;
      assert ret.data == aad.data;
    }
  }

  lemma LemmaSplitAtInclusive<T>(xs: seq<T>, pos: nat)
    requires pos <= |xs|
    ensures xs[..pos] + xs[pos..] == xs
  {
  }

  // This function is optimized for LinearLengthOfUniquePairsIsOrderIndependent
  // It is *much* easier to prove the required properties
  // for the pairs*WithoutTail when they are constructed.
  ghost function RemoveValue<K, V>(
    xs: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>,
    value: Pair<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  ): (ys: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>)
    ensures KeysAreUnique(xs) ==> KeysAreUnique(ys)
    ensures forall p <- ys :: p in xs
    ensures forall p <-xs | p != value :: p in ys
  {
    if value !in xs then xs
    else
      var i := Seq.IndexOf(xs, value);
      assert xs == xs[..i] + [value] + xs[i+1..];
      xs[..i] + xs[i+1..]
  }

  ghost function Complement<X,Y>(universal: map<X,Y>, subset: map<X,Y>): (complement: map<X,Y>)
    requires subset.Keys <= universal.Keys
    requires forall k <- subset.Keys :: universal[k] == subset[k]
    ensures complement + subset == universal
  {
    map k <- universal.Keys | k !in subset :: k := universal[k]
  }

}
