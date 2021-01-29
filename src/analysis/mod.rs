//! The `Serato Analysis` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use crate::util;
use crate::util::Res;
use crate::error::Error;

/// Represents the  `Serato Analysis` tag.
#[derive(Debug)]
pub struct Analysis {
    /// The analysis version.
    pub version: util::Version,
}

pub fn take_analysis(input: &[u8]) -> Res<&[u8], Analysis> {
    let (input, version) = util::take_version(input)?;
    let analysis = Analysis { version };

    Ok((input, analysis))
}

pub fn parse(input: &[u8]) -> Result<Analysis, Error> {
    let (_, analysis) = nom::combinator::all_consuming(take_analysis)(input)?;
    Ok(analysis)
}
