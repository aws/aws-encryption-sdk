// RUN: %verify "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT
 *******************************************************************************/

include "../../src/Unicode/Utf16EncodingForm.dfy"

module Utf16EncodingFormExamples {
  import Unicode
  import opened Utf16EncodingForm

  const TEST_SCALAR_VALUES: seq<(Unicode.ScalarValue, WellFormedCodeUnitSeq)> := [
    // One code unit: dollar sign
    (0x0024, [0x0024]),
    // Two code units: money bag emoji
    (0x1F4B0, [0xD83D, 0xDCB0])
  ]

  lemma TestEncodeScalarValue()
    ensures forall pair | pair in TEST_SCALAR_VALUES
              :: EncodeScalarValue(pair.0) == pair.1
  {}

  // Because surrogate code points are not Unicode scalar values, isolated UTF-16 code units in the range
  // D800_16 .. DFFF_16 are ill-formed. (Section 3.9 D91)
  const TEST_ILL_FORMED_SEQUENCES: seq<CodeUnitSeq> := [
    [0xD800],
    [0xDABC],
    [0xDFFF]
  ]

  lemma TestDecodeIllFormedSequence()
    ensures forall s | s in TEST_ILL_FORMED_SEQUENCES
              :: DecodeCodeUnitSequenceChecked(s).None?
  {}
}
