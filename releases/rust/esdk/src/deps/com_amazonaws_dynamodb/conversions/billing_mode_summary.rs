// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BillingModeSummary,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingModeSummary>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingModeSummary::BillingModeSummary {
        BillingMode: ::std::rc::Rc::new(match &value.billing_mode {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LastUpdateToPayPerRequestDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_update_to_pay_per_request_date_time),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingModeSummary,
    >,
) -> aws_sdk_dynamodb::types::BillingModeSummary {
    aws_sdk_dynamodb::types::BillingModeSummary::builder()
          .set_billing_mode(match &**dafny_value.BillingMode() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
 .set_last_update_to_pay_per_request_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.LastUpdateToPayPerRequestDateTime().clone()))
          .build()

}
