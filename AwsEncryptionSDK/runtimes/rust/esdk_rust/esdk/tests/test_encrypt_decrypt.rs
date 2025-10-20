// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
use aws_esdk::client::Client as EsdkClient;
use aws_esdk::types::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .keyring(kms_keyring)
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());
}
