// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum PaddingScheme {
    Pkcs1,
    OaepSha1Mgf1,
    OaepSha256Mgf1,
    OaepSha384Mgf1,
    OaepSha512Mgf1,
}

impl ::std::fmt::Display for PaddingScheme {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            PaddingScheme::Pkcs1 => write!(f, "PKCS1"),
            PaddingScheme::OaepSha1Mgf1 => write!(f, "OAEP_SHA1_MGF1"),
            PaddingScheme::OaepSha256Mgf1 => write!(f, "OAEP_SHA256_MGF1"),
            PaddingScheme::OaepSha384Mgf1 => write!(f, "OAEP_SHA384_MGF1"),
            PaddingScheme::OaepSha512Mgf1 => write!(f, "OAEP_SHA512_MGF1"),
        }
    }
}
