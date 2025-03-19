// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Test vector projects just run as a CLI
// So all the tests are in the Main.
// By creating a single file here,
// it is easy to kick off a test run.
include "../src/Index.dfy"

module {:extern} TestWrappedESDKMain {
  import WrappedESDKMain
  import EsdkTestManifests
  import EsdkManifestOptions
  import WriteVectors
  import opened Wrappers
  import Types = AwsCryptographyEncryptionSdkTypes
  import WrappedESDK
  import opened StandardLibrary.UInt
  import Time
  import MPL = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders

  // Test execution directory is different for different runtimes.
  // Runtime should define an extern to return the expected test execution directory.
  method {:extern} GetTestVectorExecutionDirectory() returns (res: string)

  method {:test} RunManifestTests() {
    // TestGenerateEncryptManifest();
    // TestEncryptManifest();
    // TestDecryptManifest();
  }

  // Read encrypt manifests for valid ESDK .NET v4.0.0 messages
  // These messages are expected to successfully decrypt without
  // having to retry.
  method /*{:test}*/ TestNetRetryFlagVectorsExpectSuccess() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/valid-Net-4.0.0/",
        manifestFileName := "manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.FORBID_RETRY
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method {:test} TestLargePayload() {
    var client :- expect WrappedESDK.WrappedESDK();

    var mpl :- expect MaterialProviders.MaterialProviders();
    var key_input := MPL.CreateRawAesKeyringInput (
      keyNamespace := "namespace",
      keyName := "keyname",
      wrappingKey := [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3],
      wrappingAlg := MPL.ALG_AES256_GCM_IV12_TAG16
    );
    var keyring :- expect mpl.CreateRawAesKeyring(key_input);

    var plain : seq<uint8> := seq(100000000, _ => 42);

    var time := Time.GetAbsoluteTime();
    var enc_output :- expect client.Encrypt(
      Types.EncryptInput (
        plaintext := plain,
        keyring := Some(keyring), 
        algorithmSuiteId := Some(MPL.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY)
      )
    );
    var elapsed := Time.TimeSince(time);
    Time.PrintTimeLong(elapsed, "Large Encrypt", Some(EsdkTestManifests.LogFileName()));

    time := Time.GetAbsoluteTime();
    var dec_output :- expect client.Decrypt(
      Types.DecryptInput (
        ciphertext := enc_output.ciphertext,
        keyring := Some(keyring)
      )
    );
    elapsed := Time.TimeSince(time);
    Time.PrintTimeLong(elapsed, "Large Decrypt", Some(EsdkTestManifests.LogFileName()));

    expect plain == dec_output.plaintext;
}

  // Read encrypt manifests for invalid ESDK .NET v4.0.0 messages
  // These messages are expected to fail if retry option is set to FORBID_RETRY
  // As of 12-7-2024, I can't think of an easy way to reuse all the test vector framework
  // to correctly pass when this configuration runs the invalid net 4.0.0 tests.
  // The errors that we get back from the MPL are opaque errors, not opaque with text...
  // This means that in dafny code we cannot check the error message :(
  method /*{:test}*/ TestNetInvalidTestVectorsExpectFailure() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/invalid-Net-4.0.0/",
        manifestFileName := "manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.FORBID_RETRY
      )
    );
    print "ONLY WORRY IF THE ABOVE TESTS PASSED!!! THESE TESTS ARE SUPPOSED TO FAIL!\n";
    print "IF THE TESTS FAIL OTHER THAN A `AES GCM TAG VALIDATION EXCEPTION` or `AES Decrypt : gather Unspecified`, CUT AN ISSUE.\n";
    print "IF THE TESTS ALL FAIL IT MEANS THE TEST PASSED!";
    expect result.Failure?;
  }

  method /*{:test}*/ TestNetInvalidTestVectorsExpectSuccessOnRetry() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/invalid-Net-4.0.0/",
        manifestFileName := "manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.ALLOW_RETRY
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method /*{:test}*/ TestNet401ValidTestVectorsExpectSuccess() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/v4-Net-4.0.1/",
        manifestFileName := "manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.FORBID_RETRY
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method TestGenerateEncryptManifest() {
    var directory := GetTestVectorExecutionDirectory();
    var result := WriteVectors.WriteTestVectors(
      EsdkManifestOptions.EncryptManifest(
        encryptManifestOutput := directory + "dafny/TestVectors/test/",
        version := 5
      ));
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method TestEncryptManifest() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartEncryptVectors(
      EsdkManifestOptions.Encrypt(
        manifestPath := directory + "dafny/TestVectors/test/",
        manifest := "encrypt-manifest.json",
        decryptManifestOutput := directory + "dafny/TestVectors/test/"
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method TestDecryptManifest()
  {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/",
        manifestFileName := "decrypt-manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.FORBID_RETRY
      )
    );

    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }
}
