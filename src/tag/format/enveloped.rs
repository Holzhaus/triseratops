// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Helper for FLAC and MP4 tags

use super::Tag;
use crate::error::Error;
use crate::util::{take_utf8, Res};
use std::io;
use std::io::Cursor;

pub trait EnvelopedTag: Tag {
    fn parse_enveloped(input: &[u8]) -> Result<Self, Error> {
        let (_, encoded) = nom::combinator::all_consuming(take_base64_with_newline)(input)?;
        let content = envelope_decode_with_name(encoded, Self::NAME)?;
        Self::parse(&content)
    }

    fn write_enveloped(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        let mut buffer = Cursor::new(vec![]);
        self.write(&mut buffer)?;
        let plain_data = &buffer.get_ref()[..];
        envelope_encode_with_name(writer, plain_data, Self::NAME)
    }
}

pub fn parse_envelope(input: &[u8]) -> Result<(&str, &[u8]), Error> {
    let (input, _) = nom::bytes::complete::tag(b"application/octet-stream\x00\x00")(input)?;
    let (input, name) = take_utf8(input)?;
    Ok((name, input))
}

#[must_use]
pub fn is_base64(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'+' || byte == b'/' || byte == b'='
}

#[must_use]
pub fn is_newline(byte: u8) -> bool {
    byte == b'\n'
}

pub fn take_base64_with_newline(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::bytes::complete::take_while(|b| is_base64(b) || is_newline(b))(input)
}

const BASE64_FORGIVING: base64::Config = base64::STANDARD_NO_PAD.decode_allow_trailing_bits(true);

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

pub fn base64_encode(writer: &mut impl io::Write, input: &[u8]) -> Result<usize, Error> {
    let mut bytes_written = 0;
    let chunks = input.chunks(54);
    let last_chunk_index = chunks.len() - 1;
    for (i, chunk) in chunks.enumerate() {
        let mut buf = Vec::new();
        buf.resize(72, 0);
        let bytes_encoded = base64::encode_config_slice(chunk, BASE64_FORGIVING, &mut buf);
        bytes_written += writer.write(&buf[..bytes_encoded])?;
        if i == last_chunk_index {
            if bytes_encoded % 4 != 2 {
                bytes_written += writer.write(b"A")?;
            }
        } else {
            bytes_written += writer.write(&[b'\n'])?;
        }
    }
    println!("{}", bytes_written);
    Ok(bytes_written)
}

pub fn envelope_decode(input: &[u8]) -> Result<(String, Vec<u8>), Error> {
    let data = base64_decode(input)?;
    parse_envelope(&data).map(|(s, b)| (s.to_owned(), b.to_owned()))
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

pub fn envelope_encode_with_name(
    writer: &mut impl io::Write,
    input: &[u8],
    name: &str,
) -> Result<usize, Error> {
    let data = [
        b"application/octet-stream\x00\x00",
        name.as_bytes(),
        b"\0",
        input,
    ]
    .concat();
    base64_encode(writer, &data)
}
