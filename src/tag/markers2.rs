//! The `Serato Markers2` tag stores various kinds of track "markers" like Cue Points, Saved Loops, Flips.
//!
//! It also stores information about the tracks' color in the tracklist and if the track's beatgrid is locked.
//!
//! Note that some of this information is also stored in `Serato Markers_`, and Serato will prefer that data over the information stored in `Serato Markers2` if it is present.
//!
//! The minimum length of this tag seems to be 470 bytes, and shorter contents are padded with null bytes.

use super::format::{enveloped, flac, id3, mp4, ogg, Tag};
use super::generic::{
    CensorFlipAction, Color, Cue, Flip, FlipAction, JumpFlipAction, Loop, UnknownFlipAction,
    Version,
};
use super::util::{take_color, take_version};
use crate::error::Error;
use crate::util::{take_utf8, Res};
use nom::error::ParseError;

/// A marker in the `Serato Markers2` tag.
///
/// Each marker is described by a header that contains type and length. The type is a
/// null-terminated ASCII string.
///
/// The length of the entry's data depends heavily on the entry type. BPMLOCK entries contain only
/// a single byte of data, while FLIP might become quite large. By storing the length explicitly
/// instead of deriving it from the type, a parser could ignore unknown entry types and still be
/// able to parse known ones.
#[derive(Debug)]
pub enum Marker {
    Unknown(UnknownMarker),
    Color(TrackColorMarker),
    BPMLock(BPMLockMarker),
    Cue(Cue),
    Loop(Loop),
    Flip(Flip),
}

/// An unknown marker that we don't have a parser for.
#[derive(Debug)]
pub struct UnknownMarker {
    pub name: String,
    pub data: Vec<u8>,
}

/// A `COLOR` marker.
///
/// `COLOR` markers describe a track's color.
#[derive(Debug)]
pub struct TrackColorMarker {
    pub color: Color,
}

/// A `BPMLOCK` marker.
///
/// The `BPMLOCK` marker contains a single boolean value that determines if [Beatgrid is
/// locked](https://support.serato.com/hc/en-us/articles/235214887-Lock-Beatgrids).
#[derive(Debug)]
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
/// use seratodj::tag::{Markers2, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Markers2::ID3_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = Markers2::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug)]
pub struct Markers2 {
    pub version: Option<Version>,
    pub size: usize,
    pub content: Markers2Content,
}

impl Markers2 {
    pub fn bpm_locked(&self) -> Option<bool> {
        for marker in &self.content.markers {
            if let Marker::BPMLock(m) = marker {
                return Some(m.is_locked);
            }
        }
        None
    }

    pub fn cues(&self) -> Vec<Cue> {
        let mut cues: Vec<Cue> = Vec::new();
        for marker in &self.content.markers {
            if let Marker::Cue(m) = marker {
                cues.push(m.clone());
            }
        }
        cues
    }

    pub fn loops(&self) -> Vec<Loop> {
        let mut loops: Vec<Loop> = Vec::new();
        for marker in &self.content.markers {
            if let Marker::Loop(m) = marker {
                loops.push(m.clone());
            }
        }
        loops
    }

    pub fn flips(&self) -> Vec<Flip> {
        let mut flips: Vec<Flip> = Vec::new();
        for marker in &self.content.markers {
            if let Marker::Flip(m) = marker {
                flips.push(m.clone());
            }
        }
        flips
    }

    pub fn track_color(&self) -> Option<Color> {
        for marker in &self.content.markers {
            if let Marker::Color(m) = marker {
                return Some(m.color);
            }
        }
        None
    }
}

impl Tag for Markers2 {
    const NAME: &'static str = "Serato Markers2";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, autotags) = nom::combinator::all_consuming(take_markers2)(input)?;
        Ok(autotags)
    }
}

impl id3::ID3Tag for Markers2 {}
impl enveloped::EnvelopedTag for Markers2 {}
impl flac::FLACTag for Markers2 {
    const FLAC_COMMENT: &'static str = "SERATO_MARKERS_V2";
}
impl mp4::MP4Tag for Markers2 {
    const MP4_ATOM: &'static str = "----:com.serato.dj:markersv2";
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
}

/// Represents the base64-encoded content of the `Serato Markers2` tag.
#[derive(Debug)]
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
    for chunk in &encoded_chunks {
        if chunk.len() > 72 {
            return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
                *chunk,
                nom::error::ErrorKind::LengthValue,
            )));
        }
        let mut buf = [0; 54];
        // TODO: Add proper error handling here
        let mut res = base64::decode_config_slice(&chunk, base64::STANDARD, &mut buf);
        if let Err(base64::DecodeError::InvalidLength) = res {
            let mut v = Vec::new();
            v.extend_from_slice(&chunk);
            v.push(b'A');
            res = base64::decode_config_slice(v.as_slice(), base64::STANDARD, &mut buf);
        }
        let num_bytes = res.unwrap();
        decoded_data.extend_from_slice(&buf[..num_bytes]);
    }

    Ok(decoded_data)
}

fn take_marker_name(input: &[u8]) -> Res<&[u8], String> {
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

    let (_, marker) = match name.as_str() {
        "BPMLOCK" => nom::combinator::all_consuming(take_bpmlock_marker)(data)?,
        "COLOR" => nom::combinator::all_consuming(take_color_marker)(data)?,
        "CUE" => nom::combinator::all_consuming(take_cue_marker)(data)?,
        "LOOP" => nom::combinator::all_consuming(take_loop_marker)(data)?,
        "FLIP" => nom::combinator::all_consuming(take_flip_marker)(data)?,
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
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = take_color(input)?;
    let marker = TrackColorMarker { color };
    Ok((input, Marker::Color(marker)))
}

fn take_cue_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, position_millis) = nom::number::complete::be_u32(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00\x00")(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = Cue {
        index,
        position_millis,
        color,
        label,
    };
    Ok((input, Marker::Cue(marker)))
}

fn take_loop_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, start_position_millis) = nom::number::complete::be_u32(input)?;
    let (input, end_position_millis) = nom::number::complete::be_u32(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\xff\xff\xff\xff")(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, color) = take_color(input)?;
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, is_locked) = take_bool(input)?;
    let (input, label) = take_utf8(input)?;
    let marker = Loop {
        index,
        start_position_millis,
        end_position_millis,
        color,
        is_locked,
        label,
    };
    Ok((input, Marker::Loop(marker)))
}

fn take_flip_marker(input: &[u8]) -> Res<&[u8], Marker> {
    let (input, _) = nom::bytes::complete::tag(b"\x00")(input)?;
    let (input, index) = nom::number::complete::u8(input)?;
    let (input, is_enabled) = take_bool(input)?;
    let (input, label) = take_utf8(input)?;
    let (input, is_loop) = take_bool(input)?;
    let (input, actions) =
        nom::multi::length_count(nom::number::complete::be_u32, take_flip_marker_action)(input)?;
    let marker = Flip {
        index,
        is_enabled,
        label,
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
                data: data.to_vec(),
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
    let (input, version) = take_version(&input)?;
    let (input, markers) = nom::multi::many0(take_marker)(&input)?;

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
    let (input, version) = take_version(&input)?;
    let version = Some(version);
    let (input, base64_chunks) = take_base64_chunks(&input)?;
    let (input, _) = take_nullbytes(&input)?;
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
