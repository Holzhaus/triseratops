//! The `Serato Markers2` tag stores various kinds of track "markers" like Cue Points, Saved Loops, Flips.
//!
//! It also stores information about the tracks' color in the tracklist and if the track's beatgrid is locked.
//!
//! Note that some of this information is also stored in `Serato Markers_`, and Serato will prefer that data over the information stored in `Serato Markers2` if it is present.
//!
//! The minimum length of this tag seems to be 470 bytes, and shorter contents are padded with null bytes.

use super::color::Color;
use super::format::{enveloped, flac, id3, mp4, ogg, Tag};
use super::generic::{
    CensorFlipAction, Cue, Flip, FlipAction, JumpFlipAction, Loop, Position, UnknownFlipAction,
    Version,
};
use super::util::{take_color, take_version, write_color, write_version};
use crate::error::Error;
use crate::util::{take_utf8, Res, NULL};
use nom::error::ParseError;
use std::io;
use std::io::Cursor;

/// A marker in the `Serato Markers2` tag.
///
/// Each marker is described by a header that contains type and length. The type is a
/// null-terminated ASCII string.
///
/// The length of the entry's data depends heavily on the entry type. BPMLOCK entries contain only
/// a single byte of data, while FLIP might become quite large. By storing the length explicitly
/// instead of deriving it from the type, a parser could ignore unknown entry types and still be
/// able to parse known ones.
#[derive(Debug, Clone)]
pub enum Marker {
    Unknown(UnknownMarker),
    Color(TrackColorMarker),
    BPMLock(BPMLockMarker),
    Cue(Cue),
    Loop(Loop),
    Flip(Flip),
}

/// An unknown marker that we don't have a parser for.
#[derive(Debug, Clone)]
pub struct UnknownMarker {
    pub name: String,
    pub data: Vec<u8>,
}

/// A `COLOR` marker.
///
/// `COLOR` markers describe a track's color.
#[derive(Debug, Clone)]
pub struct TrackColorMarker {
    pub color: Color,
}

/// A `BPMLOCK` marker.
///
/// The `BPMLOCK` marker contains a single boolean value that determines if [Beatgrid is
/// locked](https://support.serato.com/hc/en-us/articles/235214887-Lock-Beatgrids).
#[derive(Debug, Clone)]
pub struct BPMLockMarker {
    pub is_locked: bool,
}

/// Represents the `Serato Markers2` tag.
///
/// It contains all cue points, loops and [flips](https://serato.com/dj/pro/expansions/flip), as well as the BPM-lock state and color of the track.
///
/// Note that some information is also duplicated in the legacy [`Serato Markers_`](super::Markers) tag.
/// If the two tags contradict each other, Serato DJ will prefer the data from the `Serato Markers_` tag.
///
/// # Example
///
/// ```
/// use triseratops::tag::{Markers2, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Markers2::ID3_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = Markers2::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Markers2 {
    pub version: Option<Version>,
    pub size: usize,
    pub content: Markers2Content,
}

impl Markers2 {
    #[must_use]
    pub fn bpm_locked(&self) -> Option<bool> {
        for marker in &self.content.markers {
            if let Marker::BPMLock(m) = marker {
                return Some(m.is_locked);
            }
        }
        None
    }

    pub fn cues(&self) -> impl Iterator<Item = &Cue> {
        self.content.markers.iter().filter_map(|marker| {
            if let Marker::Cue(m) = marker {
                Some(m)
            } else {
                None
            }
        })
    }

    pub fn loops(&self) -> impl Iterator<Item = &Loop> {
        self.content.markers.iter().filter_map(|marker| {
            if let Marker::Loop(m) = marker {
                Some(m)
            } else {
                None
            }
        })
    }

