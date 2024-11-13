// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"

module {:options "-functionSyntax:4"} EsdkTestVectors {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import WrappedMaterialProviders
  import WrappedESDK

  import opened Wrappers
  import opened StandardLibrary.UInt
  import UTF8
  import FileIO
  import UUID

  import opened JSONHelpers
  import KeyVectors
  import KeyVectorsTypes = AwsCryptographyMaterialProvidersTestVectorKeysTypes
  import TestVectors
  import AllAlgorithmSuites

  datatype EncryptTest = EncryptTest(
    cmm: mplTypes.ICryptographicMaterialsManager,
    client: Types.IAwsEncryptionSdkClient,
    vector: EsdkEncryptTestVector
  )
  {
    ghost predicate ValidState()
    {
      && cmm.ValidState()
      && client.ValidState()
      && cmm.Modifies !! {client.History}
    }
  }

  datatype DecryptTest = DecryptTest(
    cmm: mplTypes.ICryptographicMaterialsManager,
    client: Types.IAwsEncryptionSdkClient,
    vector: EsdkDecryptTestVector
  )
  {
    ghost predicate ValidState()
    {
      && cmm.ValidState()
      && client.ValidState()
      && cmm.Modifies !! {client.History}
    }
  }

  type SupportedGenerateManifestVersion = v: nat | SupportedGenerateManifestVersion?(v) witness 4
  predicate SupportedGenerateManifestVersion?(v: nat)
  {
    || v == 4
  }

  type SupportedEncryptVersion = v: nat | SupportedEncryptVersion?(v)  witness 1
  predicate SupportedEncryptVersion?(v: nat)
  {
    || v == 1
    || v == 4
    || v == 5
  }


  datatype EsdkEncryptTestVector =
    | PositiveEncryptTestVector(
        name: string,
        version: SupportedEncryptVersion,
        manifestPath: string,
        decryptManifestPath: string,
        plaintextPath: string,
        encryptDescriptions: KeyVectorsTypes.KeyDescription,
        decryptDescriptions: KeyVectorsTypes.KeyDescription,
        encryptionContext: Option<mplTypes.EncryptionContext> := None,
        reproducedEncryptionContext: Option<mplTypes.EncryptionContext> := None,
        requiredEncryptionContextKeys: Option<mplTypes.EncryptionContextKeys> := None,
        requiredECDescription: Option<string> := None,
        commitmentPolicy: mplTypes.ESDKCommitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
        frameLength: Option<int64>,
        algorithmSuiteId: Option<mplTypes.AlgorithmSuiteInfo>,
        description: string,
        maxEncryptedDataKeys: Option<Types.CountingNumbers> := Some(1)
      )
    | PositiveEncryptNegativeDecryptTestVector (
        name: string,
        version: SupportedEncryptVersion,
        manifestPath: string,
        decryptManifestPath: string,
        plaintextPath: string,
        encryptDescriptions: KeyVectorsTypes.KeyDescription,
        decryptDescriptions: KeyVectorsTypes.KeyDescription,
        encryptionContext: Option<mplTypes.EncryptionContext> := None,
        reproducedEncryptionContext: Option<mplTypes.EncryptionContext> := None,
        requiredEncryptionContextKeys: Option<mplTypes.EncryptionContextKeys> := None,
        requiredECDescription: Option<string> := None,
        commitmentPolicy: mplTypes.ESDKCommitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
        frameLength: Option<int64>,
        algorithmSuiteId: Option<mplTypes.AlgorithmSuiteInfo>,
        decryptErrorDescription: string,
        description: string,
        maxEncryptedDataKeys: Option<Types.CountingNumbers> := Some(1)
      )
    | NegativeEncryptTestVector(
        name: string,
        version: SupportedEncryptVersion,
        manifestPath: string,
        plaintextPath: string,
        encryptDescriptions: KeyVectorsTypes.KeyDescription,
        encryptionContext: Option<mplTypes.EncryptionContext> := None,
        reproducedEncryptionContext: Option<mplTypes.EncryptionContext> := None,
        requiredEncryptionContextKeys: Option<mplTypes.EncryptionContextKeys> := None,
        requiredECDescription: Option<string> := None,
        commitmentPolicy: mplTypes.ESDKCommitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
        frameLength: Option<int64>,
        algorithmSuiteId: Option<mplTypes.AlgorithmSuiteInfo>,
        errorDescription: string,
        description: string,
        maxEncryptedDataKeys: Option<Types.CountingNumbers> := Some(1)
      )

  type SupportedDecryptVersion = v: nat | SupportedDecryptVersion?(v)  witness 1
  predicate SupportedDecryptVersion?(v: nat)
  {
    || v == 1
    || v == 2
    || v == 3
  }

  datatype EsdkDecryptTestVector =
    | PositiveDecryptTestVector(
        name: string,
        version: SupportedDecryptVersion,
        manifestPath: string,
        ciphertextPath: string,
        plaintextPath: string,
        reproducedEncryptionContext: Option<mplTypes.EncryptionContext> := None,
        requiredEncryptionContextKeys: Option<mplTypes.EncryptionContextKeys> := None,
        decryptDescriptions: KeyVectorsTypes.KeyDescription,
        commitmentPolicy: mplTypes.ESDKCommitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
        frameLength: Option<int64>,
        algorithmSuiteId: Option<mplTypes.AlgorithmSuiteInfo>,
        description: string,
        decryptionMethod: DecryptionMethod
      )
    | NegativeDecryptTestVector(
        name: string,
        version: SupportedDecryptVersion,
        manifestPath: string,
        ciphertextPath: string,
        errorDescription: string,
        reproducedEncryptionContext: Option<mplTypes.EncryptionContext> := None,
        requiredEncryptionContextKeys: Option<mplTypes.EncryptionContextKeys> := None,
        decryptDescriptions: KeyVectorsTypes.KeyDescription,
        commitmentPolicy: mplTypes.ESDKCommitmentPolicy := mplTypes.FORBID_ENCRYPT_ALLOW_DECRYPT,
        frameLength: Option<int64>,
        algorithmSuiteId: Option<mplTypes.AlgorithmSuiteInfo>,
        description: string,
        decryptionMethod: DecryptionMethod
      )

  datatype DecryptionMethod =
    | StreamingUnsignedOnly
    | OneShot

  method {:vcs_split_on_every_assert} TestDecrypt(
    keys: KeyVectors.KeyVectorsClient,
    vector: EsdkDecryptTestVector
  )
    returns (output: bool)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()
    requires vector.algorithmSuiteId.Some?
  {
    var id := AllAlgorithmSuites.ToHex(vector.algorithmSuiteId.value);
    print "\nTEST-DECRYPT===> ", vector.name, "\n", id, " ", vector.description, "\n";

    // The decrypt test vectors also test initialization
    // This is because they were developed when the MPL
    // was still part of the ESDK
    var test? := DecryptVectorToDecryptTest(keys, vector);

    if test?.Failure? {
      print test?.error, "\n", "\nFAILED! <-----------\n";
      return false;
    }

    var test := test?.value;

    var ciphertext :- expect ReadVectorsFile(test.vector.manifestPath + test.vector.ciphertextPath);
    var plaintext;
    if test.vector.PositiveDecryptTestVector? {
      plaintext :- expect ReadVectorsFile(test.vector.manifestPath + test.vector.plaintextPath);
    }

    var input := Types.DecryptInput(
      ciphertext := ciphertext,
      encryptionContext := test.vector.reproducedEncryptionContext,
      materialsManager := Some(test.cmm),
      keyring := None
    );

    var result := test.client.Decrypt(input);

    output := match test.vector
      case PositiveDecryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_,_)
        =>
        && result.Success?
        && result.value.plaintext == plaintext
      case NegativeDecryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_,_)
        =>
        && result.Failure?;
    if !output {
      if test.vector.PositiveDecryptTestVector? && result.Failure? {
        print result.error, "\n";
        if
          && result.error.AwsCryptographyMaterialProviders?
          && result.error.AwsCryptographyMaterialProviders.CollectionOfErrors?
        {
          print "list:", result.error.AwsCryptographyMaterialProviders.list, "\n";
        }
      }
      print "\nFAILED! <-----------\n";
    }
  }

  method DecryptVectorToDecryptTest(
    keys: KeyVectors.KeyVectorsClient,
    vector: EsdkDecryptTestVector
  )
    returns (output: Result<DecryptTest, KeyVectorsTypes.Error>)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()

    ensures output.Success?
            ==>
              && output.value.ValidState()
              && fresh(output.value.cmm.Modifies - keys.Modifies)
              && fresh(output.value.client.Modifies)
  {
    var cmm :- keys.CreateWrappedTestVectorCmm(
      KeyVectorsTypes.TestVectorCmmInput(
        keyDescription := vector.decryptDescriptions,
        forOperation := KeyVectorsTypes.DECRYPT
      ));

    :- Need(vector.algorithmSuiteId.Some?, KeyVectorsTypes.KeyVectorException(message := "Missing AlgorithmSuiteId in test vector"));
    var commitmentPolicy := AllAlgorithmSuites.GetCompatibleCommitmentPolicy(vector.algorithmSuiteId.value);
    :- Need(commitmentPolicy.ESDK?, KeyVectorsTypes.KeyVectorException(message := "Compatible commitment policy is not for ESDK"));

    var config := WrappedESDK.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment(
      commitmentPolicy := commitmentPolicy.ESDK
    );

    var client :- expect WrappedESDK.WrappedESDK(config := config);

    var test := DecryptTest(
      cmm := cmm,
      client := client,
      vector := vector
    );

    output := Success(test);
  }

  const plaintextPathRoot := "plaintexts/"
  const ciphertextPathPathRoot := "ciphertexts/"

  datatype EncryptTestOutput = EncryptTestOutput(
    output: bool,
    vector: Option<EsdkDecryptTestVector> := None
  )

  method {:vcs_split_on_every_assert} TestEncrypt(
    plaintexts: map<string, seq<uint8>>,
    keys: KeyVectors.KeyVectorsClient,
    test: EncryptTest
  )
    returns (output: Result<EncryptTestOutput, string>)
    requires keys.ValidState() && test.ValidState()
    modifies keys.Modifies
    modifies test.cmm.Modifies
    modifies test.client.Modifies
    ensures keys.ValidState() && test.ValidState()

    requires test.vector.frameLength.Some? ==> Types.IsValid_FrameLength(test.vector.frameLength.value)
    requires test.vector.algorithmSuiteId.Some? && test.vector.algorithmSuiteId.value.id.ESDK?
  {
    var id := AllAlgorithmSuites.ToHex(test.vector.algorithmSuiteId.value);
    print "\nTEST-ENCRYPT===> ", test.vector.name, "\n", id, " ", test.vector.description, "\n";

    // The encrypt test vectors also test initialization
    // This is because they were developed when the MPL
    // was still part of the ESDK
    var vector := test.vector;

    expect test.vector.plaintextPath in plaintexts;
    var plaintext := plaintexts[test.vector.plaintextPath];
    var frameLength: Option<Types.FrameLength> := vector.frameLength;

    var input := Types.EncryptInput(
      plaintext := plaintext,
      encryptionContext := test.vector.encryptionContext,
      materialsManager := Some(test.cmm),
      keyring := None,
      frameLength := frameLength,
      algorithmSuiteId := Some(test.vector.algorithmSuiteId.value.id.ESDK)
    );
    var result := test.client.Encrypt(input);

    if
      && result.Success?
      && (
           || test.vector.PositiveEncryptTestVector?
           || test.vector.PositiveEncryptNegativeDecryptTestVector?
         )
    {
      var decryptVector :- EncryptTestToDecryptVector(test, result.value);
      output := Success(EncryptTestOutput(
                          vector := Some(decryptVector),
                          output := true
                        ));
    } else if result.Failure? && test.vector.NegativeEncryptTestVector? {
      output := Success(EncryptTestOutput( output := true ));
    } else {
      output := Success(EncryptTestOutput( output := false ));
      if !test.vector.NegativeEncryptTestVector? && result.Failure? {
        print result.error;
      }
      print "\nFAILED! <-----------\n";
    }
  }

  method EncryptVectorToEncryptTest(
    keys: KeyVectors.KeyVectorsClient,
    vector: EsdkEncryptTestVector
  )
    returns (output: Result<EncryptTest, KeyVectorsTypes.Error>)
    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()

    ensures output.Success? ==>
              && output.value.ValidState()
              && fresh(output.value.cmm.Modifies)
              && fresh(output.value.client.Modifies)
    ensures output.Success?
            ==>
              output.value.vector == vector
  {
    var cmm :- keys.CreateWrappedTestVectorCmm(
      KeyVectorsTypes.TestVectorCmmInput(
        keyDescription := if vector.PositiveEncryptTestVector? then
          vector.encryptDescriptions
        else if vector.PositiveEncryptNegativeDecryptTestVector? then
          vector.encryptDescriptions
        else
          vector.encryptDescriptions,
        forOperation := KeyVectorsTypes.ENCRYPT
      ));

    :- Need(vector.algorithmSuiteId.Some?, KeyVectorsTypes.KeyVectorException(message := "Missing AlgorithmSuiteId in test vector"));
    var commitmentPolicy := AllAlgorithmSuites.GetCompatibleCommitmentPolicy(vector.algorithmSuiteId.value);
    :- Need(commitmentPolicy.ESDK?, KeyVectorsTypes.KeyVectorException(message := "Compatible commitment policy is not for ESDK"));

    var config := WrappedESDK.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment(
      commitmentPolicy := commitmentPolicy.ESDK
    );

    var client :- expect WrappedESDK.WrappedESDK(config := config);

    var test := EncryptTest(
      cmm := cmm,
      client := client,
      vector := vector
    );

    output := Success(test);
  }

  method EncryptTestToDecryptVector(
    test: EncryptTest,
    result: Types.EncryptOutput
  ) returns (output: Result<EsdkDecryptTestVector, string>)
    requires
      || test.vector.PositiveEncryptTestVector?
      || test.vector.PositiveEncryptNegativeDecryptTestVector?
    requires test.vector.algorithmSuiteId.Some?
  {
    output := match test.vector
      case PositiveEncryptTestVector(_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_) =>
        Success(PositiveDecryptTestVector(
                  name := test.vector.name,
                  version := 3,
                  manifestPath := test.vector.decryptManifestPath,
                  ciphertextPath := ciphertextPathPathRoot + test.vector.name,
                  plaintextPath := plaintextPathRoot + test.vector.plaintextPath,
                  reproducedEncryptionContext := test.vector.reproducedEncryptionContext,
                  requiredEncryptionContextKeys := test.vector.requiredEncryptionContextKeys,
                  decryptDescriptions := test.vector.decryptDescriptions,
                  commitmentPolicy := test.vector.commitmentPolicy,
                  frameLength := test.vector.frameLength,
                  algorithmSuiteId := test.vector.algorithmSuiteId,
                  description := test.vector.description,
                  decryptionMethod := DecryptionMethod.OneShot
                ))
      case _ =>
        Failure("Only postive tests supported");

    var decryptManifestCiphertext := test.vector.decryptManifestPath + ciphertextPathPathRoot + test.vector.name;
    // Side effect, to avoid thousands of ciphertext in memory...
    var _ :- expect WriteVectorsFile(decryptManifestCiphertext, result.ciphertext);
  }


  function MplPrintErr(e: mplTypes.Error) : (){()} by method {print e, "\n", "\n"; return ();}
  function MplVectorPrintErr(e: KeyVectorsTypes.Error) :(){()} by method {print e, "\n", "\n"; return ();}

  method KeyDescriptionToCmm(
    keys: KeyVectors.KeyVectorsClient,
    keyDescriptions: seq<KeyVectorsTypes.KeyDescription>
  )
    returns (output: Result<mplTypes.ICryptographicMaterialsManager, KeyVectorsTypes.Error>)

    requires keys.ValidState()
    modifies keys.Modifies
    ensures keys.ValidState()

    ensures output.Success?
            ==>
              && fresh(output.value.Modifies - keys.Modifies)
              && output.value.ValidState()
  {
    var keyringList: seq<mplTypes.IKeyring> := [];
    for i := 0 to |keyDescriptions|
      invariant forall k | k in keyringList ::
          && k.ValidState() && fresh(k.Modifies)
      invariant forall k | k in keyringList
          :: k.Modifies
             <= set m: object, k :mplTypes.IKeyring
                  |
                  && k in keyringList
                  && m in k.Modifies
                  :: m
    {
      var keyDescription := keyDescriptions[i];
      var keyring :- keys.CreateWrappedTestVectorKeyring(
        KeyVectorsTypes.TestVectorKeyringInput(
          keyDescription := keyDescription
        ));
      keyringList := keyringList + [keyring];
    }

    :- Need(|keyringList| == 1, KeyVectorsTypes.KeyVectorException( message := "Failed to create any keyrings" ));
    var mpl :- expect WrappedMaterialProviders.WrappedMaterialProviders();
    var generatorKeyring := keyringList[0];
    var maybeMultiKeyring := mpl.CreateMultiKeyring(
      mplTypes.CreateMultiKeyringInput(
        generator := Some(generatorKeyring),
        childKeyrings := keyringList[1..]
      )
    );

    var keyring :- maybeMultiKeyring
    .MapFailure(e => KeyVectorsTypes.AwsCryptographyMaterialProviders(e));

    var maybeCmm := mpl
    .CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput( keyring := maybeMultiKeyring.value )
    );
    output := maybeCmm
    .MapFailure(e => KeyVectorsTypes.AwsCryptographyMaterialProviders(e));
  }

  method ReadVectorsFile(location: string)
    returns (output: Result<seq<uint8>, string>)
  {
    var fileBv :- FileIO.ReadBytesFromFile(location);
    output := Success(BvToBytes(fileBv));
  }

  method WriteVectorsFile(location: string, bytes: seq<uint8>)
    returns (output: Result<(), string>)
  {
    var bv := BytesBv(bytes);
    output := FileIO.WriteBytesToFile(location, bv);
  }
}