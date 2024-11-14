// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! FLAC tags

use super::enveloped::EnvelopedTag;
use crate::error::Error;
use std::io;

pub trait FLACTag: EnvelopedTag {
    /// Name of the `VORBIS_COMMENT` that this data is stored in.
    const FLAC_COMMENT: &'static str;

    fn parse_flac(input: &[u8]) -> Result<Self, Error> {
        Self::parse_enveloped(input)
    }

    fn write_flac(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        self.write_enveloped(writer)
    }
}
