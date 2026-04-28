// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]
use crate::*;
use std::sync::LazyLock;

// all attributes with this prefix reserved for the implementation
const RESERVED_PREFIX: &str = "aws_dbe_";
const HEADER_FIELD: &str = "aws_dbe_head";
const FOOTER_FIELD: &str = "aws_dbe_foot";

pub(crate) static HEADER_PATH: LazyLock<Vec<PathSegment>> = LazyLock::new(|| {
    vec![PathSegment::Member(StructureSegment {
        key: HEADER_FIELD.to_string(),
    })]
});
pub(crate) static FOOTER_PATH: LazyLock<Vec<PathSegment>> = LazyLock::new(|| {
    vec![PathSegment::Member(StructureSegment {
        key: FOOTER_FIELD.to_string(),
    })]
});

//   const HEADER_PATHS : &[Path] = [HeaderPath, FooterPath];
const RESERVED_CRYPTO_CONTEXT_PREFIX_STRING: &str = "aws-crypto-";

const ATTR_PREFIX: &str = "aws-crypto-attr.";
const LEGEND: &str = "aws-crypto-legend";

const LEGEND_STRING: char = 'S';
const LEGEND_NUMBER: char = 'N';
const LEGEND_LITERAL: char = 'L';
const LEGEND_BINARY: char = 'B';

const NULL_STR: &str = "null";
const TRUE_STR: &str = "true";
const FALSE_STR: &str = "false";

pub(crate) type CanonicalPath = Vec<u8>;

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub(crate) struct CanonCryptoItem {
    pub(crate) key: CanonicalPath,
    pub(crate) orig_key: Vec<PathSegment>,
    pub(crate) data: StructuredDataTerminal,
    pub(crate) action: CryptoAction,
}
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub(crate) struct CanonAuthItem {
    pub(crate) key: CanonicalPath,
    pub(crate) orig_key: Vec<PathSegment>,
    pub(crate) data: StructuredDataTerminal,
    pub(crate) action: AuthenticateAction,
}

/*

type CanonCryptoList = seq<CanonCryptoItem>
type CanonAuthList = seq<CanonAuthItem>

function method CryptoListToSet(xs: CryptoList) : (ret : set<Path>)
  ensures |xs| == 0 ==> |ret| == 0
  ensures |xs| == 1 ==> ret == {xs[0].key}
  ensures |xs| == 1 ==> |ret| == 1
{
  set k <- xs :: k.key
}

function method CanonCryptoListToSet(xs: CanonCryptoList) : (ret : set<Path>)
  ensures |xs| == 0 ==> |ret| == 0
  ensures |xs| == 1 ==> ret == {xs[0].origKey}
  ensures |xs| == 1 ==> |ret| == 1
{
  set k <- xs :: k.origKey
}

function method AuthListToSet(xs: AuthList) : (ret : set<Path>)
  ensures |xs| == 0 ==> |ret| == 0
  ensures |xs| == 1 ==> ret == {xs[0].key}
  ensures |xs| == 1 ==> |ret| == 1
{
  set k <- xs :: k.key
}

predicate method CryptoListHasNoDuplicatesFromSet(xs: CryptoList)
{
  SequenceIsSafeBecauseItIsInMemory(xs);
  SetIsSafeBecauseItIsInMemory(CryptoListToSet(xs));
  |CryptoListToSet(xs)| as u64 == |xs| as u64
}

predicate method AuthListHasNoDuplicatesFromSet(xs: AuthList)
{
  SequenceIsSafeBecauseItIsInMemory(xs);
  SetIsSafeBecauseItIsInMemory(AuthListToSet(xs));
  |AuthListToSet(xs)| as u64 == |xs| as u64
}

predicate CryptoListHasNoDuplicates(xs: CryptoList)
{
  forall i, j :: 0 <= i < j < |xs| ==> xs[i].key != xs[j].key
}

predicate AuthListHasNoDuplicates(xs: AuthList)
{
  forall i, j :: 0 <= i < j < |xs| ==> xs[i].key != xs[j].key
}

predicate CanonCryptoListHasNoDuplicates(xs: CanonCryptoList)
{
  forall i, j :: 0 <= i < j < |xs| ==> xs[i].origKey != xs[j].origKey
}

predicate CanonAuthListHasNoDuplicates(xs: CanonAuthList)
{
  forall i, j :: 0 <= i < j < |xs| ==> xs[i].origKey != xs[j].origKey
}

//= specification/structured-encryption/encrypt-path-structure.md#header-field
//= type=implication
//# The Header Field name MUST be `aws_dbe_head`

//= specification/structured-encryption/encrypt-path-structure.md#footer-field
//= type=implication
//# The Footer Field name MUST be `aws_dbe_foot`
lemma CheckNames()
  ensures HeaderField == "aws_dbe_head"
  ensures FooterField == "aws_dbe_foot"
{}

// Currently, only one set of sizes are supported
ghost const KeySize = 32 // 256 bits, for 256-bit AES keys
ghost const NonceSize = 12 // 96 bits, per AES-GCM nonces
ghost const AuthTagSize = 16
ghost const MSGID_LEN = 32
*/
pub(crate) const KEY_SIZE: usize = 32; // 256 bits, for 256-bit AES keys
pub(crate) const NONCE_SIZE: usize = 12; // 96 bits, per AES-GCM nonces
pub(crate) const AUTH_TAG_SIZE: usize = 16;
pub(crate) const MSGID_LEN: usize = 32;
pub(crate) type MessageID = [u8; MSGID_LEN];
pub(crate) type Key = [u8; KEY_SIZE];
pub(crate) type Nonce = [u8; NONCE_SIZE];

