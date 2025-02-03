// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::GlobalTableDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableDescription::GlobalTableDescription {
        ReplicationGroup: ::dafny_runtime::Rc::new(match &value.replication_group {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 GlobalTableArn: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_arn),
 CreationDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date_time),
 GlobalTableStatus: ::dafny_runtime::Rc::new(match &value.global_table_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::global_table_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableDescription,
    >,
) -> aws_sdk_dynamodb::types::GlobalTableDescription {
    aws_sdk_dynamodb::types::GlobalTableDescription::builder()
          .set_replication_group(match (*dafny_value.ReplicationGroup()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableArn().clone()))
 .set_creation_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDateTime().clone()))
 .set_global_table_status(match &**dafny_value.GlobalTableStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::global_table_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_global_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableName().clone()))
          .build()
}
