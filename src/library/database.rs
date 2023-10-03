// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Error;
use crate::util::Res;
use std::path::PathBuf;

pub type Path = String;

const FIELD_BOOL: u8 = b'b';
const FIELD_CONTAINER: u8 = b'o';
const FIELD_CONTAINER_R: u8 = b'r';
const FIELD_PATH: u8 = b'p';
const FIELD_TEXT: u8 = b't';
const FIELD_U16: u8 = b's';
const FIELD_U32: u8 = b'u';

#[derive(Debug)]
pub enum Field {
    Unknown {
        field_type: u8,
        name: Vec<u8>,
        content: Vec<u8>,
    },
    UnknownBoolean {
        name: Vec<u8>,
        value: bool,
    },
    UnknownContainerField {
        name: Vec<u8>,
        fields: Vec<Field>,
    },
    UnknownContainerRField {
        name: Vec<u8>,
        fields: Vec<Field>,
    },
    UnknownPathField {
        name: Vec<u8>,
        path: PathBuf,
    },
    UnknownU16Field {
        name: Vec<u8>,
        value: u16,
    },
    UnknownU32Field {
        name: Vec<u8>,
        value: u32,
    },
    UnknownTextField {
        name: Vec<u8>,
        text: String,
    },
    // Library
    Album(String),
    Artist(String),
    BPM(String),
    BeatgridLocked(bool),
    Bitrate(String),
    Comment(String),
    Composer(String),
    DateAdded(u32),
    DateAddedStr(String),
    FilePath(PathBuf),
    FileSize(String),
    FileTime(u32),
    FileType(String),
    Genre(String),
    Grouping(String),
    Key(String),
    Label(String),
    Length(String),
    Missing(bool),
    SampleRate(String),
    SongTitle(String),
    Track(Vec<Field>),
    Version(String),
    Year(String),
    // Crates
    Sorting(Vec<Field>),
    ReverseOrder(bool),
    ColumnTitle(Vec<Field>),
    ColumnName(String),
    ColumnWidth(String),
    TrackPath(PathBuf),
}

fn take_field_type(input: &[u8]) -> Res<&[u8], u8> {
    nom::number::complete::u8(input)
}

fn take_field_name(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::bytes::complete::take(3usize)(input)
}

fn take_field_desc(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::bytes::complete::take(4usize)(input)
}

fn take_field_length(input: &[u8]) -> Res<&[u8], u32> {
    nom::number::complete::be_u32(input)
}

fn take_field_content(input: &[u8]) -> Res<&[u8], &[u8]> {
    nom::multi::length_data(take_field_length)(input)
}

fn take_u16_bytes(input: &[u8]) -> Res<&[u8], Vec<u16>> {
    nom::multi::many0(nom::number::complete::be_u16)(input)
}

fn parse_u16_text(input: &[u8]) -> Res<&[u8], String> {
    let (input, bytes) = nom::combinator::all_consuming(take_u16_bytes)(input)?;
    let text = std::char::decode_utf16(bytes)
        .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER))
        .collect::<String>();
    Ok((input, text))
}

fn parse_bool(input: &[u8]) -> Res<&[u8], bool> {
    let (input, byte) = nom::number::complete::u8(input)?;
    let value = byte != 0x00;
    Ok((input, value))
}