    pub fn flips(&self) -> impl Iterator<Item = &Flip> {
        self.content.markers.iter().filter_map(|marker| {
            if let Marker::Flip(m) = marker {
                Some(m)
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn track_color(&self) -> Option<Color> {
        self.content
            .markers
            .iter()
            .filter_map(|marker| {
                if let Marker::Color(m) = marker {
                    Some(m.color)
                } else {
                    None
                }
            })
            .next()
    }
}

impl Tag for Markers2 {
    const NAME: &'static str = "Serato Markers2";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_markers2)(input)?;
        Ok(autotags)
    }

    fn write(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        write_markers2(writer, self)
    }
}

impl id3::ID3Tag for Markers2 {}
impl enveloped::EnvelopedTag for Markers2 {}
impl flac::FLACTag for Markers2 {
    const FLAC_COMMENT: &'static str = "SERATO_MARKERS_V2";
}
impl mp4::MP4Tag for Markers2 {
    const MP4_ATOM_FREEFORM_NAME: &'static str = "markersv2";
}
impl ogg::OggTag for Markers2 {
    const OGG_COMMENT: &'static str = "serato_markers2";

    fn parse_ogg(input: &[u8]) -> Result<Self, Error> {
        let size = input.len();
        let base64_decoded = enveloped::base64_decode(input)?;
        let version = None;
        match parse_markers2_content(&base64_decoded) {
            Ok((_, content)) => {
                let markers2 = Markers2 {
                    version,
                    size,
                    content,
                };
                Ok(markers2)
            }
            Err(_) => Err(Error::ParseError),
        }
    }

    fn write_ogg(&self, writer: &mut impl io::Write) -> Result<usize, Error> {
        let mut buffer = Cursor::new(vec![]);
        write_markers2_content(&mut buffer, &self.content)?;
        let plain_data = &buffer.get_ref()[..];
        let mut bytes_written = enveloped::base64_encode(writer, plain_data)?;
        if self.size > bytes_written {
            for _ in 0..(self.size - bytes_written) {
                bytes_written += writer.write(NULL)?;
            }
        }
        Ok(bytes_written)
    }
}

/// Represents the base64-encoded content of the `Serato Markers2` tag.
#[derive(Debug, Clone)]
pub struct Markers2Content {
    pub version: Version,
    pub markers: Vec<Marker>,
}

/// Returns true if `chr` is a valid ASCII character.
fn is_base64(chr: u8) -> bool {
    chr.is_ascii_alphanumeric() || chr == b'+' || chr == b'/'
}

/// Returns a nullbyte from the input slice (non-consuming).
pub fn peek_nullbyte(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::combinator::peek(nom::bytes::complete::tag(b"\0"))(input)
}

/// Returns a nullbyte or newline character from the input slice (non-consuming).
pub fn peek_newline_or_nullbyte(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::combinator::peek(nom::branch::alt((
        nom::bytes::complete::tag(b"\n"),
        nom::bytes::complete::tag(b"\0"),
    )))(input)
}

fn take_base64_chunk(input: &[u8]) -> Res<&[u8], &[u8]> {
    let (input, encoded_data) = nom::error::context(
        "Get base64 encoded chunk",
        nom::bytes::complete::take_while1(is_base64),
    )(input)?;
    let (input, byte) = peek_newline_or_nullbyte(input)?;
    if byte == [b'\0'] {
        return Ok((input, encoded_data));
    }
    let (input, _) = nom::number::complete::u8(input)?;
    Ok((input, encoded_data))
}

fn take_base64_chunks(input: &[u8]) -> Res<&[u8], Vec<&[u8]>> {
    let (input, (base64data, _)) = nom::error::context(
        "Get all base64 encoded chunks",
        nom::multi::many_till(take_base64_chunk, peek_nullbyte),
    )(input)?;
    Ok((input, base64data))
}

fn decode_base64_chunks(
    encoded_chunks: Vec<&[u8]>,
) -> Result<Vec<u8>, nom::Err<nom::error::VerboseError<&[u8]>>> {
    let mut decoded_data = Vec::new();
    for &chunk in &encoded_chunks {
        if chunk.len() > 72 {
            return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
                chunk,
                nom::error::ErrorKind::LengthValue,
            )));
        }
        let mut buf = [0; 54];
        // TODO: Add proper error handling here
        let mut res = base64::decode_config_slice(chunk, base64::STANDARD, &mut buf);
        if let Err(base64::DecodeError::InvalidLength) = res {
            let mut v = Vec::new();
            v.extend_from_slice(chunk);
            v.push(b'A');
            res = base64::decode_config_slice(v.as_slice(), base64::STANDARD, &mut buf);
        }
        let num_bytes = res.unwrap();
        decoded_data.extend_from_slice(&buf[..num_bytes]);
    }

    Ok(decoded_data)
}

fn take_marker_name(input: &[u8]) -> Res<&[u8], &str> {
    let (input, _) = nom::combinator::not(nom::bytes::complete::tag(b"\0"))(input)?;
    let (input, name) = take_utf8(input)?;
    if name.is_empty() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    Ok((input, name))
}

