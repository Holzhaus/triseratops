//! FLAC helpers
extern crate base64;
extern crate nom;

use crate::error::Error;
use crate::util;

pub trait FLACTag: util::Tag {
    /// Name of the `VORBIS_COMMENT` that this data is stored in.
    const FLAC_COMMENT: &'static str;

    fn parse_flac(input: &[u8]) -> Result<Self, Error> {
        let (_, encoded) = nom::combinator::all_consuming(take_base64_with_newline)(input)?;
        let content = envelope_decode_with_name(encoded, Self::NAME)?;
        Self::parse(&content)
    }
}

pub fn parse_envelope(input: &[u8]) -> Result<(String, Vec<u8>), Error> {
    let (input, _) = nom::bytes::complete::tag(b"application/octet-stream\x00\x00")(input)?;
    let (input, name) = util::take_utf8(input)?;
    Ok((name, input.to_vec()))
}

pub fn is_base64(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'+' || byte == b'/' || byte == b'='
}

pub fn is_newline(byte: u8) -> bool {
    byte == b'\n'
}

pub fn take_base64_with_newline(input: &[u8]) -> util::Res<&[u8], &[u8]> {
    nom::bytes::complete::take_while(|b| is_base64(b) || is_newline(b))(input)
}

const BASE64_FORGIVING: base64::Config = base64::STANDARD.decode_allow_trailing_bits(true);

pub fn base64_decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut encoded: Vec<u8> = input.iter().filter(|&b| !is_newline(*b)).copied().collect();
    if encoded.len() % 4 != 2 {
        encoded.pop();
    }
    let decoded = base64::decode_config(encoded, BASE64_FORGIVING);
    match decoded {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::Base64DecodeError { source: e }),
    }
}

pub fn envelope_decode(input: &[u8]) -> Result<(String, Vec<u8>), Error> {
    let data = base64_decode(input)?;
    parse_envelope(data.as_slice())
}

pub fn envelope_decode_with_name(input: &[u8], expected_name: &str) -> Result<Vec<u8>, Error> {
    let (name, content) = envelope_decode(input)?;
    if expected_name != name {
        return Err(Error::EnvelopeNameMismatch {
            actual: name,
            expected: expected_name.to_owned(),
        });
    }
    Ok(content)
}
