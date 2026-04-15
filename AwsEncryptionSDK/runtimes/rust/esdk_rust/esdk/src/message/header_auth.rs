// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Header authentication tag serialization and deserialization.

use super::header_types::HeaderAuth;
use super::{Error, ser_err};
use super::serializable_types::{get_iv_length, get_tag_length};
use super::serialize_functions::{read_vec, write_bytes};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn write_header_auth_tag(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    match suite.message_version {
        //= specification/client-apis/encrypt.md#v1-authentication-tag
        //# With the authentication tag calculated,
        //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
        //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
        1 => write_header_auth_tag_v1(w, header_auth),
        //= specification/client-apis/encrypt.md#v2-authentication-tag
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
            //= specification/data-format/message-header.md#header-authentication-version-1-0
            //# The V1 Header Authentication MUST consist of, in order,
            //# IV,
            //# and Authentication Tag.

            //= specification/client-apis/encrypt.md#v1-authentication-tag
            //# - The Encrypt operation MUST serialize the [IV](../data-format/message-header.md#iv).
            //# The value MUST be the IV used in the calculation above,
            //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
            write_bytes(w, header_iv)?;
            //= specification/client-apis/encrypt.md#v1-authentication-tag
            //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
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
            //= specification/data-format/message-header.md#header-authentication-version-2-0
            //# The V2 Header Authentication MUST consist of the Authentication Tag only.

            //= specification/client-apis/encrypt.md#v2-authentication-tag
            //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
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
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //= reason=read_vec reads the IV bytes from the V1 header authentication section
    //# - The Decrypt operation MUST deserialize the [IV](../data-format/message-header.md#iv).

    //= specification/data-format/message-header.md#iv
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let iv_len = get_iv_length(suite) as usize;
    //= specification/data-format/message-header.md#iv
    //# The IV MUST be interpreted as bytes.
    let header_iv = read_vec(r, iv_len, raw)?;
    //= specification/data-format/message-header.md#authentication-tag
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.

    //= specification/data-format/message-header.md#authentication-tag
    //# The authentication tag MUST be interpreted as bytes.
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
    //= specification/data-format/message-header.md#authentication-tag
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.

    //= specification/data-format/message-header.md#authentication-tag
    //# The authentication tag MUST be interpreted as bytes.
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    let header_iv = vec![0u8; get_iv_length(suite) as usize];
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
