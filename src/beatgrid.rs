use crate::util;
use nom::length_count;
use nom::named;

#[derive(Debug)]
pub struct TerminalMarker {
    pub position: f32,
    pub bpm: f32,
}

#[derive(Debug)]
pub struct NonTerminalMarker {
    pub position: f32,
    pub beats_till_next_marker: u32,
}

#[derive(Debug)]
pub enum BeatgridMarker {
    Terminal(TerminalMarker),
    NonTerminal(NonTerminalMarker),
}

#[derive(Debug)]
pub struct Beatgrid {
    pub version: util::Version,
    pub non_terminal_markers: Vec<NonTerminalMarker>,
    pub terminal_marker: TerminalMarker,
    pub footer: u8,
}

fn non_terminal_marker_count(input: &[u8]) -> nom::IResult<&[u8], u32> {
    let (input, count) = nom::number::complete::be_u32(input)?;
    Ok((input, count - 1))
}

named!(
    take_non_terminal_markers<Vec<NonTerminalMarker>>,
    length_count!(non_terminal_marker_count, non_terminal_marker)
);

pub fn non_terminal_marker(input: &[u8]) -> nom::IResult<&[u8], NonTerminalMarker> {
    let (input, position) = nom::number::complete::be_f32(input)?;
    let (input, beats_till_next_marker) = nom::number::complete::be_u32(input)?;
    Ok((
        input,
        NonTerminalMarker {
            position,
            beats_till_next_marker,
        },
    ))
}

pub fn terminal_marker(input: &[u8]) -> nom::IResult<&[u8], TerminalMarker> {
    let (input, position) = nom::number::complete::be_f32(input)?;
    let (input, bpm) = nom::number::complete::be_f32(input)?;
    Ok((input, TerminalMarker { position, bpm }))
}

pub fn parse(input: &[u8]) -> Result<Beatgrid, nom::Err<nom::error::Error<&[u8]>>> {
    let (input, version) = util::version_info(&input)?;
    let (input, non_terminal_markers) = take_non_terminal_markers(input)?;
    let (input, terminal_marker) = terminal_marker(input)?;
    let (_, footer) = nom::combinator::all_consuming(nom::number::complete::u8)(input)?;

    Ok(Beatgrid {
        version,
        non_terminal_markers,
        terminal_marker,
        footer,
    })
}
