// RUN: %verify %s

///  Converting between strings and UTF-8/UTF-16
/// =============================================
///
/// This abstract module defines an interface for converting
/// between the Dafny `string` type and sequences of UTF-8 and UTF-16
/// codepoints, where codepoints are represents as bounded ints
/// (`uint8` and `uint16`).
///
/// Because the `--unicode-char` option changes the meaning of the `char`
/// type and hence the `string` type, there are two different concrete
/// implementations in separate files.
/// Only include the one that matches your value of that option!
///
/// If you also want to maintain code that works for either mode,
/// you have two options:
///
/// 1. Implement your logic in an abstract module as well that
///    imports this one, and define two different refining modules
///    that import the appropriate UnicodeStrings module.
/// 2. Do not `include` any of these three files in your source code,
///    instead passing the appropriate file to Dafny when verifying and building,
///    so that references to `UnicodeStrings` can be resolved.
///    See the JSON modules as an example.
///
/// Option 2. avoids needing to write boilerplate refining modules,
/// but is less IDE-friendly until we have better project configuration support.

include "../BoundedInts.dfy"
include "../Wrappers.dfy"
include "../Collections/Sequences/Seq.dfy"
include "Utf8EncodingForm.dfy"
include "Utf16EncodingForm.dfy"

abstract module {:options "-functionSyntax:4"} AbstractUnicodeStrings {

  import Seq

  import opened Wrappers
  import opened BoundedInts

  function ToUTF8Checked(s: string): Option<seq<uint8>>

  function ASCIIToUTF8(s: string): seq<uint8>
    requires forall i | 0 <= i < |s| :: 0 <= s[i] as int < 128
  {
    Seq.Map(c requires 0 <= c as int < 128 => c as uint8, s)
  }

  function FromUTF8Checked(bs: seq<uint8>): Option<string>

  function ToUTF16Checked(s: string): Option<seq<uint16>>

  function ASCIIToUTF16(s: string): seq<uint16>
    requires forall i | 0 <= i < |s| :: 0 <= s[i] as int < 128
  {
    Seq.Map(c requires 0 <= c as int < 128 => c as uint16, s)
  }

  function FromUTF16Checked(bs: seq<uint16>): Option<string>
}
