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
    let s = str::from_utf8(&value[..value.len() - 1]).map_err(|_err| Error::new(ErrorKind::Other, "Failed to decode string"));
    return Ok(s.unwrap().to_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

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
                println!("    {:#?}", serato_tags::analysis::parse(&data).unwrap());
            }
            "Serato Markers_" => {
                println!("    {:#?}", serato_tags::markers::parse(&data).unwrap());
            },
            _ => (),
        }
    }
}
