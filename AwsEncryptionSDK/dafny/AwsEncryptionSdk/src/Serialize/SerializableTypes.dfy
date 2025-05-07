// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../../Model/AwsCryptographyEncryptionSdkTypes.dfy"

module SerializableTypes {
  import StandardLibrary
  import opened UInt = StandardLibrary.UInt
  import opened UTF8
  import opened StandardLibrary.MemoryMath
  import MPL = AwsCryptographyMaterialProvidersTypes
  import AwsCryptographyPrimitivesTypes
  import SortedSets
  import Seq

  type ShortUTF8Seq = s: ValidUTF8Bytes | HasUint16Len(s)
  // To make verification and working with iterating through the encryption context
  // simpler, here we define a specific type to represent a sequence of key-value tuples.
  datatype Pair<K, V> = Pair(key: K, value: V)
  type Linear<K, V> = seq<Pair<K,V>>

  predicate method IsESDKEncryptedDataKey(edk: MPL.EncryptedDataKey) {
    && HasUint16Len(edk.keyProviderId)
    && ValidUTF8Seq(edk.keyProviderId)
    && HasUint16Len(edk.keyProviderInfo)
    && HasUint16Len(edk.ciphertext)
  }

  type ESDKEncryptedDataKey = e: MPL.EncryptedDataKey | IsESDKEncryptedDataKey(e) witness *
  type ESDKEncryptedDataKeys = seq16<ESDKEncryptedDataKey>
  // The AAD section is the total length|number of pairs|key|value|key|value...
  // The total length is uint16, so the maximum length for the keys and values
  // MUST be able to include the uint16 for the number of pairs.
  const ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH : uint64 := UINT16_LIMIT as uint64 - 2

  predicate method IsESDKEncryptionContext(ec: MPL.EncryptionContext) {
    ValueIsSafeBecauseItIsInMemory(|ec|);
    && |ec| as uint64 < UINT16_LIMIT as uint64
    && Length(ec) < ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH
    && forall element
         | element in (multiset(ec.Keys) + multiset(ec.Values))
         ::
           && HasUint16Len(element)
           && ValidUTF8Seq(element)
  }

  type ESDKEncryptionContext = ec: MPL.EncryptionContext
    | IsESDKEncryptionContext(ec)
    witness *

  function method GetIvLength(a: MPL.AlgorithmSuiteInfo)
    : (output: uint8)
  {
    match a.encrypt
    case AES_GCM(e) => e.ivLength as uint8
  }

  function method GetIvLengthZeros(a: MPL.AlgorithmSuiteInfo)
    : (output: seq<uint8>)
    ensures |output| == GetIvLength(a) as nat
    ensures forall x <- output :: x == 0
  {
    var len := GetIvLength(a);
    if len == 12 then
      [0,0,0,0,0,0,0,0,0,0,0,0]
    else
      seq(len, _ => 0)
  }

  function method GetTagLength(a: MPL.AlgorithmSuiteInfo)
    : (output: uint8)
  {
    match a.encrypt
    case AES_GCM(e) => e.tagLength as uint8
  }

  function method GetEncryptKeyLength(a: MPL.AlgorithmSuiteInfo)
    : (output: int32)
    ensures
      && AwsCryptographyPrimitivesTypes.IsValid_PositiveInteger(output)
      && AwsCryptographyPrimitivesTypes.IsValid_SymmetricKeyLength(output)
  {
    match a.encrypt
    case AES_GCM(e) => e.keyLength
  }

  /*
   * Length properties of the Encryption Context.
   * The Encryption Context has a complex relationship with length.
   * Each key or value MUST be less than Uint16,
   * However the entire thing MUST all so serialize to less than Uint16.
   * In practice, this means than the longest value,
   * given a key of 1 bytes is Uint16-2-2-1.
   * e.g.
   * 2 for the key length
   * 1 for the key data
   * 2 for the value length
   * Uint16-2-2-1 for the value data
   */
  function method Length(
    encryptionContext: MPL.EncryptionContext
  )
    : (ret: uint64)
    ensures
      var pairs := GetCanonicalLinearPairs(encryptionContext);
      && ret == LinearLength(pairs)
  {
    ValueIsSafeBecauseItIsInMemory(|encryptionContext|);
    if |encryptionContext| as uint64 == 0 then 0 else
    var pairs := GetCanonicalLinearPairs(encryptionContext);
    LinearLength(pairs)
  }

  // Defining and reasoning about order with maps
  // is simplified by using a sequence of
  // key value pairs instead.
  function method GetCanonicalLinearPairs(
    encryptionContext: MPL.EncryptionContext
  )
    :(ret: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>)
    ensures |encryptionContext| == |ret|
    ensures KeysAreUnique(ret)
  {
    //= compliance/data-format/message-header.txt#2.5.1.7.2.2
    //# These entries MUST have entries sorted, by key, in ascending order
    //# according to the UTF-8 encoded binary value.
    var keys: seq<UTF8.ValidUTF8Bytes> := SortedSets.ComputeSetToOrderedSequence2<uint8>(
                                            encryptionContext.Keys,
                                            UInt.UInt8Less
                                          );
    seq(
    |keys|,
    i
    requires 0 <= i < |keys|
    => Pair(
        keys[i],
        encryptionContext[keys[i]]))
  }

