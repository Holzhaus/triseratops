//! The `Serato RelVolAd` tag stores the analysis version.

use super::format::{enveloped, flac, mp4, Tag};
use super::generic::Version;
use super::util::{take_version, write_version};
use crate::error::Error;
use crate::util::Res;
use std::io;

/// Represents the  `Serato RelVolAd` tag.
///
/// **Note:** This tag has not been reverse-engineered yet. Judging from the name it contains
/// information about relative volume adjustments, but at this point that is just speculation.
///
/// # Example
///
/// ```
/// use triseratops::tag::{RelVolAd, format::flac::FLACTag};
///
/// // First, read the tag data from the FLAC VORBIS_COMMENT (the tag name can be accessed using the
/// // RelVolAd::FLAC_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = RelVolAd::parse_flac(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug)]
pub struct RelVolAd {
    /// The `RelVolAd` version.
    pub version: Version,
    /// The data (not reverse-engineered yet)
    pub data: Vec<u8>,
}

impl Tag for RelVolAd {
    const NAME: &'static str = "Serato RelVolAd";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, overview) = nom::combinator::all_consuming(take_relvolad)(input)?;
        Ok(overview)
    }

    fn write(&self, writer: impl io::Write) -> Result<usize, Error> {
        write_relvolad(writer, self)
    }
}

impl enveloped::EnvelopedTag for RelVolAd {}
impl flac::FLACTag for RelVolAd {
    const FLAC_COMMENT: &'static str = "SERATO_RELVOL";
}
impl mp4::MP4Tag for RelVolAd {
    const MP4_ATOM_FREEFORM_NAME: &'static str = "relvol";
}

fn take_relvolad(input: &[u8]) -> Res<&[u8], RelVolAd> {
    let (input, version) = take_version(input)?;
    let (input, data) = nom::combinator::rest(input)?;
    let data = data.to_vec();

    let relvolad = RelVolAd { version, data };
    Ok((input, relvolad))
}

fn write_relvolad(mut writer: impl io::Write, relvolad: &RelVolAd) -> Result<usize, Error> {
    let mut bytes_written = write_version(&mut writer, &relvolad.version)?;
    bytes_written += writer.write(relvolad.data.as_slice())?;
    Ok(bytes_written)
}
