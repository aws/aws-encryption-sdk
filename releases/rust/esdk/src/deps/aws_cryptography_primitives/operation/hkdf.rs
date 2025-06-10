// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Hkdf`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Hkdf;
impl Hkdf {
    /// Creates a new `Hkdf`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::hkdf::HkdfInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HHkdfInput_for_AwsCryptographicPrimitives_Hkdf(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input =
            crate::deps::aws_cryptography_primitives::conversions::hkdf::_hkdf_input::to_dafny(
                input,
            );
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone()).Hkdf(&inner_input);
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

pub use crate::deps::aws_cryptography_primitives::operation::hkdf::_hkdf_output::HkdfOutput;

pub use crate::deps::aws_cryptography_primitives::operation::hkdf::_hkdf_input::HkdfInput;

pub(crate) mod _hkdf_output;

pub(crate) mod _hkdf_input;

/// Builders
pub mod builders;
