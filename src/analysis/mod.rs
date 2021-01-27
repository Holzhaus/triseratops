//! The `Serato Analysis` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use crate::util;

/// Represents the  `Serato Analysis` tag.
#[derive(Debug)]
pub struct Analysis {
    /// The analysis version.
    pub version: util::Version,
}

pub fn parse(input: &[u8]) -> Result<Analysis, nom::Err<nom::error::Error<&[u8]>>> {
    match nom::combinator::all_consuming(util::take_version)(input) {
        Ok((_, version)) => Ok(Analysis { version }),
        Err(e) => Err(e),
    }
}
