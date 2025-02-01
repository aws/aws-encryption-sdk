// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexAutoScalingDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexAutoScalingDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexAutoScalingDescription::ReplicaGlobalSecondaryIndexAutoScalingDescription {
        IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 IndexStatus: ::dafny_runtime::Rc::new(match &value.index_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::index_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ProvisionedReadCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.provisioned_read_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ProvisionedWriteCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.provisioned_write_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexAutoScalingDescription,
    >,
) -> aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexAutoScalingDescription {
    aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexAutoScalingDescription::builder()
          .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_index_status(match &**dafny_value.IndexStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::index_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_provisioned_read_capacity_auto_scaling_settings(match (*dafny_value.ProvisionedReadCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_provisioned_write_capacity_auto_scaling_settings(match (*dafny_value.ProvisionedWriteCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
