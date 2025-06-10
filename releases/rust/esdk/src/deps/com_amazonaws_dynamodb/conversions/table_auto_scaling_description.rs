// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TableAutoScalingDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableAutoScalingDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableAutoScalingDescription::TableAutoScalingDescription {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 TableStatus: ::dafny_runtime::Rc::new(match &value.table_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Replicas: ::dafny_runtime::Rc::new(match &value.replicas {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_auto_scaling_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableAutoScalingDescription,
    >,
) -> aws_sdk_dynamodb::types::TableAutoScalingDescription {
    aws_sdk_dynamodb::types::TableAutoScalingDescription::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_table_status(match &**dafny_value.TableStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::table_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_replicas(match (*dafny_value.Replicas()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_auto_scaling_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
