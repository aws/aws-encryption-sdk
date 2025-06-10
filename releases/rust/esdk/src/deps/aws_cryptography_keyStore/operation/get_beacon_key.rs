// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetBeaconKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBeaconKey;
impl GetBeaconKey {
    /// Creates a new `GetBeaconKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
        input: crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    > {
        crate::deps::aws_cryptography_keyStore::validation::validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput_for_KeyStore_GetBeaconKey(&input)
            .map_err(crate::deps::aws_cryptography_keyStore::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_keyStore::conversions::get_beacon_key::_get_beacon_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetBeaconKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_keyStore::conversions::get_beacon_key::_get_beacon_key_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(
                crate::deps::aws_cryptography_keyStore::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::_get_beacon_key_output::GetBeaconKeyOutput;

pub use crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::_get_beacon_key_input::GetBeaconKeyInput;

pub(crate) mod _get_beacon_key_output;

pub(crate) mod _get_beacon_key_input;

/// Builders
pub mod builders;
