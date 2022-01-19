//! Various helper utilities for simplify parsing.

use super::color::Color;
use super::generic::Version;
use crate::error::Error;
use crate::util::Res;
use nom::bytes::complete::take;
use std::io;

/// Returns a `Color` struct parsed from the first 3 input bytes.
pub fn take_color(input: &[u8]) -> Res<&[u8], Color> {
    let (input, bytes) = nom::bytes::complete::take(3usize)(input)?;
    let (bytes, red) = nom::number::complete::u8(bytes)?;
    let (bytes, green) = nom::number::complete::u8(bytes)?;
    let (_, blue) = nom::combinator::all_consuming(nom::number::complete::u8)(bytes)?;
    Ok((input, Color { red, green, blue }))
}

#[test]
fn test_take_color() {
    assert_eq!(
        take_color(&[0xFF, 0x00, 0x10]),
        Ok((
            &[][..],
            Color {
                red: 0xFF,
                green: 0x00,
                blue: 0x10
            }
        ))
    );
    assert_eq!(
        take_color(&[0x11, 0x22, 0x33, 0x44]),
        Ok((
            &[0x44][..],
            Color {
                red: 0x11,
                green: 0x22,
                blue: 0x33
            }
        ))
    );
    assert!(take_color(&[0xAB, 0xCD]).is_err());
}

pub fn write_color(writer: &mut impl io::Write, color: Color) -> Result<usize, Error> {
    let Color { blue, green, red } = color;
    Ok(writer.write(&[red, green, blue])?)
}

/// Returns a `Version` struct parsed from the first 2 input bytes.
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

#[test]
fn test_take_version() {
    assert_eq!(
        take_version(&[0x02, 0x05]),
        Ok((&[][..], Version { major: 2, minor: 5 }))
    );
    assert_eq!(
        take_version(&[0x01, 0x02, 0x03]),
        Ok((&[0x03][..], Version { major: 1, minor: 2 }))
    );
    assert!(take_version(&[0x0A]).is_err());
}

pub fn write_version(writer: &mut impl io::Write, version: Version) -> Result<usize, Error> {
    let Version { major, minor } = version;
    Ok(writer.write(&[major, minor])?)
}
