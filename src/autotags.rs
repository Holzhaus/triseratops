use crate::util;
use nom::take_until;

#[derive(Debug)]
pub struct Autotags {
    pub version: util::Version,
    pub bpm: f64,
    pub auto_gain: f64,
    pub gain_db: f64,
}

const NULL: &[u8] = &[0x00];

nom::named!(take_until_nullbyte, take_until!(NULL));

pub fn double_str(input: &[u8]) -> nom::IResult<&[u8], f64> {
    let (input, text) = take_until_nullbyte(input)?;
    let (_, num) = nom::combinator::all_consuming(nom::number::complete::double)(text)?;
    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, num))
}

pub fn parse(input: &[u8]) -> Result<Autotags, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::version_info(input).unwrap();
    let (input, bpm) = double_str(input)?;
    let (input, auto_gain) = double_str(input)?;
    let (_, gain_db) = nom::combinator::all_consuming(double_str)(input)?;

    Ok(Autotags {
        version,
        bpm,
        auto_gain,
        gain_db,
    })
}
