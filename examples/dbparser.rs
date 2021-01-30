use serato_tags::library::database;
use std::env;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() -> Result<(), serato_tags::error::Error> {
    let mut args: Vec<String> = env::args().collect();
    let _prog = args.remove(0);

    if args.len() != 1 {
        panic!("Expected exactly 1 argument!")
    }
    let filename = args.remove(0);

    let mut file = File::open(&filename).expect("Failed to open file!");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("Failed to read data!");

    let fields = database::parse(&data)?;
    println!("{:#?}", fields);
    Ok(())
}
