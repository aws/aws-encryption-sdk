// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput{
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput::GenerateRSAKeyPairOutput {
        publicKey: crate::deps::aws_cryptography_primitives::conversions::rsa_public_key::to_dafny(&value.public_key.clone().unwrap())
,
 privateKey: crate::deps::aws_cryptography_primitives::conversions::rsa_private_key::to_dafny(&value.private_key.clone().unwrap())
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
) -> crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput::GenerateRSAKeyPairOutput {..} =>
            crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput::builder()
                .set_public_key(Some( crate::deps::aws_cryptography_primitives::conversions::rsa_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
 .set_private_key(Some( crate::deps::aws_cryptography_primitives::conversions::rsa_private_key::from_dafny(dafny_value.privateKey().clone())
 ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput>
{
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
