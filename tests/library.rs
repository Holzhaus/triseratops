extern crate triseratops;

use triseratops::library::Library;

#[test]
fn test_library() {
    let library = Library::read_from_path("tests/data/library/usb_drive").unwrap();

    assert_eq!(4, library.tracks().count());
    assert_eq!(2, library.subcrates().count());

    let subcrate_tracks = library.subcrate("80s Mashup").unwrap();
    assert_eq!(1, subcrate_tracks.count());

    let subcrate_tracks = library.subcrate("French House").unwrap();
    assert_eq!(2, subcrate_tracks.count());
}
