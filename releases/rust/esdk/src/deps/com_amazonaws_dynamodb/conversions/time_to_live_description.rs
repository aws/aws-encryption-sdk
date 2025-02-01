// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TimeToLiveDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveDescription::TimeToLiveDescription {
        TimeToLiveStatus: ::dafny_runtime::Rc::new(match &value.time_to_live_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 AttributeName: crate::standard_library_conversions::ostring_to_dafny(&value.attribute_name),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveDescription,
    >,
) -> aws_sdk_dynamodb::types::TimeToLiveDescription {
    aws_sdk_dynamodb::types::TimeToLiveDescription::builder()
        .set_time_to_live_status(match &**dafny_value.TimeToLiveStatus() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_status::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .set_attribute_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.AttributeName().clone(),
        ))
        .build()
}
