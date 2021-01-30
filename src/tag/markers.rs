//! The `Serato Markers_` tag stores information about the first 5 Cues, 9 Loops and the track
//! color.
//!
//! This is redundant with some of the information from the `Serato Markers2` tag. Serato will
//! prefer information from `Serato Markers_` if it's present.

use super::format::enveloped;
use super::format::id3;
use super::format::mp4;
use crate::error::Error;
use crate::util;
use crate::util::{Res, Tag};
use nom::error::ParseError;

/// Represents a single marker in the `Serato Markers_` tag.
#[derive(Debug)]
pub struct Marker {
    /// The position of the loop or cue.
    pub start_position_millis: Option<u32>,

    /// If this is a loop, this field stores the end position.
    pub end_position_millis: Option<u32>,

    /// The color of the cue.
    ///
    /// For loop, this field should always be `#27AAE1`.
    pub color: util::Color,

    /// The type of this marker.
    pub entry_type: EntryType,

    /// Indicates whether the loop is locked.
    ///
    /// For cues, this field should always be `false`.
    pub is_locked: bool,
}

/// Represents the `Serato Markers_` tag.
#[derive(Debug)]
pub struct Markers {
    /// The tag version.
    pub version: util::Version,

    /// The marker entries.
    pub entries: Vec<Marker>,

    /// The color of the track in Serato's library view.
    pub track_color: util::Color,
}

impl Markers {
    pub fn cues(&self) -> Vec<(u8, &Marker)> {
        let mut index: u8 = 0;
        let mut cues = Vec::new();
        for marker in &self.entries {
            if marker.entry_type != EntryType::INVALID && marker.entry_type != EntryType::CUE {
                continue;
            }

            cues.push((index, marker));
            index += 1;
        }
        cues
    }

    pub fn loops(&self) -> Vec<(u8, &Marker)> {
        let mut index: u8 = 0;
        let mut loops = Vec::new();
        for marker in &self.entries {
            if marker.entry_type != EntryType::LOOP {
                continue;
            }

            loops.push((index, marker));
            index += 1;
        }
        loops
    }

    pub fn track_color(&self) -> util::Color {
        self.track_color
    }
}

impl util::Tag for Markers {
    const NAME: &'static str = "Serato Markers_";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_markers)(input)?;
        Ok(autotags)
    }
}

impl id3::ID3Tag for Markers {}
impl enveloped::EnvelopedTag for Markers {}
impl mp4::MP4Tag for Markers {
    const MP4_ATOM: &'static str = "----:com.serato.dj:markers";

    fn parse_mp4(input: &[u8]) -> Result<Self, Error> {
        let (_, encoded) = nom::combinator::all_consuming(
            super::format::enveloped::take_base64_with_newline,
        )(input)?;
        let content = super::format::enveloped::envelope_decode_with_name(encoded, Self::NAME)?;
        let (_, markers) = nom::combinator::all_consuming(take_markers_mp4)(content.as_slice())?;
        Ok(markers)
    }
}

/// The Type of a Marker.
///
/// # Values
///
/// | Value  | `EntryType` | Description
/// | ------ | ----------- | ----------------------------------------
/// | `0x00` | `INVALID`   | Used for unset cues.
/// | `0x01` | `CUE`       | Used for cues.
/// | `0x03` | `LOOP`      | Used for loops (both set and unset ones).
#[derive(Debug, PartialEq)]
pub enum EntryType {
    INVALID,
    CUE,
    LOOP,
}

/// Returns a bool parsed from the next input byte.
///
/// This function returns `false` if the byte is `0x00`, else `true`.
///
/// # Example
/// ```
/// use serato_tags::tag::markers::take_bool;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_bool(&[0x00]), Ok((&[][..], false)));
/// assert_eq!(take_bool(&[0x01]), Ok((&[][..], true)));
/// assert!(take_bool(&[0xAB, 0x00, 0x01]).is_err());
/// assert!(take_bool(&[]).is_err());
/// ```
pub fn take_bool(input: &[u8]) -> Res<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, false)),
        0x01 => Ok((input, true)),
        _ => Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/// Returns a bool parsed from the next input byte that indicates if the following marker position
/// is set.
///
/// Do not use `take_bool` for this, because the value mapping is different:
///
/// | Byte   | `bool`  | Description
/// | ------ | ------- | ----------------------------------------------------------------
/// | `0x00` | `true`  | The following 4 bytes contain the position in `serato32` format.
/// | `0x7F` | `false` | The position is not set and following 4 bytes be `0x7f7f7f7f`.
/// | Other  | `_`     | Invalid data, throws an error.
///
///
/// # Example
/// ```
/// use serato_tags::tag::markers::take_has_position;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_has_position(&[0x00]), Ok((&[][..], true)));
/// assert_eq!(take_has_position(&[0x7F]), Ok((&[][..], false)));
/// assert_eq!(take_has_position(&[0x00, 0x05]), Ok((&[0x05][..], true)));
/// assert!(take_has_position(&[0xAB, 0x00, 0x01]).is_err());
/// assert!(take_has_position(&[]).is_err());
/// ```
pub fn take_has_position(input: &[u8]) -> Res<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, true)),
        0x7F => Ok((input, false)),
        _ => Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/// Returns an `Option<u32>` which contains the position parsed from the next 5 input bytes.
