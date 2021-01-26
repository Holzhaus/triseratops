extern crate nom;

use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::IResult;

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

/// Returns a `Version` struct parsed from the first 2 input bytes.
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

/// Returns the input slice until the first occurrence of a null byte.
pub fn take_until_nullbyte(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(NULL)(input)
}

/// These functions convert between a custom 4-byte format (that we'll call
/// "serato32" for brevity) and 3-byte plaintext (both quint32).
/// Serato's custom format inserts a single null bit after every 7 payload
/// bits, starting from the rightmost bit.
///
/// More information can be found in the [format documentation](https://github.com/Holzhaus/serato-tags/blob/master/docs/serato_markers_.md#custom-serato32-binary-format).
///
/// # Example
///
/// |                  | Hex           | Binary
/// | ---------------- | ------------- | ----------------------------------
/// | 3-byte plaintext | `   00 00 cc` | `     000 0000000 0000001 1001100`
/// | serato32 value   | `00 00 01 4c` | `00000000000000000000000101001100`
/// |                  |
/// | 3-byte plaintext | `   cc 88 00` | `     110 0110010 0010000 0000000`
/// | serato32 value   | `06 32 10 00` | `00000110001100100001000000000000`
pub mod serato32 {
    use super::Color;
    use nom::number::complete::u8;
    use nom::IResult;

    /// Decodes value from Serato's 32-bit custom format to 24-bit plaintext.
    ///
    /// # Example
    /// ```rust
    /// use serato_tags::util::serato32::decode;
    ///
    /// assert_eq!(decode(0x00, 0x00, 0x01, 0x4c), (0x00, 0x00, 0xCC));
    /// ```
    pub fn decode(enc1: u8, enc2: u8, enc3: u8, enc4: u8) -> (u8, u8, u8) {
        let dec3: u8 = (enc4 & 0x7F) | ((enc3 & 0x01) << 7);
        let dec2: u8 = ((enc3 & 0x7F) >> 1) | ((enc2 & 0x03) << 6);
        let dec1: u8 = ((enc2 & 0x7F) >> 2) | ((enc1 & 0x07) << 5);
        (dec1, dec2, dec3)
    }

    /// Returns a 3-byte tuple decoded from the first 4 input bytes.
    pub fn take(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
        let (input, byte1) = u8(input)?;
        let (input, byte2) = u8(input)?;
        let (input, byte3) = u8(input)?;
        let (input, byte4) = u8(input)?;
        let value = decode(byte1, byte2, byte3, byte4);
        Ok((input, value))
    }

    /// Returns an `Color` decoded from the first 4 input bytes.
    pub fn take_color(input: &[u8]) -> nom::IResult<&[u8], Color> {
        let (input, (red, green, blue)) = take(input)?;
        Ok((input, Color { red, green, blue }))
    }

    /// Returns an `u32` decoded from the first 4 input bytes.
    ///
    /// The first 8 bits are always 0.
    pub fn take_u32(input: &[u8]) -> nom::IResult<&[u8], u32> {
        let (input, (a, b, c)) = take(input)?;
        let value = (a as u32) << 16 | (b as u32) << 8 | c as u32;
        Ok((input, value))
    }
}
