// RUN: %run --disable-nonlinear-arithmetic "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT 
 *******************************************************************************/

include "../../src/Collections/Sequences/LittleEndianNatConversions.dfy"

module LittleEndianNatExample {

  import opened Uint8_32

  method Main() {
    var n := 49602234234;

    // Convert n to uint8 and uint32 sequences
    var smallSeq, largeSeq := Small.FromNat(n), Large.FromNat(n);
    expect smallSeq == [122, 7, 134, 140, 11];
    expect largeSeq == [2357593978, 11];

    Small.LemmaNatSeqNat(n);
    Large.LemmaNatSeqNat(n);
    assert Small.ToNatRight(smallSeq) == Large.ToNatRight(largeSeq) == n;

    // Extend smallSeq
    smallSeq := Small.SeqExtendMultiple(smallSeq, E());
    assert Small.ToNatRight(smallSeq) == n;

    // Convert between smallSeqExtended and largeSeq
    LemmaToSmall(largeSeq);
    LemmaToLarge(smallSeq);
    assert Small.ToNatRight(ToSmall(largeSeq)) == n;
    assert Large.ToNatRight(ToLarge(smallSeq)) == n;
    assert |ToSmall(largeSeq)|  == |largeSeq| * E();
    assert |ToLarge(smallSeq)|  == |smallSeq| / E();

    LemmaSmallLargeSmall(smallSeq);
    LemmaLargeSmallLarge(largeSeq);
    assert ToSmall(ToLarge(smallSeq)) == smallSeq;
    assert ToLarge(ToSmall(largeSeq)) == largeSeq;
  }

}
