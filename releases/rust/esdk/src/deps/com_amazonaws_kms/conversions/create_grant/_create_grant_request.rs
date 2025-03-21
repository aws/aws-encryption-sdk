// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::create_grant::CreateGrantInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantRequest::CreateGrantRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 GranteePrincipal: crate::standard_library_conversions::ostring_to_dafny(&value.grantee_principal) .Extract(),
 RetiringPrincipal: crate::standard_library_conversions::ostring_to_dafny(&value.retiring_principal),
 Operations: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.operations.clone().unwrap(),
    |e| crate::deps::com_amazonaws_kms::conversions::grant_operation::to_dafny(e.clone()),
)
,
 Constraints: ::dafny_runtime::Rc::new(match &value.constraints {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::grant_constraints::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GrantTokens: ::dafny_runtime::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 Name: crate::standard_library_conversions::ostring_to_dafny(&value.name),
 DryRun: crate::standard_library_conversions::obool_to_dafny(&value.dry_run),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantRequest,
    >,
) -> aws_sdk_kms::operation::create_grant::CreateGrantInput {
    aws_sdk_kms::operation::create_grant::CreateGrantInput::builder()
          .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_grantee_principal(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.GranteePrincipal()) ))
 .set_retiring_principal(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RetiringPrincipal().clone()))
 .set_operations(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.Operations(),
    |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation>| crate::deps::com_amazonaws_kms::conversions::grant_operation::from_dafny(e),
)
 ))
 .set_constraints(match (*dafny_value.Constraints()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::grant_constraints::from_dafny(value.clone())),
    _ => None,
}
)
 .set_grant_tokens(match (*dafny_value.GrantTokens()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Name().clone()))
 .set_dry_run(crate::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
          .build()
          .unwrap()
}
