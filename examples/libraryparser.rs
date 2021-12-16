use std::env;
use std::string::String;
use triseratops::library::Library;

fn main() -> Result<(), triseratops::error::Error> {
    let mut args: Vec<String> = env::args().collect();
    let _prog = args.remove(0);

    if args.len() != 1 {
        panic!("Expected exactly 1 argument!")
    }
    let path = args.remove(0);

    let library = Library::read_from_path(path)?;
    let tracks: Vec<_> = library.tracks().collect();
    println!("Library ({} tracks)", tracks.len());
    println!("{:#?}", tracks);
    let subcrates = library.subcrates();
    for subcrate in subcrates {
        let subcrate_tracks: Vec<_> = library.subcrate(&subcrate)?.collect();
        println!();
        print!("Subcrate: {} ({} tracks)", &subcrate, subcrate_tracks.len());
        println!("{:#?}", subcrate_tracks);
    }

    Ok(())
}
