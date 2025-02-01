// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetKeyStoreInfo`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetKeyStoreInfo;
impl GetKeyStoreInfo {
    /// Creates a new `GetKeyStoreInfo`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
        input: crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::Unit,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    >{
        crate::deps::aws_cryptography_keyStore::validation::validate_smithy_Papi_HUnit_for_KeyStore_GetKeyStoreInfo(&input)
            .map_err(crate::deps::aws_cryptography_keyStore::types::error::Error::wrap_validation_err)?;
        let inner_input = ();
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone()).GetKeyStoreInfo();
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_keyStore::conversions::get_key_store_info::_get_key_store_info_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::_get_key_store_info_output::GetKeyStoreInfoOutput;

pub use crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::_unit::Unit;

pub(crate) mod _get_key_store_info_output;

pub(crate) mod _unit;

/// Builders
pub mod builders;
