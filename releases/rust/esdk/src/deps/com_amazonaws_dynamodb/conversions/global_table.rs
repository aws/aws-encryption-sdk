// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::GlobalTable,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTable>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTable::GlobalTable {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name),
 ReplicationGroup: ::std::rc::Rc::new(match &value.replication_group {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTable,
    >,
) -> aws_sdk_dynamodb::types::GlobalTable {
    aws_sdk_dynamodb::types::GlobalTable::builder()
          .set_global_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableName().clone()))
 .set_replication_group(match (*dafny_value.ReplicationGroup()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Replica>| crate::deps::com_amazonaws_dynamodb::conversions::replica::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()

}
