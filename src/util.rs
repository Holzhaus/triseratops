//! Various helper utilities for simplify parsing.
extern crate nom;

use crate::error::Error;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;

/// Represents a 3-Byte RGB color value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Represents 2-Byte version value.
#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub trait Tag: Sized {
    const NAME: &'static str;
    fn parse(input: &[u8]) -> Result<Self, Error>;
}

pub type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

/// Returns a `Color` struct parsed from the first 3 input bytes.
///
/// # Example
/// ```
/// use serato_tags::util::Color;
/// use serato_tags::util::take_color;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_color(&[0xFF, 0x00, 0x10]), Ok((&[][..], Color { red: 0xFF, green: 0x00, blue: 0x10})));
/// assert_eq!(take_color(&[0x11, 0x22, 0x33, 0x44]), Ok((&[0x44][..], Color { red: 0x11, green: 0x22, blue: 0x33})));
/// assert_eq!(take_color(&[0xAB, 0xCD]), Err(Err::Error(Error::new(&[0xAB, 0xCD][..], ErrorKind::Eof))));
/// ```
pub fn take_color(input: &[u8]) -> Res<&[u8], Color> {
    let (input, bytes) = nom::bytes::complete::take(3usize)(input)?;
    let (bytes, red) = nom::number::complete::u8(bytes)?;
    let (bytes, green) = nom::number::complete::u8(bytes)?;
    let (_, blue) = nom::combinator::all_consuming(nom::number::complete::u8)(bytes)?;
    Ok((input, Color { red, green, blue }))
}

