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
        //= spec/client-apis/encrypt.md#v1-authentication-tag
        //# With the authentication tag calculated,
        //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
        //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
        1 => write_header_auth_tag_v1(w, header_auth),
        //= spec/client-apis/encrypt.md#v2-authentication-tag
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
            //= spec/client-apis/encrypt.md#v1-authentication-tag
            //# - MUST serialize the [IV](../data-format/message-header.md#iv).
            //
            //= spec/client-apis/encrypt.md#v1-authentication-tag
            //# The value MUST be the IV used in the calculation above,
            //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
            write_bytes(w, header_iv)?;

            //= spec/client-apis/encrypt.md#v1-authentication-tag
            //# - MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
            //
            //= spec/client-apis/encrypt.md#v1-authentication-tag
            //# The value MUST be the authentication tag calculated above.
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
            //= spec/client-apis/encrypt.md#v2-authentication-tag
            //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
            //
            //= spec/client-apis/encrypt.md#v2-authentication-tag
            //# The value MUST be the authentication tag calculated above.
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
    let iv_len = usize::from(get_iv_length(suite));

    let header_iv = read_vec(r, iv_len, raw)?;

    let header_auth_tag = read_vec(r, usize::from(get_tag_length(suite)), raw)?;
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
    let header_auth_tag = read_vec(r, usize::from(get_tag_length(suite)), raw)?;
    let header_iv = vec![0u8; usize::from(get_iv_length(suite))];
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
