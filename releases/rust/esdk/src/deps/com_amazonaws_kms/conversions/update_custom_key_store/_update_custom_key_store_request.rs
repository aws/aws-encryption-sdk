// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreRequest::UpdateCustomKeyStoreRequest {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id) .Extract(),
 NewCustomKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.new_custom_key_store_name),
 KeyStorePassword: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_password),
 CloudHsmClusterId: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_hsm_cluster_id),
 XksProxyUriEndpoint: crate::standard_library_conversions::ostring_to_dafny(&value.xks_proxy_uri_endpoint),
 XksProxyUriPath: crate::standard_library_conversions::ostring_to_dafny(&value.xks_proxy_uri_path),
 XksProxyVpcEndpointServiceName: crate::standard_library_conversions::ostring_to_dafny(&value.xks_proxy_vpc_endpoint_service_name),
 XksProxyAuthenticationCredential: ::dafny_runtime::Rc::new(match &value.xks_proxy_authentication_credential {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::xks_proxy_authentication_credential_type::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 XksProxyConnectivity: ::dafny_runtime::Rc::new(match &value.xks_proxy_connectivity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::xks_proxy_connectivity_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreRequest,
    >,
) -> aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreInput {
    aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreInput::builder()
          .set_custom_key_store_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.CustomKeyStoreId()) ))
 .set_new_custom_key_store_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NewCustomKeyStoreName().clone()))
 .set_key_store_password(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyStorePassword().clone()))
 .set_cloud_hsm_cluster_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CloudHsmClusterId().clone()))
 .set_xks_proxy_uri_endpoint(crate::standard_library_conversions::ostring_from_dafny(dafny_value.XksProxyUriEndpoint().clone()))
 .set_xks_proxy_uri_path(crate::standard_library_conversions::ostring_from_dafny(dafny_value.XksProxyUriPath().clone()))
 .set_xks_proxy_vpc_endpoint_service_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.XksProxyVpcEndpointServiceName().clone()))
 .set_xks_proxy_authentication_credential(match (*dafny_value.XksProxyAuthenticationCredential()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::xks_proxy_authentication_credential_type::from_dafny(value.clone())),
    _ => None,
}
)
 .set_xks_proxy_connectivity(match &**dafny_value.XksProxyConnectivity() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::xks_proxy_connectivity_type::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
