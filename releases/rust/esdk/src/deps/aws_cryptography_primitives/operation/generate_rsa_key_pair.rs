// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GenerateRsaKeyPair`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GenerateRsaKeyPair;
impl GenerateRsaKeyPair {
    /// Creates a new `GenerateRsaKeyPair`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput_for_AwsCryptographicPrimitives_GenerateRSAKeyPair(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_primitives::conversions::generate_rsa_key_pair::_generate_rsa_key_pair_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GenerateRSAKeyPair(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_primitives::conversions::generate_rsa_key_pair::_generate_rsa_key_pair_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::_generate_rsa_key_pair_output::GenerateRsaKeyPairOutput;

pub use crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::_generate_rsa_key_pair_input::GenerateRsaKeyPairInput;

pub(crate) mod _generate_rsa_key_pair_output;

pub(crate) mod _generate_rsa_key_pair_input;

/// Builders
pub mod builders;
