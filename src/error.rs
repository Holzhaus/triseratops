//! Error types and helper functions.

extern crate base64;
extern crate thiserror;

/// Error enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Represents an generic parse error.
    #[error("Generic parse error")]
    ParseError,

    /// Thrown when trying to parse a tag with an unsupported tag format.
    #[error("Unsupported tag format")]
    UnsupportedTagFormat,

    /// Represents an generic parse error.
    #[error("Incomplete parse error")]
    ParseIncomplete(nom::Needed),

    /// Represents an generic parse error.
    #[error("Nom parse error")]
    VerboseParseError {
        errors: Vec<(Vec<u8>, nom::error::VerboseErrorKind)>,
    },

    /// Represents decode error.
    #[error("Malformed base64 data")]
    Base64DecodeError { source: base64::DecodeError },

    /// Represents decode error.
    #[error("Invalid Base64 length")]
    Base64InvalidLengthError { length: usize },

    /// Represents decode error.
    #[error("Malformed envelope content")]
    EnvelopeParseError,

    /// Represents decode error.
    #[error("Envelope name mismatch")]
    EnvelopeNameMismatch { expected: String, actual: String },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn convert_err(
    item: &(&[u8], nom::error::VerboseErrorKind),
) -> (Vec<u8>, nom::error::VerboseErrorKind) {
    let (data, kind) = item;
    (data.to_vec(), kind.to_owned())
}

impl From<nom::Err<nom::error::VerboseError<&[u8]>>> for Error {
    fn from(e: nom::Err<nom::error::VerboseError<&[u8]>>) -> Self {
        match e {
            nom::Err::Error(err) => {
                let errors = err.errors.iter().map(convert_err).collect();
                Error::VerboseParseError { errors }
            }
            nom::Err::Failure(err) => {
                let errors = err.errors.iter().map(convert_err).collect();
                Error::VerboseParseError { errors }
            }
            nom::Err::Incomplete(_needed) => Error::ParseError,
        }
    }
}
