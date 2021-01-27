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

/// Returns a 16-byte vector of data parsed from the input slice.
///
/// # Example
/// ```
/// use serato_tags::overview::take_chunk;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_chunk(&[
///     0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]), Ok((&[0x10u8][..], [
///     0x00u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f].to_vec())));
/// assert_eq!(take_chunk(&[0xAB, 0x01]), Err(nom::Err::Error(Error::new(&[0xAB, 0x01][..], ErrorKind::Eof))));
/// ```
pub fn take_chunk(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    let (input, chunkdata) = nom::bytes::complete::take(16usize)(input)?;
    Ok((input, chunkdata.to_vec()))
}

/// Returns a vector of 16-byte vectors of data parsed from the input slice.
pub fn take_chunks(input: &[u8]) -> nom::IResult<&[u8], Vec<Vec<u8>>> {
    nom::multi::many1(take_chunk)(input)
}

pub fn parse(input: &[u8]) -> Result<Overview, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::take_version(&input)?;
    let (_, data) = nom::combinator::all_consuming(take_chunks)(input)?;

    Ok(Overview { version, data })
}
