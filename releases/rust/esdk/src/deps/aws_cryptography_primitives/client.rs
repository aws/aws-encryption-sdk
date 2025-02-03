// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
#[allow(missing_docs)]
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::primitives::internaldafny::types::IAwsCryptographicPrimitivesClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        input: crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig,
    ) -> Result<Self, crate::deps::aws_cryptography_primitives::types::error::Error> {
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HCryptoConfig(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner =
            crate::software::amazon::cryptography::primitives::internaldafny::_default::AtomicPrimitives(
                &crate::deps::aws_cryptography_primitives::conversions::crypto_config::_crypto_config::to_dafny(input),
            );
        if matches!(
            inner.as_ref(),
            crate::_Wrappers_Compile::Result::Failure { .. }
        ) {
            return Err(
                crate::deps::aws_cryptography_primitives::conversions::error::from_dafny(
                    inner.as_ref().error().clone(),
                ),
            );
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract()),
        })
    }
}

mod generate_random_bytes;

mod digest;

mod h_mac;

mod hkdf_extract;

mod hkdf_expand;

mod hkdf;

mod kdf_counter_mode;

mod aes_kdf_counter_mode;

mod aes_encrypt;

mod aes_decrypt;

mod generate_rsa_key_pair;

mod get_rsa_key_modulus_length;

mod rsa_decrypt;

mod rsa_encrypt;

mod generate_ecdsa_signature_key;

mod ecdsa_sign;

mod ecdsa_verify;

mod generate_ecc_key_pair;

mod get_public_key_from_private_key;

mod validate_public_key;

mod derive_shared_secret;

mod compress_public_key;

mod decompress_public_key;

mod parse_public_key;
