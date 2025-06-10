// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::SseDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEDescription::SSEDescription {
        Status: ::dafny_runtime::Rc::new(match &value.status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SSEType: ::dafny_runtime::Rc::new(match &value.sse_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KMSMasterKeyArn: crate::standard_library_conversions::ostring_to_dafny(&value.kms_master_key_arn),
 InaccessibleEncryptionDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.inaccessible_encryption_date_time),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEDescription,
    >,
) -> aws_sdk_dynamodb::types::SseDescription {
    aws_sdk_dynamodb::types::SseDescription::builder()
        .set_status(match &**dafny_value.Status() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::sse_status::from_dafny(value),
            ),
            _ => None,
        })
        .set_sse_type(match &**dafny_value.SSEType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => {
                Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_type::from_dafny(value))
            }
            _ => None,
        })
        .set_kms_master_key_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KMSMasterKeyArn().clone(),
        ))
        .set_inaccessible_encryption_date_time(
            crate::standard_library_conversions::otimestamp_from_dafny(
                dafny_value.InaccessibleEncryptionDateTime().clone(),
            ),
        )
        .build()
}
