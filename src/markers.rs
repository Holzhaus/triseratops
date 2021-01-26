use crate::util;
use nom::length_count;
use nom::named;
use nom::number::complete::be_u32;
use nom::tag;

#[derive(Debug)]
pub struct Marker {
    pub start_position_millis: Option<u32>,
    pub end_position_millis: Option<u32>,
    pub color: util::Color,
    pub entry_type: EntryType,
    pub locked: bool,
}

#[derive(Debug)]
pub struct Markers {
    pub version: util::Version,
    pub entries: Vec<Marker>,
    pub track_color: util::Color,
}

#[derive(Debug)]
pub enum EntryType {
    INVALID,
    CUE,
    LOOP,
}

named!(no_position, tag!(b"\x7f\x7f\x7f\x7f"));
named!(unknown, tag!(b"\x00\x7f\x7f\x7f\x7f\x7f"));
named!(take_markers<Vec<Marker>>, length_count!(be_u32, marker));

pub fn take_bool(input: &[u8]) -> nom::IResult<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, false)),
        0x01 => Ok((input, true)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

pub fn has_position(input: &[u8]) -> nom::IResult<&[u8], bool> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, true)),
        0x7f => Ok((input, false)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

pub fn position(input: &[u8]) -> nom::IResult<&[u8], Option<u32>> {
    let (input, has_position) = has_position(input)?;

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

pub fn entry_type(input: &[u8]) -> nom::IResult<&[u8], EntryType> {
    let (input, position_prefix) = nom::number::complete::u8(input)?;
    match position_prefix {
        0x00 => Ok((input, EntryType::INVALID)),
        0x01 => Ok((input, EntryType::CUE)),
        0x03 => Ok((input, EntryType::LOOP)),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

pub fn marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, start_position_millis) = position(input)?;
    let (input, end_position_millis) = position(input)?;
    let (input, _) = unknown(input)?;
    let (input, color) = util::serato32::take_color(input)?;
    let (input, entry_type) = entry_type(input)?;
    let (input, locked) = take_bool(input)?;
    Ok((
        input,
        Marker {
            start_position_millis,
            end_position_millis,
            color,
            entry_type,
            locked,
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
