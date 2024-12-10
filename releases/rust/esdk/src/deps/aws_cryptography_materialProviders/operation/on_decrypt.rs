// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `OnDecrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OnDecrypt;
impl OnDecrypt {
    /// Creates a new `OnDecrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        keyring: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.materials.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "materials",
        "materials was not specified but it is required when building OnDecryptInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encrypted_data_keys.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encrypted_data_keys",
        "encrypted_data_keys was not specified but it is required when building OnDecryptInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
        keyring.inner.borrow_mut().on_decrypt(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::_on_decrypt_output::OnDecryptOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::_on_decrypt_input::OnDecryptInput;

pub(crate) mod _on_decrypt_output;

pub(crate) mod _on_decrypt_input;

/// Builders
pub mod builders;
