// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::XksProxyConnectivityType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::XksProxyConnectivityType::PublicEndpoint => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType::PUBLIC_ENDPOINT {},
aws_sdk_kms::types::XksProxyConnectivityType::VpcEndpointService => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType::VPC_ENDPOINT_SERVICE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType,
) -> aws_sdk_kms::types::XksProxyConnectivityType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType::PUBLIC_ENDPOINT {} => aws_sdk_kms::types::XksProxyConnectivityType::PublicEndpoint,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyConnectivityType::VPC_ENDPOINT_SERVICE {} => aws_sdk_kms::types::XksProxyConnectivityType::VpcEndpointService,
    }
}
