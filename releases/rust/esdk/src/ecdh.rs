// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]
#![allow(dead_code)]

#[allow(non_snake_case)]
pub mod ECDH {
    use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
    use dafny_runtime::Rc;

    fn error(s: &str) -> Rc<DafnyError> {
        Rc::new(DafnyError::AwsCryptographicPrimitivesError {
            message:
                dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
        })
    }

    pub mod ECCUtils {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use crate::*;
        use aws_lc_sys_impl;
        use dafny_runtime::Rc;

        fn get_nid(x: &ECDHCurveSpec) -> i32 {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => aws_lc_sys_impl::NID_X9_62_prime256v1,
                ECDHCurveSpec::ECC_NIST_P384 {} => aws_lc_sys_impl::NID_secp384r1,
                ECDHCurveSpec::ECC_NIST_P521 {} => aws_lc_sys_impl::NID_secp521r1,
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }
        // NID_secp224r1 (NIST P-224),
        // NID_secp256k1 (SEC/ANSI P-256 K1)

        pub(crate) fn get_alg(x: &ECDHCurveSpec) -> &'static aws_lc_rs::agreement::Algorithm {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => &aws_lc_rs::agreement::ECDH_P256,
                ECDHCurveSpec::ECC_NIST_P384 {} => &aws_lc_rs::agreement::ECDH_P384,
                ECDHCurveSpec::ECC_NIST_P521 {} => &aws_lc_rs::agreement::ECDH_P521,
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }

        use aws_lc_sys_impl::CBB_finish;
        use aws_lc_sys_impl::CBB_init;
        use aws_lc_sys_impl::EC_GROUP_get_curve_name;
        use aws_lc_sys_impl::EC_GROUP_new_by_curve_name;
        use aws_lc_sys_impl::EC_KEY_get0_group;
        use aws_lc_sys_impl::EC_KEY_get0_public_key;
        use aws_lc_sys_impl::EC_KEY_new_by_curve_name;
        use aws_lc_sys_impl::EC_KEY_set_public_key;
        use aws_lc_sys_impl::EC_POINT_free;
        use aws_lc_sys_impl::EC_POINT_new;
        use aws_lc_sys_impl::EC_POINT_oct2point;
        use aws_lc_sys_impl::EC_POINT_point2oct;
        use aws_lc_sys_impl::EVP_PKEY_assign_EC_KEY;
        use aws_lc_sys_impl::EVP_PKEY_free;
        use aws_lc_sys_impl::EVP_PKEY_get0_EC_KEY;
        use aws_lc_sys_impl::EVP_PKEY_new;
        use aws_lc_sys_impl::EVP_PKEY_size;
        use aws_lc_sys_impl::EVP_marshal_public_key;
        use aws_lc_sys_impl::EVP_parse_public_key;
        use aws_lc_sys_impl::OPENSSL_free;
        use aws_lc_sys_impl::CBB;
        use aws_lc_sys_impl::CBS;
        use aws_lc_sys_impl::EVP_PKEY_EC;
        use std::ptr::null_mut;

        const ELEM_MAX_BITS: usize = 521;
        const ELEM_MAX_BYTES: usize = ELEM_MAX_BITS.div_ceil(8);
        const PUBLIC_KEY_MAX_LEN: usize = 1 + (2 * ELEM_MAX_BYTES);

        // This is the value checked in the Dafny test
        const INVALID_KEY: &str = "Invalid X509 Public Key.";

        pub(crate) fn X509_to_X962(
            public_key: &[u8],
            compress: bool,
            nid: Option<i32>,
        ) -> Result<Vec<u8>, String> {
            let mut cbs = CBS {
                data: public_key.as_ptr(),
                len: public_key.len(),
            };

            let evp_pkey = unsafe { EVP_parse_public_key(&mut cbs) };
            if evp_pkey.is_null() {
                return Err(INVALID_KEY.to_string());
            }
            let ec_key = unsafe { EVP_PKEY_get0_EC_KEY(evp_pkey) };

            let ec_group = unsafe { EC_KEY_get0_group(ec_key) };
            if ec_group.is_null() {
                return Err("Error in EC_KEY_get0_group in X509_to_X962.".to_string());
            }
            if nid.is_some() && nid.unwrap() != unsafe { EC_GROUP_get_curve_name(ec_group) } {
                return Err("Curve type mismatch in X509_to_X962.".to_string());
            }
            let ec_point = unsafe { EC_KEY_get0_public_key(ec_key) };
            if ec_point.is_null() {
                return Err("Error in EC_KEY_get0_public_key in X509_to_X962.".to_string());
            }

            let comp = if compress {
                aws_lc_sys_impl::point_conversion_form_t::POINT_CONVERSION_COMPRESSED
            } else {
                aws_lc_sys_impl::point_conversion_form_t::POINT_CONVERSION_UNCOMPRESSED
            };

            let mut out_buf = [0u8; PUBLIC_KEY_MAX_LEN];
            let new_size = unsafe {
                EC_POINT_point2oct(
                    ec_group,
                    ec_point,
                    comp,
                    out_buf.as_mut_ptr(),
                    PUBLIC_KEY_MAX_LEN,
                    null_mut(),
                )
            };
            unsafe { EVP_PKEY_free(evp_pkey) };
            Ok(out_buf[..new_size].to_vec())
        }

