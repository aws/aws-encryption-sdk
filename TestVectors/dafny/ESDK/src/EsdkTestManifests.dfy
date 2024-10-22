// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"
include "ParseEsdkJsonManifest.dfy"
include "EsdkTestVectors.dfy"

module {:options "-functionSyntax:4"} EsdkTestManifests {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import opened Wrappers
  import TestVectors
  import FileIO
  import JSON.API
  import JSON.Values
  import JSON.Errors
  import Seq
  import BoundedInts
  import opened StandardLibrary.UInt
  import opened JSONHelpers
  import ParseJsonManifests
  import ParseEsdkJsonManifest
  import KeyVectors
  import KeyVectorsTypes = AwsCryptographyMaterialProvidersTestVectorKeysTypes
  import AtomicPrimitives
  import UTF8

  import EsdkManifestOptions
  import opened EsdkTestVectors

  method StartDecryptVectors(
    op: EsdkManifestOptions.ManifestOptions
  )
    returns (output: Result<seq<BoundedInts.byte>, string>)
    requires op.Decrypt?
    requires 0 < |op.manifestPath|
    requires Seq.Last(op.manifestPath) == '/'
  {
    var decryptManifest :- expect GetManifest(op.manifestPath, "manifest.json");
    :- Need(decryptManifest.DecryptManifest?, "Not a decrypt manifest");

    var decryptVectors :- ParseEsdkJsonManifest.BuildDecryptTestVector(
      op,
      decryptManifest.version,
      decryptManifest.keys,
      decryptManifest.jsonTests
    );
    output := TestDecrypts(decryptManifest.keys, decryptVectors);
  }

  predicate TestDecryptVector?(v: EsdkDecryptTestVector)
  {
    && v.decryptionMethod.OneShot?
  }

  method TestDecrypts(
    keys: KeyVectors.KeyVectorsClient,
    vectors: seq<EsdkDecryptTestVector>
  )
    returns (manifest: Result<seq<BoundedInts.byte>, string>)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()
  {
    print "\n=================== Starting ", |vectors|, " Decrypt Tests =================== \n\n";

    var hasFailure := false;
    var skipped := 0;

    for i := 0 to |vectors|
    {
      var vector := vectors[i];
      if TestDecryptVector?(vector) {

        var pass := EsdkTestVectors.TestDecrypt(keys, vector);
        if !pass {
          hasFailure := true;
        }
      } else {
        skipped := skipped + 1;
        print "\nSKIP===> ", vector.name, "\n";
      }

    }
    print "\n=================== Completed ", |vectors|, " Decrypt Tests =================== \n\n";

    if 0 < skipped {
      print "Skipped: ", skipped, "\n";
    }

    expect !hasFailure;

    // var maybeManifest := ToJSONEsdkDecryptManifest(tests);

    manifest := Success([]);
  }

  method StartEncryptVectors(
    op: EsdkManifestOptions.ManifestOptions
  )
    returns (output: Result<seq<EsdkDecryptTestVector>, string>)
    requires op.Encrypt?
    requires 0 < |op.manifestPath|
  {
    return Failure("Implment me");
    // var encryptManifest :- GetManifest(op.manifestPath, op.manifest);
    // :- Need(encryptManifest.EncryptManifest?, "Not a encrypt manifest");

    // var encryptVectors :- ParseEsdkJsonManifest.BuildEncryptTestVector(
    //   op,
    //   encryptManifest.version,
    //   encryptManifest.keys,
    //   encryptManifest.jsonTests
    // );

    // var keysJsonFileName := "keys.json";
    // // Write the keys to disk
    // var keysJsonBytes :- API.Serialize(encryptManifest.keys.config.keysJson)
    // .MapFailure(( e: Errors.SerializationError ) => e.ToString());
    // var _ :- WriteVectorsFile(op.decryptManifestOutput + keysJsonFileName, keysJsonBytes);

    // var p :- expect Primitives.AtomicPrimitives();
    // var plaintext := map[];
    // for i := 0 to |encryptManifest.plaintext|
    // {
    //   var (name, length) := encryptManifest.plaintext[i];
    //   var data :- expect p.GenerateRandomBytes(
    //     Primitives.Types.GenerateRandomBytesInput(
    //       length := length
    //     ));
    //   // Write the plaintext to disk.
    //   print op.decryptManifestOutput + plaintextPathRoot + name, "\n\n";
    //   var _ :- WriteVectorsFile(op.decryptManifestOutput + plaintextPathRoot + name, data);
    //   plaintext := plaintext + map[ name := data ];
    // }

    // output := TestEncrypts(plaintext, encryptManifest.keys, encryptVectors);

    // if output.Success? {
    //   var testsJson :- Seq.MapWithResult(v => ParseEsdkJsonManifest.DecryptVectorToJson(encryptManifest.keys, v), output.value);
    //   var decryptManifestJson := Values.Object([
    //                                              ("manifest", Values.Object([
    //                                               ("type", Values.String("awses-decrypt")),
    //                                               ("version", Values.Number(Values.Int(2)))
    //                                               ])),
    //                                              ("client", Values.Object([
    //                                               ("name", Values.String("aws/aws-encryption-sdk-dafny")),
    //                                               ("version", Values.String("need-some-way-to-get-version"))
    //                                               ])),
    //                                              ("keys", Values.String(ParseEsdkJsonManifest.FILE_PREPEND + keysJsonFileName)),
    //                                              ("tests", Values.Object(testsJson))
    //                                            ]);
    //   var decryptManifestJsonBytes :- API.Serialize(decryptManifestJson)
    //   .MapFailure(( e: Errors.SerializationError ) => e.ToString());
    //   var _ :- WriteVectorsFile(op.decryptManifestOutput + "manifest.json", decryptManifestJsonBytes);
    // }

  }

  // predicate TestEncryptVector?(vector: EsdkEncryptTestVector)
  // {
  //   && (vector.frameLength.Some? ==> Types.IsValid_FrameLength(vector.frameLength.value))
  // }

  // method TestEncrypts(
  //   plaintexts: map<string, seq<uint8>>,
  //   keys: KeyVectors.KeyVectorsClient,
  //   vectors: seq<EsdkEncryptTestVector>
  // )
  //   returns (manifest: Result<seq<EsdkDecryptTestVector>, string>)
  //   requires keys.ValidState()
  //   modifies keys.Modifies
  //   ensures keys.ValidState()
  // {
  //   print "\n=================== Starting ", |vectors|, " Encrypt Tests =================== \n\n";

  //   var hasFailure := false;
  //   var skipped := [];
  //   var decryptVectors := [];

  //   for i := 0 to |vectors|
  //   {
  //     var vector := vectors[i];
  //     if TestEncryptVector?(vector) {
  //       var pass := EsdkTestVectors.TestEncrypt(plaintexts, keys, vector);
  //       if !pass.output {
  //         hasFailure := true;
  //       } else if pass.vector.Some? {
  //         decryptVectors := decryptVectors + [pass.vector.value];
  //       }
  //     } else {
  //       skipped := skipped + [vector.name + "\n"];
  //       print "\nSKIP===> ", vector.name, "\n";
  //     }
  //   }
  //   print "\n=================== Completed ", |vectors|, " Encrypt Tests =================== \n\n";

  //   if 0 < |skipped| {
  //     print "Skipped: ", skipped, "\n";
  //   }

  //   expect !hasFailure;

  //   manifest := Success(decryptVectors);
  // }

  datatype ManifestData =
    | DecryptManifest(
        version: nat,
        keys: KeyVectors.KeyVectorsClient,
        client: Values.JSON,
        jsonTests: seq<(string, Values.JSON)>
      )
    | EncryptManifest(
        version: nat,
        keys: KeyVectors.KeyVectorsClient,
        plaintext: seq<(string, AtomicPrimitives.Types.PositiveInteger)>,
        jsonTests: seq<(string, Values.JSON)>
      )

  method GetManifest(
    manifestPath: string,
    manifestFileName: string
  )
    returns (manifestData: Result<ManifestData, string>)

    ensures manifestData.Success? ==>
              && fresh(manifestData.value.keys.Modifies)
              && manifestData.value.keys.ValidState()
    ensures manifestData.Success? && manifestData.value.DecryptManifest?
            ==>
              SupportedDecryptVersion?(manifestData.value.version)
    ensures manifestData.Success? && manifestData.value.EncryptManifest?
            ==>
              SupportedEncryptVersion?(manifestData.value.version)
  {
    var decryptManifestBv :- FileIO.ReadBytesFromFile(manifestPath + manifestFileName);
    var decryptManifestBytes := BvToBytes(decryptManifestBv);
    var manifestJson :- API.Deserialize(decryptManifestBytes)
    .MapFailure(( e: Errors.DeserializationError ) => e.ToString());
    :- Need(manifestJson.Object?, "Not a JSON object");

    var manifest :- GetObject("manifest", manifestJson.obj);
    var version :- GetNat("version", manifest);
    var typ :- GetString("type", manifest);

    var keyManifestUri :- GetString("keys", manifestJson.obj);
    :- Need("file://" < keyManifestUri, "Unexpected URI prefix");
    var keyManifestPath := manifestPath + keyManifestUri[7..];
    var keys :- expect KeyVectors.KeyVectors(KeyVectorsTypes.KeyVectorsConfig(
                                               keyManifestPath := keyManifestPath
                                             ));

    var jsonTests :- GetObject("tests", manifestJson.obj);

    match typ
    case "awses-decrypt" =>
      print version;
      :- Need(SupportedDecryptVersion?(version), "Unsupported manifest version");
      var client :- Get("client", manifestJson.obj);
      manifestData := Success(DecryptManifest(
                                version := version,
                                keys := keys,
                                client := client,
                                jsonTests := jsonTests
                              ));

    case "awses-encrypt" =>
      :- Need(SupportedEncryptVersion?(version), "Unsupported manifest version");
      var plaintextsJson :- GetObject("plaintexts", manifestJson.obj);
      var plaintextsLength :- Seq.MapWithResult(
        (obj: (string, Values.JSON)) =>
          :- Need(obj.1.Number? && 0 < obj.1.num.n <= BoundedInts.INT32_MAX as nat,
                  "Size is not a natural number.");
          Success((obj.0, obj.1.num.n as int32)),
        plaintextsJson
      );
      manifestData := Success(EncryptManifest(
                                version := version,
                                keys := keys,
                                plaintext := plaintextsLength,
                                jsonTests := jsonTests
                              ));

    case _ =>
      manifestData := Failure("Unsupported manifest type:" + typ);
  }
}
