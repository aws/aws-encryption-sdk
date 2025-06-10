// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateKeyStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateKeyStore;
impl CreateKeyStore {
    /// Creates a new `CreateKeyStore`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
        input: crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    > {
        crate::deps::aws_cryptography_keyStore::validation::validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreInput_for_KeyStore_CreateKeyStore(&input)
            .map_err(crate::deps::aws_cryptography_keyStore::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_keyStore::conversions::create_key_store::_create_key_store_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateKeyStore(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_keyStore::conversions::create_key_store::_create_key_store_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_keyStore::operation::create_key_store::_create_key_store_output::CreateKeyStoreOutput;

pub use crate::deps::aws_cryptography_keyStore::operation::create_key_store::_create_key_store_input::CreateKeyStoreInput;

pub(crate) mod _create_key_store_output;

pub(crate) mod _create_key_store_input;

/// Builders
pub mod builders;
