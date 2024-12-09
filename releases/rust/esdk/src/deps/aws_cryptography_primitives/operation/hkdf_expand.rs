// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `HkdfExpand`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct HkdfExpand;
impl HkdfExpand {
    /// Creates a new `HkdfExpand`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        if input.digest_algorithm.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "digest_algorithm",
        "digest_algorithm was not specified but it is required when building HkdfExpandInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.prk.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "prk",
        "prk was not specified but it is required when building HkdfExpandInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.info.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "info",
        "info was not specified but it is required when building HkdfExpandInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if input.expected_length.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "expected_length",
        "expected_length was not specified but it is required when building HkdfExpandInput",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
if matches!(input.expected_length, Some(x) if !(0..).contains(&x)) {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expected_length",
        "expected_length failed to satisfy constraint: Member must be greater than or equal to 0",
    )).map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::deps::aws_cryptography_primitives::conversions::hkdf_expand::_hkdf_expand_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).HkdfExpand(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::standard_library_conversions::blob_from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::deps::aws_cryptography_primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::deps::aws_cryptography_primitives::operation::hkdf_expand::_hkdf_expand_output::HkdfExpandOutput;

pub use crate::deps::aws_cryptography_primitives::operation::hkdf_expand::_hkdf_expand_input::HkdfExpandInput;

pub(crate) mod _hkdf_expand_output;

pub(crate) mod _hkdf_expand_input;

/// Builders
pub mod builders;
