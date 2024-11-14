// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! The `Serato VidAssoc` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use super::format::{enveloped, flac, mp4, Tag};
use super::generic::Version;
use super::util::{take_version, write_version};
use crate::error::Error;
use crate::util::Res;
use std::io;

/// Represents the  `Serato VidAssoc` tag.
///
/// **Note:** This tag has not been reverse-engineered yet. Judging from the name it contains
/// "Video Association" data.
///
/// # Example
///
/// ```
/// use triseratops::tag::{VidAssoc, format::flac::FLACTag};
///
/// // First, read the tag data from the FLAC VORBIS_COMMENT (the tag name can be accessed using the
/// // VidAssoc::FLAC_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = VidAssoc::parse_flac(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug)]
pub struct VidAssoc {
    /// The `VidAssoc` version.
    pub version: Version,
    /// The data (not reverse-engineered yet)
    pub data: Vec<u8>,
}

impl Tag for VidAssoc {
    const NAME: &'static str = "Serato VidAssoc";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, vidassoc) = nom::combinator::all_consuming(take_vidassoc)(input)?;
        Ok(vidassoc)
    }

    fn write(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        write_vidassoc(writer, self)
    }
}

impl enveloped::EnvelopedTag for VidAssoc {}
impl flac::FLACTag for VidAssoc {
    const FLAC_COMMENT: &'static str = "SERATO_VIDASSOC";
}
impl mp4::MP4Tag for VidAssoc {
    const MP4_ATOM_FREEFORM_NAME: &'static str = "videoassociation";
}

fn take_vidassoc(input: &[u8]) -> Res<&[u8], VidAssoc> {
    let (input, version) = take_version(input)?;
    let (input, data) = nom::combinator::rest(input)?;
    let data = data.to_owned();

    let vidassoc = VidAssoc { version, data };
    Ok((input, vidassoc))
}

fn write_vidassoc(writer: &mut impl io::Write, vidassoc: &VidAssoc) -> Result<usize, Error> {
    let mut bytes_written = write_version(writer, vidassoc.version)?;
    bytes_written += writer.write(vidassoc.data.as_slice())?;
    Ok(bytes_written)
}
