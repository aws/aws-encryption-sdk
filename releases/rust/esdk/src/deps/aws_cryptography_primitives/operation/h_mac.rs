// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `HMac`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct HMac;
impl HMac {
    /// Creates a new `HMac`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_primitives::client::Client,
        input: crate::deps::aws_cryptography_primitives::operation::h_mac::HMacInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        crate::deps::aws_cryptography_primitives::validation::validate_aws_Pcryptography_Pprimitives_HHMacInput_for_AwsCryptographicPrimitives_HMac(&input)
            .map_err(crate::deps::aws_cryptography_primitives::types::error::Error::wrap_validation_err)?;
        let inner_input =
            crate::deps::aws_cryptography_primitives::conversions::h_mac::_h_mac_input::to_dafny(
                input,
            );
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone()).HMac(&inner_input);
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

pub use crate::deps::aws_cryptography_primitives::operation::h_mac::_h_mac_output::HMacOutput;

pub use crate::deps::aws_cryptography_primitives::operation::h_mac::_h_mac_input::HMacInput;

pub(crate) mod _h_mac_output;

pub(crate) mod _h_mac_input;

/// Builders
pub mod builders;
