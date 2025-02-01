// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetRsaKeyModulusLength`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetRsaKeyModulusLength;
impl GetRsaKeyModulusLength {
    /// Creates a new `GetRsaKeyModulusLength`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput_for_AwsCryptographicPrimitives_GetRSAKeyModulusLength(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_primitives::conversions::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetRSAKeyModulusLength(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_primitives::conversions::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(
                crate::deps::aws_cryptography_primitives::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_output::GetRsaKeyModulusLengthOutput;

pub use crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_input::GetRsaKeyModulusLengthInput;

pub(crate) mod _get_rsa_key_modulus_length_output;

pub(crate) mod _get_rsa_key_modulus_length_input;

/// Builders
pub mod builders;
