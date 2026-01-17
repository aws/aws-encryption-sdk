use std::backtrace::Backtrace;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
/// Individual error types for [`aws_mpl`](crate)
pub enum ErrorKind {
    /// Higher level MPL errors
    Mpl,
    /// Invalid `AlgorithmSuiteInfo` on Encrypt
    InvalidAlgorithmSuiteInfoOnEncrypt,
    /// Invalid `AlgorithmSuiteInfo` on Decrypt
    InvalidAlgorithmSuiteInfoOnDecrypt,
    /// Bad input
    ValidationError,
    /// Invalid `AlgorithmSuiteInfo`
    InvalidAlgorithmSuiteInfo,
    EntryAlreadyExists,
    InvalidEncryptionMaterialsTransition,
    InvalidDecryptionMaterialsTransition,
    InvalidEncryptionMaterials,
    InvalidDecryptionMaterials,
    Consumer,
    NotImplemented,
}
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Base error type for [`aws_mpl`](crate)
pub struct Error {
    /// Error type
    pub kind: ErrorKind,
    /// message
    pub message: String,
    /// Backtrace captured when error was encountered.
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
            ErrorKind::Mpl => write!(f, "MPL Error {}", self.message),
            ErrorKind::InvalidAlgorithmSuiteInfoOnEncrypt => {
                write!(
                    f,
                    "InvalidAlgorithmSuiteInfoOnEncrypt Error {}",
                    self.message
                )
            }
            ErrorKind::InvalidAlgorithmSuiteInfoOnDecrypt => {
                write!(
                    f,
                    "InvalidAlgorithmSuiteInfoOnDecrypt Error {}",
                    self.message
                )
            }
            ErrorKind::InvalidAlgorithmSuiteInfo => {
                write!(f, "InvalidAlgorithmSuiteInfo Error {}", self.message)
            }
            ErrorKind::ValidationError => write!(f, "Validation Error {}", self.message),
            ErrorKind::EntryAlreadyExists => {
                write!(f, "Entry Already Exists Error {}", self.message)
            }
            ErrorKind::InvalidEncryptionMaterialsTransition => {
                write!(
                    f,
                    "Invalid Encryption Materials Transition Error {}",
                    self.message
                )
            }
            ErrorKind::InvalidDecryptionMaterialsTransition => {
                write!(
                    f,
                    "Invalid Decryption Materials Transition Error {}",
                    self.message
                )
            }
            ErrorKind::InvalidEncryptionMaterials => {
                write!(f, "Invalid Encryption Materials Error {}", self.message)
            }
            ErrorKind::InvalidDecryptionMaterials => {
                write!(f, "Invalid Decryption Materials Error {}", self.message)
            }
            ErrorKind::Consumer => {
                write!(f, "Consumer {}", self.message)
            }
            ErrorKind::NotImplemented => write!(f, "{} not yet implemented.", self.message),
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

pub(crate) fn mpl_err(msg: impl Into<String>) -> Error {
    Error {
        kind: ErrorKind::Mpl,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    }
}

pub(crate) fn not_implemented<T>(msg: impl Into<String>) -> Result<T, Error> {
    Err(Error {
        kind: ErrorKind::NotImplemented,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    })
}

#[must_use]
pub fn err(e: ErrorKind, msg: impl Into<String>) -> Error {
    Error {
        kind: e,
        message: msg.into(),
        backtrace: Arc::new(Backtrace::capture()),
        cause: None,
    }
}
