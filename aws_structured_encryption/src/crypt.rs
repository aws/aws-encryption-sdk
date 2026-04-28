// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use crate::serialize::*;
use crate::utils::*;
use crate::*;
use aws_mpl_rs::suites::AlgorithmSuite;
use aws_mpl_rs::suites::DerivationAlgorithm;
use aws_mpl_rs::suites::Encrypt;

const ONE_THIRD_MAX_INT: u32 = u32::MAX / 3;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum EncryptionSelector {
    DoEncrypt,
    DoDecrypt,
}

// need to return an array, not a Vec
fn field_key(hkdf_output: &[u8], offset: u32) -> Result<Vec<u8>, Error>
//= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
//= type=implication
//# The `FieldKey` for a given key and offset MUST be the first 44 bytes
//# of the aes256ctr_stream
//# of the `FieldRootKey` and the `FieldKeyNonce` of three times the given offset.
{
    let key = aws_mpl_primitives::ctr_stream(
        &field_key_nonce(offset * 3),
        hkdf_output,
        KEY_SIZE + NONCE_SIZE,
    )?;
    Ok(key)
}

const AWS_DBE_FIELD: &[u8] = b"AwsDbeField";
const LABEL_COMMITMENT_KEY: &[u8] = b"AWS_DBE_COMMIT_KEY";
const LABEL_ENCRYPTION_KEY: &[u8] = b"AWS_DBE_DERIVE_KEY";

fn field_key_nonce(offset: u32) -> Vec<u8>
//= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
    //= type=implication
    //# The `FieldKeyNonce` for a given offset MUST be 16 bytes comprised of
    //# | Field         | Length   | Interpretation |
    //# | ------------- | -------- | -------------- |
    //# | "AwsDbeField" | 11       | Literal Ascii String |
    //# | 0x2c          | 1        | 44, the length of the eventual FieldKey |
    //# | offset        | 4        | 32 bit integer representation of offset |
{
    let mut ret = Vec::new();
    ret.extend_from_slice(AWS_DBE_FIELD);
    ret.push((KEY_SIZE + NONCE_SIZE) as u8);
    write_u32(&mut ret, offset);
    ret
}

fn get_aes_alg(x: Encrypt) -> Result<aws_mpl_primitives::AesGcm, Error> {
    match x {
        Encrypt::AesGcm(e) => Ok(e),
        _ => Err("Must be AesGcm".into()),
    }
}

fn get_hkdf_alg(x: &DerivationAlgorithm) -> Result<aws_mpl_primitives::DigestAlg, Error> {
    match x {
        DerivationAlgorithm::Hkdf(h) => Ok(h.hmac),
        _ => Err("Must be Hkdf".into()),
    }
}
fn get_hkdf_outlen(x: &DerivationAlgorithm) -> Result<usize, Error> {
    match x {
        DerivationAlgorithm::Hkdf(h) => Ok(h.output_key_length as usize),
        _ => Err("Must be Hkdf".into()),
    }
}
// suitable for header field
fn get_commit_key(alg: &AlgorithmSuite, key: &Key, msg_id: &MessageID) -> Result<Vec<u8>, Error>
//= specification/structured-encryption/header.md#commit-key
              //= type=implication
              //# The calculated Commitment Key MUST have length equal to the
              //# [algorithm suite's encryption key length](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-encryption-settings).

              //= specification/structured-encryption/header.md#commit-key
              //= type=implication
              //# The HKDF used to calculate the Commitment Key MUST be the
              //# [Commit Key KDF](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings)
              //# indicated by the algorithm suite.

              //= specification/structured-encryption/header.md#commit-key
              //= type=implication
              //# The `info` used for the HKDF function MUST be
              //# | Field                | Length   |
              //# | -------------------- | -------- |
              //# | "AWS_DBE_COMMIT_KEY" | 18       |
              //# | Message ID           | 32       |

              //= specification/structured-encryption/header.md#commit-key
              //= type=implication
              //# The HKDF calculation MUST use a supplied key, no salt, and an `info` as described above.
{
    let mut commit_key = vec![0; get_hkdf_outlen(&alg.commitment)?];
    aws_mpl_primitives::hkdf_no_salt(
        get_hkdf_alg(&alg.commitment)?,
        key,
        &[LABEL_COMMITMENT_KEY, msg_id],
        &mut commit_key,
    )?;
    Ok(commit_key)
}

