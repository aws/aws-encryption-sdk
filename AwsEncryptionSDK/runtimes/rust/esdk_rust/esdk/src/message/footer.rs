// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Message footer serialization/deserialization.
//! Maps to data-format/message-footer.md

use super::serialize_functions::{read_seq_u16, write_seq_u16};
use super::*;
use crate::types::{SafeRead, SafeWrite};

//= specification/data-format/message-footer.md#overview
//= type=implication
//# When an [algorithm suite](../framework/algorithm-suites.md) includes a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
//# the [message](message.md) MUST contain a footer.

/// Write a message footer (signature length + signature bytes).
///
/// The caller is responsible for ensuring this is only called when the algorithm suite
/// includes a signature algorithm.
//= specification/data-format/message.md#structure
//# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
//# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
//# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
//# [message footer](message-footer.md) serialized after the [message body](message-body.md).
pub(crate) fn write_footer(
    w: &mut dyn SafeWrite,
    signature: &[u8],
) -> Result<(), Error> {
    if signature.len() >= u16::MAX.into() {
        return ser_err("Length of signature bytes is larger than the uint16 limit.");
    }
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# This operation MUST then serialize a message footer with the following specifics:
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - [Signature Length](../data-format/message-footer.md#signature-length): MUST be the length of the
    //# output of the calculation above.
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - [Signature](../data-format/message-footer.md#signature): MUST be the output of the calculation above.
    write_seq_u16(w, signature)

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=implication
    //# The above serialized bytes MUST NOT be released until the entire message footer has been serialized.
}

/// Read a message footer, returning the signature bytes.
pub(crate) fn read_footer(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    read_seq_u16(r, raw)
}