///
/// Uses `take_has_position` internally to determine if the position is set, then either returns
/// the position as `Some` or ensures the that "no position" constant is used and returns `None`.
///
/// # Example
/// ```
/// use serato_tags::tag::markers::take_position;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_position(&[0x00, 0x00, 0x00, 0x00, 0x00]), Ok((&[][..], Some(0))));
/// assert_eq!(take_position(&[0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00]), Ok((&[0x00][..], None)));
/// ```
pub fn take_position(input: &[u8]) -> Res<&[u8], Option<u32>> {
    if input.len() < 5 {
        return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    let (input, has_position) = nom::error::context("take has_position", take_has_position)(input)?;
    match has_position {
        true => {
            let (input, data) = util::serato32::take_u32(input)?;
            Ok((input, Some(data)))
        }
        false => {
            let (input, _) = nom::bytes::complete::tag(b"\x7f\x7f\x7f\x7f")(input)?;
            Ok((input, None))
        }
    }
}

/// Returns the `EntryType` for the cue marker parsed from the next input byte.
///
/// # Example
/// ```
/// use serato_tags::tag::markers::{EntryType, take_entry_type};
///
/// assert_eq!(take_entry_type(&[0x00]), Ok((&[][..], EntryType::INVALID)));
/// assert_eq!(take_entry_type(&[0x03, 0x01]), Ok((&[0x01][..], EntryType::LOOP)));
/// assert!(take_entry_type(&[0xAB]).is_err());
/// ```
pub fn take_entry_type(input: &[u8]) -> Res<&[u8], EntryType> {
    let (next_input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((next_input, EntryType::INVALID)),
        0x01 => Ok((next_input, EntryType::CUE)),
        0x03 => Ok((next_input, EntryType::LOOP)),
        _ => Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/// Returns a `Marker` parsed from the input slice.
pub fn take_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, start_position_millis) =
        nom::error::context("marker start position", take_position)(input)?;
    let (input, end_position_millis) =
        nom::error::context("marker end position", take_position)(input)?;
    let (input, _) = nom::error::context(
        "marker unknown bytes",
        nom::bytes::complete::tag(b"\x00\x7F\x7F\x7F\x7F\x7F"),
    )(input)?;
    let (input, color) = nom::error::context("marker color", util::serato32::take_color)(input)?;
    let (input, entry_type) = nom::error::context("marker type", take_entry_type)(input)?;
    let (input, is_locked) = nom::error::context("marker locked state", take_bool)(input)?;
    Ok((
        input,
        Marker {
            start_position_millis,
            end_position_millis,
            color,
            entry_type,
            is_locked,
        },
    ))
}

/// Parses the data into a `Markers` struct, consuming the whole input slice.
pub fn take_markers(input: &[u8]) -> Res<&[u8], Markers> {
    let (input, version) = util::take_version(&input)?;
    let (input, entries) =
        nom::multi::length_count(nom::number::complete::be_u32, take_marker)(input)?;
    let (input, track_color) = nom::combinator::all_consuming(util::serato32::take_color)(input)?;

    let markers = Markers {
        version,
        entries,
        track_color,
    };
    Ok((input, markers))
}

/// Returns a `Marker` parsed from the input slice (MP4 version).
pub fn take_marker_mp4(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, start_position_millis_raw) =
        nom::error::context("marker start position", nom::number::complete::be_u32)(input)?;
    let (input, end_position_millis_raw) =
        nom::error::context("marker end position", nom::number::complete::be_u32)(input)?;
    let (input, _) =
        nom::error::context("marker unknown bytes", nom::bytes::complete::take(6usize))(input)?;
    let (input, color) = nom::error::context("marker color", util::take_color)(input)?;
    let (input, entry_type) = nom::error::context("marker type", take_entry_type)(input)?;
    let (input, is_locked) = nom::error::context("marker locked state", take_bool)(input)?;

    let start_position_millis = Some(start_position_millis_raw);
    let end_position_millis = if entry_type == EntryType::LOOP {
        Some(end_position_millis_raw)
    } else {
        None
    };
    Ok((
        input,
        Marker {
            start_position_millis,
            end_position_millis,
            color,
            entry_type,
            is_locked,
        },
    ))
}

/// Parses the data into a `Markers` struct, consuming the whole input slice (MP4 version).
pub fn take_markers_mp4(input: &[u8]) -> Res<&[u8], Markers> {
    let (input, version) = util::take_version(&input)?;
    let (input, entries) =
        nom::multi::length_count(nom::number::complete::be_u32, take_marker_mp4)(input)?;
    let (input, track_color) = nom::combinator::all_consuming(util::serato32::take_color)(input)?;

    let markers = Markers {
        version,
        entries,
        track_color,
    };
    Ok((input, markers))
}

pub fn parse(input: &[u8]) -> Result<Markers, Error> {
    let (_, markers) = nom::combinator::all_consuming(take_markers)(input)?;
    Ok(markers)
}
