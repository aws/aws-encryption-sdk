// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Demonstrate an encrypt/decrypt cycle using a Required Encryption Context CMM.
A required encryption context CMM asks for required keys in the encryption context field
on encrypt such that they will not be stored on the message, but WILL be included in the header signature.
On decrypt, the client MUST supply the key/value pair(s) that were not stored to successfully decrypt the message.
*/

use aws_esdk::*;

pub async fn encrypt_and_decrypt_with_cmm(
    example_data: &str,
    kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 2. Create a KMS client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // 3. Create encryption context.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
    let encryption_context = EncryptionContext::from([
        ("encryption".to_string(), "context".to_string()),
        ("is not".to_string(), "secret".to_string()),
        ("but adds".to_string(), "useful metadata".to_string()),
        (
            "that can help you".to_string(),
            "be confident that".to_string(),
        ),
        (
            "the data you are handling".to_string(),
            "is what you think it is".to_string(),
        ),
        ("requiredKey1".to_string(), "requiredValue1".to_string()),
        ("requiredKey2".to_string(), "requiredValue2".to_string()),
    ]);

    // 4. Create your required encryption context keys.
    // These keys MUST be in your encryption context.
    // These keys and their corresponding values WILL NOT be stored on the message but will be used
    // for authentication.
    let required_encryption_context_keys: Vec<String> =
        vec!["requiredKey1".to_string(), "requiredKey2".to_string()];

    // 5. Create a KMS keyring
    let mpl = mpl();

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

    let encrypt_input = EncryptInput::with_legacy_cmm(
        plaintext,
        encryption_context.clone(),
        required_ec_cmm.clone(),
    );
    let encryption_response = encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 9. Decrypt your encrypted data using the same keyring you used on encrypt.
    // Provide the encryption context that was supplied to the encrypt method
    let mut decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response = decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 11. Attempt to decrypt your encrypted data using the same cryptographic material manager
    // you used on encrypt, but we won't pass the encryption context we DID NOT store on the message.
    // This will fail
    decrypt_input.source = Some(MaterialSource::LegacyCmm(required_ec_cmm.clone()));
    decrypt_input.encryption_context = EncryptionContext::new();
    let decryption_response_without_ec = decrypt(&decrypt_input).await;

    if decryption_response_without_ec.is_ok() {
        panic!(
            "Decrypt without encryption context MUST raise AwsCryptographicMaterialProvidersError"
        )
    }

    // 12. Decrypt your encrypted data using the same cryptographic material manager
    // you used to encrypt, but supply encryption context that contains ONLY the encryption context that
    // was NOT stored. This will pass.
    let reproduced_encryption_context = EncryptionContext::from([
        ("requiredKey1".to_string(), "requiredValue1".to_string()),
        ("requiredKey2".to_string(), "requiredValue2".to_string()),
    ]);

    decrypt_input.source = Some(MaterialSource::LegacyCmm(required_ec_cmm));
    decrypt_input.encryption_context = reproduced_encryption_context;

    let decryption_response_with_reproduced_ec = decrypt(&decrypt_input).await?;

    let decrypted_plaintext_with_reproduced_ec = decryption_response_with_reproduced_ec.plaintext;

    // Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_with_reproduced_ec, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 13. You can decrypt the ciphertext using the underlying cmm, but not providing the
    // encryption context with the request will result in an AwsCryptographicMaterialProvidersError

    // This will pass
    decrypt_input.source = Some(MaterialSource::LegacyCmm(underlying_cmm));
    decrypt_input.encryption_context = encryption_context;
    let decryption_response_with_ec_underlying_cmm = decrypt(&decrypt_input).await?;

    let decrypted_plaintext_with_ec_underlying_cmm =
        decryption_response_with_ec_underlying_cmm.plaintext;

    // Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_with_ec_underlying_cmm, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // This will fail
    decrypt_input.encryption_context = EncryptionContext::default();
    let decryption_response_without_ec_underlying_cmm = decrypt(&decrypt_input).await;

    if decryption_response_without_ec_underlying_cmm.is_ok() {
        panic!(
            "Decrypt without encryption context MUST raise AwsCryptographicMaterialProvidersError"
        )
    }

    println!("Required Encryption Context CMM Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_cmm() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Required Encryption Context CMM example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_cmm(utils::TEST_EXAMPLE_DATA, utils::TEST_DEFAULT_KMS_KEY_ID).await?;

    Ok(())
}
