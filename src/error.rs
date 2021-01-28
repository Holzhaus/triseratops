extern crate base64;
extern crate thiserror;

/// Error enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Represents an generic parse error.
    #[error("Malformed input")]
    ParseError,

    /// Represents decode error.
    #[error("Malformed base64 data")]
    Base64DecodeError { source: base64::DecodeError },

    /// Represents decode error.
    #[error("Malformed envelope content")]
    EnvelopeParseError,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
