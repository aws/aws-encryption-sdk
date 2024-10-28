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

  // method {:test} TestV1Vectors() {
  //   var _ :- expect EsdkTestManifests.StartV1DecryptVectors(
  //     "aws-encryption-sdk-test-vectors/vectors/awses-decrypt/python-1.3.5/decrypt_message.json",
  //     "aws-encryption-sdk-test-vectors/vectors/awses-decrypt/python-1.3.5/keys.json"
  //   );
  // }


  // method {:test} TestV2Vectors() {
  //   var _ :- expect EsdkTestManifests.StartDecryptVectors(
  //     EsdkManifestOptions.Decrypt(
  //       manifestPath := "aws-encryption-sdk-test-vectors/vectors/awses-decrypt/python-2.3.0/"
  //     )
  //   );
  // }

  // method {:test} TestEncryptVectors() {
  //   var result := EsdkTestManifests.StartEncryptVectors(
  //     EsdkManifestOptions.Encrypt(
  //       manifestPath := "./dafny/ESDK/test/",
  //       manifest := "manifest.json",
  //       decryptManifestOutput := "decrypt-manifest/"
  //     )
  //   );

  //   if result.Failure? {
  //     print result.error, "\n";
  //   }

  //   expect result.Success?;
  // }

  // method {:test} TestDecryptVectors() {
  //   var _ :- expect EsdkTestManifests.StartDecryptVectors(
  //     EsdkManifestOptions.Decrypt(
  //       manifestPath := "decrypt-manifest/"
  //     )
  //   );
  // }

  // method {:test} TestV2Vectors() {
  //   TestEsdkManifests.StartV1Vectors(
  //     "dafny/ESDK/test/python-2.3.0/manifest.json",
  //     "dafny/ESDK/test/python-2.3.0/keys.json"
  //   );

  // }
  
  method {:test} RunManifestTests() {
    TestGenerateEncryptManifest();
    TestEncryptManifest();
    // TestDecryptManifest();
  }
  
  method TestGenerateEncryptManifest() {
    var directory := GetTestVectorExecutionDirectory();
    var result := WriteVectors.WritetestVectors(
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
        manifest := "manifest.json",
        decryptManifestOutput := directory + "dafny/TestVectors/"
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

}
