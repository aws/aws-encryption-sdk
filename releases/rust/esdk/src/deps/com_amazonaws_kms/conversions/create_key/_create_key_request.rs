// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::create_key::CreateKeyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateKeyRequest,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateKeyRequest::CreateKeyRequest {
        Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy),
 Description: crate::standard_library_conversions::ostring_to_dafny(&value.description),
 KeyUsage: ::dafny_runtime::Rc::new(match &value.key_usage {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_usage_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CustomerMasterKeySpec: ::dafny_runtime::Rc::new(match &value.customer_master_key_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::customer_master_key_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeySpec: ::dafny_runtime::Rc::new(match &value.key_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Origin: ::dafny_runtime::Rc::new(match &value.origin {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::origin_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
 BypassPolicyLockoutSafetyCheck: crate::standard_library_conversions::obool_to_dafny(&value.bypass_policy_lockout_safety_check),
 Tags: ::dafny_runtime::Rc::new(match &value.tags {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::tag::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 MultiRegion: crate::standard_library_conversions::obool_to_dafny(&value.multi_region),
 XksKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.xks_key_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateKeyRequest,
    >,
) -> aws_sdk_kms::operation::create_key::CreateKeyInput {
    aws_sdk_kms::operation::create_key::CreateKeyInput::builder()
          .set_policy(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Policy().clone()))
 .set_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Description().clone()))
 .set_key_usage(match &**dafny_value.KeyUsage() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_usage_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_customer_master_key_spec(match &**dafny_value.CustomerMasterKeySpec() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::customer_master_key_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_spec(match &**dafny_value.KeySpec() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_origin(match &**dafny_value.Origin() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::origin_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CustomKeyStoreId().clone()))
 .set_bypass_policy_lockout_safety_check(crate::standard_library_conversions::obool_from_dafny(dafny_value.BypassPolicyLockoutSafetyCheck().clone()))
 .set_tags(match (*dafny_value.Tags()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Tag>| crate::deps::com_amazonaws_kms::conversions::tag::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_multi_region(crate::standard_library_conversions::obool_from_dafny(dafny_value.MultiRegion().clone()))
 .set_xks_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.XksKeyId().clone()))
          .build()
          .unwrap()
}
