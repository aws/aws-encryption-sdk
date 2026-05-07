// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Error types for the AWS Encryption SDK.

use std::backtrace::Backtrace;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
/// Individual error types for [`aws_esdk`](crate)
pub enum ErrorKind {
    /// Higher level ESDK errors
    Esdk,
    /// Error in serializing or deserializing the data
    SerializationError,
    /// Low level cryptographic error from `aws_mpl_legacy::primitives`
    CryptographicError,
    /// Mid level cryptographic error from `aws_mpl_legacy`
    MplError,
    /// Mid level cryptographic error from `aws_mpl_legacy`
    LegacyError(Arc<aws_mpl_legacy::dafny::types::error::Error>),
    /// Malformed input. No cryptography has been attempted.
    ValidationError,
}
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Base error type for [`aws_esdk`](crate)
pub struct Error {
    /// Error type
    pub kind: ErrorKind,
    /// message
    pub message: String,
    /// Backtrace captured when error was encountered.
    /// For `LegacyError` the backtrace is not captured until the ESDK level
    pub backtrace: Arc<Backtrace>,
    /// The Error causing the Error, if any.
    pub cause: Option<Arc<dyn std::error::Error + Send + Sync + 'static>>,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.message == other.message
    }
}
impl Eq for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Esdk => write!(f, "Esdk Error {}", self.message),
            ErrorKind::SerializationError => write!(f, "Serialization Error {}", self.message),
            ErrorKind::CryptographicError => write!(f, "Cryptographic Error {}", self.message),
            ErrorKind::MplError => write!(f, "MPL Error {:?} {}", self.cause, self.message),
            ErrorKind::LegacyError(e) => write!(f, "Legacy MPL Error {e} {}", self.message),
            ErrorKind::ValidationError => write!(f, "Validation Error {}", self.message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            Some(cause) => Some(cause.as_ref()),
            None => None,
        }
    }
}

#[track_caller]
pub(crate) fn val_err(msg: impl Into<String>) -> Error {
    Error {
        kind: ErrorKind::ValidationError,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    }
}

#[track_caller]
pub(crate) fn val_err_with_cause(msg: impl Into<String>, cause: impl std::error::Error + Send + Sync + 'static) -> Error {
    Error {
        kind: ErrorKind::ValidationError,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: Some(Arc::new(cause)),
    }
}

#[track_caller]
pub(crate) fn esdk_err(msg: impl Into<String>) -> Error {
    Error {
        kind: ErrorKind::Esdk,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    }
}

#[track_caller]
pub(crate) fn ser_err<T>(msg: &str) -> Result<T, Error> {
    Err(Error {
        kind: ErrorKind::SerializationError,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    })
}

impl From<aws_mpl_legacy::dafny::types::error::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_legacy::dafny::types::error::Error) -> Self {
        Self {
            kind: ErrorKind::LegacyError(Arc::new(item)),
            message: String::new(),
            backtrace: Arc::new(Backtrace::capture()),
            cause: None,
        }
    }
}

impl From<aws_mpl_legacy::error::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_legacy::error::Error) -> Self {
        Self {
            kind: ErrorKind::MplError,
            message: String::new(),
            backtrace: item.backtrace.clone(),
            cause: Some(Arc::new(item)),
        }
    }
}

impl From<aws_mpl_legacy::primitives::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_legacy::primitives::Error) -> Self {
        Self {
            kind: ErrorKind::CryptographicError,
            message: item.message,
            backtrace: item.backtrace,
            cause: None,
        }
    }
}

impl From<aws_smithy_types::error::operation::BuildError> for Error {
    #[track_caller]
    fn from(item: aws_smithy_types::error::operation::BuildError) -> Self {
        Self {
            kind: ErrorKind::ValidationError,
            message: format!("{item:?}"),
            backtrace: Arc::new(Backtrace::capture()),
            cause: Some(Arc::new(item)),
        }
    }
}

impl From<&str> for Error {
    #[track_caller]
    fn from(item: &str) -> Self {
        Self {
            kind: ErrorKind::Esdk,
            message: item.to_string(),
            backtrace: Arc::new(Backtrace::capture()),
            cause: None,
        }
    }
}

impl From<String> for Error {
    #[track_caller]
    fn from(item: String) -> Self {
        Self {
            kind: ErrorKind::Esdk,
            message: item,
            backtrace: Arc::new(Backtrace::capture()),
            cause: None,
        }
    }
}
