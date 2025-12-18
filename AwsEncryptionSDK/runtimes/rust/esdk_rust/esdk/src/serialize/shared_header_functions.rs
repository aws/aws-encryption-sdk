// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::*;
use crate::serialize::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use std::sync::LazyLock;

static DAFNY_TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

fn convert_asi(
    x: aws_mpl_legacy::operation::get_algorithm_suite_info::AlgorithmSuiteInfo,
) -> aws_mpl_legacy::types::AlgorithmSuiteInfo {
    ESDKAlgorithmSuite::builder()
        .set_binary_id(x.binary_id)
        .set_commitment(x.commitment)
        .set_edk_wrapping(x.edk_wrapping)
        .set_encrypt(x.encrypt)
        .set_id(x.id)
        .set_kdf(x.kdf)
        .set_message_version(x.message_version)
        .set_signature(x.signature)
        .set_symmetric_signature(x.symmetric_signature)
        .build()
        .unwrap()
}

pub(crate) fn read_esdk_suite_id(
    r: &mut dyn SafeRead,
    mpl: &aws_mpl_legacy::Client,
    raw: &mut dyn SafeWrite,
) -> Result<aws_mpl_legacy::types::AlgorithmSuiteInfo, Error> {
    let mut esdk_suite_id_bytes = [0; 2];
    read_bytes(r, &mut esdk_suite_id_bytes, raw)?;

    let suite = match tokio::runtime::Handle::try_current() {
        Ok(curr) => tokio::task::block_in_place(|| {
            curr.block_on(async {
                mpl.get_algorithm_suite_info()
                    .binary_id(&esdk_suite_id_bytes[..])
                    .send()
                    .await
                    .unwrap()
            })
        }),
        Err(_) => DAFNY_TOKIO_RUNTIME
            .block_on(
                mpl.get_algorithm_suite_info()
                    .binary_id(&esdk_suite_id_bytes[..])
                    .send(),
            )
            .unwrap(),
    };

    // :- Need(suite.binaryId == esdkSuiteIdBytes.data, Error("Algorithm suite ID not supported."));
    // :- Need(suite.id.ESDK?, Error("Algorithm suite ID not supported."));
    Ok(convert_asi(suite))
}

pub(crate) fn read_message_id_v1(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)
}
pub(crate) fn read_message_id_v2(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)
}

pub(crate) fn write_esdk_suite_id(
    w: &mut dyn SafeWrite,
    suite: &aws_mpl_legacy::types::AlgorithmSuiteInfo,
) -> Result<(), Error> {
    write_bytes(w, suite.binary_id.as_ref().unwrap().as_ref())
}

/*
 * Writes the message id as bytes, which, since the message id is already stored
 * as bytes, simply returns the message id.
 *
 * Though we have different V1 and V2 methods for the read path, since
 * they read different numbers of bytes, a single method on the write path
 * is fine since writing is identical for both.
 */
pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    write_bytes(w, message_id)
}
