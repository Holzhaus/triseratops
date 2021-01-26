extern crate nom;

use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::IResult;

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub fn take_version(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, version) = take(2usize)(input)?;
    Ok((
        input,
        Version {
            major: version[0],
            minor: version[1],
        },
    ))
}

const NULL: &[u8] = &[0x00];

pub fn take_until_nullbyte(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(NULL)(input)
}

/// These functions convert between a custom 4-byte format (that we'll call
/// "serato32" for brevity) and 3-byte plaintext (both quint32).
/// Serato's custom format inserts a single null bit after every 7 payload
/// bits, starting from the rightmost bit.
///
/// Here's an example:
///
/// |                  | Hex           | Binary
/// | ---------------- | ------------- | ----------------------------------
/// | 3-byte plaintext | `   00 00 cc` | `     000 0000000 0000001 1001100`
/// | serato32 value   | `00 00 01 4c` | `00000000000000000000000101001100`
/// |                  |
/// | 3-byte plaintext | `   cc 88 00` | `     110 0110010 0010000 0000000`
/// | serato32 value   | `06 32 10 00` | `00000110001100100001000000000000`
///
/// More information can be found in the [format documentation](https://github.com/Holzhaus/serato-tags/blob/master/docs/serato_markers_.md#custom-serato32-binary-format).
///
/// Decode value from Serato's 32-bit custom format to 24-bit plaintext.
pub fn serato32_decode(input: &[u8]) -> IResult<&[u8], u32> {
    let c: u8 = (input[3] & 0x7F) | ((input[2] & 0x01) << 7);
    let b: u8 = ((input[2] & 0x7F) >> 1) | ((input[1] & 0x03) << 6);
    let a: u8 = ((input[1] & 0x7F) >> 2) | ((input[0] & 0x07) << 5);
    let number: u32 = ((a as u32) << 16) | ((b as u32) << 8) | (c as u32);
    Ok((&input[4..], number))
}
