// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexAutoScalingUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexAutoScalingUpdate::GlobalSecondaryIndexAutoScalingUpdate {
        IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 ProvisionedWriteCapacityAutoScalingUpdate: ::dafny_runtime::Rc::new(match &value.provisioned_write_capacity_auto_scaling_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexAutoScalingUpdate,
    >,
) -> aws_sdk_dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate {
    aws_sdk_dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate::builder()
          .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_provisioned_write_capacity_auto_scaling_update(match (*dafny_value.ProvisionedWriteCapacityAutoScalingUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
