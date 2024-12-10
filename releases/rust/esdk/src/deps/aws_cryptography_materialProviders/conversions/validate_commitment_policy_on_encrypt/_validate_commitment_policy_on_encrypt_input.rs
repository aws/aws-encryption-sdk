// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput::ValidateCommitmentPolicyOnEncryptInput {
        algorithm: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::to_dafny(&value.algorithm.clone().unwrap())
,
 commitmentPolicy: crate::deps::aws_cryptography_materialProviders::conversions::commitment_policy::to_dafny(&value.commitment_policy.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptInput {
    crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptInput::builder()
        .set_algorithm(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithm().clone())
 ))
 .set_commitment_policy(Some( crate::deps::aws_cryptography_materialProviders::conversions::commitment_policy::from_dafny(dafny_value.commitmentPolicy().clone())
 ))
        .build()
        .unwrap()
}