  lemma GetCanonicalLinearPairsIsBijective(encryptionContext: MPL.EncryptionContext, ret: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>)
    requires ret == GetCanonicalLinearPairs(encryptionContext)
    ensures forall p <- ret :: p.key in encryptionContext && p.value == encryptionContext[p.key]
    ensures forall k: UTF8.ValidUTF8Bytes <- encryptionContext :: Pair(k, encryptionContext[k]) in ret
  {
    forall k: UTF8.ValidUTF8Bytes <- encryptionContext
      ensures Pair(k, encryptionContext[k]) in ret
    {
      var keys: seq<UTF8.ValidUTF8Bytes> := SortedSets.ComputeSetToOrderedSequence2<uint8>(
        encryptionContext.Keys,
        UInt.UInt8Less
      );
      var i :| 0 <= i < |keys| && k == keys[i];
      var makePair := i requires 0 <= i < |keys| => Pair(keys[i], encryptionContext[keys[i]]);
      assert Pair(k, encryptionContext[k]) == makePair(i);
      assert forall i' | 0 <= i' < |keys| :: ret[i'] ==  makePair(i');
    }
  }

  predicate KeysAreUnique<K, V>(pairs: Linear<K, V>)
  {
    (forall i, j
       // This satisfies every cardinality AND i != j
       :: 0 <= i < j < |pairs|
          ==> pairs[i].key != pairs[j].key)
  }

  function LinearLength(
    pairs: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  ):
    (ret: uint64)
    ensures |pairs| == 0
            ==> ret == 0
    ensures |pairs| != 0
            ==> ret as nat == LinearLength(Seq.DropLast(pairs)) as nat + PairLength(Seq.Last(pairs)) as nat
  {
    if |pairs| == 0 then 0
    else
      Add(LinearLength(Seq.DropLast(pairs)), PairLength(Seq.Last(pairs)))
  }
  by method { // because Seq.DropLast makes a full copy
    var result : uint64 := 0;
    SequenceIsSafeBecauseItIsInMemory(pairs);
    for i : uint64 := 0 to |pairs| as uint64
      invariant result == LinearLength(pairs[..i])
    {
      result := Add(result, PairLength(pairs[i]));
      assert result == LinearLength(pairs[..i]) + PairLength(pairs[i]);
      assert Seq.DropLast(pairs[..i+1]) == pairs[..i];
      assert result == LinearLength(Seq.DropLast(pairs[..i+1])) + PairLength(Seq.Last(pairs[..i+1]));
      assert result == LinearLength(pairs[..i+1]);

    }
    assert result == LinearLength(pairs[..|pairs|]);
    assert pairs == pairs[..|pairs|];
    assert result == LinearLength(pairs);
    return result;
  }


  function method PairLength(
    pair: Pair<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  ):
    (ret: uint64)
  {
    SequenceIsSafeBecauseItIsInMemory(pair.key);
    SequenceIsSafeBecauseItIsInMemory(pair.value);
    Add4(2, |pair.key| as uint64, 2, |pair.value| as uint64)
  }

  lemma LinearLengthIsDistributive(
    a: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>,
    b: Linear<UTF8.ValidUTF8Bytes, UTF8.ValidUTF8Bytes>
  )
    ensures LinearLength(a + b) == Add(LinearLength(a), LinearLength(b))
  {
    if b == [] {
      assert a + b == a;
    } else {
      calc {
        LinearLength(a + b);
      == // Definition of LinearLength
        if |a+b| == 0 then 0
        else Add(LinearLength(Seq.DropLast(a + b)), PairLength(Seq.Last(a+b)));
      == {assert |a+b| > 0;} // Because of the above `if` block
        LinearLength(Seq.DropLast(a + b)) + PairLength(Seq.Last(a+b));
      == {assert Seq.Last(a+b) == Seq.Last(b) && Seq.DropLast(a+b) == a + Seq.DropLast(b);} // Breaking apart (a+b)
        LinearLength(a + Seq.DropLast(b)) + PairLength(Seq.Last(b));
      == {LinearLengthIsDistributive(a, Seq.DropLast(b));} // This lets us break apart LinearLength(a + Seq.DropLast(b))
        (LinearLength(a) + LinearLength(Seq.DropLast(b))) + PairLength(Seq.Last(b));
      == // Move () to prove associativity of +
        LinearLength(a) + (LinearLength(Seq.DropLast(b)) + PairLength(Seq.Last(b)));
      == {assert LinearLength(Seq.DropLast(b)) + PairLength(Seq.Last(b)) == LinearLength(b);} // join the 2 parts of b back together
        LinearLength(a) + LinearLength(b);
      }
    }
  }
}
