// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Low-level byte read/write primitives for message serialization.
//!
//! The `raw` SafeWrite parameter on every `read_*` function tees the consumed
//! bytes into a mirror buffer so callers can reconstruct the exact raw header
//! bytes used for authentication and signing.

use super::{Error, ser_err};
use crate::error::ErrorKind;
use crate::types::{SafeRead, SafeWrite};
use std::backtrace::Backtrace;
use std::sync::Arc;

#[track_caller]
fn ser_io(e: std::io::Error) -> Error {
    match e.kind() {
        std::io::ErrorKind::UnexpectedEof => Error {
            kind: ErrorKind::SerializationError,
            message: "Unexpected end of data".into(),
            cause: Some(Arc::new(e)),
            backtrace: Arc::new(Backtrace::capture()),
        },
        _ => Error {
            kind: ErrorKind::SerializationError,
            message: "IO Error".into(),
            cause: Some(Arc::new(e)),
            backtrace: Arc::new(Backtrace::capture()),
        },
    }
}

/// Read up to `buf.len()` bytes; returns the number actually read (may be < len on EOF).
pub(crate) fn read_up_to(this: &mut dyn SafeRead, buf: &mut [u8]) -> Result<usize, Error> {
    let mut curr: usize = 0;
    loop {
        match this.read(&mut buf[curr..]) {
            Ok(0) => {
                return Ok(curr);
            }
            Ok(n) => {
                curr += n;
                if curr == buf.len() {
                    return Ok(curr);
                }
            }
            // Err(ref e) if e.is_interrupted() => {}
            Err(e) => return Err(ser_io(e)),
        }
    }
}

/// Like `read_up_to`, but `first` is a one-byte peek already consumed by the caller.
pub(crate) fn read_up_to_peek(
    this: &mut dyn SafeRead,
    buf: &mut [u8],
    first: Option<u8>,
) -> Result<usize, Error> {
    if buf.is_empty() {
        return Ok(0);
    }
    match first {
        Some(f) => {
            buf[0] = f;
            match read_up_to(this, &mut buf[1..]) {
                Ok(n) => Ok(n + 1),
                Err(e) => Err(e),
            }
        }
        None => read_up_to(this, buf),
    }
}

#[track_caller]
fn ser_utf8(item: std::string::FromUtf8Error) -> Error {
    Error {
        kind: ErrorKind::SerializationError,
        message: "UTF8 Decode Error".into(),
        cause: Some(Arc::new(item)),
        backtrace: Arc::new(Backtrace::capture()),
    }
}

pub(crate) fn write_bytes(w: &mut dyn SafeWrite, data: &[u8]) -> Result<(), Error> {
    w.write_all(data).map_err(ser_io)?;
    Ok(())
}

// Big-endian fixed-width writers.
pub(crate) fn write_u8(w: &mut dyn SafeWrite, data: u8) -> Result<(), Error> {
    write_bytes(w, &[data])
}
pub(crate) fn write_u16(w: &mut dyn SafeWrite, data: u16) -> Result<(), Error> {
    write_bytes(w, &data.to_be_bytes())
}
pub(crate) fn write_u32(w: &mut dyn SafeWrite, data: u32) -> Result<(), Error> {
    write_bytes(w, &data.to_be_bytes())
}

/// Read exactly `buf.len()` bytes and mirror them into `raw`.
pub(crate) fn read_bytes(
    r: &mut dyn SafeRead,
    buf: &mut [u8],
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    r.read_exact(buf).map_err(ser_io)?;
    write_bytes(raw, buf)
}

/// Read exactly `length` bytes into a fresh Vec.
pub(crate) fn read_vec(
    r: &mut dyn SafeRead,
    length: usize,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let mut result = vec![0; length];
    read_bytes(r, &mut result, raw)?;
    Ok(result)
}

// Big-endian fixed-width readers. Each mirrors the consumed bytes into `raw`.
pub(crate) fn read_u8(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<u8, Error> {
    let mut result = [0u8; 1];
    read_bytes(r, &mut result, raw)?;
    Ok(result[0])
}

/// Read one byte, returning `Ok(None)` on clean EOF. Does NOT mirror into a
/// raw buffer (used for streaming peek).
pub(crate) fn read_opt_u8(r: &mut dyn SafeRead) -> Result<Option<u8>, Error> {
    let mut result = [0u8; 1];
    match r.read_exact(&mut result) {
        Ok(()) => Ok(Some(result[0])),
        Err(e) => match e.kind() {
            std::io::ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(ser_io(e)),
        },
    }
}

pub(crate) fn read_u16(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<u16, Error> {
    let mut result = [0u8; 2];
    read_bytes(r, &mut result, raw)?;
    Ok(u16::from_be_bytes(result))
}
pub(crate) fn read_u32(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<u32, Error> {
    let mut result = [0u8; 4];
    read_bytes(r, &mut result, raw)?;
    Ok(u32::from_be_bytes(result))
}
pub(crate) fn read_u64(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<u64, Error> {
    let mut result = [0u8; 8];
    read_bytes(r, &mut result, raw)?;
    Ok(u64::from_be_bytes(result))
}

/// Read a UInt16 length prefix followed by that many bytes.
pub(crate) fn read_seq_u16(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let len = read_u16(r, raw)?;
    read_vec(r, usize::from(len), raw)
}

/// Read a UInt32 length prefix followed by that many bytes into `data`,
/// rejecting lengths above `bound`.
pub(crate) fn read_seq_u32_bounded(
    r: &mut dyn SafeRead,
    bound: u32,
    msg: &str,
    data: &mut Vec<u8>,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    let len = read_u32(r, raw)?;
    if len > bound {
        return ser_err(msg);
    }
    let Ok(len_usize) = usize::try_from(len) else {
        return ser_err("length too large for platform");
    };
    data.resize(len_usize, 0);
    read_bytes(r, &mut data[..], raw)
}

/// Read a UInt64 length prefix followed by that many bytes, rejecting lengths
/// above `bound`.
pub(crate) fn read_seq_u64_bounded(
    r: &mut dyn SafeRead,
    bound: u64,
    msg: &str,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let len = read_u64(r, raw)?;
    if len > bound {
        return ser_err(msg);
    }
    let Ok(len_usize) = usize::try_from(len) else {
        return ser_err("length too large for platform");
    };
    read_vec(r, len_usize, raw)
}

/// Read a UInt16-prefixed UTF-8 string.
pub(crate) fn read_str_u16(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<String, Error> {
    let len = read_u16(r, raw)?;
    let result = read_vec(r, usize::from(len), raw)?;
    let result = String::from_utf8(result).map_err(ser_utf8)?;
    Ok(result)
}