pub(crate) const DBE_ALGORITHM_FAMILY: u8 = 0x67;
/*

  lemma ValidSuiteSizes(alg : CMP.AlgorithmSuiteInfo)
    requires ValidSuite(alg)
    ensures AlgorithmSuites.GetEncryptKeyLength(alg) as int == KeySize
    ensures alg.encrypt.AES_GCM.keyLength as int == KeySize
    ensures alg.encrypt.AES_GCM.tagLength as int == AuthTagSize
    ensures alg.encrypt.AES_GCM.ivLength as int == NonceSize
  {}

  type Key = x : seq<u8> | |x| == KeySize witness *
  type Nonce = x : seq<u8> | |x| == NonceSize witness *
  type AuthTag = x : seq<u8> | |x| == AuthTagSize witness *

  type Bytes = seq<u8>
  type CanonicalPath = seq<u8>

  type GoodString = x : string | ValidString(x)
  predicate method ValidString(x : string)
  {
    && HasUint64Len(x)
    && UTF8.Encode(x).Success?
  }

  // Within the context of the StructuredEncryptionClient, certain things must be true of any Algorithm Suite
  predicate method ValidSuite(alg : CMP.AlgorithmSuiteInfo)
  {
    alg.id.DBE? && AlgorithmSuites.DBEAlgorithmSuite?(alg)
  }

  // string to Error
  function method E(s : string) : Error {
    StructuredEncryptionException(message = s)
  }

  // sequences are equal if zero is returned
  // Some care should be taken to ensure that target languages don't over optimize this.
  function method {:tailrecursion} ConstantTimeCompare(a : Bytes, b : Bytes, pos : u64 = 0, acc : bv8 = 0) : bv8
    requires |a| == |b|
    requires 0 <= pos as nat <= |a|
    decreases |a| - pos as nat
  {
    SequenceIsSafeBecauseItIsInMemory(a);
    if |a| as u64 == pos then
      acc
    else
      var x = ((a[pos] as bv8) ^ (b[pos] as bv8));
      ConstantTimeCompare(a, b, pos+1, x | acc)
  }

  predicate method ConstantTimeEquals(a : Bytes, b : Bytes)
    requires |a| == |b|
  {
    ConstantTimeCompare(a, b) == 0
  }

  // attribute is "authorized", a.k.a. included in the signature
  predicate method IsAuthAttr(x : CryptoAction)
  {
    !x.DO_NOTHING?
  }
*/
// wrap a value in a StructuredData
pub(crate) const fn value_to_data(
    value: Vec<u8>,
    type_id: TerminalTypeId,
) -> StructuredDataTerminal {
    StructuredDataTerminal { value, type_id }
}
/*
  // extract a value from a StructuredData
  function method GetValue(data : StructuredDataTerminal) : Bytes
  {
    data.value
  }

  predicate method ByteLess(x : u8, y : u8)
  {
    x < y
  }

  predicate method CharLess(x : char, y : char)
  {
    x < y
  }
*/
//= specification/dynamodb-encryption-client/ddb-attribute-serialization.md#type-id
//= type=implication
//# Type ID indicates what type a DynamoDB Attribute Value MUST
//# be serialized and deserialized as.
//# | Attribute Value Data Type | Terminal Type ID |
//# | ------------------------- | ---------------- |
//# | Null (NULL)               | 0x0000           |
//# | String (S)                | 0x0001           |
//# | Number (N)                | 0x0002           |
//# | Binary (B)                | 0xFFFF           |
//# | Boolean (BOOL)            | 0x0004           |
//# | String Set (SS)           | 0x0101           |
//# | Number Set (NS)           | 0x0102           |
//# | Binary Set (BS)           | 0x01FF           |
//# | Map (M)                   | 0x0200           |
//# | List (L)                  | 0x0300           |
pub(crate) const TERM_T: u8 = 0x00;
pub(crate) const SET_T: u8 = 0x01;
pub(crate) const MAP_T: u8 = 0x02;
pub(crate) const LIST_T: u8 = 0x03;
pub(crate) const NULL_T: u8 = 0x00;
pub(crate) const STRING_T: u8 = 0x01;
pub(crate) const NUMBER_T: u8 = 0x02;
pub(crate) const BINARY_T: u8 = 0xFF;
pub(crate) const BOOLEAN_T: u8 = 0x04;

