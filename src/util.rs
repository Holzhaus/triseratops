//! Various helper utilities for simplify parsing.
extern crate nom;

use nom::bytes::complete::take_until;

pub type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

const NULL: &[u8] = &[0x00];

/// Returns the input slice until the first occurrence of a null byte.
pub fn take_until_nullbyte(input: &[u8]) -> Res<&[u8], &[u8]> {
    take_until(NULL)(input)
}

#[test]
fn test_take_until_nullbyte() {
    assert_eq!(
        take_until_nullbyte(&[0x41, 0x42, 0x00]),
        Ok((&[0x00][..], &[0x41, 0x42][..]))
    );
    assert_eq!(
        take_until_nullbyte(&[0x01, 0x02, 0x00, 0xFF]),
        Ok((&[0x00, 0xFF][..], &[0x01, 0x02][..]))
    );
    assert!(take_until_nullbyte(&[0xAB, 0xCD]).is_err());
}

pub fn parse_utf8(input: &[u8]) -> Res<&[u8], &str> {
    std::str::from_utf8(input)
        .map(|s| (&b""[..], s))
        .map_err(|_| nom::Err::Incomplete(nom::Needed::Unknown))
}

#[test]
fn test_parse_utf8() {
    assert_eq!(parse_utf8(&[0x41, 0x42]), Ok((&b""[..], "AB")));
}

pub fn take_utf8(input: &[u8]) -> Res<&[u8], &str> {
    let (input, data) = take_until_nullbyte(input)?;
    let (_, value) = parse_utf8(data)?;
    let (input, _) = nom::bytes::complete::take(1usize)(input)?;
    Ok((input, value))
}

#[test]
fn test_take_utf8() {
    assert_eq!(take_utf8(&[0x41, 0x42, 0x00]), Ok((&b""[..], "AB")));
}
