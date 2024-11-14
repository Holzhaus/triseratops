// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Error types and helper functions.

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
    #[error("Failed to encode base64 data")]
    Base64EncodeError { source: base64::EncodeSliceError },

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

    /// Happens when trying to write tag data even though no data is available.
    #[error("No tag data available")]
    NoTagDataAvailable,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn map_err(item: (&[u8], nom::error::VerboseErrorKind)) -> (Vec<u8>, nom::error::VerboseErrorKind) {
    let (data, kind) = item;
    (data.to_owned(), kind)
}

impl From<nom::Err<nom::error::VerboseError<&[u8]>>> for Error {
    fn from(e: nom::Err<nom::error::VerboseError<&[u8]>>) -> Self {
        match e {
            nom::Err::Error(err) => {
                let errors = err.errors.into_iter().map(map_err).collect();
                Error::VerboseParseError { errors }
            }
            nom::Err::Failure(err) => {
                let errors = err.errors.into_iter().map(map_err).collect();
                Error::VerboseParseError { errors }
            }
            nom::Err::Incomplete(_needed) => Error::ParseError,
        }
    }
}