// Encrypt a StructuredDataMap
pub(crate) fn encrypt(
    alg: &AlgorithmSuite,
    key: &Key,
    head: &header::PartialHeader,
    data: &[CanonCryptoItem],
) -> Result<Vec<CanonCryptoItem>, Error> {
    crypt(EncryptionSelector::DoEncrypt, alg, key, head, data)
}

// Decrypt a StructuredDataMap
pub(crate) fn decrypt(
    alg: &AlgorithmSuite,
    key: &Key,
    head: &header::PartialHeader,
    data: &[CanonCryptoItem],
) -> Result<Vec<CanonCryptoItem>, Error> {
    crypt(EncryptionSelector::DoDecrypt, alg, key, head, data)
}

// Encrypt or Decrypt a StructuredDataMap
fn crypt(
    mode: EncryptionSelector,
    alg: &AlgorithmSuite,
    key: &Key,
    head: &header::PartialHeader,
    data: &[CanonCryptoItem],
) -> Result<Vec<CanonCryptoItem>, Error>
//= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
            //= type=implication
            //# The HKDF algorithm used to calculate the Field Root Key MUST be the
            //# [Encryption Key KDF](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings)
            //# indicated by the algorithm suite, using a provided plaintext data key, no salt,
            //# and an info as calculated [above](#calculate-info)

            //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
            //= type=implication
            //# The `FieldRootKey` MUST be generated with the plaintext data key in the encryption materials
            //# and the Message ID generated for this Encrypted Structured Data.

            //= specification/structured-encryption/encrypt-path-structure.md#calculate-info
            //= type=implication
            //# The `info` used for the HKDF function MUST be
            //# | Field                | Length   |
            //# | -------------------- | -------- |
            //# | "AWS_DBE_DERIVE_KEY" | 18       |
            //# | Message ID           | 32       |
{
    //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
    //# The `FieldRootKey` MUST be generated with the plaintext data key in the encryption materials
    //# and the Message ID generated for this Encrypted Structured Data.
    let mut field_root_key = vec![0; get_hkdf_outlen(&alg.kdf)?];
    aws_mpl_primitives::hkdf_no_salt(
        get_hkdf_alg(&alg.kdf)?,
        key,
        &[LABEL_ENCRYPTION_KEY, &head.msg_id],
        &mut field_root_key,
    )?;

    //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
    //= type=implication
    //# The calculated Field Root MUST have length equal to the
    //# [algorithm suite's encryption key length](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-encryption-settings).
    crypt_list(mode, alg, &field_root_key.try_into().unwrap(), data)
}

// Encrypt or Decrypt each entry in keys, putting results in output

fn crypt_list(
    mode: EncryptionSelector,
    alg: &AlgorithmSuite,
    field_root_key: &Key,
    data: &[CanonCryptoItem],
) -> Result<Vec<CanonCryptoItem>, Error> {
    let mut result: Vec<CanonCryptoItem> = Vec::with_capacity(data.len());
    let mut pos: u32 = 0;
    for item in data {
        if item.action == CryptoAction::EncryptAndSign {
            let new_terminal = if mode == EncryptionSelector::DoEncrypt {
                encrypt_terminal(alg, field_root_key, pos, &item.key, &item.data)?
            } else {
                decrypt_terminal(alg, field_root_key, pos, &item.key, &item.data)?
            };
            pos += 1;
            let new_item = CanonCryptoItem {
                data: new_terminal,
                ..item.clone()
            };
            result.push(new_item);
        } else {
            result.push(item.clone());
        }
    }
    Ok(result)
}

