use crate::util;

#[derive(Debug)]
pub struct Autotags {
    pub version: util::Version,
    pub bpm: f64,
    pub auto_gain: f64,
    pub gain_db: f64,
}

pub fn double_str(input: &[u8]) -> nom::IResult<&[u8], f64> {
    let (input, text) = util::take_until_nullbyte(input)?;
    let (_, num) = nom::combinator::all_consuming(nom::number::complete::double)(text)?;
    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, num))
}

pub fn parse(input: &[u8]) -> Result<Autotags, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::take_version(input).unwrap();
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
