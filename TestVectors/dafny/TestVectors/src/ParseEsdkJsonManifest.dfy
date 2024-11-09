// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"
include "EsdkTestVectors.dfy"

module {:options "-functionSyntax:4"} ParseEsdkJsonManifest {
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import JSON.API
  import FileIO
  import opened JSON.Values
  import JSON.Errors
  import opened Wrappers
  import UTF8
  import Seq
  import opened StandardLibrary.UInt
  import BoundedInts
  import opened JSONHelpers
  import opened TestVectors
  import HexStrings
  import Base64
  import CompleteVectors
  import KeyVectors
  import KeyVectorsTypes = AwsCryptographyMaterialProvidersTestVectorKeysTypes
  import ParseJsonManifests
  import AlgorithmSuites
  import opened EsdkTestVectors
  import EsdkManifestOptions

  const ciphertextJsonKey := "ciphertext"
  const masterKeysJsonKey := "master-keys"
  const encryptKeyDescription := "encryptKeyDescription"
  const decryptKeyDescription := "decryptKeyDescription"
  const decryptionMethodJsonKey := "decryption-method"
  const plaintextJsonKey := "plaintext"
  const frameSizeJsonKey := "frame-size"
  const encryptionContextJsonKey := "encryption-context"
  const reproducedEncryptionContextJsonKey := "reproduced-encryption-context"

  function BuildDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    version: SupportedDecryptVersion,
    keys: KeyVectors.KeyVectorsClient,
    obj: seq<(string, JSON)>
  ) : Result<seq<EsdkDecryptTestVector>, string>
    requires op.Decrypt?
  {
    if |obj| == 0 then
      Success([])
    else
      var tail :- BuildDecryptTestVector(op, version, keys, obj[1..]);
      var encryptVector :- ToDecryptTestVectors(op, version, keys, obj[0].0, obj[0].1);
      Success([ encryptVector ] + tail)
  } by method {
    // This function ideally would be`{:tailrecursion}`
    // but it is not simple to here is a method
    // so that it does not explode with huge numbers of tests.
    var i: nat := |obj|;
    var vectors := [];

    while i != 0
      decreases i
      invariant Success(vectors) == BuildDecryptTestVector(op, version, keys, obj[i..])
    {
      i := i - 1;
      var test := ToDecryptTestVectors(op, version, keys, obj[i].0, obj[i].1);
      if test.Failure? {
        ghost var j := i;
        while j != 0
          decreases j
          invariant Failure(test.error) == BuildDecryptTestVector(op, version, keys, obj[j..])
        {
          j := j - 1;
        }
        return Failure(test.error);
      }

      vectors := [test.value] + vectors;
    }

    return Success(vectors);
  }


  function ToDecryptTestVectors(
    op: EsdkManifestOptions.ManifestOptions,
    version: SupportedDecryptVersion,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    json: JSON
  ) : Result<EsdkDecryptTestVector, string>
    requires op.Decrypt?
  {
    :- Need(json.Object?, "Vector is not an object");
    var obj := json.obj;

    match version
    case 3 => V3ToDecryptTestVector(op, keys, name, obj, version)
    case _ => Failure("Version not supported")

    // case 1 =>
    //   var plaintextPath :- GetPath(ciphertextJsonKey, obj);
    //   Success(PositiveDecryptTestVector(
    //             name := name,
    //             version := version,
    //             manifestPath := op.manifestPath,
    //             plaintextPath := plaintextPath,
    //             ciphertextPath := ciphertextPath,
    //             decryptDescriptions := decryptDescriptions,
    //             decryptionMethod := decryptionMethod
    //           ))
    // case 2 =>
    //   var result :- GetObject("result", obj);
    //   :- Need(|result| == 1 && Result?(result[0].0), "Unexpected result");
    //   match result[0].0
    //   case "output" =>
    //     var output :- GetObject("output", result);
    //     var plaintextPath :- GetPath("plaintext", output);

    //     Success(PositiveDecryptTestVector(
    //               name := name,
    //               version := version,
    //               manifestPath := op.manifestPath,
    //               plaintextPath := plaintextPath,
    //               ciphertextPath := ciphertextPath,
    //               decryptDescriptions := decryptDescriptions,
    //               decryptionMethod := decryptionMethod
    //             ))
    //   case "error" =>
    //     var output :- GetObject("error", result);
    //     var errorDescription :- GetString("error-description", output);

    //     Success(NegativeDecryptTestVector(
    //               name := name,
    //               version := version,
    //               manifestPath := op.manifestPath,
    //               errorDescription := errorDescription,
    //               ciphertextPath := ciphertextPath,
    //               decryptDescriptions := decryptDescriptions,
    //               decryptionMethod := decryptionMethod
    //             ))

  }

  function BuildEncryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    version: SupportedEncryptVersion,
    keys: KeyVectors.KeyVectorsClient,
    obj: seq<(string, JSON)>
  ) : Result<seq<EsdkEncryptTestVector>, string>
    requires op.Encrypt?
  {
    if |obj| == 0 then
      Success([])
    else
      var tail :- BuildEncryptTestVector(op, version, keys, obj[1..]);
      var encryptVector :- ToEncryptTestVector(op, version, keys, obj[0].0, obj[0].1);
      Success([ encryptVector ] + tail)
  } by method {
    // This function ideally would be`{:tailrecursion}`
    // but it is not simple to here is a method
    // so that it does not explode with huge numbers of tests.
    var i: nat := |obj|;
    var vectors := [];

    while i != 0
      decreases i
      invariant Success(vectors) == BuildEncryptTestVector(op, version, keys, obj[i..])
    {
      i := i - 1;
      var test := ToEncryptTestVector(op, version, keys, obj[i].0, obj[i].1);
      if test.Failure? {
        ghost var j := i;
        while j != 0
          decreases j
          invariant Failure(test.error) == BuildEncryptTestVector(op, version, keys, obj[j..])
        {
          j := j - 1;
        }
        return Failure(test.error);
      }

      vectors := [test.value] + vectors;
    }

    return Success(vectors);
  }


  function ToEncryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    version: SupportedEncryptVersion,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    json: JSON
  ) : Result<EsdkEncryptTestVector, string>
    requires op.Encrypt?
  {
    :- Need(json.Object?, "EncryptTestVector not an object");
    var obj := json.obj;

    match version
    // case 1 => V1ToEncryptTestVector(op, keys, name, obj)
    case 5 => V5ToEncryptTestVector(op, keys, name, obj, version)
    case _ => Failure("Version not supported")
  }

  // function V1ToEncryptTestVector(
  //   op: EsdkManifestOptions.ManifestOptions,
  //   keys: KeyVectors.KeyVectorsClient,
  //   name: string,
  //   obj: seq<(string, JSON)>
  // ) : Result<EsdkEncryptTestVector, string>
  //   requires op.Encrypt?
  // {
  //   var plaintextLoc :- GetString(plaintextJsonKey, obj);
  //   var algorithmSuite :- ParseJsonManifests.GetAlgorithmSuiteInfo(obj);
  //   :- Need(algorithmSuite.id.ESDK?, "Unsupported algorithmSuite");
  //   var frameLength :- GetOptionalPositiveLong(frameSizeJsonKey, obj);
  //   var encryptionContext :- SmallObjectToStringStringMap(encryptionContextJsonKey, obj);
  //   var masterKeyArray :- GetArray(masterKeysJsonKey, obj);
  //   var keyDescriptions :- GetKeyDescriptions(masterKeyArray, keys);

  //   Success(PositiveEncryptTestVector(
  //             name := name,
  //             version := 1,
  //             manifestPath := op.manifestPath,
  //             decryptManifestPath := op.decryptManifestOutput,
  //             plaintextPath := plaintextLoc,
  //             encryptDescriptions := keyDescriptions,
  //             decryptDescriptions := keyDescriptions,
  //             frameLength := frameLength,
  //             algorithmSuiteId := Some(algorithmSuite)
  //           ))
  // }

  function V5ToEncryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedEncryptVersion
  ) : Result<EsdkEncryptTestVector, string>
    requires op.Encrypt?
  {
    var scenarioString := "encryption-scenario";
    var scenario :- GetObject(scenarioString, obj);

    var typeString := "type";
    var typ :- GetString(typeString, scenario);

    var plaintextLoc :- GetString(plaintextJsonKey, scenario);
    var algorithmSuite :- ParseJsonManifests.GetAlgorithmSuiteInfo(scenario);
    :- Need(algorithmSuite.id.ESDK?, "Unsupported algorithmSuiteId");
    var frameLength :- GetOptionalPositiveLong(frameSizeJsonKey, scenario);

    var encryptionContextStrings :- SmallObjectToStringStringMap(encryptionContextJsonKey, scenario);
    var encryptionContext :- utf8EncodeMap(encryptionContextStrings);
    var reproducedEncryptionContextString :- SmallObjectToStringStringMap(reproducedEncryptionContextJsonKey, scenario);
    var reproducedEncryptionContext :- utf8EncodeMap(reproducedEncryptionContextString);
    var description :- GetString("description", scenario);

    match typ
    case "positive-esdk" =>
      var encryptKeyDescription :- ParseJsonManifests.GetKeyDescription(keys, encryptKeyDescription, scenario);
      var decryptKeyDescription :- ParseJsonManifests.GetKeyDescription(keys, decryptKeyDescription, scenario);
      Success(PositiveEncryptTestVector(
                name := name,
                version := version,
                manifestPath := op.manifestPath,
                decryptManifestPath := op.decryptManifestOutput,
                plaintextPath := plaintextLoc,
                encryptDescriptions := encryptKeyDescription,
                decryptDescriptions := decryptKeyDescription,
                encryptionContext := Some(encryptionContext),
                reproducedEncryptionContext := Some(reproducedEncryptionContext),
                commitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
                frameLength := frameLength,
                algorithmSuiteId := Some(algorithmSuite),
                description := description
              ))
    case _ => Failure("Unsupported ESDK TestVector type: " + typ)

  }

  function V3ToDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedDecryptVersion
  ) : Result<EsdkDecryptTestVector, string>
    requires op.Decrypt?
  {
    var scenarioString := "decryption-scenario";
    var scenario :- GetObject(scenarioString, obj);

    var typeString := "type";
    var typ :- GetString(typeString, scenario);

    var ciphertextLoc :- GetString(ciphertextJsonKey, scenario);
    var algorithmSuite :- ParseJsonManifests.GetAlgorithmSuiteInfo(scenario);
    :- Need(algorithmSuite.id.ESDK?, "Unsupported algorithmSuiteId");
    var frameLength :- GetOptionalPositiveLong(frameSizeJsonKey, scenario);

    var reproducedEncryptionContextStrings :- SmallObjectToStringStringMap(reproducedEncryptionContextJsonKey, scenario);
    var reproducedEncryptionContext :- utf8EncodeMap(reproducedEncryptionContextStrings);
    var description :- GetString("description", scenario);
    var result :- GetString("result", scenario);
    :- Need(
         && "file://" < ciphertextLoc
         && "file://" < result,
         "Invalid file prefix in test vector"
       );

    match typ
    case "positive-esdk" =>
      var decryptKeyDescription :- ParseJsonManifests.GetKeyDescription(keys, decryptKeyDescription, scenario);
      Success(PositiveDecryptTestVector(
                name := name,
                version := version,
                manifestPath := op.manifestPath,
                ciphertextPath := ciphertextLoc[|FILE_PREPEND|..],
                plaintextPath := result[|FILE_PREPEND|..],
                reproducedEncryptionContext := Some(reproducedEncryptionContext),
                decryptDescriptions := decryptKeyDescription,
                frameLength := frameLength,
                algorithmSuiteId := Some(algorithmSuite),
                description := description,
                decryptionMethod := DecryptionMethod.OneShot
              ))
    case _ => Failure("Unsupported ESDK TestVector type: " + typ)
  }

  function GetKeyDescriptions(keyArray: seq<JSON>, keys: KeyVectors.KeyVectorsClient)
    : Result<seq<KeyVectorsTypes.KeyDescription>, string>
  {
    if |keyArray| == 0 then
      Success([])
    else
      var currKey := keyArray[0];
      :- Need(currKey.Object?, "Not an object");
      var encryptStr :- API.Serialize(currKey).MapFailure((e: Errors.SerializationError) => e.ToString());
      var encryptDecryptKeyDescription :- keys
                                          .GetKeyDescription(KeyVectorsTypes.GetKeyDescriptionInput(
                                                               json := encryptStr
                                                             ))
                                          .MapFailure(ParseJsonManifests.ErrorToString);
      var tail :- GetKeyDescriptions(keyArray[1..], keys);
      Success([encryptDecryptKeyDescription.keyDescription] + tail)
  }

  function GetPath(key: string, obj: seq<(string, JSON)>)
    : Result<string, string>
  {
    var path :- GetString(key, obj);
    :- Need(FILE_PREPEND < path, "Received Invalid location for plaintext or ciphertext.");
    Success(path[|FILE_PREPEND|..])
  }

  const FILE_PREPEND := "file://"

  predicate Result?(key: string)
  {
    || key == "output"
    || key == "error"
  }

  // function DecryptVectorToJson(
  //   keys: KeyVectors.KeyVectorsClient,
  //   vector: EsdkDecryptTestVector
  // ) : Result<(string, Values.JSON), string>
  // {
  //   var optionalElements
  //     := []
  //       + if vector.decryptionMethod.OneShot? then
  //          []
  //        else
  //          assert vector.decryptionMethod.StreamingUnsignedOnly?;
  //          [("decryption-method", Values.String("streaming-unsigned-only"))];

  //   var decryptDescriptions :- Seq.MapWithResult(
  //     d =>
  //     var description :- keys.SerializeKeyDescription(
  //       KeyVectorsTypes.SerializeKeyDescriptionInput(
  //         keyDescription := d
  //       )
  //     ).MapFailure(e => "OMFG");
  //     API.Deserialize(description.json).MapFailure(( e: Errors.DeserializationError ) => e.ToString())
  //     ,
  //     vector.decryptDescriptions
  //   );
  //   Success(
  //     match vector
  //   case PositiveDecryptTestVector(_,_,_,_,_,_,_,_,_) =>
  //     (vector.name, Values.Object([
  //      ("ciphertext", Values.String(FILE_PREPEND + vector.ciphertextPath)),
  //      ("master-keys", Values.Array(decryptDescriptions)),
  //      ("result", Values.Object([
  //      ("output", Values.Object([
  //      ("plaintext", Values.String(FILE_PREPEND + vector.plaintextPath))
  //      ]))
  //      ]))
  //      ] + optionalElements
  //      ))
  //   case NegativeDecryptTestVector(_,_,_,_,_,_,_,_,_) =>
  //     (vector.name, Values.Object([
  //      ("ciphertext", Values.String(FILE_PREPEND + vector.ciphertextPath)),
  //      ("master-keys", Values.Array(decryptDescriptions)),
  //      ("result", Values.Object([
  //      ("error", Values.Object([
  //      ("error-description", Values.String(vector.errorDescription))
  //      ]))
  //      ]))
  //      ] + optionalElements
  //      ))
  //   )
  // }
}
