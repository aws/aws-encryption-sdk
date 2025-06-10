// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::create_global_table::CreateGlobalTableInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateGlobalTableInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateGlobalTableInput::CreateGlobalTableInput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name) .Extract(),
 ReplicationGroup: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.replication_group.clone().unwrap(),
    |e| crate::deps::com_amazonaws_dynamodb::conversions::replica::to_dafny(e)
,
)
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateGlobalTableInput,
    >,
) -> aws_sdk_dynamodb::operation::create_global_table::CreateGlobalTableInput {
    aws_sdk_dynamodb::operation::create_global_table::CreateGlobalTableInput::builder()
          .set_global_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.GlobalTableName()) ))
 .set_replication_group(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.ReplicationGroup(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Replica>| crate::deps::com_amazonaws_dynamodb::conversions::replica::from_dafny(e.clone())
,
)
 ))
          .build()
          .unwrap()
}
