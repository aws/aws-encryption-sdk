// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

include "Fixtures.dfy"

module TestReproducedEncryptionContext {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders
  import ESDK
  import opened Wrappers

  import Fixtures

  method {:test} TestEncryptionContextOnDecrypt()
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

    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
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
                                        encryptionContext := Some(encryptionContext)
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;

    expect cycledPlaintext == asdf;
  }

  method {:test} TestEncryptionContextOnDecryptFailure()
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

    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    var incorrectReproducedEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
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
                                        encryptionContext := Some(incorrectReproducedEncryptionContext)
                                      ));

    // We expect to fail because we pass more encryption context than was used on encrypt
    expect decryptOutput.Failure?;
  }

  method {:test} TestMismatchedEncryptionContextOnDecrypt()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var namespace, name := Fixtures.NamespaceAndName(0);
    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();
    var rawAESKeyring :- expect mpl.CreateRawAesKeyring(mplTypes.CreateRawAesKeyringInput(
                                                          keyNamespace := namespace,
                                                          keyName := name,
                                                          wrappingKey := seq(32, i => 0),
                                                          wrappingAlg := mplTypes.ALG_AES256_GCM_IV12_TAG16
                                                        ));

    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    var mismatchedEncryptionContext := Fixtures.SmallMismatchedEncryptionContex(Fixtures.SmallEncryptionContextVariation.A);

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := None,
                                        keyring := Some(rawAESKeyring),
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rawAESKeyring),
                                        encryptionContext := Some(mismatchedEncryptionContext)
                                      ));

    // We expect to fail because although the same key is present on the ec
    // their value is different.
    expect decryptOutput.Failure?;

    // test that if we supply the right ec we will succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(rawAESKeyring),
                                    encryptionContext := Some(encryptionContext)
                                  ));

    expect decryptOutput.Success?;

    // Since we store all encryption context we MST succeed if no encryption context is
    // supplied on decrypt
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(rawAESKeyring),
                                    encryptionContext := None
                                  ));

    expect decryptOutput.Success?;
  }
}