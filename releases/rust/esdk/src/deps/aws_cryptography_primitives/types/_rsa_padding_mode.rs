// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum RsaPaddingMode {
    Pkcs1,
    OaepSha1,
    OaepSha256,
    OaepSha384,
    OaepSha512,
}

impl ::std::fmt::Display for RsaPaddingMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            RsaPaddingMode::Pkcs1 => write!(f, "PKCS1"),
            RsaPaddingMode::OaepSha1 => write!(f, "OAEP_SHA1"),
            RsaPaddingMode::OaepSha256 => write!(f, "OAEP_SHA256"),
            RsaPaddingMode::OaepSha384 => write!(f, "OAEP_SHA384"),
            RsaPaddingMode::OaepSha512 => write!(f, "OAEP_SHA512"),
        }
    }
}
