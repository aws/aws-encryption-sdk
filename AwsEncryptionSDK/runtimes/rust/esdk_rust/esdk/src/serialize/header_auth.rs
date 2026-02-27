// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::*;
use crate::serialize::serializable_types::*;
use crate::serialize::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_rs::suites::AlgorithmSuite;

pub(crate) fn write_header_auth_tag(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    match suite.message_version {
        1 => write_header_auth_tag_v1(w, header_auth),
        2 => write_header_auth_tag_v2(w, header_auth),
        _ => ser_err("Unexpected message version"),
    }
}
pub(crate) fn write_header_auth_tag_v1(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
) -> Result<(), Error> {
    match header_auth {
        HeaderAuth::AESMac {
            header_iv,
            header_auth_tag,
        } => {
            //= specification/data-format/message-header.md#iv-length
            //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
            //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
            write_bytes(w, header_iv)?;
            write_bytes(w, header_auth_tag)
        }
    }
}
pub(crate) fn write_header_auth_tag_v2(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
) -> Result<(), Error> {
    match header_auth {
        HeaderAuth::AESMac {
            header_auth_tag, ..
        } => write_bytes(w, header_auth_tag),
    }
}

pub(crate) fn read_header_auth_tag(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderAuth, Error> {
    match suite.message_version {
        1 => read_header_auth_tag_v1(r, suite, raw),
        2 => read_header_auth_tag_v2(r, suite, raw),
        _ => ser_err("Unexpected message version"),
    }
}
pub(crate) fn read_header_auth_tag_v1(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderAuth, Error> {
    let header_iv = read_vec(r, get_iv_length(suite) as usize, raw)?;
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
pub(crate) fn read_header_auth_tag_v2(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderAuth, Error> {
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    let header_iv = vec![0u8; get_iv_length(suite) as usize];
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
