//! Ogg tags

use crate::error::Error;
use crate::util;

pub trait OggTag: util::Tag {
    /// Name of the `MP4_ATOM` that this data is stored in.
    const OGG_COMMENT: &'static str;

    fn parse_ogg(input: &[u8]) -> Result<Self, Error>;
}