pub(crate) const NULL: TerminalTypeId = [TERM_T, NULL_T];
pub(crate) const STRING: TerminalTypeId = [TERM_T, STRING_T];
pub(crate) const NUMBER: TerminalTypeId = [TERM_T, NUMBER_T];
pub(crate) const BINARY: TerminalTypeId = [0xFF, 0xFF];
pub(crate) const BOOLEAN: TerminalTypeId = [TERM_T, BOOLEAN_T];
pub(crate) const STRING_SET: TerminalTypeId = [SET_T, STRING_T];
pub(crate) const NUMBER_SET: TerminalTypeId = [SET_T, NUMBER_T];
pub(crate) const BINARY_SET: TerminalTypeId = [SET_T, BINARY_T];
pub(crate) const MAP: TerminalTypeId = [MAP_T, NULL_T];
pub(crate) const LIST: TerminalTypeId = [LIST_T, NULL_T];
/*
  method EcAsString(ec : CMP.EncryptionContext) returns (output : map<string, string>)
  {
    var keys : seq<UTF8.ValidUTF8Bytes> = SortedSets.ComputeSetToOrderedSequence2(ec.Keys, ByteLess);
    var ret : map<string, string> = map[];
    SequenceIsSafeBecauseItIsInMemory(keys);
    for i : u64 = 0 to |keys| as u64 {
      var key :- expect UTF8.Decode(keys[i]);
      var value :- expect UTF8.Decode(ec[keys[i]]);
      ret = ret[key = value];
    }
    return ret;
  }


  function method EncodeTerminal(t : StructuredDataTerminal) : (ret : UTF8.ValidUTF8Bytes)
    //= specification/dynamodb-encryption-client/encrypt-item.md#base-context-value-version-1
    //= type=implication
    //# The value MUST be the UTF8 Encoding of the
    //# [Base 64 encoded](https://www.rfc-editor.org/rfc/rfc4648),
    //# of the concatenation of the bytes `typeID + serializedValue`
    //# where `typeId` is the attribute's [type ID](./ddb-attribute-serialization.md#type-id)
    //# and `serializedValue` is the attribute's value serialized according to
    //# [Attribute Value Serialization](./ddb-attribute-serialization.md#attribute-value-serialization).
    ensures ret == UTF8.Encode(Base64.Encode(t.typeId + t.value)).value
  {
    var base = Base64.Encode(t.typeId + t.value);
    UTF8.Encode(base).value
  }

  function method DecodeTerminal(t : UTF8.ValidUTF8Bytes) : (ret : Result<StructuredDataTerminal, string>)
  {
    var utf8DecodedVal :- UTF8.Decode(t);
    var base64DecodedVal :- Base64.Decode(utf8DecodedVal);
    SequenceIsSafeBecauseItIsInMemory(base64DecodedVal);
    :- Need(|base64DecodedVal| as u64 >= 2, "Invalid serialization of DDB Attribute in encryption context.");
    var typeId = base64DecodedVal[..2 as u32];
    var serializedValue = base64DecodedVal[2 as u32..];
    Success(StructuredDataTerminal(value = serializedValue, typeId = typeId))
  }
*/

// pub(crate) fn write_u16(w: &mut Vec<u8>, data: u16) {
//     w.extend(&data.to_be_bytes());
// }
// pub(crate) fn write_u32(w: &mut Vec<u8>, data: u32) {
//     w.extend(&data.to_be_bytes());
// }
// pub(crate) fn write_u64(w: &mut Vec<u8>, data: u64) {
//     w.extend(&data.to_be_bytes());
// }
