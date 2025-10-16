// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Ogg tags

use super::Tag;
use crate::error::Error;
use std::io;

pub trait OggTag: Tag {
    /// Name of the `MP4_ATOM` that this data is stored in.
    const OGG_COMMENT: &'static str;

    fn parse_ogg(input: &[u8]) -> Result<Self, Error>;
    fn write_ogg(&self, writer: &mut impl io::Write) -> Result<usize, Error>;
}
