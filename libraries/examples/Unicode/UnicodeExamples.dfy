// RUN: %verify "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT
 *******************************************************************************/

include "../../src/Unicode/Unicode.dfy"

module UnicodeExamples {
  import opened Unicode

  const TEST_ASSIGNED_PLANE_CODE_POINTS: set<CodePoint> := {
    0x000055,  // Latin capital letter U
    0x01F11D,  // Parenthesized Latin capital letter N
    0x02053C,  // CJK unified ideograph 𠔼
    0x030256,  // CJK unified ideograph 𰉖
    0x0E004F,  // Tag Latin capital letter O
    0x0FDDDD,  // arbitrary code point in Private Use Area-A
    0x10EEEE   // arbitrary code point in Private Use Area-B
  }

  lemma LemmaAssignedCodePoints()
    ensures forall p | p in TEST_ASSIGNED_PLANE_CODE_POINTS :: IsInAssignedPlane(p)
  {
    reveal IsInAssignedPlane();
  }
}
