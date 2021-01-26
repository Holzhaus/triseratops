//! The `Serato BeatGrid` tag stores the beatgrid markers.

use crate::util;
use nom::length_count;
use nom::named;

/// Represents the terminal beatgrid marker in the `Serato BeatGrid` tag.
///
/// The last beatgrid marker always has to be a terminal one. This is also the case if the tag only
/// contains a single beatgrid marker.
#[derive(Debug)]
pub struct TerminalMarker {
    /// The position in seconds.
    pub position: f32,
    /// The track's beats per minute (BPM).
    pub bpm: f32,
}

/// Represents a non-terminal beatgrid marker in the `Serato BeatGrid` tag.
///
/// All beatgrid markers before the last one are non-terminal beatgrid markers.

#[derive(Debug)]
pub struct NonTerminalMarker {
    /// The position in seconds.
    pub position: f32,
    /// The number of beats between this marker and the next one (inclusive).
    pub beats_till_next_marker: u32,
}

/// Represents the `Serato BeatGrid` tag.
#[derive(Debug)]
pub struct Beatgrid {
    /// The analysis version.
    pub version: util::Version,
    /// Zero or more non-terminal beatgrid markers.
    pub non_terminal_markers: Vec<NonTerminalMarker>,
    /// The terminal beatgrid marker.
    pub terminal_marker: TerminalMarker,
    /// A single footer byte that is apparently random (?).
    pub footer: u8,
}

fn non_terminal_marker_count(input: &[u8]) -> nom::IResult<&[u8], u32> {
    let (input, count) = nom::number::complete::be_u32(input)?;
    Ok((input, count - 1))
}

named!(
    take_non_terminal_markers<Vec<NonTerminalMarker>>,
    length_count!(non_terminal_marker_count, non_terminal_marker)
);

pub fn non_terminal_marker(input: &[u8]) -> nom::IResult<&[u8], NonTerminalMarker> {
    let (input, position) = nom::number::complete::be_f32(input)?;
    let (input, beats_till_next_marker) = nom::number::complete::be_u32(input)?;
    Ok((
        input,
        NonTerminalMarker {
            position,
            beats_till_next_marker,
        },
    ))
}

pub fn terminal_marker(input: &[u8]) -> nom::IResult<&[u8], TerminalMarker> {
    let (input, position) = nom::number::complete::be_f32(input)?;
    let (input, bpm) = nom::number::complete::be_f32(input)?;
    Ok((input, TerminalMarker { position, bpm }))
}

pub fn parse(input: &[u8]) -> Result<Beatgrid, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::take_version(&input)?;
    let (input, non_terminal_markers) = take_non_terminal_markers(input)?;
    let (input, terminal_marker) = terminal_marker(input)?;
    let (_, footer) = nom::combinator::all_consuming(nom::number::complete::u8)(input)?;

    Ok(Beatgrid {
        version,
        non_terminal_markers,
        terminal_marker,
        footer,
    })
}
