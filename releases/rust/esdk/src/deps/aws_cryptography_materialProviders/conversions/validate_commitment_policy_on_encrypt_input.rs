// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput::ValidateCommitmentPolicyOnEncryptInput {
        algorithm: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::to_dafny(&value.algorithm.clone().unwrap())
,
 commitmentPolicy: crate::deps::aws_cryptography_materialProviders::conversions::commitment_policy::to_dafny(&value.commitment_policy.clone().unwrap())
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
>>>{
    ::dafny_runtime::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::dafny_runtime::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput
{
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
) -> crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput
{
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput::ValidateCommitmentPolicyOnEncryptInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput::builder()
                .set_algorithm(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithm().clone())
 ))
 .set_commitment_policy(Some( crate::deps::aws_cryptography_materialProviders::conversions::commitment_policy::from_dafny(dafny_value.commitmentPolicy().clone())
 ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidateCommitmentPolicyOnEncryptInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
