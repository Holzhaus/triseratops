// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! ID3 tags

use super::Tag;
use crate::error::Error;
use std::io;

pub trait ID3Tag: Tag {
    /// Name of the ID3 tag that this data is stored in.
    const ID3_TAG: &'static str = Self::NAME;

    fn parse_id3(input: &[u8]) -> Result<Self, Error> {
        Self::parse(input)
    }

    fn write_id3(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        self.write(writer)
    }
}
