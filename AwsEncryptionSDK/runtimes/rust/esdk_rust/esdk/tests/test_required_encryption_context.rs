// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "legacy")]

mod fixtures;
use aws_esdk::*;
use aws_mpl_legacy::aws_cryptography_keyStore::client::Client as KeystoreClient;
use aws_mpl_legacy::aws_cryptography_keyStore::types::KmsConfiguration;
use aws_mpl_legacy::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig;
use aws_mpl_legacy::client::Client as MplClient;
use aws_mpl_legacy::types::keyring::KeyringRef;
use fixtures::*;

// THIS IS A TESTING RESOURCE DO NOT USE IN A PRODUCTION ENVIRONMENT

async fn get_rsa_keyring(mpl: &MplClient) -> KeyringRef {
    let keys = generate_key_pair(2048).await;
    let (namespace, name) = namespace_and_name(0);
    mpl.create_raw_rsa_keyring()
        .key_namespace(namespace.clone())
        .key_name(name.clone())
        .padding_scheme(aws_mpl_legacy::types::PaddingScheme::OaepSha1Mgf1)
        .public_key(keys.public_key.unwrap().pem.unwrap().as_ref())
        .private_key(keys.private_key.unwrap().pem.unwrap().as_ref())
        .send()
        .await
        .unwrap()
}

