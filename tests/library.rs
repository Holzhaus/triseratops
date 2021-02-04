extern crate triseratops;

use triseratops::library::{Library, Track};

#[test]
fn test_library() {
    let library = Library::read_from_path("tests/data/library/usb_drive").unwrap();
    let tracks: Vec<&Track> = library.tracks().collect();
    assert_eq!(tracks.len(), 4);

    let tracks = library.subcrate("80s Mashup".to_string()).unwrap();
    assert_eq!(tracks.len(), 1);

    let tracks = library.subcrate("French House".to_string()).unwrap();
    assert_eq!(tracks.len(), 2);
}