fn parse_field<'a>(input: &'a [u8], name: &[u8], field_type: u8) -> Res<&'a [u8], Field> {
    match field_type {
        FIELD_BOOL => {
            let (input, value) = nom::combinator::all_consuming(parse_bool)(input)?;
            let field = match name {
                b"bgl" => Field::BeatgridLocked(value),
                b"mis" => Field::Missing(value),
                b"rev" => Field::ReverseOrder(value),
                //b"crt" => ???
                //b"hrt" => ???
                //b"iro" => ???
                //b"itu" => ???
                //b"krk" => ???
                //b"ovc" => ???
                //b"ply" => ???
                //b"uns" => ???
                //b"wlb" => ???
                //b"wll" => ???
                _ => Field::UnknownBoolean {
                    name: name.to_owned(),
                    value,
                },
            };
            Ok((input, field))
        }
        FIELD_U16 => {
            let (input, value) =
                nom::combinator::all_consuming(nom::number::complete::be_u16)(input)?;
            let field = Field::UnknownU16Field {
                name: name.to_owned(),
                value,
            };
            //b"bav" => ???
            Ok((input, field))
        }
        FIELD_U32 => {
            let (input, value) =
                nom::combinator::all_consuming(nom::number::complete::be_u32)(input)?;
            let field = match name {
                b"add" => Field::DateAdded(value),
                b"tme" => Field::FileTime(value),
                //b"lbl" => ???
                //b"fsb" => ???
                //b"tkn" => ???
                //b"dsc" => ???
                _ => Field::UnknownU32Field {
                    name: name.to_owned(),
                    value,
                },
            };
            Ok((input, field))
        }
        FIELD_PATH => {
            let (input, path) = parse_u16_text(input)?;
            let path = PathBuf::from(path);
            let field = match name {
                b"fil" => Field::FilePath(path),
                b"trk" => Field::TrackPath(path),
                _ => Field::UnknownPathField {
                    name: name.to_owned(),
                    path,
                },
            };
            Ok((input, field))
        }
        FIELD_TEXT => {
            let (input, text) = parse_u16_text(input)?;
            let field = match name {
                b"add" => Field::DateAddedStr(text),
                b"alb" => Field::Album(text),
                b"art" => Field::Artist(text),
                b"bit" => Field::Bitrate(text),
                b"bpm" => Field::BPM(text),
                b"cmp" => Field::Composer(text),
                b"com" => Field::Comment(text),
                b"gen" => Field::Genre(text),
                b"grp" => Field::Grouping(text),
                b"key" => Field::Key(text),
                b"lbl" => Field::Label(text),
                b"len" => Field::Length(text),
                b"siz" => Field::FileSize(text),
                b"smp" => Field::SampleRate(text),
                b"sng" => Field::SongTitle(text),
                b"typ" => Field::FileType(text),
                b"tyr" => Field::Year(text),
                b"vcn" => Field::ColumnName(text),
                b"vcw" => Field::ColumnWidth(text),
                b"vrsn" => Field::Version(text),
                _ => Field::UnknownTextField {
                    name: name.to_owned(),
                    text,
                },
            };
            Ok((input, field))
        }
        FIELD_CONTAINER => {
            let (input, fields) = nom::combinator::all_consuming(take_fields)(input)?;
            let field = match name {
                b"srt" => Field::Sorting(fields),
                b"trk" => Field::Track(fields),
                b"vct" => Field::ColumnTitle(fields),
                _ => Field::UnknownContainerField {
                    name: name.to_owned(),
                    fields,
                },
            };
            Ok((input, field))
        }
        FIELD_CONTAINER_R => {
            let (input, fields) = nom::combinator::all_consuming(take_fields)(input)?;
            let field = Field::UnknownContainerRField {
                name: name.to_owned(),
                fields,
            };
            Ok((input, field))
        }
        _ => {
            let name = name.to_owned();
            let content = input.to_owned();
            Ok((
                b"",
                Field::Unknown {
                    field_type,
                    name,
                    content,
                },
            ))
        }
    }
}

fn take_field(input: &[u8]) -> Res<&[u8], Field> {
    let (input, desc) = take_field_desc(input)?;
    let (input, content) = take_field_content(input)?;
    let (_, field) = match desc {
        // Special case: `vrsn` is a text field but begins with `v`
        b"vrsn" => parse_field(content, desc, FIELD_TEXT)?,
        _ => {
            let (desc, typ) = take_field_type(desc)?;
            let (_, name) = take_field_name(desc)?;
            parse_field(content, name, typ)?
        }
    };
    Ok((input, field))
}

fn take_fields(input: &[u8]) -> Res<&[u8], Vec<Field>> {
    nom::multi::many1(take_field)(input)
}

pub fn parse(input: &[u8]) -> Result<Vec<Field>, Error> {
    let (_, fields) = nom::combinator::all_consuming(take_fields)(input)?;
    Ok(fields)
}
