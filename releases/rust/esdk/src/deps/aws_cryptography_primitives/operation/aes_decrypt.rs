// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `AesDecrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AesDecrypt;
impl AesDecrypt {
    /// Creates a new `AesDecrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_for_AwsCryptographicPrimitives_AESDecrypt(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_primitives::conversions::aes_decrypt::_aes_decrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).AESDecrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(crate::standard_library_conversions::blob_from_dafny(
                inner_result.value().clone(),
            ))
        } else {
            Err(
                crate::deps::aws_cryptography_primitives::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_primitives::operation::aes_decrypt::_aes_decrypt_output::AesDecryptOutput;

pub use crate::deps::aws_cryptography_primitives::operation::aes_decrypt::_aes_decrypt_input::AesDecryptInput;

pub(crate) mod _aes_decrypt_output;

pub(crate) mod _aes_decrypt_input;

/// Builders
pub mod builders;
