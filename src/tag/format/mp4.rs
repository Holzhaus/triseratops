// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! MP4 tags

use super::enveloped::EnvelopedTag;
use crate::error::Error;
use std::io;

pub trait MP4Tag: EnvelopedTag {
    /// The mean part of the freeform `MP4_ATOM` that this data is stored in.
    const MP4_ATOM_FREEFORM_MEAN: &'static str = "com.serato.dj";

    /// The mean part of the freeform `MP4_ATOM` that this data is stored in.
    const MP4_ATOM_FREEFORM_NAME: &'static str;

    fn parse_mp4(input: &[u8]) -> Result<Self, Error> {
        Self::parse_enveloped(input)
    }

    fn write_mp4(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        self.write_enveloped(writer)
    }
}
