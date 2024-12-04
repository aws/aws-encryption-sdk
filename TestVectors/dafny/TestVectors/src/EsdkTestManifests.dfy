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
  import WriteVectors

  method StartDecryptVectors(
    op: EsdkManifestOptions.ManifestOptions
  )
    returns (output: Result<seq<BoundedInts.byte>, string>)
    requires op.Decrypt?
    requires 0 < |op.manifestPath|
    requires Seq.Last(op.manifestPath) == '/'
  {
    var decryptManifest :- expect GetManifest(op.manifestPath, op.manifestFileName);
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
        print "\nSKIP===> ", vector.id, "\n";
      }

    }
    print "\n=================== Completed ", |vectors|, " Decrypt Tests =================== \n\n";

    if 0 < skipped {
      print "Skipped: ", skipped, "\n";
    }

    expect !hasFailure;

    manifest := Success([]);
  }

  method {:vcs_split_on_every_assert} StartEncryptVectors(
    op: EsdkManifestOptions.ManifestOptions
  )
    returns (output: Result<(), string>)
    requires op.Encrypt?
    requires 0 < |op.manifestPath|
  {
    var encryptManifest :- GetManifest(op.manifestPath, op.manifest);
    :- Need(encryptManifest.EncryptManifest?, "Not a encrypt manifest");

    var encryptVectors :- ParseEsdkJsonManifest.BuildEncryptTestVector(
      op,
      encryptManifest.version,
      encryptManifest.keys,
      encryptManifest.jsonTests
    );

    var p :- expect AtomicPrimitives.AtomicPrimitives();
    var plaintext := map[];
    for i := 0 to |encryptManifest.plaintext|
    {
      var (name, length) := encryptManifest.plaintext[i];
      var data :- expect p.GenerateRandomBytes(
        AtomicPrimitives.Types.GenerateRandomBytesInput(
          length := length
        ));
      // Write the plaintext to disk.
      print op.decryptManifestOutput + plaintextPathRoot + name, "\n\n";
      var _ :- WriteVectorsFile(op.decryptManifestOutput + plaintextPathRoot + name, data);
      plaintext := plaintext + map[ name := data ];
    }

    var encryptTests? := ToEncryptTests(encryptManifest.keys, encryptVectors);
    var encryptTests :- encryptTests?.MapFailure((e: KeyVectorsTypes.Error) => var _ := MplVectorPrintErr(e); "Cmm failure");
    var decryptVectors :- TestEncrypts(plaintext, encryptManifest.keys, encryptTests);

    var _ :- WriteVectors.WriteDecryptManifest(op, encryptManifest.keys, decryptVectors);

    output := Success(());
  }


  predicate TestEncryptVector?(vector: EsdkEncryptTestVector)
  {
    && (vector.frameLength.Some? ==> Types.IsValid_FrameLength(vector.frameLength.value))
  }

  method ToEncryptTests(keys: KeyVectors.KeyVectorsClient, vectors: seq<EsdkEncryptTestVector>)
    returns (output: Result<seq<EsdkTestVectors.EncryptTest>, KeyVectorsTypes.Error>)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()
    ensures output.Success? ==>
              && forall t <- output.value ::
                && t.ValidState()
                && fresh(t.cmm.Modifies) && fresh(t.client.Modifies)
  {
    var encryptTests: seq<EsdkTestVectors.EncryptTest> := [];
    for i := 0 to |vectors|
      invariant forall t <- encryptTests ::
          && t.ValidState()
          && fresh(t.cmm.Modifies) && fresh(t.client.Modifies)
    {
      var test :- EsdkTestVectors.EncryptVectorToEncryptTest(keys, vectors[i]);
      encryptTests := encryptTests + [test];
    }

    return Success(encryptTests);
  }

  method TestEncrypts(
    plaintexts: map<string, seq<uint8>>,
    keys: KeyVectors.KeyVectorsClient,
    tests: seq<EsdkTestVectors.EncryptTest>
  )
    returns (manifest: Result<seq<EsdkDecryptTestVector>, string>)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()
    requires forall t <- tests :: t.ValidState()
    modifies set t <- tests, o | o in t.cmm.Modifies :: o
    modifies set t <- tests, o | o in t.client.Modifies :: o
    ensures forall t <- tests :: t.ValidState()
  {
    print "\n=================== Starting ", |tests|, " Encrypt Tests =================== \n\n";

    var hasFailure := false;
    var decryptVectors := [];
    var skipped := [];

    for i := 0 to |tests|
      invariant forall t <- tests ::
          && t.ValidState()
    {
      var test := tests[i];
      :- Need(
        test.vector.id.Some?,
        "Vector is missing uuid"
      );
      if TestEncryptVector?(test.vector) {
        :- Need(
          && test.vector.algorithmSuiteId.Some?
          && test.vector.algorithmSuiteId.value.id.ESDK?,
          "Vector is using an algorithm suite other than ESDK"
        );
        var pass :- EsdkTestVectors.TestEncrypt(plaintexts, keys, test);
        if !pass.output {
          hasFailure := true;
        } else if pass.vector.Some? {
          decryptVectors := decryptVectors + [pass.vector.value];
        }
      } else {
        skipped := skipped + [test.vector.id.value + "\n"];
        print "\nSKIP===> ", test.vector.id.value, "\n";
      }
    }
    print "\n=================== Completed ", |tests|, " Encrypt Tests =================== \n\n";

    expect !hasFailure;

    manifest := Success(decryptVectors);
  }

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
