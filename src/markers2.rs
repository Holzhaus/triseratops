use crate::util;
use nom::alt;
use nom::multi::many0;
use nom::named;
use nom::peek;
use nom::tag;

#[derive(Debug)]
pub enum Marker {
    Unknown(UnknownMarker),
    Color(TrackColorMarker),
    BPMLock(BPMLockMarker),
    Cue(CueMarker),
    Loop(LoopMarker),
}

#[derive(Debug)]
pub struct UnknownMarker {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct TrackColorMarker {
    pub color: util::Color,
}

#[derive(Debug)]
pub struct BPMLockMarker {
    pub is_locked: bool,
}

#[derive(Debug)]
pub struct CueMarker {
    pub index: u8,
    pub position_millis: u32,
    pub color: util::Color,
    pub label: String,
}

#[derive(Debug)]
pub struct LoopMarker {
    pub index: u8,
    pub start_position_millis: u32,
    pub end_position_millis: u32,
    pub color: util::Color,
    pub is_locked: bool,
    pub label: String,
}

#[derive(Debug)]
pub struct Markers2 {
    pub version: util::Version,
    pub size: usize,
    pub content: Markers2Content,
}

#[derive(Debug)]
pub struct Markers2Content {
    pub version: util::Version,
    pub markers: Vec<Marker>,
}

fn is_base64(chr: u8) -> bool {
    chr.is_ascii_alphanumeric() || chr == b'+' || chr == b'/'
}

named!(
    peek_newline_or_nullbyte,
    peek!(alt!(tag!(b"\n") | tag!(b"\0")))
);
named!(peek_nullbyte, peek!(tag!(b"\0")));

pub fn take_base64_chunk(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    let (input, encoded_data) = nom::bytes::complete::take_while1(is_base64)(input)?;
    if encoded_data.is_empty() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }

    let (input, byte) = peek_newline_or_nullbyte(input)?;
    if byte == [b'\0'] {
        return Ok((input, encoded_data));
    }

    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, encoded_data))
}

pub fn take_base64_chunks(input: &[u8]) -> nom::IResult<&[u8], Vec<&[u8]>> {
    let (input, (base64data, _)) = nom::multi::many_till(take_base64_chunk, peek_nullbyte)(input)?;
    Ok((input, base64data))
}

pub fn decode_base64_chunks(
    encoded_chunks: Vec<&[u8]>,
) -> Result<Vec<u8>, nom::Err<nom::error::Error<&[u8]>>> {
    let mut decoded_data = Vec::new();
    for chunk in &encoded_chunks {
        if chunk.len() > 72 {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        }
        let mut buf = [0; 54];
        let num_bytes = base64::decode_config_slice(&chunk, base64::STANDARD, &mut buf).unwrap();
        decoded_data.extend_from_slice(&buf[..num_bytes]);
    }

    Ok(decoded_data)
}

pub fn parse_utf8(input: &[u8]) -> nom::IResult<&[u8], String> {
    let res = std::str::from_utf8(&input);
    match res {
        Ok(s) => Ok((b"", s.to_owned())),
        Err(_) => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

pub fn take_utf8(input: &[u8]) -> nom::IResult<&[u8], String> {
    let (input, data) = util::take_until_nullbyte(&input)?;
    let (_, value) = parse_utf8(&data)?;
    let (input, _) = nom::bytes::complete::take(1usize)(input)?;
    Ok((input, value))
}

pub fn take_marker_name(input: &[u8]) -> nom::IResult<&[u8], String> {
    let (input, _) = nom::combinator::not(nom::bytes::complete::tag(b"\0"))(input)?;
    let (input, name) = take_utf8(input)?;
    if name.is_empty() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    Ok((input, name))
}

pub fn take_marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, name) = take_marker_name(input)?;
    let (input, data) = nom::multi::length_data(nom::number::complete::be_u32)(input)?;

    let (_, marker) = match name.as_str() {
        "BPMLOCK" => nom::combinator::all_consuming(take_bpmlock_marker)(data)?,
        "COLOR" => nom::combinator::all_consuming(take_color_marker)(data)?,
        "CUE" => nom::combinator::all_consuming(take_cue_marker)(data)?,
        "LOOP" => nom::combinator::all_consuming(take_loop_marker)(data)?,
        _ => (
            input,
            Marker::Unknown(UnknownMarker {
                name,
                data: data.to_vec(),
            }),
        ),
    };

    Ok((input, marker))
}

pub fn take_bool(input: &[u8]) -> nom::IResult<&[u8], bool> {
    let (input, number) = nom::number::complete::u8(input)?;
    let value = number != 0;
    Ok((input, value))
}

pub fn take_bpmlock_marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, is_locked) = take_bool(input)?;
    let marker = BPMLockMarker { is_locked };
    Ok((input, Marker::BPMLock(marker)))
}

pub fn take_color_marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = util::take_color(input)?;
    let marker = TrackColorMarker { color };
    Ok((input, Marker::Color(marker)))
}

pub fn take_cue_marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, position_millis) = nom::number::complete::be_u32(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = util::take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00\x00")(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = CueMarker {
        index,
        position_millis,
        color,
        label,
    };
    Ok((input, Marker::Cue(marker)))
}

pub fn take_loop_marker(input: &[u8]) -> nom::IResult<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, start_position_millis) = nom::number::complete::be_u32(input)?;
    let (input, end_position_millis) = nom::number::complete::be_u32(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\xff\xff\xff\xff")(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = util::take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, is_locked) = take_bool(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = LoopMarker {
        index,
        start_position_millis,
        end_position_millis,
        color,
        is_locked,
        label,
    };
    Ok((input, Marker::Loop(marker)))
}

pub fn parse_markers2_content(input: &[u8]) -> nom::IResult<&[u8], Markers2Content> {
    let (input, version) = util::take_version(&input)?;
    let (input, markers) = many0(take_marker)(&input)?;

    Ok((input, Markers2Content { version, markers }))
}

pub fn parse(input: &[u8]) -> Result<Markers2, nom::Err<nom::error::Error<&[u8]>>> {
    let size = input.len();
    let (input, version) = util::take_version(&input)?;
    let (_, base64_chunks) = take_base64_chunks(&input)?;
    let base64_decoded = decode_base64_chunks(base64_chunks)?;
    let markers2_result = parse_markers2_content(&base64_decoded);
    if markers2_result.is_err() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    let (_, content) = markers2_result.unwrap();

    Ok(Markers2 {
        version,
        size,
        content,
    })
}