        pub(crate) fn X962_to_X509(
            public_key: &[u8],
            alg: &ECDHCurveSpec,
        ) -> Result<Vec<u8>, String> {
            let ec_group = unsafe { EC_GROUP_new_by_curve_name(get_nid(alg)) };
            let ec_point = unsafe { EC_POINT_new(ec_group) };

            if 1 != unsafe {
                EC_POINT_oct2point(
                    ec_group,
                    ec_point,
                    public_key.as_ptr(),
                    public_key.len(),
                    null_mut(),
                )
            } {
                return Err("Error in EC_POINT_oct2point.".to_string());
            }

            let ec_key = unsafe { EC_KEY_new_by_curve_name(get_nid(alg)) };
            if 1 != unsafe { EC_KEY_set_public_key(ec_key, ec_point) } {
                return Err("Error in EC_KEY_set_public_key.".to_string());
            }

            let evp_pkey = unsafe { EVP_PKEY_new() };
            if 1 != unsafe { EVP_PKEY_assign_EC_KEY(evp_pkey, ec_key) } {
                return Err("Error in EVP_PKEY_assign_EC_KEY.".to_string());
            }

            let key_size_bytes: usize = unsafe { EVP_PKEY_size(evp_pkey) }.try_into().unwrap();
            let mut cbb: CBB = Default::default();
            unsafe { CBB_init(&mut cbb as *mut CBB, key_size_bytes * 5) };

            if 1 != unsafe { EVP_marshal_public_key(&mut cbb, evp_pkey) } {
                return Err("Error in EVP_marshal_public_key in GetPublicKey.".to_string());
            };

            let mut out_data = null_mut::<u8>();
            let mut out_len: usize = 0;

            if 1 != unsafe { CBB_finish(&mut cbb, &mut out_data, &mut out_len) } {
                return Err("Error in CBB_finish in GetPublicKey.".to_string());
            };
            let slice = unsafe { std::slice::from_raw_parts(out_data, out_len) };
            let slice = slice.to_vec();

            unsafe { OPENSSL_free(out_data as *mut ::std::os::raw::c_void) };
            unsafe { EVP_PKEY_free(evp_pkey) };
            unsafe { EC_POINT_free(ec_point) };
            Ok(slice)
        }

        fn inner_get_public_key(
            key_bytes: &[u8],
            expected_curve_nid: i32,
        ) -> Result<Vec<u8>, String> {
            let mut out = null_mut();
            let evp_pkey = unsafe {
                aws_lc_sys_impl::d2i_PrivateKey(
                    EVP_PKEY_EC,
                    &mut out,
                    &mut key_bytes.as_ptr(),
                    key_bytes
                        .len()
                        .try_into()
                        .map_err(|_| "Key too long".to_string())?,
                )
            };
            if evp_pkey.is_null() {
                return Err("Error in d2i_PrivateKey in GetPublicKey.".to_string());
            }

            let ec_key = unsafe { EVP_PKEY_get0_EC_KEY(evp_pkey) };
            if ec_key.is_null() {
                return Err("Error in EVP_PKEY_get0_EC_KEY in GetPublicKey.".to_string());
            }
            let ec_group = unsafe { EC_KEY_get0_group(ec_key) };
            if ec_group.is_null() {
                return Err("Error in EC_KEY_get0_group in GetPublicKey.".to_string());
            }
            let key_nid = unsafe { EC_GROUP_get_curve_name(ec_group) };

            if key_nid != expected_curve_nid {
                return Err("Wrong Algorithm".to_string());
            }

            let key_size_bytes: usize = unsafe { EVP_PKEY_size(evp_pkey) }.try_into().unwrap();
            let mut cbb: CBB = Default::default();
            unsafe { CBB_init(&mut cbb as *mut CBB, key_size_bytes * 5) };

            if 1 != unsafe { EVP_marshal_public_key(&mut cbb, evp_pkey) } {
                return Err("Error in EVP_marshal_public_key in GetPublicKey.".to_string());
            };

            let mut out_data = null_mut::<u8>();
            let mut out_len: usize = 0;

            if 1 != unsafe { CBB_finish(&mut cbb, &mut out_data, &mut out_len) } {
                return Err("Error in CBB_finish in GetPublicKey.".to_string());
            };
            let slice = unsafe { std::slice::from_raw_parts(out_data, out_len) };
            let slice = slice.to_vec();

            unsafe { OPENSSL_free(out_data as *mut ::std::os::raw::c_void) };
            unsafe { EVP_PKEY_free(evp_pkey) };
            Ok(slice)
        }
        fn get_public_key(alg: &ECDHCurveSpec, pem: &[u8]) -> Result<Vec<u8>, String> {
            let pem = std::str::from_utf8(pem).map_err(|e| format!("{e:?}"))?;
            let private_key = pem::parse(pem).map_err(|e| format!("{e:?}"))?;
            inner_get_public_key(private_key.contents(), get_nid(alg))
        }

