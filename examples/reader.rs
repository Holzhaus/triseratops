use id3::Tag;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::str;
use std::string::String;

fn read_str(reader: &mut dyn BufRead) -> Result<String, Error> {
    let mut value = vec![];

    reader.read_until(0, &mut value)?;
    let s = str::from_utf8(&value[..value.len() - 1])
        .map_err(|_err| Error::new(ErrorKind::Other, "Failed to decode string"));
    Ok(s.unwrap().to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut container = serato_tags::container::Container::new();

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
            "Serato Analysis" => {
                container.parse_analysis(&data);
                let output = format!("{:#?}", serato_tags::analysis::parse(&data).unwrap());
                println!("{}", textwrap::indent(&output, "    "));
            }
            "Serato Autotags" => {
                container.parse_autotags(&data);
                let output = format!("{:#?}", serato_tags::autotags::parse(&data).unwrap());
                println!("{}", textwrap::indent(&output, "    "));
            }
            "Serato BeatGrid" => {
                container.parse_beatgrid(&data);
                let output = format!("{:#?}", serato_tags::beatgrid::parse(&data).unwrap());
                println!("{}", textwrap::indent(&output, "    "));
            }
            "Serato Markers_" => {
                container.parse_markers(&data);
                let output = format!("{:#?}", serato_tags::markers::parse(&data).unwrap());
                println!("{}", textwrap::indent(&output, "    "));
            }
            "Serato Markers2" => {
                container.parse_markers2(&data);
                let output = format!("{:#?}", serato_tags::markers2::parse(&data).unwrap());
                println!("{}", textwrap::indent(&output, "    "));
            }
            "Serato Overview" => {
                container.parse_overview(&data);
                let output = format!("{:?}", serato_tags::overview::parse(&data).unwrap());
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
}
