// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use super::*;
use std::backtrace::Backtrace;
use std::sync::Arc;

pub(crate) trait SafeRead: std::io::Read + Send + Sync + std::fmt::Debug {}
impl<T: std::io::Read + Send + Sync + std::fmt::Debug> SafeRead for T {}

#[track_caller]
fn ser_io(e: std::io::Error) -> Error {
    match e.kind() {
        std::io::ErrorKind::UnexpectedEof => Error {
            kind: ErrorKind::DbEsdk,
            message: "Unexpected end of data".into(),
            cause: Some(Arc::new(e)),
            backtrace: Arc::new(Backtrace::capture()),
        },
        _ => Error {
            kind: ErrorKind::DbEsdk,
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
        kind: ErrorKind::DbEsdk,
        message: "UTF8 Decode Error".into(),
        cause: Some(Arc::new(item)),
        backtrace: Arc::new(Backtrace::capture()),
    }
}

#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) fn write_bytes(w: &mut Vec<u8>, data: &[u8]) {
    w.extend_from_slice(data);
}

pub(crate) fn write_str_u16(w: &mut Vec<u8>, data: &str) -> Result<(), Error> {
    write_seq_u16(w, data.as_bytes())
}
pub(crate) fn write_u8(w: &mut Vec<u8>, data: u8) {
    write_bytes(w, &[data]);
}
pub(crate) fn write_u16(w: &mut Vec<u8>, data: u16) {
    write_bytes(w, &data.to_be_bytes());
}
pub(crate) fn write_u32(w: &mut Vec<u8>, data: u32) {
    write_bytes(w, &data.to_be_bytes());
}
pub(crate) fn write_u64(w: &mut Vec<u8>, data: u64) {
    write_bytes(w, &data.to_be_bytes());
}
pub(crate) fn write_seq_u16(w: &mut Vec<u8>, data: &[u8]) -> Result<(), Error> {
    match u16::try_from(data.len()) {
        Ok(len) => {
            write_u16(w, len);
            write_bytes(w, data);
            Ok(())
        }
        Err(_) => Err(val_err("Sequence length too long for 16 bits")),
    }
}

pub(crate) fn read_bytes(r: &mut dyn SafeRead, buf: &mut [u8]) -> Result<(), Error> {
    r.read_exact(buf).map_err(|e| ser_io(e))
}

pub(crate) fn read_array<const N: usize>(r: &mut dyn SafeRead) -> Result<[u8; N], Error> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf).map_err(|e| ser_io(e))?;
    Ok(buf)
}

pub(crate) fn read_vec(r: &mut dyn SafeRead, length: usize) -> Result<Vec<u8>, Error> {
    let mut result = vec![0; length];
    read_bytes(r, &mut result)?;
    Ok(result)
}

pub(crate) fn read_u8(r: &mut dyn SafeRead) -> Result<u8, Error> {
    let mut result = [0u8; 1];
    read_bytes(r, &mut result)?;
    Ok(result[0])
}

pub(crate) fn read_opt_u8(r: &mut dyn SafeRead) -> Result<Option<u8>, Error> {
    let mut result = [0u8; 1];
    match r.read_exact(&mut result) {
        Ok(()) => Ok(Some(result[0])),
        Err(e) => match e.kind() {
            std::io::ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(Error {
                kind: ErrorKind::DbEsdk,
                message: "IO Error".into(),
                cause: Some(Arc::new(e)),
                backtrace: Arc::new(Backtrace::capture()),
            }),
        },
    }
}

pub(crate) fn read_u16(r: &mut dyn SafeRead) -> Result<u16, Error> {
    let mut result = [0u8; 2];
    read_bytes(r, &mut result)?;
    Ok(u16::from_be_bytes(result))
}
pub(crate) fn read_u32(r: &mut dyn SafeRead) -> Result<u32, Error> {
    let mut result = [0u8; 4];
    read_bytes(r, &mut result)?;
    Ok(u32::from_be_bytes(result))
}
pub(crate) fn read_u64(r: &mut dyn SafeRead) -> Result<u64, Error> {
    let mut result = [0u8; 8];
    read_bytes(r, &mut result)?;
    Ok(u64::from_be_bytes(result))
}

pub(crate) fn read_seq_u16(r: &mut dyn SafeRead) -> Result<Vec<u8>, Error> {
    let len = read_u16(r)?;
    read_vec(r, len as usize)
}

pub(crate) fn read_seq_u32_bounded(
    r: &mut dyn SafeRead,
    bound: u32,
    msg: &str,
    data: &mut Vec<u8>,
) -> Result<(), Error> {
    let len = read_u32(r)?;
    if len > bound {
        return Err(msg.into());
    }
    data.resize(len as usize, 0);
    read_bytes(r, &mut data[..])
}

pub(crate) fn read_seq_u64_bounded(
    r: &mut dyn SafeRead,
    bound: u64,
    msg: &str,
) -> Result<Vec<u8>, Error> {
    let len = read_u64(r)?;
    if len > bound {
        return Err(msg.into());
    }
    read_vec(r, len as usize)
}

pub(crate) fn read_str_u16(r: &mut dyn SafeRead) -> Result<String, Error> {
    let result = read_seq_u16(r)?;
    let result = String::from_utf8(result).map_err(|e| ser_utf8(e))?;
    Ok(result)
}
