// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

#[allow(non_snake_case)]
pub mod Signature {
    pub mod ECDSA {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDSASignatureAlgorithm;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use crate::*;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::rand::SystemRandom;
        use aws_lc_rs::signature::EcdsaKeyPair;
        use aws_lc_rs::signature::EcdsaSigningAlgorithm;
        use aws_lc_rs::signature::EcdsaVerificationAlgorithm;
        use aws_lc_rs::signature::KeyPair;
        use aws_lc_rs::signature::UnparsedPublicKey;
        use dafny_runtime::Rc;

        fn error(s: &str) -> Rc<DafnyError> {
            Rc::new(DafnyError::AwsCryptographicPrimitivesError {
                message:
                    dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
            })
        }

        fn get_alg(x: &ECDSASignatureAlgorithm) -> &'static EcdsaSigningAlgorithm {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => {
                    // &aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED_SIGNING
                    &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING
                }
                ECDSASignatureAlgorithm::ECDSA_P384 {} => {
                    // &aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED_SIGNING
                    &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING
                }
            }
        }

        fn get_ver_alg(x: &ECDSASignatureAlgorithm) -> &'static EcdsaVerificationAlgorithm {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => {
                    // &aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED
                    &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1
                }
                ECDSASignatureAlgorithm::ECDSA_P384 {} => {
                    // &aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED
                    &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1
                }
            }
        }

        fn get_nid(x: &ECDSASignatureAlgorithm) -> i32 {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => aws_lc_sys::NID_X9_62_prime256v1,
                ECDSASignatureAlgorithm::ECDSA_P384 {} => aws_lc_sys::NID_secp384r1,
            }
        }

        const ELEM_MAX_BITS: usize = 521;
        const ELEM_MAX_BYTES: usize = (ELEM_MAX_BITS + 7) / 8;
        const PUBLIC_KEY_MAX_LEN: usize = 1 + (2 * ELEM_MAX_BYTES);

        pub(crate) fn sec1_compress(
            data: &[u8],
            alg: &ECDSASignatureAlgorithm,
        ) -> Result<Vec<u8>, String> {
            sec1_convert(
                data,
                get_nid(alg),
                aws_lc_sys::point_conversion_form_t::POINT_CONVERSION_COMPRESSED,
            )
        }

        pub(crate) fn sec1_convert(
            data: &[u8],
            nid: i32,
            form: aws_lc_sys::point_conversion_form_t,
        ) -> Result<Vec<u8>, String> {
            use aws_lc_sys::EC_GROUP_new_by_curve_name;
            use aws_lc_sys::EC_POINT_free;
            use aws_lc_sys::EC_POINT_new;
            use aws_lc_sys::EC_POINT_oct2point;
            use aws_lc_sys::EC_POINT_point2oct;
            use std::ptr::null_mut;

            // no need to free ec_group
            let ec_group = unsafe { EC_GROUP_new_by_curve_name(nid) };
            if ec_group.is_null() {
                return Err("EC_GROUP_new_by_curve_name returned failure.".to_string());
            }

            let ec_point = unsafe { EC_POINT_new(ec_group) };
            if ec_point.is_null() {
                return Err("EC_POINT_new returned failure.".to_string());
            }
            let mut out_buf = [0u8; PUBLIC_KEY_MAX_LEN];

            let ret = unsafe {
                EC_POINT_oct2point(ec_group, ec_point, data.as_ptr(), data.len(), null_mut())
            };
            if ret == 0 {
                return Err("EC_POINT_oct2point returned failure.".to_string());
            }
            let new_size: usize = unsafe {
                EC_POINT_point2oct(
                    ec_group,
                    ec_point,
                    form,
                    out_buf.as_mut_ptr(),
                    PUBLIC_KEY_MAX_LEN,
                    null_mut(),
                )
            };
            unsafe { EC_POINT_free(ec_point) };
            Ok(out_buf[..new_size].to_vec())
        }

        fn ecdsa_key_gen(alg: &ECDSASignatureAlgorithm) -> Result<(Vec<u8>, Vec<u8>), String> {
            let pair = EcdsaKeyPair::generate(get_alg(alg)).map_err(|e| format!("{:?}", e))?;

            let public_key: Vec<u8> = sec1_compress(pair.public_key().as_ref(), alg)?;
            let private_key: Vec<u8> = pair.private_key().as_der().unwrap().as_ref().to_vec();
            Ok((public_key, private_key))
        }

        pub fn ExternKeyGen(
            alg: &Rc<ECDSASignatureAlgorithm>,
        ) -> Rc<_Wrappers_Compile::Result<Rc<Signature::SignatureKeyPair>, Rc<DafnyError>>>
        {
            match ecdsa_key_gen(alg) {
                Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: Rc::new(Signature::SignatureKeyPair::SignatureKeyPair {
                        verificationKey: x.0.iter().cloned().collect(),
                        signingKey: x.1.iter().cloned().collect(),
                    }),
                }),
                Err(e) => {
                    let msg = format!("ECDSA Key Gen : {}", e);
                    Rc::new(_Wrappers_Compile::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn ecdsa_sign_inner(
            alg: &ECDSASignatureAlgorithm,
            key: &[u8],
            msg: &[u8],
        ) -> Result<Vec<u8>, String> {
            let private_key = EcdsaKeyPair::from_private_key_der(get_alg(alg), key)
                .map_err(|e| format!("{:?}", e))?;
            let rng = SystemRandom::new();
            let sig = private_key
                .sign(&rng, msg)
                .map_err(|e| format!("{:?}", e))?;
            Ok(sig.as_ref().to_vec())
        }
        fn ecdsa_sign(
            alg: &ECDSASignatureAlgorithm,
            key: &[u8],
            msg: &[u8],
        ) -> Result<Vec<u8>, String> {
            // This loop can in theory run forever, but the chances of that are negligible.
            // We may want to consider failing, after some number of loops, if we can do so in a way consistent with other ESDKs.
            loop {
                let result = ecdsa_sign_inner(alg, key, msg)?;
                if (get_alg(alg) == &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING
                    && result.len() == 103)
                    || (get_alg(alg) == &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING
                        && result.len() == 71)
                {
                    return Ok(result);
                }
            }
        }

        pub fn Sign(
            alg: &Rc<ECDSASignatureAlgorithm>,
            key: &::dafny_runtime::Sequence<u8>,
            msg: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let key: Vec<u8> = key.iter().collect();
            let msg: Vec<u8> = msg.iter().collect();
            match ecdsa_sign(alg, &key, &msg) {
                Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDSA Sign : {}", e);
                    Rc::new(_Wrappers_Compile::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn ecdsa_verify(
            alg: &ECDSASignatureAlgorithm,
            key: &[u8],
            msg: &[u8],
            sig: &[u8],
        ) -> Result<bool, String> {
            let public_key = UnparsedPublicKey::new(get_ver_alg(alg), key);
            match public_key.verify(msg, sig) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }

        pub fn Verify(
            alg: &Rc<ECDSASignatureAlgorithm>,
            key: &::dafny_runtime::Sequence<u8>,
            msg: &::dafny_runtime::Sequence<u8>,
            sig: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<_Wrappers_Compile::Result<bool, Rc<DafnyError>>> {
            let key: Vec<u8> = key.iter().collect();
            let msg: Vec<u8> = msg.iter().collect();
            let sig: Vec<u8> = sig.iter().collect();
            match ecdsa_verify(alg, &key, &msg, &sig) {
                Ok(x) => Rc::new(_Wrappers_Compile::Result::Success { value: x }),
                Err(e) => {
                    let msg = format!("ECDSA Verify : {}", e);
                    Rc::new(_Wrappers_Compile::Result::Failure { error: error(&msg) })
                }
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use dafny_runtime::Rc;

            #[test]
            fn test_generate() {
                let alg = Rc::new(ECDSASignatureAlgorithm::ECDSA_P384 {});
                let key_pair = match &*ExternKeyGen(&alg) {
                    _Wrappers_Compile::Result::Success { value } => value.clone(),
                    _Wrappers_Compile::Result::Failure { error } => {
                        panic!("ExternKeyGen Failed : {:?}", error);
                    }
                };

                let (s_key, v_key) = match &*key_pair {
                    Signature::SignatureKeyPair::SignatureKeyPair {
                        signingKey,
                        verificationKey,
                    } => (signingKey, verificationKey),
                };

                let message: ::dafny_runtime::Sequence<u8> =
                    [1u8, 2, 3, 4, 5].iter().cloned().collect();

                let sig = match &*Sign(&alg, &s_key, &message) {
                    _Wrappers_Compile::Result::Success { value } => value.clone(),
                    _Wrappers_Compile::Result::Failure { error } => {
                        panic!("Sign Failed : {:?}", error);
                    }
                };

                let ver: bool = match &*Verify(&alg, &v_key, &message, &sig) {
                    _Wrappers_Compile::Result::Success { value } => value.clone(),
                    _Wrappers_Compile::Result::Failure { error } => {
                        panic!("Verify Failed : {:?}", error);
                    }
                };
                assert!(ver);

                let mut sig_vec: Vec<u8> = sig.iter().collect();
                sig_vec[0] = 42;
                let sig2: ::dafny_runtime::Sequence<u8> = sig_vec.iter().cloned().collect();
                assert!(sig != sig2);
                let ver2: bool = match &*Verify(&alg, &v_key, &message, &sig2) {
                    _Wrappers_Compile::Result::Success { value } => value.clone(),
                    _Wrappers_Compile::Result::Failure { error } => {
                        panic!("Verify Failed : {:?}", error);
                    }
                };
                assert!(!ver2);
            }
        }
    }
}
