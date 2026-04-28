// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]
use aws_esdk::*;

// The following are test resources that exist in tests accounts:

// THESE ARE TESTING RESOURCES DO NOT USE IN A PRODUCTION ENVIRONMENT
pub const KEY_ARN: &str =
    "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f";
pub const HIERARCHY_KEY_ARN: &str =
    "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126";
pub const MRK_KEY_ARN: &str =
    "arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297";

pub const BRANCH_KEY_STORE_NAME: &str = "KeyStoreDdbTable";
pub const LOGICAL_KEY_STORE_NAME: &str = BRANCH_KEY_STORE_NAME;
pub const BRANCH_KEY_ID: &str = "3f43a9af-08c5-4317-b694-3d3e883dcaef";

// UTF-8 encoded "aws-crypto-"
pub const RESERVED_ENCRYPTION_CONTEXT: &str = "aws-crypto-";

pub enum SmallEncryptionContextVariation {
    Empty,
    A,
    B,
    AB,
    BA,
    C,
    CD,
}

pub fn small_encryption_context(v: SmallEncryptionContextVariation) -> EncryptionContext {
    let key_a = "keyA".to_string();
    let val_a = "valA".to_string();
    let key_b = "keyB".to_string();
    let val_b = "valB".to_string();
    let key_c = "keyC".to_string();
    let val_c = "valC".to_string();
    let key_d = "keyD".to_string();
    let val_d = "valD".to_string();

    let mut h = EncryptionContext::new();
    match v {
        SmallEncryptionContextVariation::Empty => {}
        SmallEncryptionContextVariation::A => {
            h.insert(key_a, val_a);
        }
        SmallEncryptionContextVariation::B => {
            h.insert(key_b, val_b);
        }
        SmallEncryptionContextVariation::AB => {
            h.insert(key_a, val_a);
            h.insert(key_b, val_b);
        }
        SmallEncryptionContextVariation::BA => {
            h.insert(key_b, val_b);
            h.insert(key_a, val_a);
        }
        SmallEncryptionContextVariation::C => {
            h.insert(key_c, val_c);
        }
        SmallEncryptionContextVariation::CD => {
            h.insert(key_c, val_c);
            h.insert(key_d, val_d);
        }
    }
    h
}

pub fn get_reserved_ec_map() -> EncryptionContext {
    let mut h = EncryptionContext::new();
    h.insert(
        "aws-crypto-public-key".to_string(),
        "not a real public key".to_string(),
    );
    h
}

pub fn small_encryption_context_keys(v: SmallEncryptionContextVariation) -> Vec<String> {
    let key_a = "keyA".to_string();
    let key_b = "keyB".to_string();
    let key_c = "keyC".to_string();
    let key_d = "keyD".to_string();
    match v {
        SmallEncryptionContextVariation::Empty => {
            vec![]
        }
        SmallEncryptionContextVariation::A => {
            vec![key_a]
        }
        SmallEncryptionContextVariation::B => {
            vec![key_b]
        }
        SmallEncryptionContextVariation::AB => {
            vec![key_a, key_b]
        }
        SmallEncryptionContextVariation::BA => {
            vec![key_b, key_a]
        }
        SmallEncryptionContextVariation::C => {
            vec![key_c]
        }
        SmallEncryptionContextVariation::CD => {
            vec![key_c, key_d]
        }
    }
}

pub fn small_mismatched_encryption_context(
    v: SmallEncryptionContextVariation,
) -> EncryptionContext {
    let key_a = "keyA".to_string();
    let val_a = "valA".to_string();
    let key_b = "keyB".to_string();
    let val_b = "valB".to_string();
    let key_c = "keyC".to_string();
    let val_c = "valC".to_string();
    let key_d = "keyD".to_string();
    let val_d = "valD".to_string();

    let mut h: EncryptionContext = EncryptionContext::new();
    match v {
        SmallEncryptionContextVariation::Empty => {}
        SmallEncryptionContextVariation::A => {
            h.insert(key_a, val_b);
        }
        SmallEncryptionContextVariation::B => {
            h.insert(key_b, val_a);
        }
        SmallEncryptionContextVariation::AB => {
            h.insert(key_a, val_c);
            h.insert(key_b, val_d);
        }
        SmallEncryptionContextVariation::BA => {
            h.insert(key_b, val_a);
            h.insert(key_a, val_b);
        }
        SmallEncryptionContextVariation::C => {
            h.insert(key_c, val_a);
        }
        SmallEncryptionContextVariation::CD => {
            h.insert(key_c, val_a);
            h.insert(key_d, val_b);
        }
    }
    h
}

pub fn namespace_and_name(n: u8) -> (String, String) {
    let s = "child".to_string() + &n.to_string() + " Namespace";
    let t = "child".to_string() + &n.to_string() + " Name";
    (s, t)
}

pub async fn generate_key_pair(
    key_modulus_length: aws_mpl_legacy::dafny::aes_gcm::AESEncryption::RSAModulusLengthBitsToGenerate,
) -> aws_mpl_legacy::dafny::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput
{
    let crypto = aws_mpl_legacy::dafny::aws_cryptography_primitives::client::Client::from_conf(
        aws_mpl_legacy::dafny::aws_cryptography_primitives::types::CryptoConfig::builder()
            .build()
            .unwrap(),
    )
    .unwrap();

    crypto
        .generate_rsa_key_pair()
        .length_bits(key_modulus_length)
        .send()
        .await
        .unwrap()
}
