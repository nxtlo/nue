use core::{
    array::TryFromSliceError,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

use alloc::string::ToString;

/// Possible error types which may occur in NFC card operations, HTTP requests, and database operations.
#[derive(Debug)]
pub enum Error {
    // System Errors
    NfcReadError,
    NfcWriteError,
    NfcCardEmpty,
    NfcCardUnrecognized,
    // General Card Errors.
    // May be used for both HTTP and DB errors.
    CardAlreadyExist,
    CardNotFound,
    // Authentication Errors.
    CardDecryptionError,
    CardEncryptionError,
    // Database Errors.
    CommitError,
    DBError,
    // General Errors.
    ByteFillError(getrandom::Error),
    ConvertError(TryFromSliceError),
}

impl From<chacha20poly1305::Error> for Error {
    fn from(e: chacha20poly1305::Error) -> Self {
        let src = e.to_string();
        if src.contains("encrypt") {
            Error::CardEncryptionError
        } else {
            Error::CardDecryptionError
        }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Error::ConvertError(e)
    }
}

impl From<getrandom::Error> for Error {
    fn from(e: getrandom::Error) -> Self {
        Error::ByteFillError(e)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(e: core::convert::Infallible) -> Self {
        match e {}
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl StdError for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::NfcReadError => write!(f, "Failed to read NFC card"),
            Error::NfcWriteError => write!(f, "Failed to write NFC card"),
            Error::NfcCardEmpty => write!(f, "This card is empty or unintialized"),
            Error::NfcCardUnrecognized => write!(f, "NFC card unrecognized"),
            Error::CardAlreadyExist => write!(f, "Card already exists"),
            Error::CardNotFound => write!(f, "Card not found"),
            Error::CardDecryptionError => write!(f, "Card decryption error"),
            Error::CardEncryptionError => write!(f, "Card encryption error"),
            Error::CommitError => write!(f, "Storage failed to commit"),
            Error::DBError => write!(f, "Database storage error"),
            Error::ByteFillError(e) => write!(f, "Failed to fill bytes: {}", e),
            Error::ConvertError(e) => write!(f, "Failed to convert: {}", e),
        }
    }
}
