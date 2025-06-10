// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Digest`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Digest;
impl Digest {
    /// Creates a new `Digest`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::digest::DigestInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HDigestInput_for_AwsCryptographicPrimitives_Digest(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input =
            crate::deps::aws_cryptography_primitives::conversions::digest::_digest_input::to_dafny(
                input,
            );
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone()).Digest(&inner_input);
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

pub use crate::deps::aws_cryptography_primitives::operation::digest::_digest_output::DigestOutput;

pub use crate::deps::aws_cryptography_primitives::operation::digest::_digest_input::DigestInput;

pub(crate) mod _digest_output;

pub(crate) mod _digest_input;

/// Builders
pub mod builders;
