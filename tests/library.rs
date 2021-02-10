extern crate triseratops;

use triseratops::library::{Library, Track};

#[test]
fn test_library() {
    let library = Library::read_from_path("tests/data/library/usb_drive").unwrap();
    let tracks: Vec<&Track> = library.tracks().collect();
    assert_eq!(tracks.len(), 4);

    let subcrates: Vec<String> = library.subcrates().collect();
    assert_eq!(subcrates.len(), 2);

    let tracks: Vec<&Track> = library.subcrate("80s Mashup").unwrap().collect();
    assert_eq!(tracks.len(), 1);

    let tracks: Vec<&Track> = library.subcrate("French House").unwrap().collect();
    assert_eq!(tracks.len(), 2);
}
