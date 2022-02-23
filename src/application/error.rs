//! The `error` module defines specific error types.

/// An application wide error type.
#[derive(Debug)]
pub enum PhyrexianError {
    /// A generic error.
    GenericError(String),
    /// An IO error.
    IOError(String),
    /// An IO error.
    ConversionError(String),
}

impl From<serde_json::error::Error> for PhyrexianError {
    fn from(error: serde_json::error::Error) -> Self {
        PhyrexianError::ConversionError(error.to_string())
    }
}

impl From<bincode::ErrorKind> for PhyrexianError {
    fn from(error: bincode::ErrorKind) -> Self {
        PhyrexianError::ConversionError(error.to_string())
    }
}

impl From<Box<bincode::ErrorKind>> for PhyrexianError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        (*error).into()
    }
}

impl From<std::io::Error> for PhyrexianError {
    fn from(error: std::io::Error) -> Self {
        PhyrexianError::IOError(error.to_string())
    }
}

impl From<uuid::Error> for PhyrexianError {
    fn from(error: uuid::Error) -> Self {
        PhyrexianError::ConversionError(error.to_string())
    }
}

impl From<chrono::ParseError> for PhyrexianError {
    fn from(error: chrono::ParseError) -> Self {
        PhyrexianError::ConversionError(error.to_string())
    }
}

impl From<String> for PhyrexianError {
    fn from(error: String) -> Self {
        PhyrexianError::GenericError(error)
    }
}

impl From<&str> for PhyrexianError {
    fn from(error: &str) -> Self {
        PhyrexianError::GenericError(error.to_string())
    }
}