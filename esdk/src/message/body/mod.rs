// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Message body encryption and decryption.

pub(crate) mod body_aad;
mod body_decrypt;
mod body_encrypt;

pub(crate) use body_aad::*;
pub(crate) use body_decrypt::*;
pub(crate) use body_encrypt::*;

use crate::error::{Error, val_err};
use aws_mpl_legacy::primitives::AesGcm;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn get_alg_suite(info: &AlgorithmSuite) -> Result<AesGcm, Error> {
    match &info.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(aes_gcm) => Ok(*aes_gcm),
        _ => Err(val_err("Algorithm suite encrypt must be AES-GCM")),
    }
}
