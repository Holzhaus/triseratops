extern crate id3;
extern crate triseratops;

use id3::Tag;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::str;
use std::string::String;
use triseratops::tag::format::id3::ID3Tag;
use triseratops::tag::TagType;

fn read_str(reader: &mut dyn BufRead) -> Result<String, Error> {
    let mut value = vec![];

    reader.read_until(0, &mut value)?;
    let s = str::from_utf8(&value[..value.len() - 1])
        .map_err(|_err| Error::new(ErrorKind::Other, "Failed to decode string"));
    Ok(s.unwrap().to_string())
}

fn main() -> Result<(), triseratops::error::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut container = triseratops::tag::TagContainer::new();

    let tag = Tag::read_from_path(filename).unwrap();
    for frame in tag.frames() {
        if frame.id() != "GEOB" {
            continue;
        }
        let content = frame.content();
        let buf = content.unknown().unwrap();
        let encoding = buf[0];

        let mut reader = BufReader::new(&buf[1..]);
        let mimetype = read_str(&mut reader).expect("Failed to read mime-type");
        let filename = read_str(&mut reader).expect("Failed to read filename");
        let content_desc = read_str(&mut reader).expect("Failed to read content description");
        if !content_desc.starts_with("Serato ") {
            continue;
        }

        let mut data = vec![];
        reader.read_to_end(&mut data).expect("Failed to read data");

        println!("{}", content_desc);
        println!("  Encoding: {:#?}", encoding);
        println!("  Mime-Type: {:#?}", mimetype);
        println!("  Filename: {:#?}", filename);

        println!("  Data: {} bytes", data.len());
        match &content_desc[..] {
            triseratops::tag::Analysis::ID3_TAG => {
                let tag = triseratops::tag::Analysis::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            triseratops::tag::Autotags::ID3_TAG => {
                let tag = triseratops::tag::Autotags::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            triseratops::tag::Beatgrid::ID3_TAG => {
                let tag = triseratops::tag::Beatgrid::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            triseratops::tag::Markers::ID3_TAG => {
                let tag = triseratops::tag::Markers::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_markers(&data, TagType::ID3)?;
            }
            triseratops::tag::Markers2::ID3_TAG => {
                let tag = triseratops::tag::Markers2::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            triseratops::tag::Overview::ID3_TAG => {
                let tag = triseratops::tag::Overview::parse_id3(&data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            _ => (),
        }
    }

    println!();
    println!("Merged values");

    println!("  Auto Gain: {:?}", container.auto_gain());
    println!("  Gain DB: {:?}", container.gain_db());

    println!("  Cues");
    let output = format!("{:#?}", container.cues());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  Loops");
    let output = format!("{:#?}", container.loops());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  Track Color");
    let output = format!("{:#?}", container.track_color());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  BPM Locked");
    let output = format!("{:?}", container.bpm_locked());
    println!("{}", textwrap::indent(&output, "    "));

    Ok(())
}
