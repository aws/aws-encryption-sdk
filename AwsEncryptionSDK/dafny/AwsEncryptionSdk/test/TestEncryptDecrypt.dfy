// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

include "Fixtures.dfy"

module TestEncryptDecrypt {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders
  import ESDK
  import opened Wrappers

  import Fixtures

  method {:test} TestEncryptDecrypt()
  {
    var kmsKey :=  Fixtures.keyArn;
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();
    var clientSupplier :- expect mpl.CreateDefaultClientSupplier(mplTypes.CreateDefaultClientSupplierInput);
    var kmsClient :- expect clientSupplier.GetClient(mplTypes.GetClientInput(region := "us-west-2"));

    var kmsKeyring :- expect mpl.CreateAwsKmsKeyring(
      mplTypes.CreateAwsKmsKeyringInput(
        kmsKeyId := kmsKey,
        kmsClient := kmsClient,
        grantTokens := None
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := None,
                                        materialsManager := None,
                                        keyring := Some(kmsKeyring),
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(kmsKeyring),
                                        encryptionContext := None
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;

    expect cycledPlaintext == asdf;
  }
}
