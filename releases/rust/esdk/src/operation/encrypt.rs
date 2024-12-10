// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Encrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Encrypt;
impl Encrypt {
    /// Creates a new `Encrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::encrypt::EncryptInput,
    ) -> ::std::result::Result<
        crate::operation::encrypt::EncryptOutput,
        crate::types::error::Error,
    > {
        if input.plaintext.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "plaintext",
        "plaintext was not specified but it is required when building EncryptInput",
    )).map_err(crate::types::error::Error::wrap_validation_err);
}
if matches!(input.frame_length, Some(x) if !(1..=4294967296).contains(&x)) {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "frame_length",
        "frame_length failed to satisfy constraint: Member must be between 1 and 4294967296, inclusive",
    )).map_err(crate::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::conversions::encrypt::_encrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Encrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::encrypt::_encrypt_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::encrypt::_encrypt_output::EncryptOutput;

pub use crate::operation::encrypt::_encrypt_input::EncryptInput;

pub(crate) mod _encrypt_output;

pub(crate) mod _encrypt_input;

/// Builders
pub mod builders;
