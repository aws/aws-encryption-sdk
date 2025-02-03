// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_random::GenerateRandomInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomRequest::GenerateRandomRequest {
        NumberOfBytes: crate::standard_library_conversions::oint_to_dafny(value.number_of_bytes),
 CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
 Recipient: ::dafny_runtime::Rc::new(match &value.recipient {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::recipient_info::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomRequest,
    >,
) -> aws_sdk_kms::operation::generate_random::GenerateRandomInput {
    aws_sdk_kms::operation::generate_random::GenerateRandomInput::builder()
        .set_number_of_bytes(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.NumberOfBytes().clone(),
        ))
        .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.CustomKeyStoreId().clone(),
        ))
        .set_recipient(match (*dafny_value.Recipient()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::recipient_info::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .build()
        .unwrap()
}
