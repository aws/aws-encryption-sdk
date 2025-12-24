
## The `Binary Operations` module {#sec-binaryop}

The `Binary Operations` module defines a number of properties of binary operations.

Properties of binary operations `bop`:
- Associative: `bop(bop(x, y), z) == bop(x, bop(y, z))`
- Unital: `bop(unit, x) == x == bop(x, unit)`
- Abelian: `bop(x, y) == bop(y, x)`
- Inverse: `bop(x, inverse(x)) == unit == bop(inverse(x), x)`
- Distributive: `bop2(bop1(x, y), z) == bop1(bop2(x, z), bop2(y, z))` and `bop2(x, bop1(y, z)) == bop1(bop2(x, y), bop2(x, z))`
- Monoid: Associative, Unital
- Group: Monoid, Inverse
- Abelian Group: Abelian, Group
- Ring: AbelianGroup A, Monoid B, Distributive
- Field: Abelian Group A, Abelian Group B (without 0), Distributive