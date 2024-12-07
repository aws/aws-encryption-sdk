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


  // Test execution directory is different for different runtimes.
  // Runtime should define an extern to return the expected test execution directory.
  method {:extern} GetTestVectorExecutionDirectory() returns (res: string)

  method {:test} RunManifestTests() {
    TestGenerateEncryptManifest();
    TestEncryptManifest();
    TestDecryptManifest();
  }

  // Read encrypt manifests for valid ESDK .NET v4.0.0 messages
  method {:test} TestNetRetryFlagVectorsExpectSuccess() {
    var directory := GetTestVectorExecutionDirectory();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/valid-Net-4.0.0/",
        manifestFileName := "manifest.json"
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method {:test} TestNetRetryFlagVectorsExpectFailure() {

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
        manifestFileName := "decrypt-manifest.json"
      )
    );

    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }
}
