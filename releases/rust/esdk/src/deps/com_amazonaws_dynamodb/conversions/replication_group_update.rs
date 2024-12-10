// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicationGroupUpdate,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicationGroupUpdate>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicationGroupUpdate::ReplicationGroupUpdate {
        Create: ::std::rc::Rc::new(match &value.create {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::create_replication_group_member_action::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Update: ::std::rc::Rc::new(match &value.update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::update_replication_group_member_action::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Delete: ::std::rc::Rc::new(match &value.delete {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::delete_replication_group_member_action::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicationGroupUpdate,
    >,
) -> aws_sdk_dynamodb::types::ReplicationGroupUpdate {
    aws_sdk_dynamodb::types::ReplicationGroupUpdate::builder()
          .set_create(match (*dafny_value.Create()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::create_replication_group_member_action::from_dafny(value.clone())),
    _ => None,
}
)
 .set_update(match (*dafny_value.Update()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::update_replication_group_member_action::from_dafny(value.clone())),
    _ => None,
}
)
 .set_delete(match (*dafny_value.Delete()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::delete_replication_group_member_action::from_dafny(value.clone())),
    _ => None,
}
)
          .build()

}
