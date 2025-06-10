// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::SseSpecification,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification::SSESpecification {
        Enabled: crate::standard_library_conversions::obool_to_dafny(&value.enabled),
 SSEType: ::dafny_runtime::Rc::new(match &value.sse_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KMSMasterKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.kms_master_key_id),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification,
    >,
) -> aws_sdk_dynamodb::types::SseSpecification {
    aws_sdk_dynamodb::types::SseSpecification::builder()
        .set_enabled(crate::standard_library_conversions::obool_from_dafny(
            dafny_value.Enabled().clone(),
        ))
        .set_sse_type(match &**dafny_value.SSEType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => {
                Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_type::from_dafny(value))
            }
            _ => None,
        })
        .set_kms_master_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KMSMasterKeyId().clone(),
        ))
        .build()
}
