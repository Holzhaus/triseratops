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
