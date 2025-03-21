// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"

module {:options "-functionSyntax:4"} WriteEsdkJsonManifests {
  import opened JSON.Values
  import AllAlgorithmSuites
  import AllAlgorithmSuites.Types
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import opened Wrappers
  import UTF8
  import SortedSets
  import Seq
  import StandardLibrary
  import StandardLibrary.String
  import Base64

  import TestVectors
  import KeyDescription
  import EsdkTestVectors

  function EncryptionContextKeysToJson(keys: Option<Types.EncryptionContextKeys>)
    : Result<seq<(string, JSON)>, string>
  {
    if keys.Some? then
      var tmp :- Seq.MapWithResult(
                   bytes =>
                     var key :- UTF8.Decode(bytes);
                     Success(String(key)),
                   keys.value);
      Success([("requiredEncryptionContextKeys", Array(tmp))])
    else
      Success([])
  }

  function EncryptionContextToJson(key: string, m: mplTypes.EncryptionContext)
    : Result<seq<(string, JSON)>, string>
  {
    var keys := SortedSets.ComputeSetToOrderedSequence2(m.Keys, (a, b) => a < b);
    var pairsBytes
      :- Seq.MapWithResult(
           k requires k in m.Keys =>
             var key :- UTF8.Decode(k);
             var value :- UTF8.Decode(m[k]);
             Success((key, String(value))),
           keys);
    Success([(key, Object(pairsBytes))])
  }

  function printJson(j: JSON) : (){()} by method {print j, "\n", "\n"; return ();}

  function {:vcs_split_on_every_assert} EncryptTestVectorToJson(
    test: EsdkTestVectors.EsdkEncryptTestVector,
    version: int
  ): Result<JSON, string>
  {
    :- Need(
         && test.algorithmSuiteId.Some?
         && test.frameLength.Some?,
         "test is missing algorithmSuite ID, or frameLength"
       );
    var id := AllAlgorithmSuites.ToHex(test.algorithmSuiteId.value);

    var encryptionContext
      :- if test.encryptionContext.Some? then
           EncryptionContextToJson("encryption-context", test.encryptionContext.value)
         else
           EncryptionContextToJson("encryption-context", map[]);

    :- Need(
         |encryptionContext| == 1,
         "Error parsing encryption context"
       );

    var reproducedEncryptionContext
      :- if test.reproducedEncryptionContext.Some? then
           EncryptionContextToJson("reproduced-encryption-context", test.reproducedEncryptionContext.value)
         else
           EncryptionContextToJson("reproduced-encryption-context", map[]);

    var optionalValues := if version == 5 then
                            encryptionContext + reproducedEncryptionContext
                          else
                            encryptionContext;

    if version == 4 then
      var test? :- ToV4Test(test, id, optionalValues);
      Success(test?)
    else if version == 5 then
      var test? :- ToV5Test(test, id, optionalValues);
      Success(test?)
    else
      Failure("The Dafny Test Vector Framework can only write manifests with version >= 4.")

  }

  function ToV4Test(test: EsdkTestVectors.EsdkEncryptTestVector, id: string, optionalValues: seq<(string, JSON)>) : Result<JSON, string>
    requires test.algorithmSuiteId.Some? && test.frameLength.Some?
  {
    match test
    case PositiveEncryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_,_,_) =>
      var encrypt? :- KeyDescription.ToJson(test.encryptDescriptions, 2);
      var encrypt := if encrypt?.Array? then encrypt? else Array([encrypt?]);
      Success(Object([
                       ("plaintext", String("small")),
                       ("algorithm", String(id)),
                       ("frame-size", Number(Int(test.frameLength.value as int))),
                       ("master-keys", encrypt),
                       ("cmm", String("Default"))
                     ] + optionalValues))

    case _ =>
      Failure("Only Positive Tests supported :(")
  }

  function ToV5Test(test: EsdkTestVectors.EsdkEncryptTestVector, id: string, optionalValues: seq<(string, JSON)>) : Result<JSON, string>
    requires test.algorithmSuiteId.Some? && test.frameLength.Some?
  {
    match test
    case PositiveEncryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_,_,_) =>
      var encrypt :- KeyDescription.ToJson(test.encryptDescriptions, 3);
      var decrypt :- KeyDescription.ToJson(test.decryptDescriptions, 3);
      var scenario := Object([
                               ("type", String("positive-esdk")),
                               ("plaintext", String("small")),
                               ("description", String(test.description)),
                               ("algorithmSuiteId", String(id)),
                               ("frame-size", Number(Int(test.frameLength.value as int))),
                               ("encryptKeyDescription", encrypt),
                               ("decryptKeyDescription", decrypt)
                             ] + optionalValues);
      Success(Object([
                       ("encryption-scenario", scenario)
                     ]))
    case _ =>
      Failure("Only Positive Tests supported :(")
      // Left here for future reference on how you would start to add negative test vectors
      // match test
      // case PositiveEncryptKeyringVector(_,_,_,_,_,_,_,_,_,_) =>
      //   var encrypt :- KeyDescription.ToJson(test.encryptDescription, 3);
      //   var decrypt :- KeyDescription.ToJson(test.decryptDescription, 3);
      //   Success(Object([
      //                    ("type", String("positive-keyring")),
      //                    ("description", String(description)),
      //                    ("algorithmSuiteId", String(id)),
      //                    ("encryptKeyDescription", encrypt),
      //                    ("decryptKeyDescription", decrypt)
      //                  ] + optionalValues))
      // case PositiveEncryptNegativeDecryptKeyringVector(_,_,_,_,_,_,_,_,_,_,_) =>
      //   var encrypt :- KeyDescription.ToJson(test.encryptDescription, 3);
      //   var decrypt :- KeyDescription.ToJson(test.decryptDescription, 3);
      //   Success(Object([
      //                    ("type", String("negative-decrypt-keyring")),
      //                    ("description", String(description)),
      //                    ("decryptErrorDescription", String(test.decryptErrorDescription)),
      //                    ("algorithmSuiteId", String(id)),
      //                    ("encryptKeyDescription", encrypt),
      //                    ("decryptKeyDescription", decrypt)
      //                  ] + optionalValues))
      // case NegativeEncryptKeyringVector(_,_,_,_,_,_,_,_,_) =>
      //   var keyDescription :- KeyDescription.ToJson(test.keyDescription, 3);
      //   Success(Object([
      //                    ("type", String("negative-encrypt-keyring")),
      //                    ("description", String(description)),
      //                    ("errorDescription", String(test.errorDescription)),
      //                    ("algorithmSuiteId", String(id)),
      //                    ("keyDescription", keyDescription)
      //                  ] + optionalValues))
  }

  function OptionalBytes(key: string, secret: Option<Types.Secret>)
    : seq<(string, JSON)>
  {
    if secret.Some? then
      var base64 := Base64.Encode(secret.value);
      [(key, String(base64))]
    else
      []
  }

  function {:vcs_split_on_every_assert} DecryptTestVectorToJson(
    test: EsdkTestVectors.EsdkDecryptTestVector,
    version: int
  ): Result<JSON, string>
  {
    :- Need(
         && test.algorithmSuiteId.Some?
         && test.frameLength.Some?,
         "test is missing algorithmSuite ID, or frameLength"
       );
    var id := AllAlgorithmSuites.ToHex(test.algorithmSuiteId.value);

    var ec := if version == 5 then "reproduced-encryption-context" else "encryption-context";

    var reproducedEncryptionContext
      :- if test.reproducedEncryptionContext.Some? then
           EncryptionContextToJson(ec, test.reproducedEncryptionContext.value)
         else
           EncryptionContextToJson(ec, map[]);

    :- Need(
         |reproducedEncryptionContext| == 1,
         "Error parsing encryption context"
       );

    var optionalValues := reproducedEncryptionContext;

      if version == 4 then
        ToV4DecryptTest(test, id, optionalValues)
      else if version == 5 then
        ToV5DecryptTest(test, id, optionalValues)
      else
        Failure("The Dafny Test Vector Framework can only write manifests with version >= 4.")

  }

  function ToV4DecryptTest(test: EsdkTestVectors.EsdkDecryptTestVector, id: string, optionalValues: seq<(string, JSON)>) : Result<JSON, string>
    requires test.algorithmSuiteId.Some? && test.frameLength.Some?
  {
    match test
    case PositiveDecryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_) =>
      var decrypt? :- KeyDescription.ToJson(test.decryptDescriptions, 2);
      var decrypt := if decrypt?.Array? then decrypt? else Array([decrypt?]);
      var fileName := String("file://" + test.plaintextPath);
      var fileLoc := Object([("plaintext", fileName)]);
      var scenario := Object([
                               ("ciphertext", String("file://ciphertexts/" + test.id)),
                               ("result", Object([("output", fileLoc)])),
                               ("master-keys", decrypt),
                               ("description", String(test.description)),
                               ("cmm", String("Default"))
                             ] + optionalValues);
      Success(scenario)
    case _ =>
      Failure("Only Positive Tests supported :(")
  }

  function ToV5DecryptTest(test: EsdkTestVectors.EsdkDecryptTestVector, id: string, optionalValues: seq<(string, JSON)>) : Result<JSON, string>
    requires test.algorithmSuiteId.Some? && test.frameLength.Some?
  {
    match test
    case PositiveDecryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_) =>
      var decrypt :- KeyDescription.ToJson(test.decryptDescriptions, 3);
      var scenario := Object([
                               ("type", String("positive-esdk")),
                               ("ciphertext", String("file://ciphertexts/" + test.id)),
                               ("result", String("file://" + test.plaintextPath)),
                               ("algorithmSuiteId", String(id)),
                               ("frame-size", Number(Int(test.frameLength.value as int))),
                               ("description", String(test.description)),
                               ("decryptKeyDescription", decrypt)
                             ] + optionalValues);
      Success(Object([
                       ("decryption-scenario", scenario)
                     ]))
    case _ =>
      Failure("Only Positive Tests supported :(")
  }
}