/// Returns a [`Marker`] parsed from the input slice.
fn take_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, name) = take_marker_name(input)?;
    let (input, data) = nom::multi::length_data(nom::number::complete::be_u32)(input)?;

    let (_, marker) = match name {
        "BPMLOCK" => nom::combinator::all_consuming(take_bpmlock_marker)(data)?,
        "COLOR" => nom::combinator::all_consuming(take_color_marker)(data)?,
        "CUE" => nom::combinator::all_consuming(take_cue_marker)(data)?,
        "LOOP" => nom::combinator::all_consuming(take_loop_marker)(data)?,
        "FLIP" => nom::combinator::all_consuming(take_flip_marker)(data)?,
        _ => (
            input,
            Marker::Unknown(UnknownMarker {
                name: name.to_owned(),
                data: data.to_owned(),
            }),
        ),
    };

    Ok((input, marker))
}

/// Returns a boolean parsed from the input slice.
fn take_bool(input: &[u8]) -> Res<&[u8], bool> {
    let (input, number) = nom::number::complete::u8(input)?;
    let value = number != 0;
    Ok((input, value))
}

fn take_bpmlock_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, is_locked) = take_bool(input)?;
    let marker = BPMLockMarker { is_locked };
    Ok((input, Marker::BPMLock(marker)))
}

fn take_color_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, color) = take_color(input)?;
    let marker = TrackColorMarker { color };
    Ok((input, Marker::Color(marker)))
}

/// Returns a `Position` struct parsed from the first 4 input bytes.
fn take_position(input: &[u8]) -> Res<&[u8], Position> {
    let (input, millis) = nom::number::complete::be_u32(input)?;
    let position = Position { millis };
    Ok((input, position))
}

fn take_cue_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, position) = take_position(input)?;
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, color) = take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00\x00")(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = Cue {
        index,
        position,
        color,
        label: label.to_owned(),
    };
    Ok((input, Marker::Cue(marker)))
}

fn take_loop_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, start_position_millis) = nom::number::complete::be_u32(input)?;
    let start_position = super::generic::Position {
        millis: start_position_millis,
    };
    let (input, end_position_millis) = nom::number::complete::be_u32(input)?;
    let end_position = super::generic::Position {
        millis: end_position_millis,
    };
    let (input, _) = nom::bytes::complete::tag(b"\xff\xff\xff\xff")(input)?;
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, color) = take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, is_locked) = take_bool(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = Loop {
        index,
        start_position,
        end_position,
        color,
        is_locked,
        label: label.to_owned(),
    };
    Ok((input, Marker::Loop(marker)))
}

fn take_flip_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(NULL)(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, is_enabled) = take_bool(input)?;
    let (input, label) = take_utf8(input)?;
    let (input, is_loop) = take_bool(input)?;
    let (input, actions) =
        nom::multi::length_count(nom::number::complete::be_u32, take_flip_marker_action)(input)?;
    let marker = Flip {
        index,
        is_enabled,
        label: label.to_owned(),
        is_loop,
        actions,
    };
    Ok((input, Marker::Flip(marker)))
}

/// Returns a flip `FLIP` action parsed from the input slice.
///
/// Each action starts with a header that contains its type and length.
fn take_flip_marker_action(input: &[u8]) -> Res<&[u8], FlipAction> {
    let (input, id) = nom::number::complete::u8(input)?;
    let (input, data) = nom::multi::length_data(nom::number::complete::be_u32)(input)?;
    let (_, action) = match id {
        0 => nom::combinator::all_consuming(take_flip_marker_action_jump)(data)?,
        1 => nom::combinator::all_consuming(take_flip_marker_action_censor)(data)?,
        _ => (
            input,
            FlipAction::Unknown(UnknownFlipAction {
                id,
                data: data.to_owned(),
            }),
        ),
    };

    Ok((input, action))
}

fn take_flip_marker_action_jump(input: &[u8]) -> Res<&[u8], FlipAction> {
    let (input, source_position_seconds) = nom::number::complete::be_f64(input)?;
    let (input, target_position_seconds) = nom::number::complete::be_f64(input)?;
    let action = JumpFlipAction {
        source_position_seconds,
        target_position_seconds,
    };
    Ok((input, FlipAction::Jump(action)))
}

fn take_flip_marker_action_censor(input: &[u8]) -> Res<&[u8], FlipAction> {
    let (input, start_position_seconds) = nom::number::complete::be_f64(input)?;
    let (input, end_position_seconds) = nom::number::complete::be_f64(input)?;
    let (input, speed_factor) = nom::number::complete::be_f64(input)?;
    let action = CensorFlipAction {
        start_position_seconds,
        end_position_seconds,
        speed_factor,
    };
    Ok((input, FlipAction::Censor(action)))
}

