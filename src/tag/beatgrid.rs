// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! The `Serato BeatGrid` tag stores the beatgrid markers.

use super::format::{enveloped, flac, id3, mp4, Tag};
use super::generic::Version;
use super::util::{take_version, write_version};
use crate::error::Error;
use crate::util::Res;
use std::io;

/// Represents the terminal beatgrid marker in the `Serato BeatGrid` tag.
///
/// The last beatgrid marker always has to be a terminal one. This is also the case if the tag only
/// contains a single beatgrid marker.
#[derive(Debug, Clone)]
pub struct TerminalMarker {
    /// The position in seconds.
    pub position: f32,
    /// The track's beats per minute (BPM).
    pub bpm: f32,
}

/// Represents a non-terminal beatgrid marker in the `Serato BeatGrid` tag.
///
/// All beatgrid markers before the last one are non-terminal beatgrid markers.
#[derive(Debug, Clone)]
pub struct NonTerminalMarker {
    /// The position in seconds.
    pub position: f32,
    /// The number of beats between this marker and the next one (inclusive).
    pub beats_till_next_marker: u32,
}

/// Represents the `Serato BeatGrid` tag.
///
/// It stores the Beatgrid as a sequence of zero or more [non-terminal beatgrid markers](NonTerminalMarker) and
/// a single [terminal beatgrid markers](TerminalMarker).
///
/// # Example
///
/// ```
/// use triseratops::tag::{Beatgrid, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Beatgrid::ID3_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = Beatgrid::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Beatgrid {
    /// The analysis version.
    pub version: Version,
    /// Zero or more non-terminal beatgrid markers.
    pub non_terminal_markers: Vec<NonTerminalMarker>,
    /// The terminal beatgrid marker.
    pub terminal_marker: TerminalMarker,
    /// A single footer byte that is apparently random (?).
    pub footer: u8,
}

impl Tag for Beatgrid {
    const NAME: &'static str = "Serato BeatGrid";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_beatgrid)(input)?;
        Ok(autotags)
    }

    fn write(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        write_beatgrid(writer, self)
    }
}

impl id3::ID3Tag for Beatgrid {}
impl enveloped::EnvelopedTag for Beatgrid {}
impl flac::FLACTag for Beatgrid {
    const FLAC_COMMENT: &'static str = "SERATO_BEATGRID";
}
impl mp4::MP4Tag for Beatgrid {
    const MP4_ATOM_FREEFORM_NAME: &'static str = "beatgrid";
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
    let (input, version) = take_version(input)?;
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

pub fn write_non_terminal_marker(
    writer: &mut impl io::Write,
    marker: &NonTerminalMarker,
) -> Result<usize, Error> {
    let mut bytes_written = writer.write(&marker.position.to_be_bytes())?;
    bytes_written += writer.write(&marker.beats_till_next_marker.to_be_bytes())?;
    Ok(bytes_written)
}

pub fn write_terminal_marker(
    writer: &mut impl io::Write,
    marker: &TerminalMarker,
) -> Result<usize, Error> {
    let mut bytes_written = writer.write(&marker.position.to_be_bytes())?;
    bytes_written += writer.write(&marker.bpm.to_be_bytes())?;
    Ok(bytes_written)
}

pub fn write_beatgrid(writer: &mut impl io::Write, beatgrid: &Beatgrid) -> Result<usize, Error> {
    let mut bytes_written = write_version(writer, beatgrid.version)?;
    let num_markers = beatgrid.non_terminal_markers.len() as u32 + 1;
    bytes_written += writer.write(&num_markers.to_be_bytes())?;
    for marker in &beatgrid.non_terminal_markers {
        bytes_written += write_non_terminal_marker(writer, marker)?;
    }
    bytes_written += write_terminal_marker(writer, &beatgrid.terminal_marker)?;
    bytes_written += writer.write(&[beatgrid.footer])?;
    Ok(bytes_written)
}