/// Returns a `Version` struct parsed from the first 2 input bytes.
///
/// # Example
/// ```
/// use serato_tags::util::Version;
/// use serato_tags::util::take_version;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_version(&[0x02, 0x05]), Ok((&[][..], Version { major: 2, minor: 5 })));
/// assert_eq!(take_version(&[0x01, 0x02, 0x03]), Ok((&[0x03][..], Version { major: 1, minor: 2 })));
/// assert_eq!(take_version(&[0x0A]), Err(Err::Error(Error::new(&[0x0A][..], ErrorKind::Eof))));
/// ```
pub fn take_version(input: &[u8]) -> Res<&[u8], Version> {
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
///
/// # Example
/// ```
/// use serato_tags::util::take_until_nullbyte;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_until_nullbyte(&[0x41, 0x42, 0x00]), Ok((&[0x00][..], &[0x41, 0x42][..])));
/// assert_eq!(take_until_nullbyte(&[0x01, 0x02, 0x00, 0xFF]), Ok((&[0x00, 0xFF][..], &[0x01, 0x02][..])));
/// assert_eq!(take_until_nullbyte(&[0xAB, 0xCD]), Err(Err::Error(Error::new(&[0xAB, 0xCD][..], ErrorKind::TakeUntil))));
/// ```
pub fn take_until_nullbyte(input: &[u8]) -> Res<&[u8], &[u8]> {
    take_until(NULL)(input)
}

/// These functions convert between a custom 4-byte format (that we'll call `serato32` for brevity)
/// and 3-byte plaintext (both `u32`). Serato's custom format inserts a single null bit after every 7
/// payload bits, starting from the rightmost bit.
///
/// This format is used to encode the 3-byte RGB color values (track color, cue colors) and the cue
/// positions and the `Serato Markers_` tag.
///
/// # Binary Format Details
///
/// ```text
/// serato32     |     Byte1     |     Byte2     |     Byte3     |     Byte4     |
///              | Nibb1 | Nibb2 | Nibb3 | Nibb4 | Nibb5 | Nibb6 | Nibb7 | Nibb8 |
/// Bits         |A A A A B B B B C C C C D D D D E E E E F F F F G G G G H H H H|
/// Ignored Bits |^ ^ ^ ^ ^       ^               ^               ^              |
/// Plaintext    |||||||||||     Byte1       |      Byte2      |      Byte3      |
///              |||||||||||  Nibb1  | Nibb2 |  Nibb3  | Nibb4 |  Nibb5  | Nibb6 |
/// ```
///
/// More information can be found in the [format
/// documentation](https://github.com/Holzhaus/serato-tags/blob/master/docs/serato_markers_.md#custom-serato32-binary-format).
///
/// ## Example
///
/// |                  | Hex           | Binary
/// | ---------------- | ------------- | ----------------------------------
/// | 3-byte plaintext | `   00 00 cc` | `     000 0000000 0000001 1001100`
/// | `serato32` value | `00 00 01 4c` | `00000000000000000000000101001100`
/// |                  |
/// | 3-byte plaintext | `   cc 88 00` | `     110 0110010 0010000 0000000`
/// | `serato32` value | `06 32 10 00` | `00000110001100100001000000000000`

pub mod serato32 {
    use super::{Color, Res};
    use nom::number::complete::u8;

    /// Decodes value from Serato's 32-bit custom format to 24-bit plaintext.
    ///
    /// # Example
    /// ```rust
    /// use serato_tags::util::serato32::{decode, encode};
    ///
    /// assert_eq!(decode(0x00, 0x00, 0x01, 0x4C), (0x00, 0x00, 0xCC));
    ///
    /// let (a, b, c, d) = encode(0x00, 0x00, 0xCC);
    /// assert_eq!(decode(a, b, c, d), (0x00, 0x00, 0xCC));
    /// ```
    pub fn decode(enc1: u8, enc2: u8, enc3: u8, enc4: u8) -> (u8, u8, u8) {
        let dec3: u8 = (enc4 & 0x7F) | ((enc3 & 0x01) << 7);
        let dec2: u8 = ((enc3 & 0x7F) >> 1) | ((enc2 & 0x03) << 6);
        let dec1: u8 = ((enc2 & 0x7F) >> 2) | ((enc1 & 0x07) << 5);
        (dec1, dec2, dec3)
    }

    /// Encodes 3-byte value to to Serato's 32-bit custom format.
    ///
    /// # Example
    /// ```rust
    /// use serato_tags::util::serato32::{decode, encode};
    ///
    /// assert_eq!(encode(0x00, 0x00, 0xCC), (0x00, 0x00, 0x01, 0x4C));
    ///
    /// let (x, y, z) = decode(0x00, 0x00, 0x01, 0x4C);
    /// assert_eq!(encode(x, y, z), (0x00, 0x00, 0x01, 0x4C));
    /// ```
    pub fn encode(dec1: u8, dec2: u8, dec3: u8) -> (u8, u8, u8, u8) {
        let enc4: u8 = dec3 & 0x7F;
        let enc3: u8 = ((dec3 >> 7) | (dec2 << 1)) & 0x7F;
        let enc2: u8 = ((dec2 >> 6) | (dec1 << 2)) & 0x7F;
        let enc1: u8 = dec1 >> 5;
        (enc1, enc2, enc3, enc4)
    }

    /// Returns a 3-byte tuple decoded from the first 4 input bytes.
    ///
    /// # Example
    /// ```
    /// use serato_tags::util::serato32::take;
    /// use nom::Err;
    /// use nom::error::{Error, ErrorKind};
    ///
    /// assert_eq!(take(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], (0x00, 0x00, 0xCC))));
    /// assert_eq!(take(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], (0x00, 0x00, 0xCC))));
    /// assert_eq!(take(&[0x00, 0x00, 0x01]), Err(Err::Error(Error::new(&[0x00, 0x00, 0x01][..], ErrorKind::Eof))));
    /// ```
    pub fn take(input: &[u8]) -> Res<&[u8], (u8, u8, u8)> {
        let (input, bytes) = nom::bytes::complete::take(4usize)(input)?;
        let (bytes, byte1) = u8(bytes)?;
        let (bytes, byte2) = u8(bytes)?;
        let (bytes, byte3) = u8(bytes)?;
        let (_, byte4) = nom::combinator::all_consuming(u8)(bytes)?;
        let value = decode(byte1, byte2, byte3, byte4);
        Ok((input, value))
    }

    /// Returns a `Color` decoded from the first 4 input bytes.
    ///
    /// # Example
    /// ```
    /// use serato_tags::util::Color;
    /// use serato_tags::util::serato32::take_color;
    /// use nom::Err;
    /// use nom::error::{Error, ErrorKind};
    ///
    /// assert_eq!(take_color(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], Color { red: 0x00, green: 0x00, blue: 0xCC})));
    /// assert_eq!(take_color(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], Color { red: 0x00, green: 0x00, blue: 0xCC})));
    /// assert_eq!(take_color(&[0x00, 0x00, 0x01]), Err(Err::Error(Error::new(&[0x00, 0x00, 0x01][..], ErrorKind::Eof))));
    /// ```
    pub fn take_color(input: &[u8]) -> Res<&[u8], Color> {
        let (input, (red, green, blue)) = take(input)?;
        Ok((input, Color { red, green, blue }))
    }

    /// Returns a `u32` decoded from the first 4 input bytes.
    ///
    /// The first 8 bits are always 0.
    ///
    /// # Example
    /// ```
    /// use serato_tags::util::serato32::take_u32;
    /// use nom::Err;
    /// use nom::error::{Error, ErrorKind};
    ///
    /// assert_eq!(take_u32(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], 0x0000CC)));
    /// assert_eq!(take_u32(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], 0x0000CC)));
    /// assert_eq!(take_u32(&[0x00, 0x00, 0x01]), Err(Err::Error(Error::new(&[0x00, 0x00, 0x01][..], ErrorKind::Eof))));
    /// ```
    pub fn take_u32(input: &[u8]) -> Res<&[u8], u32> {
        let (input, (a, b, c)) = take(input)?;
        let value = (a as u32) << 16 | (b as u32) << 8 | c as u32;
        Ok((input, value))
    }
}
