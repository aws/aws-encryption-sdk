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
  import OsLang

  // Test execution directory is different for different runtimes.
  // Runtime should define an extern to return the expected test execution directory.
  method {:extern} GetTestVectorExecutionDirectory() returns (res: string)

  method GetDirPrefix() returns (res: string)
  {
    if OsLang.GetLanguageShort() == "Java" {
      res := "../../";
    } else {
      res := GetTestVectorExecutionDirectory();
    }
  }

  function method AllowRetry() : Types.NetV4_0_0_RetryPolicy
  {
    if OsLang.GetLanguageShort() == "Java" then
      Types.NetV4_0_0_RetryPolicy.FORBID_RETRY
    else
      Types.NetV4_0_0_RetryPolicy.ALLOW_RETRY
  }

  method {:test} RunManifestTests() {
    TestGenerateEncryptManifest();
    TestEncryptManifest();
    TestDecryptManifest();
  }

  // Read encrypt manifests for valid ESDK .NET v4.0.0 messages
  // These messages are expected to successfully decrypt without
  // having to retry.
  method {:test} TestNetRetryFlagVectorsExpectSuccess() {
    var directory := GetDirPrefix();
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

  method {:test} TestPerfManifest() {
    var directory := GetDirPrefix();
    var result := EsdkTestManifests.StartEncryptVectors(
      EsdkManifestOptions.Encrypt(
        manifestPath := directory + "dafny/TestVectors/test/",
        manifest := "perf-encrypt-manifest.json",
        decryptManifestOutput := "dafny/TestVectors/test/perf/",
        report := EsdkManifestOptions.ReportAll
      )
    );
    if result.Failure? {
      print "\nTestPerfManifest Encrypt Failure\n", result.error, "\n";
    }
    expect result.Success?;

    var result2 := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/perf/",
        manifestFileName := "decrypt-manifest.json",
        retryPolicy := AllowRetry(),
        report := EsdkManifestOptions.ReportAll
      )
    );
    if result2.Failure? {
      print "\nTestPerfManifest Decrypt Failure\n", result2.error, "\n";
    }
    expect result2.Success?;
  }

  // Read encrypt manifests for invalid ESDK .NET v4.0.0 messages
  // These messages are expected to fail if retry option is set to FORBID_RETRY
  // As of 12-7-2024, I can't think of an easy way to reuse all the test vector framework
  // to correctly pass when this configuration runs the invalid net 4.0.0 tests.
  // The errors that we get back from the MPL are opaque errors, not opaque with text...
  // This means that in dafny code we cannot check the error message :(
  method {:test} TestNetInvalidTestVectorsExpectFailure() {
    var directory := GetDirPrefix();
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

  method {:test} TestNetInvalidTestVectorsExpectSuccessOnRetry() {
    // we can't retry in Java
    if OsLang.GetLanguageShort() == "Java" {
      return;
    }
    var directory := GetDirPrefix();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/invalid-Net-4.0.0/",
        manifestFileName := "manifest.json",
        retryPolicy := AllowRetry()
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method {:test} TestNet401ValidTestVectorsExpectSuccess() {
    var directory := GetDirPrefix();
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
    var directory := GetDirPrefix();
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
    var directory := GetDirPrefix();
    var result := EsdkTestManifests.StartEncryptVectors(
      EsdkManifestOptions.Encrypt(
        manifestPath := directory + "dafny/TestVectors/test/",
        manifest := "encrypt-manifest.json",
        decryptManifestOutput := directory + "dafny/TestVectors/test/",
        report := EsdkManifestOptions.ReportFinal
      )
    );
    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }

  method TestDecryptManifest()
  {
    var directory := GetDirPrefix();
    var result := EsdkTestManifests.StartDecryptVectors(
      EsdkManifestOptions.Decrypt(
        manifestPath := directory + "dafny/TestVectors/test/",
        manifestFileName := "decrypt-manifest.json",
        retryPolicy := Types.NetV4_0_0_RetryPolicy.FORBID_RETRY,
        report := EsdkManifestOptions.ReportFinal
      )
    );

    if result.Failure? {
      print result.error;
    }
    expect result.Success?;
  }
}
