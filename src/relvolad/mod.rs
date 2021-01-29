//! The `Serato RelVolAd` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use crate::flac;
use crate::util;
use crate::util::Res;
use crate::error::Error;

/// Represents the  `Serato RelVolAd` tag.
#[derive(Debug)]
pub struct RelVolAd {
    /// The `RelVolAd` version.
    pub version: util::Version,
}

impl util::Tag for RelVolAd {
    const NAME : &'static str = "Serato RelVolAd";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, overview) = nom::combinator::all_consuming(take_relvolad)(input)?;
        Ok(overview)
    }

}

impl flac::FLACTag for RelVolAd {}


pub fn take_relvolad(input: &[u8]) -> Res<&[u8], RelVolAd> {
    let (input, version) = util::take_version(input)?;
    let (input, _) = nom::error::context("unknown bytes", nom::bytes::complete::tag(b"\x01\x00\x00"))(input)?;

    let relvolad = RelVolAd { version };
    Ok((input, relvolad))
}

pub fn parse(input: &[u8]) -> Result<RelVolAd, Error> {
    let (_, relvolad) = nom::combinator::all_consuming(take_relvolad)(input)?;
    Ok(relvolad)
}
