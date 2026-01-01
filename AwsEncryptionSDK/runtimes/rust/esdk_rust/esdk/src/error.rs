use std::backtrace::Backtrace;

#[derive(Debug)]
#[non_exhaustive]
/// Individual error types for [`aws_esdk`](crate)
pub enum ErrorKind {
    /// Higher level ESDK errors
    Esdk(String),
    /// Error in serializing or deserializing the data
    SerializationError(String),
    /// Low level cryptographic error from `aws_mpl_primitives`
    CryptographicError(String),
    /// Mid level cryptographic error from `aws_mpl_rs`
    MplError(Box<aws_mpl_rs::error::Error>),
    /// Mid level cryptographic error from `aws_mpl_legacy`
    #[cfg(feature = "legacy")]
    LegacyError(Box<aws_mpl_legacy::types::error::Error>),
    /// Malformed input. No cryptography has been attempted.
    ValidationError(String),
}
#[derive(Debug)]
#[non_exhaustive]
/// Base error type for [`aws_esdk`](crate)
pub struct Error {
    /// Error type
    pub kind: ErrorKind,
    /// Backtrace captured when error was encountered.
    /// For `MplError` the backtrace is not captured until the ESDK level
    pub backtrace: Backtrace,
    /// The Error causing the Error, if any.
    pub cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Esdk(message) => write!(f, "Esdk Error {message}"),
            ErrorKind::SerializationError(message) => write!(f, "Serialization Error {message}"),
            ErrorKind::CryptographicError(message) => write!(f, "Cryptographic Error {message}"),
            ErrorKind::MplError(message) => write!(f, "MPL Error {message}"),
            #[cfg(feature = "legacy")]
            ErrorKind::LegacyError(message) => write!(f, "Legacy MPL Error {message}"),
            ErrorKind::ValidationError(message) => write!(f, "Validation Error {message}"),
        }
    }
}

pub(crate) fn val_err(msg: impl Into<String>) -> Error {
    Error {
        kind: ErrorKind::ValidationError(msg.into()),
        backtrace: Backtrace::capture(),
        cause: None,
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

#[cfg(feature = "legacy")]
impl From<aws_mpl_legacy::types::error::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_legacy::types::error::Error) -> Self {
        Self {
            kind: ErrorKind::LegacyError(Box::new(item)),
            backtrace: Backtrace::capture(),
            cause: None,
        }
    }
}

impl From<aws_mpl_rs::error::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_rs::error::Error) -> Self {
        Self {
            kind: ErrorKind::MplError(Box::new(item)),
            backtrace: Backtrace::capture(),
            cause: None,
        }
    }
}

impl From<aws_mpl_primitives::Error> for Error {
    #[track_caller]
    fn from(item: aws_mpl_primitives::Error) -> Self {
        Self {
            kind: ErrorKind::CryptographicError(item.msg),
            backtrace: item.backtrace,
            cause: None,
        }
    }
}

impl From<aws_smithy_types::error::operation::BuildError> for Error {
    fn from(item: aws_smithy_types::error::operation::BuildError) -> Self {
        Self {
            kind: ErrorKind::ValidationError(format!("{item:?}")),
            backtrace: Backtrace::capture(),
            cause: Some(Box::new(item)),
        }
    }
}

impl From<&str> for Error {
    fn from(item: &str) -> Self {
        Self {
            kind: ErrorKind::Esdk(item.to_string()),
            backtrace: Backtrace::capture(),
            cause: None,
        }
    }
}

impl From<String> for Error {
    fn from(item: String) -> Self {
        Self {
            kind: ErrorKind::Esdk(item),
            backtrace: Backtrace::capture(),
            cause: None,
        }
    }
}
