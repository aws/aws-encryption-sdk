// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Header authentication tag serialization and deserialization.

use super::header_types::HeaderAuth;
use super::serializable_types::{get_iv_length, get_tag_length};
use super::serialize_functions::{read_vec, write_bytes};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn write_header_auth_tag(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    match suite.message_version {
        //= spec/client-apis/encrypt.md#authentication-tag
        //# With the authentication tag calculated,
        //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
        //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
        1 => write_header_auth_tag_v1(w, header_auth),
        //= spec/client-apis/encrypt.md#authentication-tag
        //# With the authentication tag calculated,
        //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
        //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
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
            //= spec/client-apis/encrypt.md#authentication-tag
            //# - [IV](../data-format/message-header.md#iv): MUST have the value of the IV used in the calculation above,
            //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
            write_bytes(w, header_iv)?;

            //= spec/client-apis/encrypt.md#authentication-tag
            //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
            //# of the authentication tag calculated above.
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
        } => {
            //= spec/client-apis/encrypt.md#authentication-tag
            //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
            //# of the authentication tag calculated above.
            write_bytes(w, header_auth_tag)
        }
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
    let iv_len = get_iv_length(suite) as usize;

    //= spec/client-apis/decrypt.md#parse-the-header
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    let header_iv = read_vec(r, iv_len, raw)?;

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
    //= spec/client-apis/decrypt.md#parse-the-header
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    let header_iv = vec![0u8; get_iv_length(suite) as usize];
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
