//! Ogg tags

use super::Tag;
use crate::error::Error;

pub trait OggTag: Tag {
    /// Name of the `MP4_ATOM` that this data is stored in.
    const OGG_COMMENT: &'static str;

    fn parse_ogg(input: &[u8]) -> Result<Self, Error>;
}
