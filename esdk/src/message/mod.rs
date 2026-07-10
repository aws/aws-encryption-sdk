// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Message format serialization and deserialization.
//!
//! This module implements the [AWS Encryption SDK message format](../spec/data-format/message.md),
//! which consists of a header, body, and optional footer.
//!
//! # Module layout
//!
//! - `header` — top-level header read/write orchestration
//! - `header_types` — header structs, enums, and shared read/write functions (suite ID, message ID)
//! - `header_auth` — header authentication tag serialization (V1 and V2)
//! - `v1_header_body` — V1-specific header body serialization/deserialization
//! - `v2_header_body` — V2-specific header body serialization/deserialization
//! - `encrypted_data_keys` — EDK list serialization
//! - `encryption_context` — AAD / encryption context serialization
//! - `serializable_types` — algorithm-suite-derived constants (IV length, tag length)
//! - `serialize_functions` — low-level read/write primitives (u8, u16, u32, bytes, etc.)
//! - `body/` — message body framing (encrypt, decrypt, AAD construction)
//! - `footer` — message footer (ECDSA signature) serialization

pub(crate) mod body;
pub(crate) mod encrypted_data_keys;
pub(crate) mod encryption_context;
pub(crate) mod footer;
pub(crate) mod header;
pub(crate) mod header_auth;
pub(crate) mod header_types;
pub(crate) mod serializable_types;
pub(crate) mod serialize_functions;
pub(crate) mod v1_header_body;
pub(crate) mod v2_header_body;
use aws_mpl_legacy::primitives::DigestContext;

pub(crate) use super::error::Error;
pub(crate) use super::error::ser_err;

/// A no-op writer that discards all bytes written to it.
/// Used when a write target is required by the interface but the output isn't needed
/// (e.g., computing a digest without also serializing to a buffer).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct NoopWriter;
impl std::io::Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// A writer that feeds all bytes into a cryptographic digest context.
/// Used during header serialization to simultaneously write the header bytes
/// and accumulate them into the ECDSA signature digest, without buffering
/// the entire header in memory.
#[derive(Clone, Debug, Default)]
pub struct DigestWriter {
    pub(crate) context: Option<DigestContext>,
}
impl DigestWriter {
    /// Create a `DigestWriter` from a signature algorithm.
    /// If the algorithm is ECDSA, initializes the digest context.
    /// Otherwise, returns a writer that does nothing (context = None).
    pub(crate) fn from_old_ecdsa(
        alg: aws_mpl_legacy::suites::SignatureAlgorithm,
    ) -> Result<Self, Error> {
        match alg {
            aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(x) => {
                let context = Some(DigestContext::new_from_ecdsa(x)?);
                Ok(Self { context })
            }
            _ => Ok(Self { context: None }),
        }
    }
}

impl std::io::Write for DigestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(ref mut context) = self.context {
            context.update(buf);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
