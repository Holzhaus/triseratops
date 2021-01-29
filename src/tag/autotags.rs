//! The `Serato Autotags` tag stores BPM and Gain values.

use super::format::flac;
use super::format::id3;
use crate::error::Error;
use crate::util;
use crate::util::Res;

/// Represents the  `Serato AutoTags` tag.
#[derive(Debug)]
pub struct Autotags {
    /// The tag version.
    pub version: util::Version,
    /// The track's number of beats per minute (BPM).
    pub bpm: f64,
    /// The track's autogain values (probably comparable to ReplayGain).
    pub auto_gain: f64,
    /// The track's gain value (manual?).
    pub gain_db: f64,
}

impl util::Tag for Autotags {
    const NAME: &'static str = "Serato Autotags";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_autotags)(input)?;
        Ok(autotags)
    }
}

impl id3::ID3Tag for Autotags {}
impl flac::FLACTag for Autotags {}

/// Returns an `f64` parsed from zero-terminated ASCII chars the input slice.
///
/// # Example
/// ```
/// use serato_tags::tag::autotags::take_double_str;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_double_str(&[0x31, 0x31, 0x35, 0x2E, 0x30, 0x30, 0x00]), Ok((&[][..], 115.0)));
/// assert_eq!(take_double_str(&[0x2D, 0x33, 0x2E, 0x32, 0x35, 0x37, 0x00, 0xAB]), Ok((&[0xAB][..], -3.257)));
/// assert!(take_double_str(&[0xAB, 0x01]).is_err());
/// ```
pub fn take_double_str(input: &[u8]) -> Res<&[u8], f64> {
    let (input, text) = util::take_until_nullbyte(input)?;
    let (_, num) = nom::combinator::all_consuming(nom::number::complete::double)(text)?;
    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, num))
}

pub fn take_autotags(input: &[u8]) -> Res<&[u8], Autotags> {
    let (input, version) = util::take_version(input).unwrap();
    let (input, bpm) = take_double_str(input)?;
    let (input, auto_gain) = take_double_str(input)?;
    let (input, gain_db) = take_double_str(input)?;

    let autotags = Autotags {
        version,
        bpm,
        auto_gain,
        gain_db,
    };

    Ok((input, autotags))
}
