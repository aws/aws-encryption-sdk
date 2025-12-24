// RUN: %verify --disable-nonlinear-arithmetic "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT 
 *******************************************************************************/

include "../../NonlinearArithmetic/DivMod.dfy"
include "../../NonlinearArithmetic/Mul.dfy"
include "../../NonlinearArithmetic/Power.dfy"
include "Seq.dfy"
include "LittleEndianNat.dfy"

/* Sequence with smaller base. */
abstract module {:options "-functionSyntax:4"} SmallSeq refines LittleEndianNat {

  function BITS(): nat
    ensures BITS() > 1

  function BASE(): nat
  {
    LemmaPowPositive(2, BITS() - 1);
    LemmaPowStrictlyIncreases(2, BITS() - 1, BITS());
    Pow(2, BITS())
  }

}

/* Sequence with larger base. */
abstract module {:options "-functionSyntax:4"} LargeSeq refines LittleEndianNat {

  import Small : SmallSeq

  function BITS(): nat
    ensures BITS() > Small.BITS() && BITS() % Small.BITS() == 0

  function BASE(): nat
  {
    LemmaPowPositive(2, BITS() - 1);
    LemmaPowStrictlyIncreases(2, BITS() - 1, BITS());
    Pow(2, BITS())
  }

}

abstract module {:options "-functionSyntax:4"} LittleEndianNatConversions {

  import opened DivMod
  import opened Mul
  import opened Power
  import opened Seq

  import opened Large : LargeSeq

  /* Small.BASE() to the power of E is Large.BASE(). */
  function E(): (E: nat)
    ensures Pow(Small.BASE(), E) == Large.BASE()
    ensures E > 0
  {
    LemmaDivBasicsAuto();
    assert forall x :: x != 0 ==> 0 / x == 0;
    LemmaPowMultipliesAuto();
    LemmaFundamentalDivMod(Large.BITS(), Small.BITS());
    assert Large.BITS() == Small.BITS() * (Large.BITS() / Small.BITS()) + (Large.BITS() % Small.BITS());
    assert (Large.BITS() / Small.BITS()) != 0;
    Large.BITS() / Small.BITS()
  }

  /* Converts a sequence from Large.BASE() to Small.BASE(). */
  function {:opaque} ToSmall(xs: seq<Large.uint>): (ys: seq<Small.uint>)
    ensures |ys| == |xs| * E()
  {
    if |xs| == 0 then []
    else
      LemmaMulIsDistributiveAddOtherWay(E(), 1, |xs| - 1);
      Small.FromNatWithLen(First(xs), E()) + ToSmall(DropFirst(xs))
  }

  /* Converts a sequence from Small.BASE() to Large.BASE(). */
  function {:opaque} ToLarge(xs: seq<Small.uint>): (ys: seq<Large.uint>)
    requires |xs| % E() == 0
    ensures |ys| == |xs| / E()
  {
    if |xs| == 0 then
      var ys := (LemmaDivBasicsAuto(); []);
      assert |ys| == |xs| / E();
      ys

    else
      LemmaModIsZero(|xs|, E());
      assert |xs| >= E();
      Small.LemmaSeqNatBound(xs[..E()]);
      LemmaDivMinusOne(|xs|, E());
      assert |xs[E()..]| % E() == 0 by {
        LemmaModSubMultiplesVanishAuto();
      }
      var ys := ([Small.ToNatRight(xs[..E()]) as Large.uint] + ToLarge(xs[E()..]));
      assert |ToLarge(xs[E()..])| == |xs[E()..]| / E();
      assert |ys| == |xs| / E() by {
        // To obtain a proof like this, first write a detailed proof
        // as much as you can, not assuming anything about non-linear arithmetic (use only lemmas for that)
        // Then remove intermediate computation steps and lemma calls if doing so decrease the resource count
        // until arriving at at a minimum
        calc {
          |ys|;
          1 + |xs[E()..]| / E();
          1 + (|xs| - E()) / E();
          { DivMod.ModINL.LemmaFundamentalDivMod(|xs|, E()); }
          1 + (E() * (|xs| / E()) + |xs| % E() - E()) / E();
          1 + (E() * (|xs| / E()) - E()) / E();
          1 + (E() * (|xs| / E()) + E() * -1) / E();
          { Mul.LemmaMulIsDistributiveAdd(E(), |xs| / E(), -1); }
          1 + (E() * (|xs| / E() + -1)) / E();
        }
      }
      ys
  }

  /* Sequence conversion from Large.BASE() to Small.BASE() does not
  change its nat representation. */
  lemma LemmaToSmall(xs: seq<Large.uint>)
    ensures Small.ToNatRight(ToSmall(xs)) == Large.ToNatRight(xs)
  {
    reveal Small.ToNatRight();
    reveal Large.ToNatRight();
    reveal ToSmall();
    if |xs| == 0 {
    } else {
      calc {
        Small.ToNatRight(ToSmall(xs));
        Small.ToNatRight(Small.FromNatWithLen(First(xs), E()) + ToSmall(DropFirst(xs)));
        {
          Small.LemmaSeqPrefix(Small.FromNatWithLen(First(xs), E()) + ToSmall(DropFirst(xs)), E());
          LemmaToSmall(DropFirst(xs));
        }
        First(xs) + Large.ToNatRight(DropFirst(xs)) * Pow(Small.BASE(), E());
        { assert Pow(Small.BASE(), E()) == Large.BASE(); }
        Large.ToNatRight(xs);
      }
    }
  }

  /* Sequence conversion from Small.BASE() to Large.BASE() does not
  change its nat representation. */
  lemma LemmaToLarge(xs: seq<Small.uint>)
    requires |xs| % E() == 0
    ensures Large.ToNatRight(ToLarge(xs)) == Small.ToNatRight(xs)
  {
    reveal Large.ToNatRight();
    reveal Small.ToNatRight();
    reveal ToLarge();
    if |xs| == 0 {
    } else {
      calc {
        Large.ToNatRight(ToLarge(xs));
        {
          LemmaModIsZero(|xs|, E());
          LemmaModSubMultiplesVanishAuto();
          Small.LemmaSeqNatBound(xs[..E()]);
        }
        Large.ToNatRight([Small.ToNatRight(xs[..E()]) as Large.uint] + ToLarge(xs[E()..]));
        { LemmaToLarge(xs[E()..]); }
        Small.ToNatRight(xs[..E()]) + Small.ToNatRight(xs[E()..]) * Pow(Small.BASE(), E());
        { Small.LemmaSeqPrefix(xs, E()); }
        Small.ToNatRight(xs);
      }
    }
  }

  /* ToSmall is injective. */
  lemma LemmaToSmallIsInjective(xs: seq<Large.uint>, ys: seq<Large.uint>)
    requires ToSmall(xs) == ToSmall(ys)
    requires |xs| == |ys|
    ensures xs == ys
  {
    LemmaToSmall(xs);
    LemmaToSmall(ys);
    assert Large.ToNatRight(xs) == Large.ToNatRight(ys);
    Large.LemmaSeqEq(xs, ys);
  }

  /* ToLarge is injective. */
  lemma LemmaToLargeIsInjective(xs: seq<Small.uint>, ys: seq<Small.uint>)
    requires |xs| % E() == |ys| % E() == 0
    requires ToLarge(xs) == ToLarge(ys)
    requires |xs| == |ys|
    ensures xs == ys
  {
    LemmaToLarge(xs);
    LemmaToLarge(ys);
    assert Small.ToNatRight(xs) == Small.ToNatRight(ys);
    Small.LemmaSeqEq(xs, ys);
  }

  /* If we start with a Small sequence, convert it to a Large sequence,
  and convert it back, we get the same sequence we started with. */
  lemma LemmaSmallLargeSmall(xs: seq<Small.uint>)
    requires |xs| % E() == 0
    ensures ToSmall(ToLarge(xs)) == xs
  {
    reveal ToSmall();
    reveal ToLarge();
    if |xs| == 0 {
    } else {
      calc {
        ToSmall(ToLarge(xs));
        {
          LemmaModIsZero(|xs|, E());
          Small.LemmaSeqNatBound(xs[..E()]);
          LemmaModSubMultiplesVanishAuto();
        }
        ToSmall([Small.ToNatRight(xs[..E()]) as Large.uint] + ToLarge(xs[E()..]));
        Small.FromNatWithLen(Small.ToNatRight(xs[..E()]), E()) + ToSmall(ToLarge(xs[E()..]));
        {
          Small.LemmaSeqNatSeq(xs[..E()]);
          LemmaSmallLargeSmall(xs[E()..]);
        }
        xs;
      }
    }
  }

  /* If we start with a Large sequence, convert it to a Sequence sequence,
  and convert it back, we get the same sequence we started with. */
  lemma LemmaLargeSmallLarge(xs: seq<Large.uint>)
    ensures |ToSmall(xs)| % E() == 0
    ensures ToLarge(ToSmall(xs)) == xs
  {
    reveal ToSmall();
    reveal ToLarge();
    LemmaModMultiplesBasicAuto();
    if |xs| == 0 {
    } else {
      calc {
        ToLarge(ToSmall(xs));
        ToLarge(Small.FromNatWithLen(First(xs), E()) + ToSmall(DropFirst(xs)));
        [Small.ToNatRight(Small.FromNatWithLen(First(xs), E())) as Large.uint] + ToLarge(ToSmall(DropFirst(xs)));
        [First(xs)] + ToLarge(ToSmall(DropFirst(xs)));
        { LemmaLargeSmallLarge(DropFirst(xs)); }
        [First(xs)] + DropFirst(xs);
        xs;
      }
    }
  }

}

