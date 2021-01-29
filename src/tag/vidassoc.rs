//! The `Serato VidAssoc` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use super::format::flac;
use crate::error::Error;
use crate::util;
use crate::util::Res;

const TAGNAME: &str = "Serato VidAssoc";

/// Represents the  `Serato VidAssoc` tag.
#[derive(Debug)]
pub struct VidAssoc {
    /// The `VidAssoc` version.
    pub version: util::Version,
}

impl util::Tag for VidAssoc {
    const NAME: &'static str = "Serato VidAssoc";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, vidassoc) = nom::combinator::all_consuming(take_vidassoc)(input)?;
        Ok(vidassoc)
    }
}

impl flac::FLACTag for VidAssoc {}

pub fn take_vidassoc(input: &[u8]) -> Res<&[u8], VidAssoc> {
    let (input, version) = util::take_version(input)?;
    let (input, _) =
        nom::error::context("unknown bytes", nom::bytes::complete::tag(b"\x01\x00\x00"))(input)?;

    let vidassoc = VidAssoc { version };
    Ok((input, vidassoc))
}

pub fn parse_common(input: &[u8]) -> Result<VidAssoc, Error> {
    let (_, vidassoc) =
        nom::combinator::all_consuming(nom::error::context("vidassoc", take_vidassoc))(input)?;
    Ok(vidassoc)
}

pub fn parse(input: &[u8]) -> Result<VidAssoc, Error> {
    let content = flac::envelope_decode_with_name(input, TAGNAME)?;
    parse_common(&content)
}
