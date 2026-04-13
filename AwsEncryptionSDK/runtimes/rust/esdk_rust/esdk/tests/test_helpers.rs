// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

mod fixtures;

use aws_esdk::*;
use fixtures::*;

pub const IV_LEN: usize = 12;
pub const TAG_LEN: usize = 16;
pub const ENDFRAME_MARKER: [u8; 4] = 0xFFFF_FFFFu32.to_be_bytes();

#[derive(Clone, Copy, Debug)]
pub enum Version {
    V1,
    V2,
}

pub const VERSIONS: [Version; 2] = [Version::V1, Version::V2];

/// Create a raw AES keyring for testing (no KMS needed).
pub async fn test_keyring() -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}
