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
    test: EsdkTestVectors.EsdkEncryptTestVector
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

    var optionalValues := encryptionContext + reproducedEncryptionContext;

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
    test: EsdkTestVectors.EsdkDecryptTestVector
  ): Result<JSON, string>
  {
    :- Need(
         && test.algorithmSuiteId.Some?
         && test.frameLength.Some?,
         "test is missing algorithmSuite ID, or frameLength"
       );
    var id := AllAlgorithmSuites.ToHex(test.algorithmSuiteId.value);
    var description := test.description + " " + id;

    var reproducedEncryptionContext
      :- if test.reproducedEncryptionContext.Some? then
           EncryptionContextToJson("reproduced-encryption-context", test.reproducedEncryptionContext.value)
         else
           EncryptionContextToJson("reproduced-encryption-context", map[]);

    :- Need(
         |reproducedEncryptionContext| == 1,
         "Error parsing encryption context"
       );

    var optionalValues := reproducedEncryptionContext;

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