async fn get_aes_keyring(mpl: &MplClient) -> KeyringRef {
    let (namespace, name) = namespace_and_name(0);
    mpl.create_raw_aes_keyring()
        .key_namespace(namespace.clone())
        .key_name(name.clone())
        .wrapping_key(aws_smithy_types::Blob::new([0; 32]))
        .wrapping_alg(aws_mpl_legacy::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

async fn get_kms_keyring(mpl: &MplClient) -> KeyringRef {
    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();

    mpl.create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap()
}

async fn get_hierarchical_keyring(mpl: &MplClient) -> KeyringRef {
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);
    let ddb_client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let kms_config = KmsConfiguration::KmsKeyArn(HIERARCHY_KEY_ARN.to_string());

    let key_store_config = KeyStoreConfig::builder()
        .kms_client(kms_client)
        .ddb_client(ddb_client)
        .ddb_table_name(BRANCH_KEY_STORE_NAME)
        .logical_key_store_name(LOGICAL_KEY_STORE_NAME)
        .kms_configuration(kms_config)
        .build()
        .unwrap();

    let key_store = KeystoreClient::from_conf(key_store_config).unwrap();

    mpl.create_aws_kms_hierarchical_keyring()
        .key_store(key_store)
        .branch_key_id(BRANCH_KEY_ID)
        .ttl_seconds(600000)
        .send()
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_repr_encryption_context_with_same_ec_happy_case() {
    let asdf = "asdf".as_bytes();
    let mpl = mpl();

    // get keyrings
    let rsa_keyring = get_rsa_keyring(&mpl).await;
    let kms_keyring = get_kms_keyring(&mpl).await;
    let aes_keyring = get_aes_keyring(&mpl).await;
    let h_keyring = get_hierarchical_keyring(&mpl).await;

    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(aes_keyring.clone())
        .child_keyrings([rsa_keyring.clone(), kms_keyring.clone(), h_keyring.clone()])
        .send()
        .await
        .unwrap();

    // HAPPY CASE 1
    // Test supply same encryption context on encrypt and decrypt NO filtering
    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);

    let encrypt_input =
        EncryptInput::with_legacy_keyring(asdf, encryption_context.clone(), multi_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;

    // Test RSA
    let mut decrypt_input = DecryptInput::with_legacy_keyring(
        &esdk_ciphertext,
        encryption_context,
        rsa_keyring.clone(),
    );
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test KMS
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(kms_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test AES
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(aes_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test Hierarchy Keyring
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(h_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_remove_on_encrypt_and_supply_on_decrypt_happy_case() {
    let asdf = "asdf".as_bytes();
    let mpl = mpl();

    // get keyrings
    let rsa_keyring = get_rsa_keyring(&mpl).await;
    let kms_keyring = get_kms_keyring(&mpl).await;
    let aes_keyring = get_aes_keyring(&mpl).await;
    let h_keyring = get_hierarchical_keyring(&mpl).await;

    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(aes_keyring.clone())
        .child_keyrings([rsa_keyring.clone(), kms_keyring.clone(), h_keyring.clone()])
        .send()
        .await
        .unwrap();

    // Happy Test Case 2
    // On Encrypt we will only write one encryption context key value to the header
    // we will then supply only what we didn't write wth no required ec cmm,
    // This test case is checking that the default cmm is doing the correct filtering by using
    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);
    let reproduced_encryption_context =
        small_encryption_context(SmallEncryptionContextVariation::A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    let required_encryption_context_keys =
        small_encryption_context_keys(SmallEncryptionContextVariation::A);

    // TEST RSA
    let default_cmm = mpl
        .create_default_cryptographic_materials_manager()
        .keyring(multi_keyring.clone())
        .send()
        .await
        .unwrap();

    // Create Required EC CMM with the required EC Keys we want
    let req_cmm = mpl
        .create_required_encryption_context_cmm()
        .underlying_cmm(default_cmm)
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        .required_encryption_context_keys(required_encryption_context_keys)
        .send()
        .await
        .unwrap();

    let encrypt_input = EncryptInput::with_legacy_cmm(asdf, encryption_context, req_cmm);
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;

    // Test RSA
    let mut decrypt_input = DecryptInput::with_legacy_keyring(
        &esdk_ciphertext,
        reproduced_encryption_context,
        rsa_keyring.clone(),
    );
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test KMS
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(kms_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test AES
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(aes_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);

    // Test Hierarchy Keyring
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(h_keyring.clone()));
    let decrypt_output: DecryptOutput = decrypt(&decrypt_input).await.unwrap();
    assert!(decrypt_output.plaintext == asdf);
}

/*
  method {:test} TestRemoveOnEncryptAndSupplyOnDecryptHappyCase()
  {
    // The string "asdf" as bytes
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // Happy Test Case 2
    // On Encrypt we will only write one encryption context key value to the header
    // we will then supply only what we didn't write wth no required ec cmm,
    // This test case is checking that the default cmm is doing the correct filtering by using
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let reproducedEncryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    let requiredEncryptionContextKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    // TEST RSA
    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = None,
                                        keyring = Some(rsaKeyring),
                                        encryptionContext = Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    let cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only KMS keyring
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only AES keyring
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // Switch to only Hierarchical keyring
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

  }

  method {:test} TestRemoveOnEncryptRemoveAndSupplyOnDecryptHappyCase()
  {
    // The string "asdf" as bytes
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // HAPPY CASE 3
    // On Encrypt we will only write one encryption context key value to the header
    // we will then supply only what we didn't write but included in the signature while we
    // are configured with the required encryption context cmm
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let reproducedEncryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    let requiredEncryptionContextKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    // TEST RSA
    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        encryptionContext = Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    let cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST KMS
    // Switch to only KMS keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = kmsKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST AES
    // switch to only aes
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = aesKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST HIERARCHY
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = hKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;
  }

  method {:test} TestRemoveOnDecryptIsBackwardsCompatibleHappyCase()
  {
    // The string "asdf" as bytes
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // HAPPY CASE 4
    // On Encrypt we write all encryption context
    // as if the message was encrypted before the feature existed.
    // We will then have a required encryption context cmm
    // that will require us to supply the encryption context on decrypt.
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let reproducedEncryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);
    // These keys mean that we will not write these on the message but are required for message authentication on decrypt.
    let requiredEncryptionContextKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // All encryption context is stored in the message
    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(defaultCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Switch to only RSA keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        encryptionContext = Some(reproducedEncryptionContext)
                                      ));

    expect decryptOutput.Success?;
    let cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST KMS
    // Switch to only KMS keyring
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = kmsKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST AES
    // switch to only aes
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = aesKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;

    // TEST HIERARCHY
    defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = hKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredEncryptionContextKeys
      )
    );
    // Since we are passing in the correct reproduced encryption context this
    // decrypt SHOULD succeed
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = Some(reqCMM),
                                    keyring = None,
                                    encryptionContext = Some(reproducedEncryptionContext)
                                  ));

    expect decryptOutput.Success?;
    cycledPlaintext = decryptOutput.value.plaintext;
    expect cycledPlaintext == asdf;
  }

  method {:test} TestDifferentECOnDecryptFailure()
  {
    // encrypt {a, b} => decrypt {b:c} => fail
    // encrypt {a, b} => decrypt {d} => fail

    // The string "asdf" as bytes
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 1
    // Encrypt with and store all encryption context in header
    // On Decrypt supply additional encryption context not stored in the header; this MUST fail
    // On Decrypt supply mismatched encryption context key values; this MUST fail
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    // Additional EC
    let reproducedAdditionalEncryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.C);
    // Mismatched EncryptionContext
    let reproducedMismatchedEncryptionContext = Fixtures.SmallMismatchedEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = None,
                                        keyring = Some(multiKeyring),
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Test RSA Failures
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = None,
                                        keyring = Some(rsaKeyring),
                                        encryptionContext = Some(reproducedAdditionalEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(rsaKeyring),
                                    encryptionContext = Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = Some(reproducedMismatchedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = Some(reproducedAdditionalEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = Some(reproducedMismatchedEncryptionContext)
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
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 2
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signature.
    // Decrypt will not supply any reproduced Encryption Context; this MUST fail.
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let requiredECKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);

    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredECKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Test RSA Failure
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = None,
                                        keyring = Some(rsaKeyring),
                                        encryptionContext = None
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = None
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = None
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = None
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
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 3
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signature.
    // Decrypt will supply the correct key but incorrect value; this MUST fail.
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let requiredECKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);
    // this reproduced encryption context contains the key we didn't store, but it has the wrong value
    let mismatchedReproducedEncryptionContext = Fixtures.SmallMismatchedEncryptionContext(Fixtures.SmallEncryptionContextVariation.A);

    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredECKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Test RSA Failure
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = None,
                                        keyring = Some(rsaKeyring),
                                        encryptionContext = Some(mismatchedReproducedEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = Some(mismatchedReproducedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = Some(mismatchedReproducedEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failures
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = Some(mismatchedReproducedEncryptionContext)
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
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();
    let kmsKeyring = GetKmsKeyring();
    let aesKeyring = GetAesKeyring();
    let hKeyring = GetHierarchicalKeyring();

    let multiKeyring :- expect mpl.CreateMultiKeyring(mplTypes.CreateMultiKeyringInput(
                                                        generator = Some(aesKeyring),
                                                        childKeyrings = [rsaKeyring, kmsKeyring, hKeyring]
                                                      ));

    // FAILURE CASE 4
    // Encrypt will not store all Encryption Context, we will drop one entry but it will still get included in the
    // header signature.
    // Decrypt will supply the correct key but incorrect value; this MUST fail.
    let encryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.AB);
    let requiredECKeys = Fixtures.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation.A);
    // this reproduced encryption context does not contain the key that was dropped
    let droppedRequiredKeyEncryptionContext = Fixtures.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation.B);

    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = multiKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredECKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Success?;
    let esdkCiphertext = encryptOutput.value.ciphertext;

    // Test RSA Failure
    let decryptOutput = Decrypt(Types.DecryptInput(
                                        ciphertext = esdkCiphertext,
                                        materialsManager = None,
                                        keyring = Some(rsaKeyring),
                                        encryptionContext = Some(droppedRequiredKeyEncryptionContext)
                                      ));

    expect decryptOutput.Failure?;

    // Test KMS Failure
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(kmsKeyring),
                                    encryptionContext = Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test AES Failure
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(aesKeyring),
                                    encryptionContext = Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

    // Test Hierarchical Failure
    decryptOutput = Decrypt(Types.DecryptInput(
                                    ciphertext = esdkCiphertext,
                                    materialsManager = None,
                                    keyring = Some(hKeyring),
                                    encryptionContext = Some(droppedRequiredKeyEncryptionContext)
                                  ));

    expect decryptOutput.Failure?;

  }

  method {:test} TestReservedEncryptionContextKeyFailure()
  {
    // The string "asdf" as bytes
    let asdf = [ 97, 115, 100, 102 ];

    let defaultConfig = ESDK.DefaultAwsEncryptionSdkConfig();
    let esdk :- expect ESDK.ESDK(config = defaultConfig);
    let mpl :- expect MaterialProviders.MaterialProviders();

    // get keyrings
    let rsaKeyring = GetRsaKeyring();

    let encryptionContext = Fixtures.GetReservedECMap();
    let requiredECKeys = [Fixtures.RESERVED_ENCRYPTION_CONTEXT];

    let defaultCMM :- expect mpl.CreateDefaultCryptographicMaterialsManager(
      mplTypes.CreateDefaultCryptographicMaterialsManagerInput(
        keyring = rsaKeyring
      )
    );

    // Create Required EC CMM with the required EC Keys we want
    // Although we are requesting that we remove a RESERVED key word from the encryption context
    // The CMM instantiation will still succeed because the CMM is meant to work with different higher level
    // encryption libraries who may have different reserved keys. Encryption will ultimately fail.
    let reqCMM :- expect mpl.CreateRequiredEncryptionContextCMM(
      mplTypes.CreateRequiredEncryptionContextCMMInput(
        underlyingCMM = Some(defaultCMM),
        // At the moment reqCMM can only be created with a CMM, you cannot
        // create one by only passing in a keyring.
        keyring = None,
        requiredEncryptionContextKeys = requiredECKeys
      )
    );

    let encryptOutput = Encrypt(Types.EncryptInput(
                                        plaintext = asdf,
                                        encryptionContext = Some(encryptionContext),
                                        materialsManager = Some(reqCMM),
                                        keyring = None,
                                        algorithmSuiteId = None,
                                        frameLength = None
                                      ));

    expect encryptOutput.Failure?;

  }

  method GetHierarchicalKeyring()
    returns (output: mplTypes.IKeyring)
    ensures output.ValidState() && fresh(output) && fresh(output.History) && fresh(output.Modifies)
  {
    let branchKeyId = BRANCH_KEY_ID;
    let ttl : mplTypes.PositiveLong = (1 * 60000) * 10;
    let mpl :- expect MaterialProviders.MaterialProviders();

    let kmsClient :- expect KMS.KMSClient();
    let ddbClient :- expect DDB.DynamoDBClient();
    let kmsConfig = KeyStoreTypes.KMSConfiguration.kmsKeyArn(hierarchyKeyArn);

    let keyStoreConfig = KeyStoreTypes.KeyStoreConfig(
      id = None,
      kmsConfiguration = kmsConfig,
      logicalKeyStoreName = logicalKeyStoreName,
      grantTokens = None,
      ddbTableName = branchKeyStoreName,
      ddbClient = Some(ddbClient),
      kmsClient = Some(kmsClient)
    );

    let keyStore :- expect KeyStore.KeyStore(keyStoreConfig);

    output :- expect mpl.CreateAwsKmsHierarchicalKeyring(
      mplTypes.CreateAwsKmsHierarchicalKeyringInput(
        branchKeyId = Some(branchKeyId),
        branchKeyIdSupplier = None,
        keyStore = keyStore,
        ttlSeconds = ttl,
        cache = None
      ));
  }

*/
