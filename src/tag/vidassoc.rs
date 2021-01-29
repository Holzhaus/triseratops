//! The `Serato VidAssoc` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use super::format::enveloped;
use super::format::flac;
use super::format::mp4;
use crate::error::Error;
use crate::util;
use crate::util::Res;

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

impl enveloped::EnvelopedTag for VidAssoc {}
impl flac::FLACTag for VidAssoc {
    const FLAC_COMMENT: &'static str = "SERATO_VIDASSOC";
}
impl mp4::MP4Tag for VidAssoc {
    const MP4_ATOM: &'static str = "----:com.serato.dj:videoassociation";
}

pub fn take_vidassoc(input: &[u8]) -> Res<&[u8], VidAssoc> {
    let (input, version) = util::take_version(input)?;
    let (input, _) =
        nom::error::context("unknown bytes", nom::bytes::complete::tag(b"\x01\x00\x00"))(input)?;

    let vidassoc = VidAssoc { version };
    Ok((input, vidassoc))
}
