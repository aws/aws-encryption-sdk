// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput::CreateRawEcdhKeyringInput {
        KeyAgreementScheme: crate::deps::aws_cryptography_materialProviders::conversions::raw_ecdh_static_configurations::to_dafny(&value.key_agreement_scheme.clone().unwrap())
,
 curveSpec: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.curve_spec.clone().unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput::CreateRawEcdhKeyringInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput::builder()
                .set_key_agreement_scheme(Some( crate::deps::aws_cryptography_materialProviders::conversions::raw_ecdh_static_configurations::from_dafny(dafny_value.KeyAgreementScheme().clone())
 ))
 .set_curve_spec(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.curveSpec()) ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawEcdhKeyringInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
