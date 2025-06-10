// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::CustomKeyStoresListEntry,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoresListEntry>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoresListEntry::CustomKeyStoresListEntry {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
 CustomKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_name),
 CloudHsmClusterId: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_hsm_cluster_id),
 TrustAnchorCertificate: crate::standard_library_conversions::ostring_to_dafny(&value.trust_anchor_certificate),
 ConnectionState: ::dafny_runtime::Rc::new(match &value.connection_state {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::connection_state_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ConnectionErrorCode: ::dafny_runtime::Rc::new(match &value.connection_error_code {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::connection_error_code_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CreationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date),
 CustomKeyStoreType: ::dafny_runtime::Rc::new(match &value.custom_key_store_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::custom_key_store_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 XksProxyConfiguration: ::dafny_runtime::Rc::new(match &value.xks_proxy_configuration {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::xks_proxy_configuration_type::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoresListEntry,
    >,
) -> aws_sdk_kms::types::CustomKeyStoresListEntry {
    aws_sdk_kms::types::CustomKeyStoresListEntry::builder()
          .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CustomKeyStoreId().clone()))
 .set_custom_key_store_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CustomKeyStoreName().clone()))
 .set_cloud_hsm_cluster_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CloudHsmClusterId().clone()))
 .set_trust_anchor_certificate(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TrustAnchorCertificate().clone()))
 .set_connection_state(match &**dafny_value.ConnectionState() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::connection_state_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_connection_error_code(match &**dafny_value.ConnectionErrorCode() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::connection_error_code_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_creation_date(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDate().clone()))
 .set_custom_key_store_type(match &**dafny_value.CustomKeyStoreType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::custom_key_store_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_xks_proxy_configuration(match (*dafny_value.XksProxyConfiguration()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::xks_proxy_configuration_type::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
