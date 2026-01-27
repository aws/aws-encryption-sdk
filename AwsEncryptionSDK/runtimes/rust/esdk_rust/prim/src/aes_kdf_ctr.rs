// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::serr;

use aws_lc_rs::cipher::{AES_256, EncryptingKey, EncryptionContext, UnboundCipherKey};

fn as_array(nonce: &[u8]) -> &[u8; aws_lc_rs::iv::IV_LEN_128_BIT] {
    nonce.try_into().unwrap()
}

/// AES KDF CTR
pub fn ctr_stream(nonce: &[u8], key: &[u8], length: usize) -> Result<Vec<u8>, Error> {
    if nonce.len() != aws_lc_rs::iv::IV_LEN_128_BIT {
        return Err(serr(format!(
            "Nonce length of {} not supported in AesKdfCtrStream. Nonce length must be {}.",
            nonce.len(),
            aws_lc_rs::iv::IV_LEN_128_BIT
        )));
    }

    let mut in_out_buffer = vec![0; length];

    let key = UnboundCipherKey::new(&AES_256, key).map_err(|e| serr(format!("ctr new {e:?}")))?;
    let encrypting_key = EncryptingKey::ctr(key).map_err(|e| serr(format!("ctr {e:?}")))?;
    let nonce = aws_lc_rs::iv::FixedLength::<16>::from(as_array(nonce));
    let context = EncryptionContext::Iv128(nonce);
    encrypting_key
        .less_safe_encrypt(&mut in_out_buffer, context)
        .map_err(|e| serr(format!("ctr encrypt{e:?}")))?;
    Ok(in_out_buffer)
}
