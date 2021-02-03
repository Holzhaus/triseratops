//! FLAC tags
extern crate base64;
extern crate nom;

use super::enveloped::EnvelopedTag;
use crate::error::Error;
use std::io;

pub trait FLACTag: EnvelopedTag {
    /// Name of the `VORBIS_COMMENT` that this data is stored in.
    const FLAC_COMMENT: &'static str;

    fn parse_flac(input: &[u8]) -> Result<Self, Error> {
        Self::parse_enveloped(&input)
    }

    fn write_flac(&self, writer: impl io::Write) -> Result<usize, Error> {
        self.write_enveloped(writer)
    }
}
