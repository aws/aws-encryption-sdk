// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

// Extern methods with a foreign module name
#[allow(non_snake_case)]
pub mod RSAEncryption {
    pub mod RSA {
        use crate::_Wrappers_Compile as Wrappers;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode;
        use crate::*;
        use aws_lc_rs::encoding::{AsDer, Pkcs8V1Der, PublicKeyX509Der};
        use std::rc::Rc;

        use aws_lc_rs::rsa::KeySize;
        use aws_lc_rs::rsa::OaepAlgorithm;
        use aws_lc_rs::rsa::OaepPrivateDecryptingKey;
        use aws_lc_rs::rsa::OaepPublicEncryptingKey;
        use aws_lc_rs::rsa::Pkcs1PrivateDecryptingKey;
        use aws_lc_rs::rsa::Pkcs1PublicEncryptingKey;
        use aws_lc_rs::rsa::PrivateDecryptingKey;
        use aws_lc_rs::rsa::PublicEncryptingKey;
        use pem;
        use software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;

        pub fn key_size_from_length(length: i32) -> KeySize {
            match length {
                2048 => KeySize::Rsa2048,
                3072 => KeySize::Rsa3072,
                4096 => KeySize::Rsa4096,
                8192 => KeySize::Rsa8192,
                _ => panic!("Bad length for GenerateKeyPair"),
            }
        }

        fn error(s: &str) -> Rc<DafnyError> {
            Rc::new(DafnyError::AwsCryptographicPrimitivesError {
                message:
                    dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
            })
        }

        fn generate_key_pair(length_bits: i32) -> Result<(Vec<u8>, Vec<u8>), String> {
            // Generate an RSA key.
            let private_key = PrivateDecryptingKey::generate(key_size_from_length(length_bits))
                .map_err(|e| format!("{:?}", e))?;

            // Serialize the RSA private key to DER encoded PKCS#8 format for later usage.
            let private_key_der =
                AsDer::<Pkcs8V1Der>::as_der(&private_key).map_err(|e| format!("{:?}", e))?;

            // Retrieve the RSA public key
            let public_key = private_key.public_key();

            // Serialize the RSA public key to DER encoded X.509 SubjectPublicKeyInfo for later usage.
            let public_key_der =
                AsDer::<PublicKeyX509Der>::as_der(&public_key).map_err(|e| format!("{:?}", e))?;

            let public_key = pem::Pem::new("RSA PUBLIC KEY", public_key_der.as_ref());
            let public_key = pem::encode(&public_key);
            let private_key = pem::Pem::new("RSA PRIVATE KEY", private_key_der.as_ref());
            let private_key = pem::encode(&private_key);

            Ok((public_key.into(), private_key.into()))
        }
        #[allow(non_snake_case)]
        pub fn GenerateKeyPairExtern(
            length_bits: i32,
        ) -> (::dafny_runtime::Sequence<u8>, ::dafny_runtime::Sequence<u8>) {
            match generate_key_pair(length_bits) {
                Ok(x) => (x.0.iter().cloned().collect(), x.1.iter().cloned().collect()),
                Err(e) => {
                    panic!("Unexpected error generating RSA Key Pair{}", e);
                }
            }
        }

        fn get_alg_for_padding(mode: &RSAPaddingMode) -> Result<&'static OaepAlgorithm, String> {
            match mode {
                RSAPaddingMode::PKCS1 {} => {
                    Err("No support for RSA with PKCS1 in Rust.".to_string())
                }
                RSAPaddingMode::OAEP_SHA1 {} => Ok(&aws_lc_rs::rsa::OAEP_SHA1_MGF1SHA1),
                RSAPaddingMode::OAEP_SHA256 {} => Ok(&aws_lc_rs::rsa::OAEP_SHA256_MGF1SHA256),
                RSAPaddingMode::OAEP_SHA384 {} => Ok(&aws_lc_rs::rsa::OAEP_SHA384_MGF1SHA384),
                RSAPaddingMode::OAEP_SHA512 {} => Ok(&aws_lc_rs::rsa::OAEP_SHA512_MGF1SHA512),
            }
        }

