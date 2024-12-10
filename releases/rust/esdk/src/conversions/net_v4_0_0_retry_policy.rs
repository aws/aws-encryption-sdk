// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::types::NetV400RetryPolicy,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy>{
    ::std::rc::Rc::new(match value {
        crate::types::NetV400RetryPolicy::ForbidRetry => crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy::FORBID_RETRY {},
crate::types::NetV400RetryPolicy::AllowRetry => crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy::ALLOW_RETRY {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy,
) -> crate::types::NetV400RetryPolicy {
    match dafny_value {
        crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy::FORBID_RETRY {} => crate::types::NetV400RetryPolicy::ForbidRetry,
crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::NetV4_0_0_RetryPolicy::ALLOW_RETRY {} => crate::types::NetV400RetryPolicy::AllowRetry,
    }
}
