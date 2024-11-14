// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! This library provides access to metadata and libraries written by the [Serato
//! DJ](https://serato.com/dj) software.
//!
//! # Metadata
//!
//! Serato's formats are pretty complex and store redundant data which might lead to situation
//! where that data contradicts each other (e.g. if the data was edited by hand).
//!
//! ## Parsing
//!
//! To make it straightforward to access the metadata, this library provides
//! the [`TagContainer` struct](`tag::TagContainer`), which provides access to all important
//! attributes and implements the same conflict resolution strategies that the Serato DJ software
//! uses.
//!
//! ```
//! use triseratops::tag::{TagContainer, TagFormat};
//!
//! fn parse_and_print_cues(markers_data: &[u8], markers2_data: &[u8]) {
//!     let mut tags = TagContainer::new();
//!     tags.parse_markers(markers_data, TagFormat::ID3).expect("Failed to parse Markers data!");
//!     tags.parse_markers2(markers2_data, TagFormat::ID3).expect("Failed to parse Markers2 data!");
//!
//!     for cue in tags.cues() {
//!         println!("{:?}", cue);
//!     }
//! }
//! ```
//!
//! If you'd rather parse the tag data representation yourself, you can do that by using the
//! individual tag structs directly:
//!
//! ```
//! use triseratops::tag::{Markers, format::id3::ID3Tag};
//!
//! fn parse_and_print_markers(data: &[u8]) {
//!     let markers = Markers::parse_id3(data).expect("Failed to parse data!");
//!     println!("{:?}", markers);
//! }
//! ```
//!
//! **Note:** This library does *not* provide means to read the metadata from music files directly, so you need to
//! use other libraries (e.g. [`id3`](https://lib.rs/crates/id3) to do that. You can check the
//! `examples/` directory for some toy examples.
//!
//! ## Serialization
//!
//! This library aims to provide support for lossless data roundtripping, i.e. parsing and then
//! serializing data results in the exact same bytes as the original input (except for FLAC/MP4
//! data, see last bullet point in the *Caveats* section below).
//!
//! ```
//! use std::io::Write;
//! use triseratops::tag::{Markers, format::id3::ID3Tag};
//!
//! fn write(mut writer: impl Write, markers: &Markers) {
//!     let bytes_written = markers.write_id3(&mut writer).expect("Failed to serialize data!");
//!     println!("Wrote {} bytes", bytes_written);
//! }
//! ```
//!
//! ## Supported File Types
//!
//! Support for the following tags has already been implemented:
//!
//! | Tag          | ID3     | FLAC    | MP4/M4A | Ogg Vorbis | XML (e.g. AAC) | *Description*
//! | ------------ | ------- | ------- | ------- | ---------- | -------------- | ----------
//! | `Analysis`   | **Yes** | **Yes** | **Yes** | **Yes**    | No             | Serato Analysis version
//! | `Autotags`   | **Yes** | **Yes** | **Yes** | No         | No             | BPM and Gain values
//! | `BeatGrid`   | **Yes** | **Yes** | **Yes** | No         | No             | Beatgrid Markers
//! | `Markers_`   | **Yes** | *n/a*   | **Yes** | No         | No             | Hotcues, Saved Loops, etc.
//! | `Markers2`   | **Yes** | **Yes** | **Yes** | **Yes**    | No             | Hotcues, Saved Loops, etc.
//! | `Offsets_`   | No      | No      | No      | No         | No             | ?
//! | `Overview`   | **Yes** | **Yes** | **Yes** | No         | No             | Overview Waveform data
//! | `RelVol`     | *n/a*   | Partial | Partial | *n/a*      | No             | Relative Volume Adjument data (?)
//! | `VideoAssoc` | *n/a*   | Partial | Partial | *n/a*      | No             | Video Association data (?)
//!
//! ## Caveats
//!
//! - Most Ogg tags are currently not supported. Their format is completely different from
//!   the other tag types and need to be reverse-engineered first.
//! - The `Serato Offsets_` tag haven't been reverse engineed yet, and no support has been
//!   implemented.
//! - The `Serato RelVolAd` and the `Serato VidAssoc` tags haven't been reverse engineed yet, but
//!   preliminary support has been added. For now, they just return a tag version and a byte vector.
//! - AAC files (among others) do not store metadata in tags, and use XML files in the
//!   `_Serato_/Metadata` directory instead. No support has been added yet.
//! - The cue colors stored in the metadata are *not* the same as displayed in Serato DJ Pro.
//!   Instead, they uses the color palette from Serato DJ Into. Serato then maps them to a new
//!   color palette. Support for converting between the two is currently missing.
//! - The track colors stores in the metadata are those from the color picker, not those shown in
//!   the library table. Support for converting between the two is currently missing.
//! - Unfortunately, full lossless roundtrips  aren't possible for FLAC and MP4, because the data
//!   is wrapped in base64-encoding where the decoded last byte seems to be random junk that change
//!   when writing tags from Serato DJ even if no actual changes were made (possibly an
//!   out-of-bounds read or uninitialized data in Serato DJ). For parsing and using this in Serato
//!   that doesn't make a difference, but the roundtrip tests will ignore those last two bytes.
//!
//! # Library
//!
//! Parsing the Serato library (e.g. the `database V2` file in the `_Serato_` directory) is also
//! possible, but since this feature is still under development, the API is *not* stable yet and
//! might change in the future.

#![warn(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
// TODO: Add missing docs
//#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::all)]
#![deny(clippy::explicit_deref_methods)]
#![deny(clippy::explicit_into_iter_loop)]
#![deny(clippy::explicit_iter_loop)]
#![deny(clippy::must_use_candidate)]
#![cfg_attr(not(test), deny(clippy::panic_in_result_fn))]
#![cfg_attr(not(debug_assertions), deny(clippy::used_underscore_binding))]

pub mod error;
pub mod library;
pub mod tag;
pub(crate) mod util;
