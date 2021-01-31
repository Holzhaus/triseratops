//! The `Serato BeatGrid` tag stores the beatgrid markers.

use super::format::enveloped;
use super::format::flac;
use super::format::id3;
use super::format::mp4;
use crate::error::Error;
use crate::util;
use crate::util::Res;

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

impl util::Tag for Beatgrid {
    const NAME: &'static str = "Serato BeatGrid";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_beatgrid)(input)?;
        Ok(autotags)
    }
}

impl id3::ID3Tag for Beatgrid {}
impl enveloped::EnvelopedTag for Beatgrid {}
impl flac::FLACTag for Beatgrid {
    const FLAC_COMMENT: &'static str = "SERATO_BEATGRID";
}
impl mp4::MP4Tag for Beatgrid {
    const MP4_ATOM: &'static str = "----:com.serato.dj:beatgrid";
}

/// Returns a `u32` parsed from the input slice, decremented by 1.
///
/// This is necessary to get the number of *non*-terminal beatgrid markers (in contrast to *all* markers).
fn take_non_terminal_marker_count(input: &[u8]) -> Res<&[u8], u32> {
    let (input, count) =
        nom::combinator::verify(nom::number::complete::be_u32, |x: &u32| x > &0u32)(input)?;
    Ok((input, count - 1))
}

#[test]
fn test_take_non_terminal_marker_count() {
    assert_eq!(
        take_non_terminal_marker_count(&[0x00, 0x00, 0x00, 0x01]),
        Ok((&[][..], 0x00))
    );
    assert_eq!(
        take_non_terminal_marker_count(&[0x89, 0xAB, 0xCD, 0xEF, 0x12]),
        Ok((&[0x12][..], 0x89ABCDEE))
    );
    assert!(take_non_terminal_marker_count(&[0x00, 0x00, 0x00, 0x00]).is_err());
    assert!(take_non_terminal_marker_count(&[0xC0, 0xFF, 0xEE]).is_err());
}

/// Returns a non-terminal beatgrid marker parsed from the input slice.
fn take_non_terminal_marker(input: &[u8]) -> Res<&[u8], NonTerminalMarker> {
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

/// Returns a terminal beatgrid marker parsed from the input slice.
fn take_terminal_marker(input: &[u8]) -> Res<&[u8], TerminalMarker> {
    let (input, position) = nom::number::complete::be_f32(input)?;
    let (input, bpm) = nom::number::complete::be_f32(input)?;
    Ok((input, TerminalMarker { position, bpm }))
}

/// Take a [`Beatgrid` struct] parsed from the input slice.
fn take_beatgrid(input: &[u8]) -> Res<&[u8], Beatgrid> {
    let (input, version) = util::take_version(&input)?;
    let (input, non_terminal_markers) =
        nom::multi::length_count(take_non_terminal_marker_count, take_non_terminal_marker)(input)?;
    let (input, terminal_marker) = take_terminal_marker(input)?;
    let (input, footer) = nom::number::complete::u8(input)?;

    let beatgrid = Beatgrid {
        version,
        non_terminal_markers,
        terminal_marker,
        footer,
    };
    Ok((input, beatgrid))
}
