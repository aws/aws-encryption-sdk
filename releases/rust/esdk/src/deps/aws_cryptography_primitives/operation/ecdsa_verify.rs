// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EcdsaVerify`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EcdsaVerify;
impl EcdsaVerify {
    /// Creates a new `EcdsaVerify`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput,
    ) -> ::std::result::Result<
        ::std::primitive::bool,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        if input.signature_algorithm.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "signature_algorithm",
        "signature_algorithm was not specified but it is required when building EcdsaVerifyInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.verification_key.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "verification_key",
        "verification_key was not specified but it is required when building EcdsaVerifyInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.message.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "message",
        "message was not specified but it is required when building EcdsaVerifyInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.signature.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "signature",
        "signature was not specified but it is required when building EcdsaVerifyInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::deps::aws_cryptography_primitives::conversions::ecdsa_verify::_ecdsa_verify_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ECDSAVerify(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                inner_result.value() .clone(),
            )
        } else {
            Err(crate::deps::aws_cryptography_primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::_ecdsa_verify_output::EcdsaVerifyOutput;

pub use crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::_ecdsa_verify_input::EcdsaVerifyInput;

pub(crate) mod _ecdsa_verify_output;

pub(crate) mod _ecdsa_verify_input;

/// Builders
pub mod builders;