fn parse_markers2_content(input: &[u8]) -> Res<&[u8], Markers2Content> {
    let (input, version) = take_version(input)?;
    let (input, markers) = nom::multi::many0(take_marker)(input)?;

    Ok((input, Markers2Content { version, markers }))
}

fn take_nullbytes(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::error::context(
        "Take nullbytes",
        nom::bytes::complete::take_while(|x| x == 0),
    )(input)
}

fn take_markers2(input: &[u8]) -> Res<&[u8], Markers2> {
    let size = input.len();
    let (input, version) = take_version(input)?;
    let version = Some(version);
    let (input, base64_chunks) = take_base64_chunks(input)?;
    let (input, _) = take_nullbytes(input)?;
    let base64_decoded = decode_base64_chunks(base64_chunks)?;
    let markers2_result = parse_markers2_content(&base64_decoded);
    if markers2_result.is_err() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    let (_, content) = markers2_result.unwrap();
    let markers2 = Markers2 {
        version,
        size,
        content,
    };
    Ok((input, markers2))
}

fn write_position(writer: &mut impl io::Write, position: Position) -> Result<usize, Error> {
    let Position { millis } = position;
    Ok(writer.write(&millis.to_be_bytes())?)
}
fn write_markers2(writer: &mut impl io::Write, markers2: &Markers2) -> Result<usize, Error> {
    let version = match markers2.version {
        Some(version) => version,
        None => {
            return Err(Error::ParseError);
        }
    };
    let mut bytes_written = write_version(writer, version)?;
    let mut buffer = Cursor::new(vec![]);
    write_markers2_content(&mut buffer, &markers2.content)?;
    let plain_data = &buffer.get_ref()[..];
    bytes_written += enveloped::base64_encode(writer, plain_data)?;
    if markers2.size > bytes_written {
        for _ in 0..(markers2.size - bytes_written) {
            bytes_written += writer.write(NULL)?;
        }
    }
    Ok(bytes_written)
}

fn write_markers2_content(
    writer: &mut impl io::Write,
    content: &Markers2Content,
) -> Result<usize, Error> {
    let mut bytes_written = write_version(writer, content.version)?;
    for marker in &content.markers {
        bytes_written += write_marker(writer, marker)?;
    }
    Ok(bytes_written)
}

fn write_unknown_marker(
    writer: &mut impl io::Write,
    marker: &UnknownMarker,
) -> Result<usize, Error> {
    let UnknownMarker { name, data } = marker;
    let mut bytes_written = writer.write(name.as_bytes())?;
    bytes_written += writer.write(b"\0")?;
    let size = marker.data.len() as u32;
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += writer.write(data.as_slice())?;
    Ok(bytes_written)
}

fn write_marker(writer: &mut impl io::Write, marker: &Marker) -> Result<usize, Error> {
    match marker {
        Marker::Unknown(marker) => write_unknown_marker(writer, marker),
        Marker::BPMLock(marker) => write_bpmlock_marker(writer, marker),
        Marker::Color(marker) => write_color_marker(writer, marker),
        Marker::Cue(marker) => write_cue_marker(writer, marker),
        Marker::Loop(marker) => write_loop_marker(writer, marker),
        Marker::Flip(marker) => write_flip_marker(writer, marker),
    }
}

fn write_bool(writer: &mut impl io::Write, value: bool) -> Result<usize, Error> {
    let byte: u8 = match value {
        true => 1,
        false => 0,
    };
    Ok(writer.write(&[byte])?)
}

fn write_bpmlock_marker(
    writer: &mut impl io::Write,
    marker: &BPMLockMarker,
) -> Result<usize, Error> {
    let BPMLockMarker { is_locked } = marker;
    let mut bytes_written = writer.write(b"BPMLOCK\0")?;
    let size: u32 = 1;
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += write_bool(writer, *is_locked)?;
    Ok(bytes_written)
}

fn write_color_marker(
    writer: &mut impl io::Write,
    marker: &TrackColorMarker,
) -> Result<usize, Error> {
    let &TrackColorMarker { color } = marker;
    let mut bytes_written = writer.write(b"COLOR\0")?;
    let size: u32 = 4;
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += write_color(writer, color)?;
    Ok(bytes_written)
}

