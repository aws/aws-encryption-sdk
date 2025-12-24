
## The `NonlinearArith` module {#sec-nonlinear-arith}

Non-linear math uses operations like multiplication, division and modulo (where neither argument is a literal integer). Reasoning problems using non-linear arithmetic are generally undecidable; hence Dafny (and Z3) rely on heuristics to prove such properties.
While wonderful when they work, these heuristics can lead to unstable proofs.  Hence, in many projects,
it can be helpful to turn off these heuristics (via Dafny's `--disable-nonlinear-arithmetic` flag) and instead explicitly
invoke the lemmas in this library.  All files in this portion of the library (except those in `Internals/*Nonlinear.dfy`)
verify with the `--disable-nonlinear-arithmetic` flag, which should keep the library itself stable.

In general, it shouldn't be necessary to directly reference anything in `Internals`.

