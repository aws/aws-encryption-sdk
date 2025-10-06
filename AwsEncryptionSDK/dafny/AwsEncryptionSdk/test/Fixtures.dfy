// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/Index.dfy"
include "../../../../mpl/StandardLibrary/src/Index.dfy"

module Fixtures {
  import opened StandardLibrary
  import UTF8
  import primitivesTypes = AwsCryptographyPrimitivesTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import opened UInt = StandardLibrary.UInt
  import Primitives = AtomicPrimitives
  
  // The following are test resources that exist in tests accounts:

  // THESE ARE TESTING RESOURCES DO NOT USE IN A PRODUCTION ENVIRONMENT
  const keyArn := "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f"
  const hierarchyKeyArn := "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126"
  const mkrKeyArn := "arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297"
  
  const branchKeyStoreName := "KeyStoreDdbTable"
  const logicalKeyStoreName := branchKeyStoreName
  const branchKeyId := "75789115-1deb-4fe3-a2ec-be9e885d1945"
  
  // UTF-8 encoded "aws-crypto-"
  const RESERVED_ENCRYPTION_CONTEXT: UTF8.ValidUTF8Bytes :=
    var s := [ 0x61, 0x77, 0x73, 0x2D, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6F, 0x2D ];
    assert UTF8.ValidUTF8Range(s, 0, 11);
    s

  datatype SmallEncryptionContextVariation = Empty | A | B | AB | BA | C | CD

  method SmallEncryptionContext(v: SmallEncryptionContextVariation)
    returns (encryptionContext: mplTypes.EncryptionContext)
  {
    var keyA :- expect UTF8.Encode("keyA");
    var valA :- expect UTF8.Encode("valA");
    var keyB :- expect UTF8.Encode("keyB");
    var valB :- expect UTF8.Encode("valB");
    var keyC :- expect UTF8.Encode("keyC");
    var valC :- expect UTF8.Encode("valC");
    var keyD :- expect UTF8.Encode("keyD");
    var valD :- expect UTF8.Encode("valD");
    
    match v {
      case Empty =>
        encryptionContext := map[];
      case A =>
        encryptionContext := map[keyA := valA];
      case B =>
        encryptionContext := map[keyB := valB];
      case AB =>
        encryptionContext := map[keyA := valA, keyB := valB];
      case BA =>
        // this is really the same as AB; this lets us test that this is also true at run time
        encryptionContext := map[keyB := valB, keyA := valA];
      case C =>
        encryptionContext := map[keyC := valC];
      case CE =>
        encryptionContext := map[keyC := valC, keyD := valD];
    }
  }

  method GetResrvedECMap()
    returns (encryptionContext: mplTypes.EncryptionContext)
  {
    var reservedKey :- expect UTF8.Encode("aws-crypto-public-key");
    var val :- expect UTF8.Encode("not a real public key");
    
    encryptionContext := map[reservedKey := val];
  }

  method SmallEncryptionContextKeys(v: SmallEncryptionContextVariation)
    returns (encryptionContextKeys: seq<UTF8.ValidUTF8Bytes>)
  {
    var keyA :- expect UTF8.Encode("keyA");
    var keyB :- expect UTF8.Encode("keyB");
    var keyC :- expect UTF8.Encode("keyC");
    var keyD :- expect UTF8.Encode("keyD");
    match v {
      case Empty =>
        encryptionContextKeys := [];
      case A =>
        encryptionContextKeys := [keyA];
      case B =>
        encryptionContextKeys := [keyB];
      case AB =>
        encryptionContextKeys := [keyA, keyB];
      case BA =>
        // this is really the same as AB; this lets us test that this is also true at run time
        encryptionContextKeys := [keyB, keyA];
      case C =>
        encryptionContextKeys := [keyC];
      case CE =>
        encryptionContextKeys := [keyC, keyD];
    }
  }

  method SmallMismatchedEncryptionContex(v: SmallEncryptionContextVariation)
    returns (encryptionContext: mplTypes.EncryptionContext)
  {
    var keyA :- expect UTF8.Encode("keyA");
    var valA :- expect UTF8.Encode("valA");
    var keyB :- expect UTF8.Encode("keyB");
    var valB :- expect UTF8.Encode("valB");
    var keyC :- expect UTF8.Encode("keyC");
    var valC :- expect UTF8.Encode("valC");
    var keyD :- expect UTF8.Encode("keyD");
    var valD :- expect UTF8.Encode("valD");
    match v {
      case Empty =>
        encryptionContext := map[];
      case A =>
        encryptionContext := map[keyA := valB];
      case B =>
        encryptionContext := map[keyB := valA];
      case AB =>
        encryptionContext := map[keyA := valC, keyB := valD];
      case BA =>
        encryptionContext := map[keyB := valA, keyA := valB];
      case C =>
        encryptionContext := map[keyC := valA];
      case CE =>
        encryptionContext := map[keyC := valA, keyD := valB];
    }
  }


  
  method NamespaceAndName(n: nat) returns (namespace: string, name: string)
    requires 0 <= n < 10
    ensures |namespace| < UINT16_LIMIT
    ensures |name| < UINT16_LIMIT
  {
    var s := "child" + [n as char + '0'];
    namespace := s + " Namespace";
    name := s + " Name";
  }
  
  method GenerateKeyPair( keyModulusLength: primitivesTypes.RSAModulusLengthBitsToGenerate )
    returns (keys: primitivesTypes.GenerateRSAKeyPairOutput)
  {
      var cryptoX: primitivesTypes.IAwsCryptographicPrimitivesClient :- expect Primitives.AtomicPrimitives();
      assert cryptoX is Primitives.AtomicPrimitivesClient;
      var crypto := cryptoX as Primitives.AtomicPrimitivesClient;

      keys :- expect crypto.GenerateRSAKeyPair(
      primitivesTypes.GenerateRSAKeyPairInput(
          lengthBits := keyModulusLength
      )
      );
  }
    
}
