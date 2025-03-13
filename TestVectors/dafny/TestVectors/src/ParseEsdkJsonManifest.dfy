// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"
include "EsdkTestVectors.dfy"

module {:options "-functionSyntax:4"} ParseEsdkJsonManifest {
  import Types = AwsCryptographyEncryptionSdkTypes
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
  const buildTestVectorError := "Error other than negative test vector found thrown"
  const negativeTestVectorFound := "Negative test vector found; not supported yet."

  function {:vcs_split_on_every_assert} BuildDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    clientName: string,
    clientVersion: string,
    version: SupportedDecryptVersion,
    keys: KeyVectors.KeyVectorsClient,
    obj: seq<(string, JSON)>
  ) : Result<seq<EsdkDecryptTestVector>, string>
    requires op.Decrypt?
  {
    if |obj| == 0 then
      Success([])
    else
      var tail :- BuildDecryptTestVector(op, clientName, clientVersion, version, keys, obj[1..]);
      var encryptVector? := ToDecryptTestVectors(op, clientName, clientVersion, version, keys, obj[0].0, obj[0].1);
      if encryptVector?.Success? then
        Success([ encryptVector?.value ] + tail)
      else
        :- Need(encryptVector?.error == negativeTestVectorFound, buildTestVectorError);
        Success(tail)
  } by method {
    // This function ideally would be`{:tailrecursion}`
    // but it is not simple to here is a method
    // so that it does not explode with huge numbers of tests.
    var i: nat := |obj|;
    var vectors := [];

    while i != 0
      decreases i
      invariant Success(vectors) == BuildDecryptTestVector(op, clientName, clientVersion, version, keys, obj[i..])
    {
      i := i - 1;
      var test := ToDecryptTestVectors(op, clientName, clientVersion, version, keys, obj[i].0, obj[i].1);
      if test.Failure? && test.error != negativeTestVectorFound {
        assert Failure(buildTestVectorError) == BuildDecryptTestVector(op, clientName, clientVersion, version, keys, obj[i..]);
        ghost var j: nat := i;
        while j != 0
          decreases j
          invariant Failure(buildTestVectorError) == BuildDecryptTestVector(op, clientName, clientVersion, version, keys, obj[j..])
        {
          j := j - 1;
          assert obj[j..][1..] == obj[j+1..];
        }
        assert Failure(buildTestVectorError) == BuildDecryptTestVector(op, clientName, clientVersion, version, keys, obj);
        return Failure(buildTestVectorError);
      }

      if test.Success? {
        vectors := [test.value] + vectors;
      }
      if test.Failure? && test.error == negativeTestVectorFound {
        vectors := vectors;
      }
    }
    return Success(vectors);
  }


  function {:vcs_split_on_every_assert} ToDecryptTestVectors(
    op: EsdkManifestOptions.ManifestOptions,
    clientName: string,
    clientVersion: string,
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
    case 5 =>
      :- Need(op.Decrypt?, "Err parsing manifest expected Decrypt");
      V5ToDecryptTestVector(op, keys, name, obj, version)
    case 4 =>
      :- Need(op.Decrypt?, "Err parsing manifest; expected Decrypt");
      :- Need((clientName == "ESDK-NET") || (clientName == "aws-encryption-sdk-dafny"), "Err; Version 4 manifest only supported for NET");
      V4ToDecryptTestVector(op, keys, name, obj, version)
    case 2 =>
      // Case 2 Needs negative test vectors..
      :- Need(op.Decrypt?, "Err parsing manifest expected Decrypt");
      V2ToDecryptTestVector(op, keys, name, obj, version)
    case 1 =>
      :- Need(op.Decrypt?, "Err parsing manifest expected Decrypt");
      V1ToDecryptTestVector(op, keys, name, obj, version)
    case _ => Failure("Version not supported\n")
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
    case 4 => V4ToEncryptTestVector(op, keys, name, obj, version)
    case _ => Failure("Version not supported")
  }

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
                id := Some(name),
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

  function V4ToEncryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedEncryptVersion
  ) : Result<EsdkEncryptTestVector, string>
    requires op.Encrypt?
  {
    var scenario := obj; 
    
    var plaintextLoc :- GetString(plaintextJsonKey, scenario);
    var algorithmSuite :- GetAlgorithmInfo(scenario);
    :- Need(algorithmSuite.id.ESDK?, "Unsupported algorithmSuiteId");
    var frameLength :- GetOptionalPositiveLong(frameSizeJsonKey, scenario);
    
    var masterKeys :- GetArray("master-keys", scenario);
    var keyDescriptions :- GetKeyDescriptions(masterKeys, keys);
    var keyDescription :- ToMultiKeyDescription(keyDescriptions);
    
    var encryptionContextStrings :- SmallObjectToStringStringMap(encryptionContextJsonKey, scenario);
    var encryptionContext :- utf8EncodeMap(encryptionContextStrings);

    var cmm? :- GetString("cmm", scenario);
    :- Need(cmm? == "Default", "Only Default CMM supported on encrypt-manifest version 4.");
    
    Success(PositiveEncryptTestVector(
              id := Some(name),
              version := version,
              manifestPath := op.manifestPath,
              decryptManifestPath := op.decryptManifestOutput,
              plaintextPath := plaintextLoc,
              encryptDescriptions := keyDescription,
              decryptDescriptions := keyDescription,
              encryptionContext := Some(encryptionContext),
              commitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
              frameLength := frameLength,
              algorithmSuiteId := Some(algorithmSuite),
              description := name
            ))
  }
  
  function GetAlgorithmInfo(
    obj: seq<(string, JSON)>
  ) : Result<mplTypes.AlgorithmSuiteInfo, string>
  {
    var algorithmSuiteHex :- GetString("algorithm", obj);
    :- Need(HexStrings.IsLooseHexString(algorithmSuiteHex), "Not hex encoded binary");
    var binaryId := HexStrings.FromHexString(algorithmSuiteHex);
    // TODO change this to use AlgorithmSuiteInfoByHexString
    AlgorithmSuites
    .GetAlgorithmSuiteInfo(binaryId)
    .MapFailure(e => "Invalid Suite:" + algorithmSuiteHex)
  }

  function V1ToDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedDecryptVersion
  ) : Result<EsdkDecryptTestVector, string>
    requires op.Decrypt?
  {
    var plaintextLoc :- GetString("plaintext", obj);
    var ciphertextLoc :- GetString("ciphertext", obj);
    :- Need(
         && "file://" < ciphertextLoc
         && "file://" < plaintextLoc,
         "Invalid file prefix in test vector"
       );
    var masterKeys :- GetArray("master-keys", obj);
    var keyDescriptions :- GetKeyDescriptions(masterKeys, keys);
    var keyDescription :- ToMultiKeyDescription(keyDescriptions);

    Success(PositiveV1OrV2DecryptTestVector(
              id := name,
              version := version,
              manifestPath := op.manifestPath,
              ciphertextPath := ciphertextLoc[|FILE_PREPEND|..],
              plaintextPath := plaintextLoc[|FILE_PREPEND|..],
              decryptDescriptions := keyDescription,
              frameLength := None,
              algorithmSuiteId := None,
              description :=  name,
              decryptionMethod := DecryptionMethod.OneShot
            ))
  }

  function V2ToDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedDecryptVersion
  ) : Result<EsdkDecryptTestVector, string>
    requires op.Decrypt?
  {
    var resultLoc :- GetObject("result", obj);
    var errorLoc? := GetObject("error", resultLoc);
    // TODO build negative test vectors appropriately instead of not
    // accepting any.
    // Return early if we have a negative test vector
    if errorLoc?.Success? then
      Failure(negativeTestVectorFound)
    else

      var outputLoc :- GetObject("output", resultLoc);

      var plaintextLoc :- GetString("plaintext", outputLoc);
      var ciphertextLoc :- GetString("ciphertext", obj);
      :- Need(
           && "file://" < ciphertextLoc
           && "file://" < plaintextLoc,
           "Invalid file prefix in test vector"
         );
      var masterKeys :- GetArray("master-keys", obj);
      var keyDescriptions :- GetKeyDescriptions(masterKeys, keys);
      var keyDescription :- ToMultiKeyDescription(keyDescriptions);

      var newKeyDescription := if keyDescription.RSA? then
          KeyVectorsTypes.RSA(
            KeyVectorsTypes.RawRSA(
              keyId := "rsa-4096-private",
              providerId := keyDescription.RSA.providerId,
              padding := keyDescription.RSA.padding
            )
          )
        else
          keyDescription;

      Success(PositiveV1OrV2DecryptTestVector(
                id := name,
                version := version,
                manifestPath := op.manifestPath,
                ciphertextPath := ciphertextLoc[|FILE_PREPEND|..],
                plaintextPath := plaintextLoc[|FILE_PREPEND|..],
                decryptDescriptions := newKeyDescription,
                frameLength := None,
                algorithmSuiteId := None,
                description :=  name,
                decryptionMethod := DecryptionMethod.OneShot
              ))
  }

  function V5ToDecryptTestVector(
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
                id := name,
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

  function V4ToDecryptTestVector(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    name: string,
    obj: seq<(string, JSON)>,
    version: SupportedDecryptVersion
  ) : Result<EsdkDecryptTestVector, string>
    requires op.Decrypt?
  {
    var resultLoc :- GetObject("result", obj);

    var outputLoc :- GetObject("output", resultLoc);
    var plaintextLoc :- GetString("plaintext", outputLoc);

    var ciphertextLoc :- GetString("ciphertext", obj);
    :- Need(
         && "file://" < ciphertextLoc
         && "file://" < plaintextLoc,
         "Invalid file prefix in test vector"
       );

    var masterKeys :- GetArray("master-keys", obj);
    var keyDescriptions :- GetKeyDescriptions(masterKeys, keys);
    var keyDescription :- ToMultiKeyDescription(keyDescriptions);

    var cmm :- GetString("cmm", obj);
    var encryptionContextStrings :- SmallObjectToStringStringMap("encryption-context", obj);
    var encryptionContext :- utf8EncodeMap(encryptionContextStrings);

    Success(PositiveV4DecryptTestVector(
              id := name,
              version := version,
              manifestPath := op.manifestPath,
              ciphertextPath := ciphertextLoc[|FILE_PREPEND|..],
              plaintextPath := plaintextLoc[|FILE_PREPEND|..],
              decryptDescriptions := keyDescription,
              reproducedEncryptionContext := Some(encryptionContext),
              frameLength := None,
              algorithmSuiteId := None,
              description :=  name,
              decryptionMethod := DecryptionMethod.OneShot,
              cmm := cmm,
              retryPolicy := op.retryPolicy
            ))
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

  function ToMultiKeyDescription(keyDescriptions: seq<KeyVectorsTypes.KeyDescription>)
    : Result<KeyVectorsTypes.KeyDescription, string>
  {
    if |keyDescriptions| == 1 then
      Success(keyDescriptions[0])
    else
      :- Need(|keyDescriptions| > 1, "Received invalid key description length");
      Success(KeyVectorsTypes.KeyDescription.Multi(
                KeyVectorsTypes.MultiKeyring(
                  generator := Some(keyDescriptions[0]),
                  childKeyrings := keyDescriptions[1..]
                )
              ))
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
