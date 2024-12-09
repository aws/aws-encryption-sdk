// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_time_to_live::UpdateTimeToLiveOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTimeToLiveOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTimeToLiveOutput::UpdateTimeToLiveOutput {
        TimeToLiveSpecification: ::std::rc::Rc::new(match &value.time_to_live_specification {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTimeToLiveOutput,
    >
) -> aws_sdk_dynamodb::operation::update_time_to_live::UpdateTimeToLiveOutput {
    aws_sdk_dynamodb::operation::update_time_to_live::UpdateTimeToLiveOutput::builder()
          .set_time_to_live_specification(match (*dafny_value.TimeToLiveSpecification()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_specification::from_dafny(value.clone())),
    _ => None,
}
)
          .build()


}
