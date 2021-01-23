extern crate nom;

use nom::named;
use nom::take;
use nom::IResult;

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

named!(take_version, take!(2));

pub fn version_info(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, version) = take_version(input)?;
    Ok((
        input,
        Version {
            major: version[0],
            minor: version[1],
        },
    ))
}