        fn get_modulus(public_key: &[u8]) -> Result<u32, String> {
            let public_key = std::str::from_utf8(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = pem::parse(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = PublicEncryptingKey::from_der(public_key.contents())
                .map_err(|e| format!("{:?}", e))?;
            Ok(public_key.key_size_bits() as u32)
        }

        #[allow(non_snake_case)]
        pub fn GetRSAKeyModulusLengthExtern(
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<u32, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match get_modulus(&public_key) {
                Ok(v) => Rc::new(Wrappers::Result::Success { value: v }),
                Err(e) => Rc::new(Wrappers::Result::Failure { error: error(&e) }),
            }
        }

        fn decrypt_extern(
            mode: &RSAPaddingMode,
            private_key: &[u8],
            cipher_text: &[u8],
        ) -> Result<Vec<u8>, String> {
            let private_key =
                std::str::from_utf8(private_key).map_err(|e| format!("from_utf8 : {:?}", e))?;
            let private_key =
                pem::parse(private_key).map_err(|e| format!("pem::parse : {:?}", e))?;
            if mode == &(RSAPaddingMode::PKCS1 {}) {
                return decrypt_pkcs1(private_key.contents(), cipher_text);
            }
            let mode = get_alg_for_padding(mode)?;

            let private_key = PrivateDecryptingKey::from_pkcs8(private_key.contents())
                .map_err(|e| format!("from_pkcs8 : {:?}", e))?;
            let private_key =
                OaepPrivateDecryptingKey::new(private_key).map_err(|e| format!("new : {:?}", e))?;
            let mut message: Vec<u8> = vec![0; cipher_text.len()];
            let message = private_key
                .decrypt(mode, cipher_text, &mut message, None)
                .map_err(|e| format!("decrypt {:?}", e))?;
            Ok(message.to_vec())
        }

        #[allow(non_snake_case)]
        pub fn DecryptExtern(
            mode: &RSAPaddingMode,
            private_key: &::dafny_runtime::Sequence<u8>,
            cipher_text: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.iter().collect();
            let cipher_text: Vec<u8> = cipher_text.iter().collect();
            match decrypt_extern(mode, &private_key, &cipher_text) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("RSA Decrypt : {}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn encrypt_extern(
            mode: &RSAPaddingMode,
            public_key: &[u8],
            message: &[u8],
        ) -> Result<Vec<u8>, String> {
            let public_key = std::str::from_utf8(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = pem::parse(public_key).map_err(|e| format!("{:?}", e))?;
            if mode == &(RSAPaddingMode::PKCS1 {}) {
                return encrypt_pkcs1(public_key.contents(), message);
            }
            let mode = get_alg_for_padding(mode)?;

            let public_key = PublicEncryptingKey::from_der(public_key.contents())
                .map_err(|e| format!("{:?}", e))?;
            let public_key =
                OaepPublicEncryptingKey::new(public_key).map_err(|e| format!("{:?}", e))?;
            let mut ciphertext: Vec<u8> = vec![0; message.len() + public_key.key_size_bytes()];
            let cipher_text = public_key
                .encrypt(mode, message, &mut ciphertext, None)
                .map_err(|e| format!("{:?}", e))?;
            Ok(cipher_text.to_vec())
        }

        #[allow(non_snake_case)]
        pub fn EncryptExtern(
            mode: &RSAPaddingMode,
            public_key: &::dafny_runtime::Sequence<u8>,
            message: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            let message: Vec<u8> = message.iter().collect();
            match encrypt_extern(mode, &public_key, &message) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("RSA Encrypt : {}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }

        pub fn encrypt_pkcs1(public_key: &[u8], plain_text: &[u8]) -> Result<Vec<u8>, String> {
            let public_key =
                PublicEncryptingKey::from_der(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key =
                Pkcs1PublicEncryptingKey::new(public_key).map_err(|e| format!("{:?}", e))?;
            let mut ciphertext: Vec<u8> = vec![0; plain_text.len() + public_key.key_size_bytes()];
            let cipher_text = public_key
                .encrypt(plain_text, &mut ciphertext)
                .map_err(|e| format!("{:?}", e))?;
            Ok(cipher_text.to_vec())
        }

        pub fn decrypt_pkcs1(private_key: &[u8], cipher_text: &[u8]) -> Result<Vec<u8>, String> {
            let private_key = PrivateDecryptingKey::from_pkcs8(private_key)
                .map_err(|e| format!("from_pkcs8 : {:?}", e))?;
            let private_key = Pkcs1PrivateDecryptingKey::new(private_key)
                .map_err(|e| format!("new : {:?}", e))?;
            let mut message: Vec<u8> = vec![0; cipher_text.len()];
            let message = private_key
                .decrypt(cipher_text, &mut message)
                .map_err(|e| format!("decrypt {:?}", e))?;
            Ok(message.to_vec())
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_generate() {
                let (public_key, private_key) = GenerateKeyPairExtern(2048);

                let modulus: u32 = match &*GetRSAKeyModulusLengthExtern(&public_key) {
                    Wrappers::Result::Success { value } => *value,
                    Wrappers::Result::Failure { error } => panic!("{:?}", error),
                };
                assert_eq!(modulus, 2048);

                let mode = RSAPaddingMode::OAEP_SHA256 {};

                let plain_text: ::dafny_runtime::Sequence<u8> =
                    [1u8, 2, 3, 4, 5].iter().cloned().collect();

                let cipher: ::dafny_runtime::Sequence<u8> =
                    match &*EncryptExtern(&mode, &public_key, &plain_text) {
                        Wrappers::Result::Success { value } => value.clone(),
                        Wrappers::Result::Failure { error } => panic!("{:?}", error),
                    };

                let message: ::dafny_runtime::Sequence<u8> =
                    match &*DecryptExtern(&mode, &private_key, &cipher) {
                        Wrappers::Result::Success { value } => value.clone(),
                        Wrappers::Result::Failure { error } => panic!("{:?}", error),
                    };

                assert_eq!(plain_text, message);
            }
        }
    }
}
