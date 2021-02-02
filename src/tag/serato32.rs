//! These functions convert between a custom 4-byte format (that we'll call `serato32` for brevity)
//! and 3-byte plaintext (both `u32`). Serato's custom format inserts a single null bit after every 7
//! payload bits, starting from the rightmost bit.
//!
//! This format is used to encode the 3-byte RGB color values (track color, cue colors) and the cue
//! positions and the `Serato Markers_` tag.
//!
//! # Binary Format Details
//!
//! ```text
//! serato32     |     Byte1     |     Byte2     |     Byte3     |     Byte4     |
//!              | Nibb1 | Nibb2 | Nibb3 | Nibb4 | Nibb5 | Nibb6 | Nibb7 | Nibb8 |
//! Bits         |A A A A B B B B C C C C D D D D E E E E F F F F G G G G H H H H|
//! Ignored Bits |^ ^ ^ ^ ^       ^               ^               ^              |
//! Plaintext    |||||||||||     Byte1       |      Byte2      |      Byte3      |
//!              |||||||||||  Nibb1  | Nibb2 |  Nibb3  | Nibb4 |  Nibb5  | Nibb6 |
//! ```
//!
//! More information can be found in the [format
//! documentation](https://github.com/Holzhaus/serato-tags/blob/master/docs/serato_markers_.md#custom-serato32-binary-format).
//!
//! ## Example
//!
//! |                  | Hex           | Binary
//! | ---------------- | ------------- | ----------------------------------
//! | 3-byte plaintext | `   00 00 cc` | `     000 0000000 0000001 1001100`
//! | `serato32` value | `00 00 01 4c` | `00000000000000000000000101001100`
//! |                  |
//! | 3-byte plaintext | `   cc 88 00` | `     110 0110010 0010000 0000000`
//! | `serato32` value | `06 32 10 00` | `00000110001100100001000000000000`

use super::generic::Color;
use crate::util::Res;
use nom::number::complete::u8;

/// Decodes value from Serato's 32-bit custom format to 24-bit plaintext.
///
/// # Example
/// ```rust
/// use triseratops::tag::serato32::{decode, encode};
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
/// use triseratops::tag::serato32::{decode, encode};
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
/// use triseratops::tag::serato32::take;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], (0x00, 0x00, 0xCC))));
/// assert_eq!(take(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], (0x00, 0x00, 0xCC))));
/// assert!(take(&[0x00, 0x00, 0x01]).is_err());
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
/// use triseratops::tag::generic::Color;
/// use triseratops::tag::serato32::take_color;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_color(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], Color { red: 0x00, green: 0x00, blue: 0xCC})));
/// assert_eq!(take_color(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], Color { red: 0x00, green: 0x00, blue: 0xCC})));
/// assert!(take_color(&[0x00, 0x00, 0x01]).is_err());
/// ```
pub fn take_color(input: &[u8]) -> Res<&[u8], Color> {
    let (input, (red, green, blue)) = take(input)?;
    let color = Color { red, green, blue };
    Ok((input, color))
}

/// Returns a `u32` decoded from the first 4 input bytes.
///
/// The first 8 bits are always 0.
///
/// # Example
/// ```
/// use triseratops::tag::serato32::take_u32;
/// use nom::Err;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(take_u32(&[0x00, 0x00, 0x01, 0x4C]), Ok((&[][..], 0x0000CC)));
/// assert_eq!(take_u32(&[0x00, 0x00, 0x01, 0x4C, 0x7F]), Ok((&[0x07F][..], 0x0000CC)));
/// assert!(take_u32(&[0x00, 0x00, 0x01]).is_err());
/// ```
pub fn take_u32(input: &[u8]) -> Res<&[u8], u32> {
    let (input, (a, b, c)) = take(input)?;
    let value = (a as u32) << 16 | (b as u32) << 8 | c as u32;
    Ok((input, value))
}
