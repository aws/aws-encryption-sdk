// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example sets up an MRK multi-keyring and an MRK discovery
 multi-keyring using a custom client supplier.
 A custom client supplier grants users access to more granular
 configuration aspects of their authentication details and KMS
 client. In this example, we create a simple custom client supplier
 that authenticates with a different IAM role based on the
 region of the KMS key.

 This example creates a MRK multi-keyring configured with a custom
 client supplier using a single MRK and encrypts the example_data with it.
 Then, it creates a MRK discovery multi-keyring to decrypt the ciphertext.
*/

use super::regional_role_client_supplier::RegionalRoleClientSupplier;
use aws_esdk::*;
use aws_mpl_legacy::types::DiscoveryFilter;
use aws_mpl_legacy::types::error::Error::AwsCryptographicMaterialProvidersException;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    mrk_key_id_encrypt: &str,
    aws_account_id: &str,
    aws_regions: Vec<String>,
) -> Result<(), crate::BoxError> {
    // 2. Create encryption context.
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
    ]);

    // 3. Create a single MRK multi-keyring.
    //    This can be either a single-region KMS key or an MRK.
    //    For this example to succeed, the key's region must either
    //    1) be in the regions list, or
    //    2) the key must be an MRK with a replica defined
    //    in a region in the regions list, and the client
    //    must have the correct permissions to access the replica.
    let mpl = mpl();

    // Create the multi-keyring using our custom client supplier
    // defined in the RegionalRoleClientSupplier class in this directory.
    // Note: RegionalRoleClientSupplier will internally use the key_arn's region
    // to retrieve the correct IAM role.
    let mrk_keyring_with_client_supplier = mpl
        .create_aws_kms_mrk_multi_keyring()
        .client_supplier(RegionalRoleClientSupplier {})
        .generator(mrk_key_id_encrypt)
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context using the encrypt_keyring.
    let plaintext = example_data.as_bytes();
    let encrypt_input = EncryptInput::with_keyring(
        plaintext,
        encryption_context.clone(),
        mrk_keyring_with_client_supplier,
    );
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 5. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 6. Create a MRK discovery multi-keyring with a custom client supplier.
    //    A discovery MRK multi-keyring will be composed of
    //    multiple discovery MRK keyrings, one for each region.
    //    Each component keyring has its own KMS client in a particular region.
    //    When we provide a client supplier to the multi-keyring, all component
    //    keyrings will use that client supplier configuration.
    //    In our tests, we make `mrk_key_id_encrypt` an MRK with a replica, and
    //    provide only the replica region in our discovery filter.
    let discovery_filter = DiscoveryFilter::builder()
        .account_ids(vec![aws_account_id.to_string()])
        .partition("aws".to_string())
        .build()?;

    let mrk_discovery_client_supplier_keyring = mpl
        .create_aws_kms_mrk_discovery_multi_keyring()
        .client_supplier(RegionalRoleClientSupplier {})
        .discovery_filter(discovery_filter.clone())
        .regions(aws_regions)
        .send()
        .await?;

    // 7. Decrypt your encrypted data using the discovery multi keyring.
    // On Decrypt, the header of the encrypted message (ciphertext) will be parsed.
    // The header contains the Encrypted Data Keys (EDKs), which, if the EDK
    // was encrypted by a KMS Keyring, includes the KMS Key ARN.
    // For each member of the Multi Keyring, every EDK will try to be decrypted until a decryption
    // is successful.
    // Since every member of the Multi Keyring is a Discovery Keyring:
    //   Each Keyring will filter the EDKs by the Discovery Filter and the Keyring's region.
    //      For each filtered EDK, the keyring will attempt decryption with the keyring's client.
    // All of this is done serially, until a success occurs or all keyrings have failed
    // all (filtered) EDKs. KMS MRK Discovery Keyrings will attempt to decrypt
    // Multi Region Keys (MRKs) and regular KMS Keys.
    // Provide the encryption context that was supplied to the encrypt method
    let decrypt_input = DecryptInput::with_keyring(
        &ciphertext,
        encryption_context,
        mrk_discovery_client_supplier_keyring,
    );
    let decryption_response = decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 8. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 9. Test the Missing Region Exception
    // (This is an example for demonstration; you do not need to do this in your own code.)
    let mrk_discovery_client_supplier_keyring_missing_region = mpl
        .create_aws_kms_mrk_discovery_multi_keyring()
        .client_supplier(RegionalRoleClientSupplier {})
        .discovery_filter(discovery_filter)
        .regions(vec!["fake-region".to_string()])
        .send()
        .await;

    // Swallow the exception
    // (This is an example for demonstration; you do not need to do this in your own code.)
    match mrk_discovery_client_supplier_keyring_missing_region {
        Ok(_) => panic!(
            "Decryption using discovery keyring with missing region MUST \
                            raise AwsCryptographicMaterialProvidersException"
        ),
        Err(AwsCryptographicMaterialProvidersException { message: _e }) => (),
        _ => panic!("Unexpected error type"),
    }

    println!("Client Supplier Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Client Supplier example
    use crate::example_utils::utils;

    // Note that we pass in an MRK in us-east-1. The RegionalRoleClientSupplier
    // will internally use the key_arn's region (us-east-1)
    // to retrieve the correct IAM role.
    // and access its replica in eu-west-1
    let aws_regions: Vec<String> = vec!["eu-west-1".to_string()];

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        aws_regions,
    )
    .await?;

    Ok(())
}
