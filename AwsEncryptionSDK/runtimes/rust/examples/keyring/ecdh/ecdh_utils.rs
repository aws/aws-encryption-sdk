// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::aws_cryptography_primitives::types::EcdhCurveSpec;
use std::path::Path;
use std::io::Write;

// This file contains the helper functions for running ECDH examples

pub const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER: &str = "RawEcdhKeyringExamplePrivateKeySender.pem";
pub const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT: &str = "RawEcdhKeyringExamplePrivateKeyRecipient.pem";
pub const EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT: &str = "RawEcdhKeyringExamplePublicKeyRecipient.pem";

pub(crate) fn generate_ecc_key_pair(
    ecdh_curve_spec: EcdhCurveSpec
) -> Result<(String, String), crate::BoxError> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;

    // This code will generate new ECC keys for example use.
    // The public and private keys will be written to the files:
    //  - public: EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
    //  - private: EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
    // This example uses aws-lc-rs's KeyPairGenerator to generate the key pair.
    // In practice, you should not generate this in your code, and should instead
    // retrieve this key from a secure key management system (e.g. HSM)
    // These examples only demonstrate using the P256 curve while the keyring accepts
    // P256, P384, or P521.
    // This key is created here for example purposes only.
    let private_key =
        aws_lc_rs::agreement::PrivateKey::generate(super::ecdh_utils::get_alg(ecdh_curve_spec))
            .map_err(|e| format!("{:?}", e))?;

    let public_key = private_key
        .compute_public_key()
        .map_err(|e| format!("{:?}", e))?;

    let public_key: Vec<u8> = super::ecdh_utils::x962_to_x509(public_key.as_ref(), super::ecdh_utils::get_nid(ecdh_curve_spec))?;
    let public_key = pem::Pem::new("PUBLIC KEY", public_key);
    let public_key = pem::encode(&public_key);

    let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key)
        .map_err(|e| format!("{:?}", e))?;
    let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
    let private_key = pem::encode(&private_key);

    Ok((public_key, private_key))
}

pub(crate) fn x962_to_x509(
    public_key: &[u8],
    nid: i32
) -> Result<Vec<u8>, String> {
    use aws_lc_sys::EC_POINT_new;
    use aws_lc_sys::EC_GROUP_new_by_curve_name;
    use aws_lc_sys::EC_POINT_oct2point;
    use aws_lc_sys::EC_KEY_new_by_curve_name;
    use aws_lc_sys::EC_KEY_set_public_key;
    use aws_lc_sys::EVP_PKEY_new;
    use aws_lc_sys::EVP_PKEY_assign_EC_KEY;
    use aws_lc_sys::EVP_PKEY_size;
    use aws_lc_sys::EVP_marshal_public_key;
    use aws_lc_sys::CBB_finish;
    use aws_lc_sys::CBB_init;
    use aws_lc_sys::CBB;
    use aws_lc_sys::OPENSSL_free;
    use aws_lc_sys::EVP_PKEY_free;
    use aws_lc_sys::EC_POINT_free;
    use std::ptr::null_mut;

    let ec_group = unsafe { EC_GROUP_new_by_curve_name(nid) };
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

    let ec_key = unsafe { EC_KEY_new_by_curve_name(nid) };
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

pub(crate) fn get_nid(x: EcdhCurveSpec) -> i32 {
    match x {
        EcdhCurveSpec::EccNistP256 {} => aws_lc_sys::NID_X9_62_prime256v1,
        EcdhCurveSpec::EccNistP384 {} => aws_lc_sys::NID_secp384r1,
        EcdhCurveSpec::EccNistP521 {} => aws_lc_sys::NID_secp521r1,
        EcdhCurveSpec::Sm2 {} => panic!("No SM2 in Rust"),
    }
}

pub(crate) fn get_alg(x: EcdhCurveSpec) -> &'static aws_lc_rs::agreement::Algorithm {
    match x {
        EcdhCurveSpec::EccNistP256 {} => &aws_lc_rs::agreement::ECDH_P256,
        EcdhCurveSpec::EccNistP384 {} => &aws_lc_rs::agreement::ECDH_P384,
        EcdhCurveSpec::EccNistP521 {} => &aws_lc_rs::agreement::ECDH_P521,
        EcdhCurveSpec::Sm2 {} => panic!("No SM2 in Rust"),
    }
}

pub(crate) fn exists(f: &str) -> bool {
    Path::new(f).exists()
}

pub(crate) fn write_ecc_key_pair_all(
    ecdh_curve_spec: EcdhCurveSpec
) -> Result<(), crate::BoxError> {
    // Safety check: Validate neither file is present
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        || exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        || exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            return Err(crate::BoxError(
                "write_ecc_key_pair will not overwrite existing PEM files".to_string(),
            ));
        }

    let (_public_key_sender, private_key_sender) = super::ecdh_utils::generate_ecc_key_pair(ecdh_curve_spec)?;
    let (public_key_recipient, private_key_recipient) = super::ecdh_utils::generate_ecc_key_pair(ecdh_curve_spec)?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER))?
        .write_all(private_key_sender.as_bytes())?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT))?
        .write_all(private_key_recipient.as_bytes())?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?
        .write_all(public_key_recipient.as_bytes())?;

    Ok(())
}
