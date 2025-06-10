// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::GrantListEntry,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantListEntry,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantListEntry::GrantListEntry {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 GrantId: crate::standard_library_conversions::ostring_to_dafny(&value.grant_id),
 Name: crate::standard_library_conversions::ostring_to_dafny(&value.name),
 CreationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date),
 GranteePrincipal: crate::standard_library_conversions::ostring_to_dafny(&value.grantee_principal),
 RetiringPrincipal: crate::standard_library_conversions::ostring_to_dafny(&value.retiring_principal),
 IssuingAccount: crate::standard_library_conversions::ostring_to_dafny(&value.issuing_account),
 Operations: ::dafny_runtime::Rc::new(match &value.operations {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::grant_operation::to_dafny(e.clone()),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 Constraints: ::dafny_runtime::Rc::new(match &value.constraints {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::grant_constraints::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantListEntry,
    >,
) -> aws_sdk_kms::types::GrantListEntry {
    aws_sdk_kms::types::GrantListEntry::builder()
          .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_grant_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantId().clone()))
 .set_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Name().clone()))
 .set_creation_date(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDate().clone()))
 .set_grantee_principal(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GranteePrincipal().clone()))
 .set_retiring_principal(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RetiringPrincipal().clone()))
 .set_issuing_account(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IssuingAccount().clone()))
 .set_operations(match (*dafny_value.Operations()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation>| crate::deps::com_amazonaws_kms::conversions::grant_operation::from_dafny(e),
            )
        ),
    _ => None
}
)
 .set_constraints(match (*dafny_value.Constraints()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::grant_constraints::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
