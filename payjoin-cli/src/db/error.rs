use std::fmt;

#[cfg(feature = "v2")]
use bitcoincore_rpc::jsonrpc::serde_json;
use payjoin::ImplementationError;
use r2d2::Error as R2d2Error;
use rusqlite::Error as RusqliteError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Sled(SledError),
    #[cfg(feature = "v2")]
    Serialize(serde_json::Error),
    #[cfg(feature = "v2")]
    Deserialize(serde_json::Error),
    #[cfg(feature = "v2")]
    NotFound(String),
    #[cfg(feature = "v2")]
    TryFromSlice(std::array::TryFromSliceError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Sled(e) => write!(f, "Database operation failed: {e}"),
            #[cfg(feature = "v2")]
            Error::Serialize(e) => write!(f, "Serialization failed: {e}"),
            #[cfg(feature = "v2")]
            Error::Deserialize(e) => write!(f, "Deserialization failed: {e}"),
            #[cfg(feature = "v2")]
            Error::NotFound(key) => write!(f, "Key not found: {key}"),
            #[cfg(feature = "v2")]
            Error::Deserialize(e) => Some(e),
        }
    }
}

impl From<RusqliteError> for Error {
    fn from(error: RusqliteError) -> Self { Error::Rusqlite(error) }
}

impl From<R2d2Error> for Error {
    fn from(error: R2d2Error) -> Self { Error::R2d2(error) }
}

#[cfg(feature = "v2")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        match error.classify() {
            serde_json::error::Category::Io => Error::Serialize(error), // I/O errors during writing/serialization
            serde_json::error::Category::Syntax
            | serde_json::error::Category::Data
            | serde_json::error::Category::Eof => Error::Deserialize(error), // All parsing/reading errors
        }
    }
}

impl From<Error> for ImplementationError {
    fn from(error: Error) -> Self { ImplementationError::new(error) }
}
