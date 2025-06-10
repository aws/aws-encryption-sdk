// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::XksProxyConfigurationType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConfigurationType>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConfigurationType::XksProxyConfigurationType {
        Connectivity: ::dafny_runtime::Rc::new(match &value.connectivity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::xks_proxy_connectivity_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 AccessKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.access_key_id),
 UriEndpoint: crate::standard_library_conversions::ostring_to_dafny(&value.uri_endpoint),
 UriPath: crate::standard_library_conversions::ostring_to_dafny(&value.uri_path),
 VpcEndpointServiceName: crate::standard_library_conversions::ostring_to_dafny(&value.vpc_endpoint_service_name),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConfigurationType,
    >,
) -> aws_sdk_kms::types::XksProxyConfigurationType {
    aws_sdk_kms::types::XksProxyConfigurationType::builder()
          .set_connectivity(match &**dafny_value.Connectivity() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::xks_proxy_connectivity_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_access_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.AccessKeyId().clone()))
 .set_uri_endpoint(crate::standard_library_conversions::ostring_from_dafny(dafny_value.UriEndpoint().clone()))
 .set_uri_path(crate::standard_library_conversions::ostring_from_dafny(dafny_value.UriPath().clone()))
 .set_vpc_endpoint_service_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.VpcEndpointServiceName().clone()))
          .build()
}
