use crate::util;
use crate::error::Error;

pub trait ID3Tag: util::Tag {
    fn parse_id3(input: &[u8]) -> Result<Self, Error> {
        Self::parse(&input)
    }
}
