// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ValidateCommitmentPolicyOnEncrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ValidateCommitmentPolicyOnEncrypt;
impl ValidateCommitmentPolicyOnEncrypt {
    /// Creates a new `ValidateCommitmentPolicyOnEncrypt`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptInput,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_for_AwsCryptographicMaterialProviders_ValidateCommitmentPolicyOnEncrypt(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::validate_commitment_policy_on_encrypt::_validate_commitment_policy_on_encrypt_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone())
            .ValidateCommitmentPolicyOnEncrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(())
        } else {
            Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::_validate_commitment_policy_on_encrypt_input::ValidateCommitmentPolicyOnEncryptInput;

pub(crate) mod _unit;

pub(crate) mod _validate_commitment_policy_on_encrypt_input;

/// Builders
pub mod builders;
