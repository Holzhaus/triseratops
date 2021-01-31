//! The `Serato Analysis` tag stores the analysis version.

use super::format::{enveloped, flac, id3, mp4, ogg, Tag};
use super::generic::Version;
use super::util::take_version;
use crate::error::Error;
use crate::util::{parse_utf8, Res};
use nom::error::ParseError;

/// Represents the  `Serato Analysis` tag.
///
/// This is probably the Serato Version number that performed the analysis.
///
/// # Example
///
/// ```
/// use serato_tags::tag::{Analysis, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Analysis::ID3_TAG), then parse the data like this:
/// fn parse_and_print_markers(data: &[u8]) {
///     let markers = Analysis::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", markers);
/// }
/// ```
#[derive(Debug)]
pub struct Analysis {
    /// The analysis version.
    pub version: Version,
}

impl Tag for Analysis {
    const NAME: &'static str = "Serato Analysis";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, analysis) = nom::combinator::all_consuming(take_analysis)(input)?;
        Ok(analysis)
    }
}

impl id3::ID3Tag for Analysis {}
impl enveloped::EnvelopedTag for Analysis {}
impl flac::FLACTag for Analysis {
    const FLAC_COMMENT: &'static str = "SERATO_ANALYSIS";
}
impl mp4::MP4Tag for Analysis {
    const MP4_ATOM: &'static str = "----:com.serato.dj:analysisVersion";
}

impl ogg::OggTag for Analysis {
    const OGG_COMMENT: &'static str = "serato_analysis_ver";

    fn parse_ogg(input: &[u8]) -> Result<Self, Error> {
        let (_, analysis) = nom::combinator::all_consuming(take_analysis_ogg)(input)?;
        Ok(analysis)
    }
}

/// Returns an `u8` parsed from ASCII char the input slice.
fn take_ascii_u8(input: &[u8]) -> Res<&[u8], u8> {
    let (input, ascii_number) = nom::error::context(
        "take ascii integer",
        nom::bytes::complete::take_while(|b: u8| b.is_ascii_digit()),
    )(input)?;
    let (_, ascii_number) = parse_utf8(ascii_number)?;
    match ascii_number.parse::<u8>() {
        Ok(number) => Ok((input, number)),
        Err(std::num::ParseIntError { .. }) => Err(nom::Err::Error(
            nom::error::VerboseError::from_error_kind(input, nom::error::ErrorKind::Digit),
        )),
    }
}

/// Returns an [`Analysis` struct](Analysis) parsed from the input slice.
fn take_analysis(input: &[u8]) -> Res<&[u8], Analysis> {
    let (input, version) = nom::error::context("take version", take_version)(input)?;
    let analysis = Analysis { version };

    Ok((input, analysis))
}

/// Returns an [`Analysis` struct](Analysis) parsed from the input slice ([MP4](mp4) version).
fn take_analysis_ogg(input: &[u8]) -> Res<&[u8], Analysis> {
    let (input, major) = nom::error::context("take major version", take_ascii_u8)(input)?;
    let (input, _) =
        nom::error::context("take version separator", nom::bytes::complete::tag(b"."))(input)?;
    let (input, minor) = nom::error::context("take major version", take_ascii_u8)(input)?;
    let version = Version { major, minor };

    let analysis = Analysis { version };
    Ok((input, analysis))
}
