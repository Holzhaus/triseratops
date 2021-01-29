//! FLAC helpers
extern crate base64;
extern crate nom;

use super::enveloped::EnvelopedTag;
use crate::error::Error;

pub trait FLACTag: EnvelopedTag {
    /// Name of the `VORBIS_COMMENT` that this data is stored in.
    const FLAC_COMMENT: &'static str;

    fn parse_flac(input: &[u8]) -> Result<Self, Error> {
        Self::parse_enveloped(&input)
    }
}
