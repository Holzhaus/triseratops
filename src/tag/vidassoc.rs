//! The `Serato VidAssoc` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use super::format::{enveloped, flac, mp4, Tag};
use super::generic::Version;
use super::util::take_version;
use crate::error::Error;
use crate::util::Res;

/// Represents the  `Serato VidAssoc` tag.
///
/// **Note:** This tag has not been reverse-engineered yet. Judging from the name it contains
/// "Video Association" data.
///
/// # Example
///
/// ```
/// use seratodj::tag::{VidAssoc, format::flac::FLACTag};
///
/// // First, read the tag data from the FLAC VORBIS_COMMENT (the tag name can be accessed using the
/// // VidAssoc::FLAC_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = VidAssoc::parse_flac(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug)]
pub struct VidAssoc {
    /// The `VidAssoc` version.
    pub version: Version,
}

impl Tag for VidAssoc {
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

fn take_vidassoc(input: &[u8]) -> Res<&[u8], VidAssoc> {
    let (input, version) = take_version(input)?;
    let (input, _) =
        nom::error::context("unknown bytes", nom::bytes::complete::tag(b"\x01\x00"))(input)?;
    // TODO: what do these bytes mean?
    let (input, _) = nom::bytes::complete::take_while(|_| true)(input)?;

    let vidassoc = VidAssoc { version };
    Ok((input, vidassoc))
}
