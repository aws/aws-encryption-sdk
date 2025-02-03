// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreRequest::CreateCustomKeyStoreRequest {
        CustomKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_name) .Extract(),
 CloudHsmClusterId: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_hsm_cluster_id),
 TrustAnchorCertificate: crate::standard_library_conversions::ostring_to_dafny(&value.trust_anchor_certificate),
 KeyStorePassword: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_password),
 CustomKeyStoreType: ::dafny_runtime::Rc::new(match &value.custom_key_store_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::custom_key_store_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
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
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreRequest,
    >,
) -> aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreInput {
    aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreInput::builder()
          .set_custom_key_store_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.CustomKeyStoreName()) ))
 .set_cloud_hsm_cluster_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CloudHsmClusterId().clone()))
 .set_trust_anchor_certificate(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TrustAnchorCertificate().clone()))
 .set_key_store_password(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyStorePassword().clone()))
 .set_custom_key_store_type(match &**dafny_value.CustomKeyStoreType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::custom_key_store_type::from_dafny(value)
    ),
    _ => None,
}
)
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
