//! ID3 tags

use super::Tag;
use crate::error::Error;

pub trait ID3Tag: Tag {
    /// Name of the ID3 tag that this data is stored in.
    const ID3_TAG: &'static str = Self::NAME;

    fn parse_id3(input: &[u8]) -> Result<Self, Error> {
        Self::parse(&input)
    }
}
