// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DecompressPublicKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecompressPublicKey;
impl DecompressPublicKey {
    /// Creates a new `DecompressPublicKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_for_AwsCryptographicPrimitives_DecompressPublicKey(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_primitives::conversions::decompress_public_key::_decompress_public_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DecompressPublicKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_primitives::conversions::decompress_public_key::_decompress_public_key_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_primitives::operation::decompress_public_key::_decompress_public_key_output::DecompressPublicKeyOutput;

pub use crate::deps::aws_cryptography_primitives::operation::decompress_public_key::_decompress_public_key_input::DecompressPublicKeyInput;

pub(crate) mod _decompress_public_key_output;

pub(crate) mod _decompress_public_key_input;

/// Builders
pub mod builders;
