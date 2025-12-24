// RUN: %verify "%s"

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT
 *******************************************************************************/

include "../../src/Unicode/Utf8EncodingForm.dfy"

module Utf8EncodingFormExamples {
  import Unicode
  import opened Utf8EncodingForm

  const TEST_SCALAR_VALUES: seq<(Unicode.ScalarValue, WellFormedCodeUnitSeq)> := [
    // One byte: dollar sign
    (0x0024, [0x24]),
    // Two bytes: pound sign
    (0x00A3, [0xC2, 0xA3]),
    // Three bytes: euro sign
    (0x20AC, [0xE2, 0x82, 0xAC]),
    // Four bytes: money bag emoji
    (0x1F4B0, [0xF0, 0x9F, 0x92, 0xB0])
  ]

  lemma TestEncodeScalarValue()
    ensures forall pair | pair in TEST_SCALAR_VALUES
              :: EncodeScalarValue(pair.0) == pair.1
  {}

  // Examples taken from description of Table 3-7.
  const TEST_ILL_FORMED_SEQUENCES: seq<CodeUnitSeq> := [
    // C0 is not well-formed as a first byte
    [0xC0, 0xAF],
    // 9F is not well-formed as a second byte when E0 is a well-formed first byte
    [0xE0, 0x9F, 0x80]
  ]

  lemma TestDecodeIllFormedSequence()
    ensures forall s | s in TEST_ILL_FORMED_SEQUENCES
              :: DecodeCodeUnitSequenceChecked(s).None?
  {}

}
