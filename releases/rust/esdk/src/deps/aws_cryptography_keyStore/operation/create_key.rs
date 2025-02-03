// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateKey;
impl CreateKey {
    /// Creates a new `CreateKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
        input: crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    > {
        crate::deps::aws_cryptography_keyStore::validation::validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_for_KeyStore_CreateKey(&input)
            .map_err(crate::deps::aws_cryptography_keyStore::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_keyStore::conversions::create_key::_create_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_keyStore::conversions::create_key::_create_key_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_keyStore::operation::create_key::_create_key_output::CreateKeyOutput;

pub use crate::deps::aws_cryptography_keyStore::operation::create_key::_create_key_input::CreateKeyInput;

pub(crate) mod _create_key_output;

pub(crate) mod _create_key_input;

/// Builders
pub mod builders;
