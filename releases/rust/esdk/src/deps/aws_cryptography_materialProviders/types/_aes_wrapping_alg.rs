// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum AesWrappingAlg {
    AlgAes128GcmIv12Tag16,
    AlgAes192GcmIv12Tag16,
    AlgAes256GcmIv12Tag16,
}

impl ::std::fmt::Display for AesWrappingAlg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            AesWrappingAlg::AlgAes128GcmIv12Tag16 => write!(f, "ALG_AES128_GCM_IV12_TAG16"),
            AesWrappingAlg::AlgAes192GcmIv12Tag16 => write!(f, "ALG_AES192_GCM_IV12_TAG16"),
            AesWrappingAlg::AlgAes256GcmIv12Tag16 => write!(f, "ALG_AES256_GCM_IV12_TAG16"),
        }
    }
}
