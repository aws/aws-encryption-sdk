// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum EsdkCommitmentPolicy {
    ForbidEncryptAllowDecrypt,
    RequireEncryptAllowDecrypt,
    RequireEncryptRequireDecrypt,
}

impl ::std::fmt::Display for EsdkCommitmentPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
                write!(f, "FORBID_ENCRYPT_ALLOW_DECRYPT")
            }
            EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
                write!(f, "REQUIRE_ENCRYPT_ALLOW_DECRYPT")
            }
            EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
                write!(f, "REQUIRE_ENCRYPT_REQUIRE_DECRYPT")
            }
        }
    }
}
