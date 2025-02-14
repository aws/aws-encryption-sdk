// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput;
use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
use crate::software::amazon::cryptography::primitives::internaldafny::types::AES_GCM;
use crate::*;
use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey};
use dafny_runtime::Rc;

struct DoAESEncryptOutput {
    cipher_text: Vec<u8>,
    auth_tag: Vec<u8>,
}

fn error(s: &str) -> Rc<DafnyError> {
    Rc::new(DafnyError::AwsCryptographicPrimitivesError {
        message:
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
    })
}

fn enc_result(s: &str) -> Rc<_Wrappers_Compile::Result<Rc<AESEncryptOutput>, Rc<DafnyError>>> {
    Rc::new(_Wrappers_Compile::Result::Failure { error: error(s) })
}

fn dec_result(
    s: &str,
) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
    Rc::new(_Wrappers_Compile::Result::Failure { error: error(s) })
}

#[allow(non_snake_case)]
pub mod AESEncryption {
    pub use crate::software::amazon::cryptography::primitives::internaldafny::types::*;
}
impl AES_GCM {
    fn get_alg(&self) -> Result<&'static aws_lc_rs::aead::Algorithm, String> {
        if *self.tagLength() != 16i32 {
            Err(format!(
                "Tag length of {} not supported in Rust. Tag length must be 16.",
                self.tagLength()
            ))
        } else if *self.ivLength() != 12i32 {
            Err(format!(
                "IV length of {} not supported in Rust. IV length must be 12.",
                self.ivLength()
            ))
        } else if *self.keyLength() == 32i32 {
            Ok(&aws_lc_rs::aead::AES_256_GCM)
        } else if *self.keyLength() == 24i32 {
            Ok(&aws_lc_rs::aead::AES_192_GCM)
        } else if *self.keyLength() == 16i32 {
            Ok(&aws_lc_rs::aead::AES_128_GCM)
        } else {
            Err(format!(
                "Key length of {} not supported in Rust. Key length must be 16, 24 or 32.",
                self.keyLength()
            ))
        }
    }

    fn do_aes_encrypt(
        &self,
        iv: &[u8],
        key: &[u8],
        msg: &[u8],
        aad: &[u8],
    ) -> Result<DoAESEncryptOutput, String> {
        let alg = self.get_alg()?;
        let mut in_out_buffer = Vec::from(msg);
        let key = UnboundKey::new(alg, key).map_err(|e| format!("new {:?}", e))?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        let tag = key
            .seal_in_place_separate_tag(nonce, aad, &mut in_out_buffer)
            .map_err(|e| format!("Seal {:?}", e))?;
        Ok(DoAESEncryptOutput {
            cipher_text: in_out_buffer,
            auth_tag: Vec::from(tag.as_ref()),
        })
    }

    fn do_aes_decrypt(
        &self,
        key: &[u8],
        cipher_text: &[u8],
        auth_tag: &[u8],
        iv: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, String> {
        let alg = self.get_alg()?;
        let mut out_buffer = Vec::from(cipher_text);
        let key = UnboundKey::new(alg, key).map_err(|e| format!("new {:?}", e))?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        key.open_separate_gather(nonce, aad, cipher_text, auth_tag, &mut out_buffer)
            .map_err(|e| format!("gather {:?}", e))?;
        Ok(out_buffer)
    }

    #[allow(non_snake_case)]
    pub fn AESEncryptExtern(
        &self,
        iv: &::dafny_runtime::Sequence<u8>,
        key: &::dafny_runtime::Sequence<u8>,
        msg: &::dafny_runtime::Sequence<u8>,
        aad: &::dafny_runtime::Sequence<u8>,
    ) -> Rc<_Wrappers_Compile::Result<Rc<AESEncryptOutput>, Rc<DafnyError>>> {
        let iv: Vec<u8> = iv.iter().collect();
        let key: Vec<u8> = key.iter().collect();
        let msg: Vec<u8> = msg.iter().collect();
        let aad: Vec<u8> = aad.iter().collect();

        if *self.keyLength() as usize != key.len() {
            let msg = format!(
                "AESEncrypt : algorithm key length was {} but actual key length was {}.",
                self.keyLength(),
                key.len()
            );
            return enc_result(&msg);
        }
        if *self.ivLength() as usize != iv.len() {
            let msg = format!(
                "AESEncrypt : algorithm nonce length was {} but actual nonce length was {}.",
                self.ivLength(),
                iv.len()
            );
            return enc_result(&msg);
        }

        match self.do_aes_encrypt(&iv, &key, &msg, &aad) {
            Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                value: Rc::new(AESEncryptOutput::AESEncryptOutput {
                    cipherText: x.cipher_text.iter().cloned().collect(),
                    authTag: x.auth_tag.iter().cloned().collect(),
                }),
            }),
            Err(e) => {
                let msg = format!("AES Encrypt : {}", e);
                enc_result(&msg)
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn AESDecryptExtern(
        &self,
        key: &::dafny_runtime::Sequence<u8>,
        cipher_text: &::dafny_runtime::Sequence<u8>,
        auth_tag: &::dafny_runtime::Sequence<u8>,
        iv: &::dafny_runtime::Sequence<u8>,
        aad: &::dafny_runtime::Sequence<u8>,
    ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
        let key: Vec<u8> = key.iter().collect();
        let cipher_text: Vec<u8> = cipher_text.iter().collect();
        let auth_tag: Vec<u8> = auth_tag.iter().collect();
        let iv: Vec<u8> = iv.iter().collect();
        let aad: Vec<u8> = aad.iter().collect();

        if *self.keyLength() as usize != key.len() {
            let msg = format!(
                "AESEncrypt : algorithm key length was {} but actual key length was {}.",
                self.keyLength(),
                key.len()
            );
            return dec_result(&msg);
        }

        if *self.ivLength() as usize != iv.len() {
            let msg = format!(
                "AESEncrypt : algorithm nonce length was {} but actual nonce length was {}.",
                self.ivLength(),
                iv.len()
            );
            return dec_result(&msg);
        }

        if *self.tagLength() as usize != auth_tag.len() {
            let msg = format!(
                "AESEncrypt : algorithm auth tag length was {} but actual auth tag length was {}.",
                self.tagLength(),
                auth_tag.len()
            );
            return dec_result(&msg);
        }

        match self.do_aes_decrypt(&key, &cipher_text, &auth_tag, &iv, &aad) {
            Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                value: x.iter().cloned().collect(),
            }),
            Err(e) => {
                let msg = format!("AES Decrypt : {}", e);
                dec_result(&msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate() {
        let iv: ::dafny_runtime::Sequence<u8> = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            .iter()
            .cloned()
            .collect();
        let key: ::dafny_runtime::Sequence<u8> = [
            2u8, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33,
        ]
        .iter()
        .cloned()
        .collect();
        let msg: ::dafny_runtime::Sequence<u8> = [2u8, 4, 6, 8, 10, 12].iter().cloned().collect();
        let aad: ::dafny_runtime::Sequence<u8> =
            [3u8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
                .iter()
                .cloned()
                .collect();

        let alg = AES_GCM::AES_GCM {
            keyLength: 32,
            tagLength: 16,
            ivLength: 12,
        };
        let cipher = match &*alg.AESEncryptExtern(&iv, &key, &msg, &aad) {
            _Wrappers_Compile::Result::Success { value } => value.clone(),
            _Wrappers_Compile::Result::Failure { error } => {
                panic!("AESEncryptExtern Failed : {:?}", error);
            }
        };

        let (cipher_text, auth_tag) = match &*cipher {
            AESEncryptOutput::AESEncryptOutput {
                cipherText,
                authTag,
            } => (cipherText, authTag),
        };

        let output = match &*alg.AESDecryptExtern(&key, &cipher_text, &auth_tag, &iv, &aad) {
            _Wrappers_Compile::Result::Success { value } => value.clone(),
            _Wrappers_Compile::Result::Failure { error } => {
                panic!("AESEncryptExtern Failed : {:?}", error);
            }
        };

        assert_eq!(output, msg);
    }
}
