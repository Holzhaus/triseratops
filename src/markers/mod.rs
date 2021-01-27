//! The `Serato Markers_` tag stores information about the first 5 Cues, 9 Loops and the track
//! color.
//!
//! This is redundant with some of the information from the `Serato Markers2` tag. Serato will
//! prefer information from `Serato Markers_` if it's present.

use crate::util;
use nom::length_count;
use nom::named;
use nom::number::complete::be_u32;
use nom::tag;

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

named!(no_position, tag!(b"\x7f\x7f\x7f\x7f"));
named!(unknown, tag!(b"\x00\x7f\x7f\x7f\x7f\x7f"));
named!(take_markers<Vec<Marker>>, length_count!(be_u32, marker));

/// Returns a bool parsed from the next input byte.
///
/// This function returns `false` if the byte is `0x00`, else `true`.
///
/// # Example
/// ```
/// use serato_tags::markers::take_bool;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_bool(&[0x00]), Ok((&[][..], false)));
/// assert_eq!(take_bool(&[0x01]), Ok((&[][..], true)));
/// assert_eq!(take_bool(&[0xAB, 0x00, 0x01]), Err(nom::Err::Incomplete(nom::Needed::Unknown)));
/// assert_eq!(take_bool(&[]), Err(Err::Error(Error::new(&[][..], ErrorKind::Eof))));
/// ```
pub fn take_bool(input: &[u8]) -> nom::IResult<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, false)),
        0x01 => Ok((input, true)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
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
/// use serato_tags::markers::take_has_position;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_has_position(&[0x00]), Ok((&[][..], true)));
/// assert_eq!(take_has_position(&[0x7F]), Ok((&[][..], false)));
/// assert_eq!(take_has_position(&[0x00, 0x05]), Ok((&[0x05][..], true)));
/// assert_eq!(take_has_position(&[0xAB, 0x00, 0x01]), Err(nom::Err::Incomplete(nom::Needed::Unknown)));
/// assert_eq!(take_has_position(&[]), Err(Err::Error(Error::new(&[][..], ErrorKind::Eof))));
/// ```
pub fn take_has_position(input: &[u8]) -> nom::IResult<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, true)),
        0x7F => Ok((input, false)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

/// Returns an `Option<u32>` which contains the position parsed from the next 5 input bytes.
///
/// Uses `take_has_position` internally to determine if the position is set, then either returns
/// the position as `Some` or ensures the that "no position" contant is used and returns `None`.
///
/// # Example
/// ```
/// use serato_tags::markers::take_position;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_position(&[0x00, 0x00, 0x00, 0x00, 0x00]), Ok((&[][..], Some(0))));
/// assert_eq!(take_position(&[0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00]), Ok((&[0x00][..], None)));
/// ```
pub fn take_position(input: &[u8]) -> nom::IResult<&[u8], Option<u32>> {
    let (input, has_position) = take_has_position(input)?;

    if input.len() < 4 {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    match has_position {
        true => {
            let (input, data) = util::serato32::take_u32(input)?;
            Ok((input, Some(data)))
        }
        false => {
            let (input, _) = no_position(input)?;
            Ok((input, None))
        }
    }
}

/// Returns the `EntryType` for the cue marker parsed from the next input byte.
///
/// # Example
/// ```
/// use serato_tags::markers::{EntryType, take_entry_type};
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_entry_type(&[0x00]), Ok((&[][..], EntryType::INVALID)));
/// assert_eq!(take_entry_type(&[0x03, 0x01]), Ok((&[0x01][..], EntryType::LOOP)));
/// assert_eq!(take_entry_type(&[0xAB]), Err(nom::Err::Incomplete(nom::Needed::Unknown)));
/// ```
pub fn take_entry_type(input: &[u8]) -> nom::IResult<&[u8], EntryType> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, EntryType::INVALID)),
        0x01 => Ok((input, EntryType::CUE)),
        0x03 => Ok((input, EntryType::LOOP)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

pub fn marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, start_position_millis) = take_position(input)?;
    let (input, end_position_millis) = take_position(input)?;
    let (input, _) = unknown(input)?;
    let (input, color) = util::serato32::take_color(input)?;
    let (input, entry_type) = take_entry_type(input)?;
    let (input, is_locked) = take_bool(input)?;
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

pub fn parse(input: &[u8]) -> Result<Markers, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::take_version(&input)?;
    let (input, entries) = take_markers(input)?;
    //let (_, track_color) = util::serato32::take_color(input)?;
    let (_, track_color) = nom::combinator::all_consuming(util::serato32::take_color)(input)?;

    Ok(Markers {
        version,
        entries,
        track_color,
    })
}
