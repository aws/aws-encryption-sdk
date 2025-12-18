use crate::error::*;
use crate::suites::AlgorithmSuiteId;

pub const fn validate_commitment_policy_on_encrypt(_input : ValidateCommitmentPolicyOnEncryptInput) -> Result<(), Error>
{
    Ok(())
    // InvalidAlgorithmSuiteInfoOnEncrypt
}

pub const fn validate_commitment_policy_on_decrypt(_input : ValidateCommitmentPolicyOnDecryptInput) -> Result<(), Error>
{
    Ok(())
    // InvalidAlgorithmSuiteInfoOnDecrypt
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
#[non_exhaustive]
pub struct ValidateCommitmentPolicyOnEncryptInput {
  pub algorithm: AlgorithmSuiteId,
  pub commitment_policy: CommitmentPolicy,
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
#[non_exhaustive]
pub struct ValidateCommitmentPolicyOnDecryptInput {
  pub algorithm: AlgorithmSuiteId,
  pub commitment_policy: CommitmentPolicy,
}


// Commitment

//= aws-encryption-sdk-specification/framework/commitment-policy.md#supported-format-commitment-policy-enum
//= type=implication
//# The Material Providers Library MUST provide
//# a distinct commitment policy ENUM for each format.

//= aws-encryption-sdk-specification/framework/commitment-policy.md#supported-format-commitment-policy-enum
//= type=implication
//# | ESDK Commitment Policy ENUM     |
//# | ------------------------------- |
//# | FORBID_ENCRYPT_ALLOW_DECRYPT    |
//# | REQUIRE_ENCRYPT_ALLOW_DECRYPT   |
//# | REQUIRE_ENCRYPT_REQUIRE_DECRYPT |
#[derive(Debug, Clone, Default, PartialEq, Copy)]
#[non_exhaustive]
pub enum EsdkCommitmentPolicy {
    ForbidEncryptAllowDecrypt,
    RequireEncryptAllowDecrypt,
    #[default]
    RequireEncryptRequireDecrypt,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum DbeCommitmentPolicy {
    #[default]
    RequireEncryptRequireDecrypt,
}

//= aws-encryption-sdk-specification/framework/commitment-policy.md#supported-commitment-policy-enum
//= type=implication
//# This means that different formats MAY have duplicate Format Commitment Policy ENUM.

//= aws-encryption-sdk-specification/framework/commitment-policy.md#supported-commitment-policy-enum
//= type=implication
//# The Material Providers Library also MUST provide
//# a union ENUM for all distinct commitment policy ENUMs
//# called the Commitment Policy ENUM.
#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum CommitmentPolicy {
    Esdk(EsdkCommitmentPolicy),
    Dbe(DbeCommitmentPolicy),
}

impl Default for CommitmentPolicy {
    fn default() -> Self {
        Self::Esdk(EsdkCommitmentPolicy::default())
    }
}