// Encrypt a single Terminal
fn encrypt_terminal(
    alg: &AlgorithmSuite,
    field_root_key: &Key,
    offset: u32,
    path: &[u8],
    data: &StructuredDataTerminal,
) -> Result<StructuredDataTerminal, Error>
//= specification/structured-encryption/encrypt-path-structure.md#terminal-data-encryption
              //= type=implication
              //# The output encrypted Terminal Data MUST have a [Terminal Type Id](./structures.md#terminal-type-id)
              //# equal `0xFFFF`.

              //= specification/structured-encryption/encrypt-path-structure.md#terminal-data-encryption
              //= type=implication
              //# The output encrypted Terminal Data MUST have a [Terminal Value](./structures.md#terminal-value)
              //# with the following serialization:
                 // | Field                      | Length   |
                 // | -------------------------- | -------- |
                 // | Terminal Type Id           | 2        |
                 // | Encrypted Terminal Value   | Variable |

                 //= specification/structured-encryption/encrypt-path-structure.md#terminal-type-id
                 //= type=implication
                 //# Terminal Type Id MUST equal the input Terminal Data's Terminal Type Id.

                 //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
              //= type=implication
              //# The `Cipherkey` MUST be the first 32 bytes of the `FieldKey`

              //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
                 //= type=implication
                 //# The `Nonce` MUST be the remaining 12 bytes of the `FieldKey`
{
    let data_key: Vec<u8> = field_key(field_root_key, offset)?;
    //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
    //# The `Cipherkey` MUST be the first 32 bytes of the `FieldKey`
    let encryption_key: Key = data_key[..KEY_SIZE].try_into().unwrap();
    //= specification/structured-encryption/encrypt-path-structure.md#calculate-cipherkey-and-nonce
    //# The `Nonce` MUST be the remaining 12 bytes of the `FieldKey`
    let nonce: Nonce = data_key[KEY_SIZE..].try_into().unwrap();
    let value = &data.value;

    //= specification/structured-encryption/encrypt-path-structure.md#encrypted-terminal-value
    //# The Encrypted Terminal Value MUST be derived according to the following encryption:
    // - The encryption algorithm used is the
    //   [encryption algorithm](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-encryption-settings)
    //   indicated in the algorithm suite.
    // - The AAD is the [canonical path](./header.md#canonical-path) for this Terminal Data
    // - The [Cipherkey and Nonce](#calculate-cipherkey-and-nonce) are as calculated above.
    // - The plaintext is the [Terminal Value](./structures.md#terminal-value) for this Terminal Data.

    // TODO - make new aes_encrypt that takes &mut[u8]
    // let mut cipher_text = vec![0; value.len() + TYPE_ID_LEN + AUTH_TAG_SIZE];
    let mut cipher_text = Vec::with_capacity(value.len() + AUTH_TAG_SIZE);
    aws_mpl_primitives::aes_encrypt(
        get_aes_alg(alg.encrypt)?,
        &nonce,
        &encryption_key,
        value,
        path,
        &mut cipher_text,
    )?;
    let mut result = data.type_id.to_vec();
    result.extend_from_slice(&cipher_text);
    Ok(value_to_data(result, BINARY))
}

// Can we take data by value, and return it?
// Decrypt a single Terminal
fn decrypt_terminal(
    alg: &AlgorithmSuite,
    field_root_key: &Key,
    offset: u32,
    path: &[u8],
    data: &StructuredDataTerminal,
) -> Result<StructuredDataTerminal, Error> {
    let data_key: Vec<u8> = field_key(field_root_key, offset)?;
    let encryption_key: Key = data_key[..KEY_SIZE].try_into().unwrap();
    let nonce: Nonce = data_key[KEY_SIZE..].try_into().unwrap();
    let value = &data.value;
    need((AUTH_TAG_SIZE + 2) <= value.len(), "cipherTxt too short.")?;

    //= specification/structured-encryption/decrypt-path-structure.md#terminal-data-decryption
    //# The input [Terminal Value](./structures.md#terminal-value) MUST be deserialized as follows:
    // | Field                      | Length   |
    // | -------------------------- | -------- |
    // | Terminal Type Id           | 2        |
    // | Encrypted Terminal Value   | Variable |

    //= specification/structured-encryption/decrypt-path-structure.md#terminal-data-decryption
    //# The output Terminal Data MUST have a [Terminal Value](./structures.md#terminal-type-id)
    //# equal to the following decryption:
    // - The decryption algorithm used is the
    //   [encryption algorithm](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-encryption-settings)
    //   indicated in the algorithm suite.
    // - The AAD is the [canonical path](./header.md#canonical-path) for this Terminal Data
    // - The Cipherkey and Nonce are as calculate [above](#calculate-cipherkey-and-nonce).
    // - The ciphertext is the deserialized Encrypted Terminal Value.
    let mut plain_text = vec![0; value.len() - TYPE_ID_LEN - AUTH_TAG_SIZE];
    aws_mpl_primitives::aes_decrypt(
        get_aes_alg(alg.encrypt)?,
        &encryption_key,
        &value[TYPE_ID_LEN..value.len() - AUTH_TAG_SIZE],
        &value[value.len() - AUTH_TAG_SIZE..],
        &nonce,
        path,
        &mut plain_text,
    )?;

    Ok(value_to_data(
        plain_text,
        value[..TYPE_ID_LEN].try_into().unwrap(),
    ))
}
