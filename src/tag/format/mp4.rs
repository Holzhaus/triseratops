//! MP4 tags

extern crate base64;
extern crate nom;

use super::enveloped::EnvelopedTag;
use crate::error::Error;
use std::io;

pub trait MP4Tag: EnvelopedTag {
    /// The mean part of the freeform `MP4_ATOM` that this data is stored in.
    const MP4_ATOM_FREEFORM_MEAN: &'static str = "com.serato.dj";

    /// The mean part of the freeform `MP4_ATOM` that this data is stored in.
    const MP4_ATOM_FREEFORM_NAME: &'static str;

    fn parse_mp4(input: &[u8]) -> Result<Self, Error> {
        Self::parse_enveloped(input)
    }

    fn write_mp4(&self, writer: impl io::Write) -> Result<usize, Error> {
        self.write_enveloped(writer)
    }
}
