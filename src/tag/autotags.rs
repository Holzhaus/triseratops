// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! The `Serato Autotags` tag stores BPM and Gain values.

use super::format::Tag;
use super::format::enveloped;
use super::format::flac;
use super::format::id3;
use super::format::mp4;
use super::generic::Version;
use super::util::{take_version, write_version};
use crate::error::Error;
use crate::util::{Res, take_until_nullbyte};
use std::io;

/// Represents the  `Serato AutoTags` tag.
///
/// It stores BPM an gain values.
///
/// # Example
///
/// ```
/// use triseratops::tag::{Autotags, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Autotags::ID3_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = Autotags::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Autotags {
    /// The tag version.
    pub version: Version,
    /// The track's number of beats per minute (BPM).
    pub bpm: f64,
    /// The track's autogain values (probably comparable to ReplayGain).
    pub auto_gain: f64,
    /// The track's gain value (manual?).
    pub gain_db: f64,
}

impl Tag for Autotags {
    const NAME: &'static str = "Serato Autotags";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_autotags)(input)?;
        Ok(autotags)
    }

    fn write(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        write_autotags(writer, self)
    }
}

impl id3::ID3Tag for Autotags {}
impl enveloped::EnvelopedTag for Autotags {}
impl flac::FLACTag for Autotags {
    const FLAC_COMMENT: &'static str = "SERATO_AUTOGAIN";
}
impl mp4::MP4Tag for Autotags {
    const MP4_ATOM_FREEFORM_NAME: &'static str = "autgain";
}

/// Returns an `f64` parsed from zero-terminated ASCII chars the input slice.
fn take_double_str(input: &[u8]) -> Res<&[u8], f64> {
    let (input, text) = take_until_nullbyte(input)?;
    let (_, num) = nom::combinator::all_consuming(nom::number::complete::double)(text)?;
    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, num))
}

#[test]
fn test_take_double_str() {
    assert_eq!(
        take_double_str(&[0x31, 0x31, 0x35, 0x2E, 0x30, 0x30, 0x00]),
        Ok((&[][..], 115.0))
    );
    assert_eq!(
        take_double_str(&[0x2D, 0x33, 0x2E, 0x32, 0x35, 0x37, 0x00, 0xAB]),
        Ok((&[0xAB][..], -3.257))
    );
    assert!(take_double_str(&[0xAB, 0x01]).is_err());
}

/// Returns an [`Autotags` struct] parsed from input slice.
fn take_autotags(input: &[u8]) -> Res<&[u8], Autotags> {
    let (input, version) = take_version(input).unwrap();
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

pub fn write_double_str(
    writer: &mut impl io::Write,
    number: f64,
    width: usize,
) -> Result<usize, Error> {
    let number_str = format!("{number:.width$}\0");
    Ok(writer.write(number_str.as_bytes())?)
}

pub fn write_autotags(writer: &mut impl io::Write, autotags: &Autotags) -> Result<usize, Error> {
    let mut bytes_written = write_version(writer, autotags.version)?;
    bytes_written += write_double_str(writer, autotags.bpm, 2)?;
    bytes_written += write_double_str(writer, autotags.auto_gain, 3)?;
    bytes_written += write_double_str(writer, autotags.gain_db, 3)?;
    Ok(bytes_written)
}
