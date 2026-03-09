// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::types::{SafeRead, SafeWrite};
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
    w.write_all(data).map_err(|e| ser_io(e))?;
    Ok(())
}

pub(crate) fn write_str_u16(w: &mut dyn SafeWrite, data: &str) -> Result<(), Error> {
    write_seq_u16(w, data.as_bytes())
}
pub(crate) fn write_u8(w: &mut dyn SafeWrite, data: u8) -> Result<(), Error> {
    write_bytes(w, &[data])
}
//= specification/data-format/message-header.md#structure
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_u16(w: &mut dyn SafeWrite, data: u16) -> Result<(), Error> {
    write_bytes(w, &data.to_be_bytes())
}
pub(crate) fn write_u32(w: &mut dyn SafeWrite, data: u32) -> Result<(), Error> {
    write_bytes(w, &data.to_be_bytes())
}
pub(crate) fn write_seq_u16(w: &mut dyn SafeWrite, data: &[u8]) -> Result<(), Error> {
    match u16::try_from(data.len()) {
        Ok(len) => {
            write_u16(w, len)?;
            write_bytes(w, data)
        }
        Err(_) => ser_err("Sequence length too long for 16 bits"),
    }
}

pub(crate) fn read_bytes(
    r: &mut dyn SafeRead,
    buf: &mut [u8],
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    r.read_exact(buf).map_err(|e| ser_io(e))?;
    write_bytes(raw, buf)
}

pub(crate) fn read_vec(
    r: &mut dyn SafeRead,
    length: usize,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let mut result = vec![0; length];
    read_bytes(r, &mut result, raw)?;
    Ok(result)
}

pub(crate) fn read_u8(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<u8, Error> {
    let mut result = [0u8; 1];
    read_bytes(r, &mut result, raw)?;
    Ok(result[0])
}

pub(crate) fn read_opt_u8(r: &mut dyn SafeRead) -> Result<Option<u8>, Error> {
    let mut result = [0u8; 1];
    match r.read_exact(&mut result) {
        Ok(()) => Ok(Some(result[0])),
        Err(e) => match e.kind() {
            std::io::ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(Error {
                kind: ErrorKind::SerializationError,
                message: "IO Error".into(),
                cause: Some(Arc::new(e)),
                backtrace: Arc::new(Backtrace::capture()),
            }),
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

pub(crate) fn read_seq_u16(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let len = read_u16(r, raw)?;
    read_vec(r, len as usize, raw)
}

pub(crate) fn read_seq_u32_bounded(
    r: &mut dyn SafeRead,
    bound: u32,
    msg: &str,
    data: &mut Vec<u8>,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    let len = read_u32(r, raw)?;
    if len > bound {
        return Err(msg.into());
    }
    data.resize(len as usize, 0);
    read_bytes(r, &mut data[..], raw)
}

pub(crate) fn read_seq_u64_bounded(
    r: &mut dyn SafeRead,
    bound: u64,
    msg: &str,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let len = read_u64(r, raw)?;
    if len > bound {
        return Err(msg.into());
    }
    read_vec(r, len as usize, raw)
}

pub(crate) fn read_str_u16(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<String, Error> {
    let len = read_u16(r, raw)?;
    let result = read_vec(r, len as usize, raw)?;
    let result = String::from_utf8(result).map_err(|e| ser_utf8(e))?;
    Ok(result)
}
