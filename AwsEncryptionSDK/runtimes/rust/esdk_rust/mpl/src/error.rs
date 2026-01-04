use std::backtrace::Backtrace;

#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
/// Individual error types for [`aws_mpl`](crate)
pub enum ErrorKind {
    /// Higher level MPL errors
    Mpl(String),
    /// Invalid `AlgorithmSuiteInfo` on Encrypt
    InvalidAlgorithmSuiteInfoOnEncrypt(String),
    /// Invalid `AlgorithmSuiteInfo` on Decrypt
    InvalidAlgorithmSuiteInfoOnDecrypt(String),
    /// Bad input
    ValidationError(String),
    /// Invalid `AlgorithmSuiteInfo`
    InvalidAlgorithmSuiteInfo(String),
    EntryAlreadyExists(String),
    InvalidEncryptionMaterialsTransition(String),
    InvalidDecryptionMaterialsTransition(String),
    InvalidEncryptionMaterials(String),
    InvalidDecryptionMaterials(String),
}
#[derive(Debug)]
#[non_exhaustive]
/// Base error type for [`aws_mpl`](crate)
pub struct Error {
    /// Error type
    pub kind: ErrorKind,
    /// Backtrace captured when error was encountered.
    pub backtrace: Backtrace,
    /// The Error causing the Error, if any.
    pub cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Mpl(message) => write!(f, "MPL Error {message}"),
            ErrorKind::InvalidAlgorithmSuiteInfoOnEncrypt(message) => write!(f, "InvalidAlgorithmSuiteInfoOnEncrypt Error {message}"),
            ErrorKind::InvalidAlgorithmSuiteInfoOnDecrypt(message) => write!(f, "InvalidAlgorithmSuiteInfoOnDecrypt Error {message}"),
            ErrorKind::InvalidAlgorithmSuiteInfo(message) => write!(f, "InvalidAlgorithmSuiteInfo Error {message}"),
            ErrorKind::ValidationError(message) => write!(f, "Validation Error {message}"),
            ErrorKind::EntryAlreadyExists(message) => write!(f, "Entry Already Exists Error {message}"),
            ErrorKind::InvalidEncryptionMaterialsTransition(message) => write!(f, "Invalid Encryption Materials Transition Error {message}"),
            ErrorKind::InvalidDecryptionMaterialsTransition(message) => write!(f, "Invalid Decryption Materials Transition Error {message}"),
            ErrorKind::InvalidEncryptionMaterials(message) => write!(f, "Invalid Encryption Materials Error {message}"),
            ErrorKind::InvalidDecryptionMaterials(message) => write!(f, "Invalid Decryption Materials Error {message}"),
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
        kind: ErrorKind::Mpl(msg.into()),
        backtrace: Backtrace::capture(),
        cause: None,
    }
}

pub(crate) fn err(e : ErrorKind) -> Error {
    Error {
        kind: e,
        backtrace: Backtrace::capture(),
        cause: None,
    }
}
