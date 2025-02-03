// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `OnEncrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OnEncrypt;
impl OnEncrypt {
    /// Creates a new `OnEncrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        keyring: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput_for_Keyring_OnEncrypt(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        keyring.inner.lock().unwrap().on_encrypt(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::_on_encrypt_output::OnEncryptOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::_on_encrypt_input::OnEncryptInput;

pub(crate) mod _on_encrypt_output;

pub(crate) mod _on_encrypt_input;

/// Builders
pub mod builders;
