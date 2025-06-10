// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresResponse::DescribeCustomKeyStoresResponse {
        CustomKeyStores: ::dafny_runtime::Rc::new(match &value.custom_key_stores {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::custom_key_stores_list_entry::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 NextMarker: crate::standard_library_conversions::ostring_to_dafny(&value.next_marker),
 Truncated: crate::standard_library_conversions::obool_to_dafny(&Some(value.truncated)),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresResponse,
    >,
) -> aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput {
    aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput::builder()
          .set_custom_key_stores(match (*dafny_value.CustomKeyStores()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoresListEntry>| crate::deps::com_amazonaws_kms::conversions::custom_key_stores_list_entry::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_next_marker(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NextMarker().clone()))
 .set_truncated(crate::standard_library_conversions::obool_from_dafny(dafny_value.Truncated().clone()))
          .build()
}
