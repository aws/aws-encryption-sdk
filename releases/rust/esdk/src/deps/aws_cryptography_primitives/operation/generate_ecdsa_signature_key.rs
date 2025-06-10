// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GenerateEcdsaSignatureKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GenerateEcdsaSignatureKey;
impl GenerateEcdsaSignatureKey {
    /// Creates a new `GenerateEcdsaSignatureKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput_for_AwsCryptographicPrimitives_GenerateECDSASignatureKey(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_primitives::conversions::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone())
            .GenerateECDSASignatureKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_primitives::conversions::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_output::GenerateEcdsaSignatureKeyOutput;

pub use crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_input::GenerateEcdsaSignatureKeyInput;

pub(crate) mod _generate_ecdsa_signature_key_output;

pub(crate) mod _generate_ecdsa_signature_key_input;

/// Builders
pub mod builders;
