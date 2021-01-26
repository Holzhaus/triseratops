use crate::util;

#[derive(Debug)]
pub struct Analysis {
    pub version: util::Version,
}

pub fn parse(input: &[u8]) -> Result<Analysis, nom::Err<nom::error::Error<&[u8]>>> {
    match nom::combinator::all_consuming(util::take_version)(input) {
        Ok((_, version)) => Ok(Analysis { version }),
        Err(e) => Err(e),
    }
}