/* Conversions between sequences of uint8 and uint16. */
module {:options "-functionSyntax:4"} Uint8_16 refines LittleEndianNatConversions {

  module Uint8Seq refines SmallSeq {
    function BITS(): nat { 8 }
  }

  module Uint16Seq refines LargeSeq {
    import Small = Uint8Seq
    function BITS(): nat { 16 }
  }

  import opened Large = Uint16Seq
  import Small = Large.Small

}

/* Conversions between sequences of uint8 and uint32. */
module {:options "-functionSyntax:4"} Uint8_32 refines LittleEndianNatConversions {

  module Uint8Seq refines SmallSeq {
    function BITS(): nat { 8 }
  }

  module Uint32Seq refines LargeSeq {
    import Small = Uint8Seq
    function BITS(): nat { 32 }
  }

  import opened Large = Uint32Seq
  import Small = Large.Small

}

/* Conversions between sequences of uint8 and uint64. */
module {:options "-functionSyntax:4"} Uint8_64 refines LittleEndianNatConversions {

  module Uint8Seq refines SmallSeq {
    function BITS(): nat { 8 }
  }

  module Uint64Seq refines LargeSeq {
    import Small = Uint8Seq
    function BITS(): nat { 64 }
  }

  import opened Large = Uint64Seq
  import Small = Large.Small

}

/* Conversions between sequences of uint16 and uint32. */
module {:options "-functionSyntax:4"} Uint16_32 refines LittleEndianNatConversions {

  module Uint16Seq refines SmallSeq {
    function BITS(): nat { 16 }
  }

  module Uint32Seq refines LargeSeq {
    import Small = Uint16Seq
    function BITS(): nat { 32 }
  }

  import opened Large = Uint32Seq
  import Small = Large.Small

}

/* Conversions between sequences of uint32 and uint64. */
module {:options "-functionSyntax:4"} Uint32_64 refines LittleEndianNatConversions {

  module Uint32Seq refines SmallSeq {
    function BITS(): nat { 32 }
  }

  module Uint64Seq refines LargeSeq {
    import Small = Uint32Seq
    function BITS(): nat { 64 }
  }

  import opened Large = Uint64Seq
  import Small = Large.Small

}
