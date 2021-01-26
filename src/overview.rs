//! The `Serato Overview` tag stores the waveform overview data.
//!
//! The overview data consists of multiple chunks of 16 bytes.

use crate::util;

/// Represents the `Serato Overview` tag.
#[derive(Debug)]
pub struct Overview {
    /// The tag version.
    pub version: util::Version,
    /// The Waveform overview data.
    pub data: Vec<Vec<u8>>,
}

pub fn take_chunk(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    let (input, chunkdata) = nom::bytes::complete::take(16usize)(input)?;
    Ok((input, chunkdata.to_vec()))
}

pub fn take_data(input: &[u8]) -> nom::IResult<&[u8], Vec<Vec<u8>>> {
    nom::multi::many1(take_chunk)(input)
}

pub fn parse(input: &[u8]) -> Result<Overview, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::take_version(&input)?;
    let (_, data) = nom::combinator::all_consuming(take_data)(input)?;

    Ok(Overview { version, data })
}
