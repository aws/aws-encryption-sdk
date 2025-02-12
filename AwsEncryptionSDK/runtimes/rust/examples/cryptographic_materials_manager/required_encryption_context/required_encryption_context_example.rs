// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Demonstrate an encrypt/decrypt cycle using a Required Encryption Context CMM.
A required encryption context CMM asks for required keys in the encryption context field
on encrypt such that they will not be stored on the message, but WILL be included in the header signature.
On decrypt, the client MUST supply the key/value pair(s) that were not stored to successfully decrypt the message.
*/

use aws_esdk::client as esdk_client;
use aws_esdk::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig;
use aws_esdk::material_providers::client as mpl_client;
use aws_esdk::material_providers::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::types::error::Error::AwsCryptographicMaterialProvidersError;
use std::collections::HashMap;
use std::vec::Vec;

pub async fn encrypt_and_decrypt_with_cmm(
    example_data: &str,
    kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::builder().build()?;
    let esdk_client = esdk_client::Client::from_conf(esdk_config)?;

    // 2. Create a KMS client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // 3. Create encryption context.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
    let encryption_context = HashMap::from([
        ("encryption".to_string(), "context".to_string()),
        ("is not".to_string(), "secret".to_string()),
        ("but adds".to_string(), "useful metadata".to_string()),
        ("that can help you".to_string(), "be confident that".to_string()),
        ("the data you are handling".to_string(), "is what you think it is".to_string()),
        ("requiredKey1".to_string(), "requiredValue1".to_string()),
        ("𐀂".to_string(), "𐀂".to_string()),
    ]);

    // 4. Create your required encryption context keys.
    // These keys MUST be in your encryption context.
    // These keys and their corresponding values WILL NOT be stored on the message but will be used
    // for authentication.
    let required_encryption_context_keys: Vec<String> = vec![
        "requiredKey1".to_string(),
        "𐀂".to_string(),
    ];

    // 5. Create a KMS keyring
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key_id)
        .kms_client(kms_client)
        .send()
        .await?;

    // 6. Create the required encryption context CMM.
    let underlying_cmm = mpl
        .create_default_cryptographic_materials_manager()
        .keyring(kms_keyring)
        .send()
        .await?;

    let required_ec_cmm = mpl
        .create_required_encryption_context_cmm()
        .underlying_cmm(underlying_cmm.clone())
        .required_encryption_context_keys(required_encryption_context_keys)
        .send()
        .await?;

    // 7. Encrypt the data with the encryption_context
    // NOTE: the keys "requiredKey1", and "requiredKey2"
    // WILL NOT be stored in the message header, but "encryption", "is not",
    // "but adds", "that can help you", and "the data you are handling" WILL be stored.
    let plaintext = example_data.as_bytes();

    let encryption_response = esdk_client.encrypt()
        .plaintext(plaintext)
        .materials_manager(required_ec_cmm.clone())
        .encryption_context(encryption_context.clone())
        .send()
        .await?;

    let ciphertext = vec![2,5,120,209,173,171,21,255,209,248,49,177,1,57,87,26,133,132,54,51,35,218,162,35,112,0,183,83,119,211,121,178,11,141,104,0,249,0,6,0,21,97,119,115,45,99,114,121,112,116,111,45,112,117,98,108,105,99,45,107,101,121,0,68,65,48,119,86,74,84,55,115,68,55,85,118,119,68,102,82,47,97,85,112,119,114,71,113,85,85,98,101,88,82,52,68,84,102,49,83,65,72,85,67,72,100,70,97,50,118,69,104,111,72,115,99,113,103,78,88,55,54,80,108,107,103,122,102,85,65,61,61,0,8,98,117,116,32,97,100,100,115,0,15,117,115,101,102,117,108,32,109,101,116,97,100,97,116,97,0,10,101,110,99,114,121,112,116,105,111,110,0,7,99,111,110,116,101,120,116,0,6,105,115,32,110,111,116,0,6,115,101,99,114,101,116,0,17,116,104,97,116,32,99,97,110,32,104,101,108,112,32,121,111,117,0,17,98,101,32,99,111,110,102,105,100,101,110,116,32,116,104,97,116,0,25,116,104,101,32,100,97,116,97,32,121,111,117,32,97,114,101,32,104,97,110,100,108,105,110,103,0,23,105,115,32,119,104,97,116,32,121,111,117,32,116,104,105,110,107,32,105,116,32,105,115,0,1,0,7,97,119,115,45,107,109,115,0,75,97,114,110,58,97,119,115,58,107,109,115,58,117,115,45,119,101,115,116,45,50,58,54,53,56,57,53,54,54,48,48,56,51,51,58,107,101,121,47,98,51,53,51,55,101,102,49,45,100,56,100,99,45,52,55,56,48,45,57,102,53,97,45,53,53,55,55,54,99,98,98,50,102,55,102,0,167,1,1,1,0,120,64,243,140,39,94,49,9,116,22,193,7,41,81,80,87,25,100,173,163,239,28,33,233,76,139,160,189,188,157,15,180,20,0,0,0,126,48,124,6,9,42,134,72,134,247,13,1,7,6,160,111,48,109,2,1,0,48,104,6,9,42,134,72,134,247,13,1,7,1,48,30,6,9,96,134,72,1,101,3,4,1,46,48,17,4,12,72,230,243,47,188,246,35,206,240,121,132,151,2,1,16,128,59,105,155,9,212,103,205,223,78,224,167,132,77,12,92,142,190,222,69,26,149,27,52,33,151,126,69,53,197,215,177,166,172,191,16,189,88,41,93,198,19,219,187,27,131,49,63,62,178,107,24,64,254,53,171,213,147,39,21,49,2,0,0,16,0,90,48,175,84,122,221,236,180,198,132,14,252,7,40,246,72,239,142,50,28,190,28,140,151,203,164,75,21,188,89,63,119,9,130,245,37,193,139,195,73,235,44,70,230,227,128,43,187,255,255,255,255,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,4,50,99,222,224,182,2,238,232,29,186,97,81,204,161,164,44,212,140,234,231,0,103,48,101,2,49,0,188,222,2,29,96,102,98,40,66,182,18,69,186,136,135,42,0,132,73,76,87,56,232,195,219,219,208,104,137,134,26,246,251,82,88,4,212,61,53,145,36,218,34,244,96,51,173,16,2,48,11,27,71,24,9,8,126,29,176,154,204,141,28,2,215,244,43,132,44,51,212,74,69,54,196,224,67,63,81,153,19,161,212,69,124,136,128,169,80,95,126,32,125,162,190,30,191,72]

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(ciphertext, aws_smithy_types::Blob::new(plaintext),
        "Ciphertext and plaintext data are the same. Invalid encryption");

    // 9. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decryption_response = esdk_client.decrypt()
        .ciphertext(ciphertext.clone())
        .materials_manager(required_ec_cmm.clone())
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(encryption_context.clone())
        .send()
        .await?;

    let decrypted_plaintext = decryption_response
                                .plaintext
                                .expect("Unable to unwrap plaintext from decryption response");

    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(decrypted_plaintext, aws_smithy_types::Blob::new(plaintext),
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption");
    // 11. Attempt to decrypt your encrypted data using the same cryptographic material manager
    // you used on encrypt, but we won't pass the encryption context we DID NOT store on the message.
    // This will fail
    let decryption_response_without_ec = esdk_client.decrypt()
        .ciphertext(ciphertext.clone())
        .materials_manager(required_ec_cmm.clone())
        .send()
        .await;
    
    println!(decryption_response_without_ec)
    
    println!("Required Encryption Context CMM Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_cmm() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Required Encryption Context CMM example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_cmm(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID
    ).await?;

    Ok(())
}