        fn get_out_of_bounds(curve: &ECDHCurveSpec) -> Vec<u8> {
            match curve {
                ECDHCurveSpec::ECC_NIST_P256 {} => vec![
                    48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61,
                    3, 1, 7, 3, 66, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                ],
                ECDHCurveSpec::ECC_NIST_P384 {} => vec![
                    48, 118, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3,
                    98, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255,
                ],
                ECDHCurveSpec::ECC_NIST_P521 {} => vec![
                    48, 129, 155, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0,
                    35, 3, 129, 134, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255,
                ],
                ECDHCurveSpec::SM2 {} => vec![],
            }
        }
        pub fn GetOutOfBoundsPublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let result = get_out_of_bounds(curve_algorithm);
            Rc::new(_Wrappers_Compile::Result::Success {
                value: dafny_runtime::Sequence::from_array_owned(result),
            })
        }

        fn get_infinity(curve: &ECDHCurveSpec) -> Vec<u8> {
            match curve {
                ECDHCurveSpec::ECC_NIST_P256 {} => vec![
                    48, 25, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61,
                    3, 1, 7, 3, 2, 0, 0,
                ],
                ECDHCurveSpec::ECC_NIST_P384 {} => vec![
                    48, 22, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3,
                    2, 0, 0,
                ],
                ECDHCurveSpec::ECC_NIST_P521 {} => vec![
                    48, 22, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 35, 3,
                    2, 0, 0,
                ],
                ECDHCurveSpec::SM2 {} => vec![],
            }
        }

        pub fn GetInfinityPublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let result = get_infinity(curve_algorithm);
            Rc::new(_Wrappers_Compile::Result::Success {
                value: dafny_runtime::Sequence::from_array_owned(result),
            })
        }
        pub fn GetPublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key = &private_key.pem().to_array();
            match get_public_key(curve_algorithm, private_key) {
                Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: dafny_runtime::Sequence::from_array_owned(x),
                }),
                Err(e) => {
                    let msg = format!("ECDH Get Public Key : {e}");
                    Rc::new(_Wrappers_Compile::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        // for the moment, it's valid if we can use it to generate a shared secret
        fn valid_public_key(alg: &ECDHCurveSpec, public_key: &[u8]) -> Result<(), String> {
            let mut cbs = CBS {
                data: public_key.as_ptr(),
                len: public_key.len(),
            };

            let evp_pkey = unsafe { EVP_parse_public_key(&mut cbs) };
            if evp_pkey.is_null() {
                return Err(INVALID_KEY.to_string());
            }
            let ec_key = unsafe { EVP_PKEY_get0_EC_KEY(evp_pkey) };

            if unsafe { aws_lc_sys_impl::EC_KEY_check_fips(ec_key) } != 1 {
                return Err(INVALID_KEY.to_string());
            }
            let ec_group = unsafe { EC_KEY_get0_group(ec_key) };
            if ec_group.is_null() {
                return Err(INVALID_KEY.to_string());
            }
            if get_nid(alg) != unsafe { EC_GROUP_get_curve_name(ec_group) } {
                return Err(INVALID_KEY.to_string());
            }
            unsafe { EVP_PKEY_free(evp_pkey) };

            Ok(())
        }

        pub fn ValidatePublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<_Wrappers_Compile::Result<bool, Rc<DafnyError>>> {
            let public_key = &public_key.to_array();
            match valid_public_key(curve_algorithm, public_key) {
                Ok(_) => Rc::new(_Wrappers_Compile::Result::Success { value: true }),
                Err(e) => Rc::new(_Wrappers_Compile::Result::Failure {
                    error: super::error(&e),
                }),
            }
        }

        pub fn CompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            _curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key = &public_key.to_array();
            match X509_to_X962(public_key, true, None) {
                Ok(v) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: dafny_runtime::Sequence::from_array_owned(v),
                }),
                Err(e) => {
                    let msg = format!("ECDH Compress Public Key {e}");
                    Rc::new(_Wrappers_Compile::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        pub fn DecompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key = &public_key.to_array();
            match X962_to_X509(public_key, curve_algorithm) {
                Ok(v) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: dafny_runtime::Sequence::from_array_owned(v),
                }),
                Err(e) => {
                    let msg = format!("ECDH Decompress Public Key {e}");
                    Rc::new(_Wrappers_Compile::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        pub fn ParsePublicKey(
            publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key = &publicKey.to_array();
            match X509_to_X962(public_key, false, None) {
                Ok(_) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: publicKey.clone(),
                }),
                Err(e) => Rc::new(_Wrappers_Compile::Result::Failure {
                    error: super::error(&e),
                }),
            }
        }
    }
    pub mod DeriveSharedSecret {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use crate::*;
        use dafny_runtime::Rc;

        pub fn agree(
            curve_algorithm: &ECDHCurveSpec,
            private_key_pem: &[u8],
            public_key_der: &[u8],
        ) -> Result<Vec<u8>, String> {
            let pem = std::str::from_utf8(private_key_pem).map_err(|e| format!("{e:?}"))?;
            let private_key = pem::parse(pem).map_err(|e| format!("{e:?}"))?;
            let private_key = aws_lc_rs::agreement::PrivateKey::from_private_key_der(
                super::ECCUtils::get_alg(curve_algorithm),
                private_key.contents(),
            )
            .map_err(|e| format!("{e:?}"))?;
            let public_key = super::ECCUtils::X509_to_X962(public_key_der, false, None)?;
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(
                super::ECCUtils::get_alg(curve_algorithm),
                &public_key,
            );
            let shared: Vec<u8> =
                aws_lc_rs::agreement::agree(&private_key, public_key, "foo", |x| Ok(x.to_vec()))
                    .map_err(|_e| "Failure in aws_lc_rs::agreement::agree.".to_string())?;
            Ok(shared)
        }
        pub fn CalculateSharedSecret(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
            public_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPublicKey>,
        ) -> Rc<_Wrappers_Compile::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key = &private_key.pem().to_array();
            let public_key = &public_key.der().to_array();
            match agree(curve_algorithm, private_key, public_key) {
                Ok(v) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: dafny_runtime::Sequence::from_array_owned(v),
                }),
                Err(e) => {
                    let msg = format!("ECDH Calculate Shared Secret : {e}");
                    Rc::new(_Wrappers_Compile::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    pub mod KeyGeneration {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use crate::*;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;
        use dafny_runtime::Rc;

        fn ecdsa_key_gen(alg: &ECDHCurveSpec) -> Result<(Vec<u8>, Vec<u8>), String> {
            let private_key =
                aws_lc_rs::agreement::PrivateKey::generate(super::ECCUtils::get_alg(alg))
                    .map_err(|e| format!("{e:?}"))?;

            let public_key = private_key
                .compute_public_key()
                .map_err(|e| format!("{e:?}"))?;

            let public_key: Vec<u8> = super::ECCUtils::X962_to_X509(public_key.as_ref(), alg)?;

            let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key)
                .map_err(|e| format!("{e:?}"))?;
            let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
            let private_key = pem::encode(&private_key);
            let private_key: Vec<u8> = private_key.into_bytes();

            Ok((public_key, private_key))
        }

        pub fn GenerateKeyPair(
            s: &Rc<ECDHCurveSpec>,
        ) -> Rc<_Wrappers_Compile::Result<Rc<crate::ECDH::EccKeyPair>, Rc<DafnyError>>> {
            match ecdsa_key_gen(s) {
                Ok(x) => Rc::new(_Wrappers_Compile::Result::Success {
                    value: Rc::new(crate::ECDH::EccKeyPair::EccKeyPair {
                        publicKey: dafny_runtime::Sequence::from_array_owned(x.0),
                        privateKey: dafny_runtime::Sequence::from_array_owned(x.1),
                    }),
                }),
                Err(e) => {
                    let msg = format!("ECDH Generate Key Pair : {e}");
                    Rc::new(_Wrappers_Compile::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::*;
        use dafny_runtime::Rc;

        #[test]
        fn test_generate() {
            let alg = Rc::new(ECDHCurveSpec::ECC_NIST_P256 {});

            let pair: crate::ECDH::EccKeyPair = match &*KeyGeneration::GenerateKeyPair(&alg) {
                _Wrappers_Compile::Result::Success { value } => (**value).clone(),
                _Wrappers_Compile::Result::Failure { error } => panic!("{error:?}"),
            };

            match &*ECCUtils::ValidatePublicKey(&alg, pair.publicKey()) {
                _Wrappers_Compile::Result::Success { .. } => {}
                _Wrappers_Compile::Result::Failure { error } => panic!("{error:?}"),
            };
        }
    }
}
