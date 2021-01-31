//! This library provides access to metadata and libraries written by the [Serato
//! DJ](https://serato.com/dj) software.
//!
//! # Metadata
//!
//! Serato's formats are pretty complex and store redundant data which might lead to situation
//! where that data contradicts each other (e.g. if the data was edited by hand).
//!
//! To make it straightforward to access the metadata, this library provides
//! the [`TagContainer` struct](`tag::TagContainer`), which provides access to all important
//! attributes and implements the same conflict resolution strategies that the Serato DJ software
//! uses.
//!
//! ```
//! use serato_tags::tag::{TagContainer, TagType};
//!
//! fn parse_and_print_cues(markers_data: &[u8], markers2_data: &[u8]) {
//!     let mut tags = TagContainer::new();
//!     tags.parse_markers(markers_data, TagType::ID3).expect("Failed to parse Markers data!");
//!     tags.parse_markers2(markers2_data, TagType::ID3).expect("Failed to parse Markers2 data!");
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
//! use serato_tags::tag::{Markers, format::id3::ID3Tag};
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
//! ## Library
//!
//! Parsing the Serato library (e.g. the `database V2` file in the `_Serato_` directory) is also
//! possible, but since this feature is still under development, the API is *not* stable yet and
//! might change in the future.

pub mod error;
pub mod library;
pub mod tag;
pub mod util;
