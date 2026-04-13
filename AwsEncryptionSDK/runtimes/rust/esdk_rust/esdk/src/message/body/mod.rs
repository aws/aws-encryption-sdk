// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod body_aad;
mod body_decrypt;
mod body_encrypt;

pub(crate) use body_aad::*;
pub(crate) use body_decrypt::*;
pub(crate) use body_encrypt::*;

use aws_mpl_legacy::primitives::AesGcm;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn get_encrypt(info: &AlgorithmSuite) -> AesGcm {
    match &info.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(aes_gcm) => *aes_gcm,
        _ => panic!("not an aes gcm"),
    }
}
