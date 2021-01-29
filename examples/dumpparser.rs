use serato_tags::id3::ID3Tag;
use std::env;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: {} ( --analysis | --autotags | --beatgrid | --markers | --markers2 | --overview ) FILENAME", &args[0]);
        return;
    }

    let flag = &args[1];
    let filename = &args[2];
    let mut file = File::open(filename).expect("Failed to open file!");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("Failed to read data!");

    match &flag[..] {
        "--analysis" => {
            println!(
                "{:#?}",
                serato_tags::analysis::Analysis::parse_id3(&data).unwrap()
            );
        }
        "--autotags" => {
            println!(
                "{:#?}",
                serato_tags::autotags::Autotags::parse_id3(&data).unwrap()
            );
        }
        "--beatgrid" => {
            println!(
                "{:#?}",
                serato_tags::beatgrid::Beatgrid::parse_id3(&data).unwrap()
            );
        }
        "--markers" => {
            println!(
                "{:#?}",
                serato_tags::markers::Markers::parse_id3(&data).unwrap()
            );
        }
        "--markers2" => {
            println!(
                "{:#?}",
                serato_tags::markers2::Markers2::parse_id3(&data).unwrap()
            );
        }
        "--overview" => {
            println!(
                "{:#?}",
                serato_tags::overview::Overview::parse_id3(&data).unwrap()
            );
        }
        _ => {
            panic!("Unknown argument!");
        }
    }
}