fn write_cue_marker(writer: &mut impl io::Write, marker: &Cue) -> Result<usize, Error> {
    let Cue {
        color,
        index,
        label,
        position,
    } = marker;
    let mut bytes_written = writer.write(b"CUE\0")?;
    let size: u32 = 13 + label.as_bytes().len() as u32;
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += writer.write(&[*index])?;
    bytes_written += write_position(writer, *position)?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += write_color(writer, *color)?;
    bytes_written += writer.write(b"\0\0")?;
    bytes_written += writer.write(label.as_bytes())?;
    bytes_written += writer.write(b"\0")?;
    Ok(bytes_written)
}

fn write_loop_marker(writer: &mut impl io::Write, marker: &Loop) -> Result<usize, Error> {
    let Loop {
        label,
        index,
        start_position,
        end_position,
        color,
        is_locked,
    } = marker;
    let mut bytes_written = writer.write(b"LOOP\0")?;
    let size: u32 = 21 + marker.label.as_bytes().len() as u32;
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += writer.write(&[*index])?;
    bytes_written += write_position(writer, *start_position)?;
    bytes_written += write_position(writer, *end_position)?;
    bytes_written += writer.write(b"\xFF\xFF\xFF\xFF\0")?;
    bytes_written += write_color(writer, *color)?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += write_bool(writer, *is_locked)?;
    bytes_written += writer.write(label.as_bytes())?;
    bytes_written += writer.write(b"\0")?;
    Ok(bytes_written)
}

fn write_flip_marker(writer: &mut impl io::Write, marker: &Flip) -> Result<usize, Error> {
    let Flip {
        actions,
        index,
        is_enabled,
        is_loop,
        label,
    } = marker;
    let mut bytes_written = writer.write(b"FLIP\0")?;
    let mut size: u32 = 9 + marker.label.as_bytes().len() as u32;
    for action in &marker.actions {
        size += match action {
            FlipAction::Jump(_) => 21u32,
            FlipAction::Censor(_) => 29u32,
            FlipAction::Unknown(act) => act.data.len() as u32 + 1,
        }
    }
    bytes_written += writer.write(&size.to_be_bytes())?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += writer.write(&[*index])?;
    bytes_written += write_bool(writer, *is_enabled)?;
    bytes_written += writer.write(label.as_bytes())?;
    bytes_written += writer.write(b"\0")?;
    bytes_written += write_bool(writer, *is_loop)?;
    let num_actions = actions.len() as u32;
    bytes_written += writer.write(&num_actions.to_be_bytes())?;
    for action in actions {
        bytes_written = write_flip_marker_action(writer, action)?;
    }
    Ok(bytes_written)
}

fn write_flip_marker_action(
    writer: &mut impl io::Write,
    action: &FlipAction,
) -> Result<usize, Error> {
    match action {
        FlipAction::Jump(act) => {
            let mut bytes_written = writer.write(NULL)?;
            let size = 16u32;
            bytes_written += writer.write(&size.to_be_bytes())?;
            bytes_written += write_flip_marker_action_jump(writer, act)?;
            Ok(bytes_written)
        }
        FlipAction::Censor(act) => {
            let mut bytes_written = writer.write(b"\x01")?;
            let size = 24u32;
            bytes_written += writer.write(&size.to_be_bytes())?;
            bytes_written += write_flip_marker_action_censor(writer, act)?;
            Ok(bytes_written)
        }
        FlipAction::Unknown(act) => {
            let mut bytes_written = writer.write(&[act.id])?;
            let size = act.data.len() as u32;
            bytes_written += writer.write(&size.to_be_bytes())?;
            bytes_written += writer.write(act.data.as_slice())?;
            Ok(bytes_written)
        }
    }
}

fn write_flip_marker_action_jump(
    writer: &mut impl io::Write,
    action: &JumpFlipAction,
) -> Result<usize, Error> {
    let JumpFlipAction {
        source_position_seconds,
        target_position_seconds,
    } = action;
    let mut bytes_written = writer.write(&source_position_seconds.to_be_bytes())?;
    bytes_written += writer.write(&target_position_seconds.to_be_bytes())?;
    Ok(bytes_written)
}

fn write_flip_marker_action_censor(
    writer: &mut impl io::Write,
    action: &CensorFlipAction,
) -> Result<usize, Error> {
    let CensorFlipAction {
        start_position_seconds,
        end_position_seconds,
        speed_factor,
    } = action;
    let mut bytes_written = writer.write(&start_position_seconds.to_be_bytes())?;
    bytes_written += writer.write(&end_position_seconds.to_be_bytes())?;
    bytes_written += writer.write(&speed_factor.to_be_bytes())?;
    Ok(bytes_written)
}
