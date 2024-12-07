// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
/// During Decryption, Allow or Forbid ESDK-NET v4.0.0 Behavior if the ESDK Message Header fails the Header Authentication check.
pub enum NetV400RetryPolicy {
    ForbidRetry,
AllowRetry,
}

impl ::std::fmt::Display for NetV400RetryPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            NetV400RetryPolicy::ForbidRetry => write!(f, "FORBID_RETRY"),
NetV400RetryPolicy::AllowRetry => write!(f, "ALLOW_RETRY"),
        }
    }
}
