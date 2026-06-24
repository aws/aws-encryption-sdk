// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"
include "Fixtures.dfy"

module TestHierarchicalKeyringJs {
  import Types = AwsCryptographyEncryptionSdkTypes
  import MaterialProviders
  import ESDK
  import opened Wrappers

  import Fixtures
  import Base64

  // This is a direct import,
  // but it is for a test,
  // importing from a test project.
  // Not the best, but tractable.
  // Neither project is expecting this to never break.
  import KeyringFromKeyDescription

  method {:test} StaticHierarchyKeyringTestVectors() {

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

      var keyring :- expect KeyringFromKeyDescription.ToKeyring(
      mpl := mpl,
      keys := Fixtures.Keys,
      description := Fixtures.HierarchyKeyDescription
    );
    print "Testing StaticHierarchyKeyringTestVectors\n";

    for i := 0 to |Fixtures.StaticTestVectors|
    {
      var Vector := Fixtures.StaticTestVectors[i];
      print Vector.Comment, "\n";
      var esdkCiphertext :- expect Base64.Decode(Vector.CiphertextBase64);
      var plaintext :- expect Base64.Decode(Vector.PlaintextBase64);

      var decryptOutput :- expect esdk.Decrypt(
        Types.DecryptInput(
        ciphertext := esdkCiphertext,
        materialsManager := None,
        keyring := Some(keyring),
        encryptionContext := None // Some(Vector.EncryptionContext)
        )
      );

      expect decryptOutput.plaintext == plaintext;

    }
  }


}