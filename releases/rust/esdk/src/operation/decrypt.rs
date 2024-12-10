// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Decrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Decrypt;
impl Decrypt {
    /// Creates a new `Decrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::decrypt::DecryptInput,
    ) -> ::std::result::Result<
        crate::operation::decrypt::DecryptOutput,
        crate::types::error::Error,
    > {
        if input.ciphertext.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "ciphertext",
        "ciphertext was not specified but it is required when building DecryptInput",
    )).map_err(crate::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::conversions::decrypt::_decrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Decrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::decrypt::_decrypt_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::decrypt::_decrypt_output::DecryptOutput;

pub use crate::operation::decrypt::_decrypt_input::DecryptInput;

pub(crate) mod _decrypt_output;

pub(crate) mod _decrypt_input;

/// Builders
pub mod builders;
