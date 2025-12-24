// RUN: %verify "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT 
 *******************************************************************************/

/** Defines a number of (ghost) properties of binary operations */
module {:options "-functionSyntax:4"} BinaryOperations
{

  ghost predicate IsAssociative<T(!new)>(bop: (T, T) -> T) {
    forall x, y, z :: bop(bop(x, y), z) == bop(x, bop(y, z))
  }

  ghost predicate IsLeftUnital<T(!new)>(bop: (T, T) -> T, unit: T) {
    forall x :: bop(unit, x) == x
  }

  ghost predicate IsRightUnital<T(!new)>(bop: (T, T) -> T, unit: T) {
    forall x :: bop(x, unit) == x
  }

  ghost predicate IsUnital<T(!new)>(bop: (T, T) -> T, unit: T) {
    && IsLeftUnital(bop, unit)
    && IsRightUnital(bop, unit)
  }

  ghost predicate IsLeftInverse<T(!new)>(bop: (T, T) -> T, inverse: T --> T, unit: T) {
    forall x | inverse.requires(x) :: bop(inverse(x), x) == unit
  }

  ghost predicate IsRightInverse<T(!new)>(bop: (T, T) -> T, inverse: T --> T, unit: T) {
    forall x | inverse.requires(x) :: bop(x, inverse(x)) == unit
  }

  ghost predicate IsInverse<T(!new)>(bop: (T, T) -> T, inverse: T --> T, unit: T) {
    && IsLeftInverse(bop, inverse, unit)
    && IsRightInverse(bop, inverse, unit)
  }

  ghost predicate IsCommutative<T(!new)>(bop: (T, T) -> T) {
    forall x, y :: bop(x, y) == bop(y, x)
  }

  ghost predicate IsRightDistributive<T(!new)>(bop1: (T, T) -> T, bop2: (T, T) -> T) {
    forall x, y, z :: bop2(bop1(x, y), z) == bop1(bop2(x, z), bop2(y, z))
  }

  ghost predicate IsLeftDistributive<T(!new)>(bop1: (T, T) -> T, bop2: (T, T) -> T) {
    forall x, y, z :: bop2(x, bop1(y, z)) == bop1(bop2(x, y), bop2(x, z))
  }

  ghost predicate IsDistributive<T(!new)>(bop1: (T, T) -> T, bop2: (T, T) -> T) {
    && IsLeftDistributive(bop1, bop2)
    && IsRightDistributive(bop1, bop2)
  }

  ghost predicate IsMonoid<T(!new)>(bop: (T, T) -> T, unit: T) {
    && IsAssociative(bop)
    && IsUnital(bop, unit)
  }

  datatype MonoidStructure<!T(!new)> = MonoidStructure(bop: (T, T) -> T, unit: T)
  type Monoid<!T(!new)> = g: MonoidStructure | IsMonoid(g.bop, g.unit) witness *

  ghost predicate IsGroup<T(!new)>(bop: (T, T) -> T, inverse: T --> T, unit: T) {
    && IsMonoid(bop, unit)
    && IsInverse(bop, inverse, unit)
  }

  datatype GroupStructure<!T(!new)> = GroupStructure(bop: (T, T) -> T, inverse: T -> T, unit: T)
  type Group<!T(!new)> = g: GroupStructure | IsGroup(g.bop, g.inverse, g.unit) witness *

  ghost predicate IsAbelianGroup<T(!new)>(bop: (T, T) -> T, inverse: T --> T, unit: T) {
    && IsGroup(bop, inverse, unit)
    && IsCommutative(bop)
  }

  type AbelianGroup<!T(!new)> = g: GroupStructure | IsAbelianGroup(g.bop, g.inverse, g.unit) witness *

  ghost predicate IsRing<T(!new)>(add: (T, T) -> T, minus: T -> T, zero: T, mult: (T, T) -> T, one: T) {
    && IsAbelianGroup(add, minus, zero)
    && IsMonoid(mult, one)
    && IsDistributive(add, mult)
  }

  datatype RingStructure<!T(!new)> = RingStructure(add: (T, T) -> T, minus: T -> T, zero: T, mult: (T, T) -> T, one: T)
  type Ring<!T(!new)> = r: RingStructure | IsRing(r.add, r.minus, r.zero, r.mult, r.one) witness *

  ghost predicate IsField<T(!new)>(add: (T, T) -> T, minus: T -> T, zero: T, mult: (T, T) -> T, div: T --> T, one: T)
    requires forall t :: (t != zero) ==> div.requires(t)
  {
    && IsAbelianGroup(add, minus, zero)
    && IsAbelianGroup(mult, div, one)
    && IsLeftDistributive(add, mult)
  }

  datatype FieldStructure<!T(!new)> = FieldStructure(add: (T, T) -> T, minus: T -> T, zero: T, mult: (T, T) -> T, div: T --> T, one: T)
  type FieldPre<!T(!new)> = f: FieldStructure | (forall t :: (t != f.zero) ==> f.div.requires(t)) witness *
  type Field<!T(!new)> = f: FieldPre | IsField(f.add, f.minus, f.zero, f.mult, f.div, f.one) witness *

  ghost predicate IsHomomorphism<S(!new),T>(bop1: (S, S) -> S, bop2: (T, T) -> T, f: S -> T) {
    forall x, y :: f(bop1(x, y)) == bop2(f(x), f(y))
  }

  lemma MonoidInverseIsUnique<T(!new)>(m: Monoid<T>, inverse1: T -> T, inverse2: T -> T)
    requires IsInverse(m.bop, inverse1, m.unit)
    requires IsInverse(m.bop, inverse2, m.unit)
    ensures forall x :: inverse1(x) == inverse2(x)
  {
    assert IsMonoid(m.bop, m.unit);
    forall x ensures inverse1(x) == inverse2(x) {
      calc {
        inverse1(x);
      == { assert IsRightUnital(m.bop, m.unit); }
        m.bop(inverse1(x), m.unit);
      == { assert IsRightInverse(m.bop, inverse2, m.unit); assert m.unit == m.bop(x, inverse2(x)); }
        m.bop(inverse1(x), m.bop(x, inverse2(x)));
      == { assert IsAssociative(m.bop); }
        m.bop(m.bop(inverse1(x), x), inverse2(x));
      == { assert IsLeftInverse(m.bop, inverse1, m.unit); }
        m.bop(m.unit, inverse2(x));
      == { assert IsLeftUnital(m.bop, m.unit); }
        inverse2(x);
      }
    }
  }

  lemma UnitIsUnique<T(!new)>(bop: (T, T) -> T, unit1: T, unit2: T)
    requires IsUnital(bop, unit1)
    requires IsUnital(bop, unit2)
    ensures unit1 == unit2
  {
    calc {
      unit1;
    == { assert IsRightUnital(bop, unit2); }
      bop(unit1, unit2);
    == { assert IsLeftUnital(bop, unit1); }
      unit2;
    }
  }

  lemma GroupUnitIsSelfInverse<T(!new)>(g: Group<T>)
    ensures g.unit == g.inverse(g.unit)
  {
    calc {
      g.unit;
    == { assert IsLeftInverse(g.bop, g.inverse, g.unit); }
      g.bop(g.inverse(g.unit), g.unit);
    == { assert IsRightUnital(g.bop, g.unit); }
      g.inverse(g.unit);
    }
  }

  lemma GroupHomomorphismPreservesUnit<S(!new),T(!new)>(g1: Group<S>, g2: Group<T>, f: S -> T)
    requires IsHomomorphism(g1.bop, g2.bop, f)
    ensures f(g1.unit) == g2.unit
  {
    calc {
      f(g1.unit);
    == { assert IsRightUnital(g2.bop, g2.unit); }
      g2.bop(f(g1.unit), g2.unit);
    == { assert IsRightInverse(g2.bop, g2.inverse, g2.unit); }
      g2.bop(f(g1.unit), g2.bop(f(g1.unit), g2.inverse(f(g1.unit))));
    == { assert IsAssociative(g2.bop); }
      g2.bop(g2.bop(f(g1.unit), f(g1.unit)), g2.inverse(f(g1.unit)));
    == { assert IsHomomorphism(g1.bop, g2.bop, f); }
      g2.bop(f(g1.bop(g1.unit, g1.unit)), g2.inverse(f(g1.unit)));
    == { assert IsLeftUnital(g1.bop, g1.unit); }
      g2.bop(f(g1.unit), g2.inverse(f(g1.unit)));
    == { assert IsRightInverse(g2.bop, g2.inverse, g2.unit); }
      g2.unit;
    }
  }

  lemma GroupHomomorphismPreservesInverse<S(!new),T(!new)>(g1: Group<S>, g2: Group<T>, f: S -> T)
    requires IsHomomorphism(g1.bop, g2.bop, f)
    ensures forall x | g1.inverse.requires(x) :: f(g1.inverse(x)) == g2.inverse(f(x))
  {
    assert IsGroup(g1.bop, g1.inverse, g1.unit);
    assert IsGroup(g2.bop, g2.inverse, g2.unit);
    forall x | g1.inverse.requires(x) ensures f(g1.inverse(x)) == g2.inverse(f(x)) {
      calc {
        f(g1.inverse(x));
      == { assert IsRightUnital(g2.bop, g2.unit); }
        g2.bop(f(g1.inverse(x)), g2.unit);
      == { assert IsRightInverse(g2.bop, g2.inverse, g2.unit); assert g2.unit == g2.bop(f(x), g2.inverse(f(x))); }
        g2.bop(f(g1.inverse(x)), g2.bop(f(x), g2.inverse(f(x))));
      == { assert IsAssociative(g2.bop); }
        g2.bop(g2.bop(f(g1.inverse(x)), f(x)), g2.inverse(f(x)));
      == { assert IsHomomorphism(g1.bop, g2.bop, f); assert g2.bop(f(g1.inverse(x)), f(x)) == f(g1.bop(g1.inverse(x), x)); }
        g2.bop(f(g1.bop(g1.inverse(x), x)), g2.inverse(f(x)));
      == { assert IsLeftInverse(g1.bop, g1.inverse, g1.unit); assert g1.bop(g1.inverse(x), x) == g1.unit; assert f(g1.bop(g1.inverse(x), x)) == f(g1.unit); }
        g2.bop(f(g1.unit), g2.inverse(f(x)));
      == { GroupHomomorphismPreservesUnit(g1, g2, f); assert f(g1.unit) == g2.unit; }
        g2.bop(g2.unit, g2.inverse(f(x)));
      == { assert IsLeftUnital(g2.bop, g2.unit); }
        g2.inverse(f(x));
      }
    }
  }

}
