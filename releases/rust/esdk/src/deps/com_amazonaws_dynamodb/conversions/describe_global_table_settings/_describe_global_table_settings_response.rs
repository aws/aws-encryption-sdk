// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableSettingsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableSettingsOutput::DescribeGlobalTableSettingsOutput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name),
 ReplicaSettings: ::dafny_runtime::Rc::new(match &value.replica_settings {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_settings_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableSettingsOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput
{
    aws_sdk_dynamodb::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput::builder()
          .set_global_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableName().clone()))
 .set_replica_settings(match (*dafny_value.ReplicaSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_settings_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
