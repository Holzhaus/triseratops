// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Traits for the different tag types ([ID3](id3::ID3Tag), [FLAC](flac::FLACTag), etc.)

pub mod enveloped;
pub mod flac;
pub mod id3;
pub mod mp4;
pub mod ogg;

use crate::error::Error;
use std::io;

pub trait Tag: Sized {
    const NAME: &'static str;
    fn parse(input: &[u8]) -> Result<Self, Error>;
    fn write(&self, writer: &mut impl io::Write) -> Result<usize, Error>;
}
