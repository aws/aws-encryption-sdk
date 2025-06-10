// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TableClassSummary,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary::TableClassSummary {
        TableClass: ::dafny_runtime::Rc::new(match &value.table_class {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_class::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LastUpdateDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_update_date_time),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary,
    >,
) -> aws_sdk_dynamodb::types::TableClassSummary {
    aws_sdk_dynamodb::types::TableClassSummary::builder()
        .set_table_class(match &**dafny_value.TableClass() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::table_class::from_dafny(value),
            ),
            _ => None,
        })
        .set_last_update_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.LastUpdateDateTime().clone(),
        ))
        .build()
}
