// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

include "Fixtures.dfy"

module TestRequiredEncryptionContext {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import primitivesTypes = AwsCryptographyPrimitivesTypes
  import KeyStoreTypes = AwsCryptographyKeyStoreTypes
  import MaterialProviders
  import KeyStore
  import ComAmazonawsKmsTypes
  import KMS = Com.Amazonaws.Kms
  import DDB = Com.Amazonaws.Dynamodb
  import DDBTypes = ComAmazonawsDynamodbTypes
  import ESDK
  import opened Wrappers
  import UTF8

  import Fixtures

  // THIS IS A TESTING RESOURCE DO NOT USE IN A PRODUCTION ENVIRONMENT
  const keyArn := Fixtures.keyArn
  const hierarchyKeyArn := Fixtures.hierarchyKeyArn;
  const branchKeyStoreName: DDBTypes.TableName := Fixtures.branchKeyStoreName
  const logicalKeyStoreName := branchKeyStoreName
  // These tests require a keystore populated with these keys
  const BRANCH_KEY_ID := Fixtures.branchKeyId

  method {:test} TestReprEncryptionContextWithSameECHappyCase()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // HAPPY CASE 1
    // Test supply same encryption context on encrypt and decrypt NO filtering
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := None,
                                        keyring := Some(multiKeyring),
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));
    print encryptOutput;
    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Test RSA
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := Some(encryptionContext)
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Test KMS
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(encryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Test AES
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(encryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Test Hierarchy Keyring
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(encryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;
  }

  method {:test} TestRemoveOnEncryptAndSupplyOnDecryptHappyCase()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // Happy Test Case 2
    // On Encrypt we will only write one encryption context key value to the header
    // we will then supply only what we didn't write wth no required ec cmm,
    // This test case is checking that the default cmm is doing the correct filtering by using
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var reproducedEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    var requiredEncryptionContextKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    // TEST RSA
    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));
    print encryptOutput;
    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only KMS keyring
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only AES keyring
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only Hierarchical keyring
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

  }

  method {:test} TestRemoveOnEncryptRemoveAndSupplyOnDecryptHappyCase()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // HAPPY CASE 3
    // On Encrypt we will only write one encryption context key value to the header
    // we will then supply only what we didn't write but included in the signature while we
    // are configured with the required encryption context cmm
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var reproducedEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    var requiredEncryptionContextKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    // TEST RSA
    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        encryptionContext := Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST KMS
    // Switch to only KMS keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := kmsKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST AES
    // switch to only aes
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := aesKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST HIERARCHY
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := hKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;
  }

  method {:test} TestRemoveOnDecryptIsBackwardsCompatibleHappyCase()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // HAPPY CASE 4
    // On Encrypt we write all encryption context
    // as if the message was encrypted before the feature existed.
    // We will then have a required encryption context cmm
    // that will require us to supply the encryption context on decrypt.
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var reproducedEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    var requiredEncryptionContextKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // All encryption context is stored in the message
    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(defaultCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        encryptionContext := Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    var cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST KMS
    // Switch to only KMS keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := kmsKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST AES
    // switch to only aes
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := aesKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST HIERARCHY
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := hKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := Some(reqCMM),
                                    keyring := None,
                                    encryptionContext := Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext := decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;
  }

  method {:test} TestDifferentECOnDecryptFailure()
  {
    // encrypt {a, b} => decrypt {b:c} => fail
    // encrypt {a, b} => decrypt {d} => fail

    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 1
    // Encrypt with and store all encryption context in header
    // On Decrypt supply additional encryption context not stored in the header; this MUST fail
    // On Decrypt supply mismatched encryption context key values; this MUST fail
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    // Additional EC
    var reproducedAdditionalEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.C);
    // Mismatched EncryptionContext
    var reproducedMismatchedEncryptionContext := Fixtures.SmallMismatchedEncryptionContex(Fixtures.SmallEncryptionContextVariation.AB);

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := None,
                                        keyring := Some(multiKeyring),
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Test RSA Failures
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := Some(reproducedAdditionalEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(rsaKeyring),
                                    encryptionContext := Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;
  }

  method {:test} TestRemoveECAndNotSupplyOnDecryptFailure()
  {

    // encrypt remove(a) RSA {a, b} => decrypt => fail
    // encrypt remove(a) KMS {a, b} => decrypt => fail
    // encrypt remove(a) AES {a, b} => decrypt => fail
    // encrypt remove(a) Hie {a, b} => decrypt => fail

    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 2
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signture.
    // Decrypt will not supply any reproduced Encryption Context; this MUST fail.
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var requiredECKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredECKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Test RSA Failure
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := None
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := None
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := None
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := None
                                  ));

    expect decryptOutput.Failure?;
  }

  method {:test} TestRemoveECAndSupplyMismatchedReprECFailure()
  {

    // encrypt remove(a) RSA {a, b} => decrypt {b:c} => fail
    // encrypt remove(a) KMS {a, b} => decrypt {b:c} => fail
    // encrypt remove(a) AES {a, b} => decrypt {b:c} => fail
    // encrypt remove(a) Hie {a, b} => decrypt {b:c} => fail

    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 3
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signture.
    // Decrypt will supply the correct key but incorrect value; this MUST fail.
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var requiredECKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);
    // this reproduced encryption context contains the key we didn't store, but it has the wrong value
    var mismatchedReproducedEncryptionContext := Fixtures.SmallMismatchedEncryptionContex(Fixtures.SmallEncryptionContextVariation.A);

    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredECKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Test RSA Failure
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := Some(mismatchedReproducedEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(mismatchedReproducedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(mismatchedReproducedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(mismatchedReproducedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;
  }

  method {:test} TestRemoveECAndSupplyWithMissingRequiredValueDecryptFailure()
  {
    // encrypt remove(a) RSA {a, b} => decrypt remove(a) => fail
    // encrypt remove(a) KMS {a, b} => decrypt remove(a) => fail
    // encrypt remove(a) AES {a, b} => decrypt remove(a) => fail
    // encrypt remove(a) Hie {a, b} => decrypt remove(a) => fail

    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();
    var kmsKeyring := GetKmsKeyring();
    var aesKeyring := GetAesKeyring();
    var hKeyring := GetHierarchicalKeyring();

    var multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator := Some(aesKeyring),
                                                        childKeyrings := [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 4
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signture.
    // Decrypt will supply the correct key but incorrect value; this MUST fail.
    var encryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    var requiredECKeys := Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);
    // this reproduced encryption context does not contain the key that was dropped
    var droppedRequiredKeyEncryptionContext := Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.B);

    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredECKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Success?;
    var esdkCiphertext := encryptOutput.value.ciphertext;

    // Test RSA Failure
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(rsaKeyring),
                                        encryptionContext := Some(droppedRequiredKeyEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failure
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(kmsKeyring),
                                    encryptionContext := Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failure
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(aesKeyring),
                                    encryptionContext := Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failure
    decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                    ciphertext := esdkCiphertext,
                                    materialsManager := None,
                                    keyring := Some(hKeyring),
                                    encryptionContext := Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

  }

  method {:test} TestReservedEncryptionContextKeyFailure()
  {
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];

    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);
    var mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    var rsaKeyring := GetRsaKeyring();

    var encryptionContext := Fixtures.GetResrvedECMap();
    var requiredECKeys := [Fixtures.RESERVED_ENCRYPTION_CONTEXT];

    var defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring := rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    // Although we are requesting that we remove a RESERVED key word from the encryption context
    // The CMM instantiation will still succeed because the CMM is meant to work with different higher level
    // encryption libraries who may have different reserved keys. Encryption will ultimately fail.
    var reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM := Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring := None,
        requiredEncryptionContextKeys := requiredECKeys
      )
    );

    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := Some(encryptionContext),
                                        materialsManager := Some(reqCMM),
                                        keyring := None,
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    expect encryptOutput.Failure?;

  }

  method GetHierarchicalKeyring()
    returns (output: mplTypes.IKeyring)
    ensures output.ValidState() && fresh(output) && fresh(output.History) && fresh(output.Modifies)
  {
    var branchKeyId := BRANCH_KEY_ID;
    var ttl : mplTypes.PositiveLong := (1 * 60000) * 10;
    var mpl :- expect MaterialProviders.MaterialProviders();

    var kmsClient :- expect KMS.KMSClient();
    var ddbClient :- expect DDB.DynamoDBClient();
    var kmsConfig := KeyStoreTypes.KMSConfiguration.kmsKeyArn(hierarchyKeyArn);

    var keyStoreConfig := KeyStoreTypes.KeyStoreConfig(
      id := None,
      kmsConfiguration := kmsConfig,
      logicalKeyStoreName := logicalKeyStoreName,
      grantTokens := None,
      ddbTableName := branchKeyStoreName,
      ddbClient := Some(ddbClient),
      kmsClient := Some(kmsClient)
    );

    var keyStore :- expect KeyStore.KeyStore(keyStoreConfig);

    output :- expect mpl.CreateAwsKmsHierarchicalKeyring(
      mplTypes.CreateAwsKmsHierarchicalKeyringInput(
        branchKeyId := Some(branchKeyId),
        branchKeyIdSupplier := None,
        keyStore := keyStore,
        ttlSeconds := ttl,
        cache := None
      ));
  }

  method GetRsaKeyring()
    returns (output: mplTypes.IKeyring)
    ensures output.ValidState() && fresh(output) && fresh(output.History) && fresh(output.Modifies)
  {
    var mpl :- expect MaterialProviders.MaterialProviders();

    var namespace, name := Fixtures.NamespaceAndName(0);
    var keys := Fixtures.GenerateKeyPair(2048 as primitivesTypes.RSAModulusLengthBits);
    output :- expect mpl.CreateRawRsaKeyring(mplTypes.CreateRawRsaKeyringInput(
                                               keyNamespace := namespace,
                                               keyName := name,
                                               paddingScheme := mplTypes.PaddingScheme.OAEP_SHA1_MGF1,
                                               publicKey := Option.Some(keys.publicKey.pem),
                                               privateKey := Option.Some(keys.privateKey.pem)
                                             ));

  }

  method GetAesKeyring()
    returns (output: mplTypes.IKeyring)
    ensures output.ValidState() && fresh(output) && fresh(output.History) && fresh(output.Modifies)
  {
    var mpl :- expect MaterialProviders.MaterialProviders();

    var namespace, name := Fixtures.NamespaceAndName(0);
    output :- expect mpl.CreateRawAesKeyring(mplTypes.CreateRawAesKeyringInput(
                                               keyNamespace := namespace,
                                               keyName := name,
                                               wrappingKey := seq(32, i => 0),
                                               wrappingAlg := mplTypes.ALG_AES256_GCM_IV12_TAG16
                                             ));
  }

  method GetKmsKeyring()
    returns (output: mplTypes.IKeyring)
    ensures output.ValidState() && fresh(output) && fresh(output.History) && fresh(output.Modifies)
  {
    var kmsKey :=  Fixtures.keyArn;
    print "=== GetKmsKeyring ===\n";
    print "Using KMS Key ARN: ", kmsKey, "\n";
    
    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdkResult := ESDK.ESDK(config := defaultConfig);
    if esdkResult.Failure? {
      print "ESDK creation failed: ", esdkResult.error, "\n";
      expect false;
    }
    var esdk := esdkResult.value;
    print "ESDK created successfully\n";
    
    var mplResult := MaterialProviders.MaterialProviders();
    if mplResult.Failure? {
      print "MaterialProviders creation failed: ", mplResult.error, "\n";
      expect false;
    }
    var mpl := mplResult.value;
    print "MaterialProviders created successfully\n";
    
    var clientSupplierResult := mpl.CreateDefaultClientSupplier(mplTypes.CreateDefaultClientSupplierInput);
    if clientSupplierResult.Failure? {
      print "Client supplier creation failed: ", clientSupplierResult.error, "\n";
      expect false;
    }
    var clientSupplier := clientSupplierResult.value;
    print "Client supplier created successfully\n";
    
    var kmsClientResult := clientSupplier.GetClient(mplTypes.GetClientInput(region := "us-west-2"));
    if kmsClientResult.Failure? {
      print "KMS client creation failed: ", kmsClientResult.error, "\n";
      expect false;
    }
    var kmsClient := kmsClientResult.value;
    print "KMS client created successfully\n";

    var keyringResult := mpl.CreateAwsKmsKeyring(
      mplTypes.CreateAwsKmsKeyringInput(
        kmsKeyId := kmsKey,
        kmsClient := kmsClient,
        grantTokens := None
      )
    );
    
    if keyringResult.Failure? {
      print "KMS keyring creation failed: ", keyringResult.error, "\n";
      print "This is likely due to account access issues\n";
      print "Expected account: 370957321024 (from assumed role)\n";
      print "Key account: Check if key ARN matches assumed role account\n";
      expect false;
    }
    
    output := keyringResult.value;
    print "KMS keyring created successfully\n";
  }

}
