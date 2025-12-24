// RUN: %verify "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT
 *******************************************************************************/

include "../src/dafny/Relations.dfy"

module {:options "-functionSyntax:4"} RelationsExamples {

  import opened Dafny.Relations

  lemma BuiltInIntLTIsStrictTotalOrdering()
    ensures StrictTotalOrdering((x: int, y: int) => x < y)
  {}

  lemma BuiltInIntLEIsTotalOrdering()
    ensures TotalOrdering((x: int, y: int) => x <= y)
  {}

  function Modulo(n: nat): (R: (int, int) -> bool)
    requires n >= 1
    ensures EquivalenceRelation(R)
  {
    (x, y) => x % n ==  y % n
  }

  lemma BuiltInIntEqIsEquivalenceRelation()
    ensures EquivalenceRelation((x: int, y: int) => x == y)
  {}

  lemma BuiltInIntGeIsAntiSymmetricRelation()
    ensures AntiSymmetric((x: int, y: int) => x >= y)
  {}

  lemma BuiltInIntLtIsAsymmetricRelation()
    ensures Asymmetric((x: int, y: int) => x < y)
  {
  }

  lemma AlwaysTrueIsNotAntiSymmetric()
    ensures !AntiSymmetric((x: int, y: int) => true)
  {
    var f := (x: int, y: int) => true;
    assert f(2,3) && f(3,2) && 3 != 2;
    assert !AntiSymmetric(f);
  }

  lemma BuiltInIntLtIsNotReflexiveRelation()
    ensures !Reflexive((x: int, y: int) => x < y)
  {
    var f := (x: int, y: int) => x < y;
    assert !f(0, 0);
    assert !forall x: int :: f(x, x);
    assert !Reflexive(f);
  }

  lemma BuiltInIntLtIsIrreflexiveRelation()
    ensures Irreflexive((x: int, y: int) => x < y)
  {}

  lemma BuiltInIntEqIsNotIrreflexiveRelation()
    ensures !Irreflexive((x: int, y: int) => x == y)
  {
    var f := (x: int, y: int) => x == y;
    assert f(0, 0);
    assert !Irreflexive(f);
  }

  lemma AsymmetricIsAntiSymmetric<T(!new)>(f: (T, T) -> bool)
    ensures Asymmetric(f) ==> AntiSymmetric(f)
  {}

  lemma AsymmetricIsIrreflexive<T(!new)>(f: (T, T) -> bool)
    ensures Asymmetric(f) ==> Irreflexive(f)
  {}
}
