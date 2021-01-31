extern crate seratodj;

use seratodj::library::filesystem;
use std::path::PathBuf;

#[test]
fn library_dir_detection() {
    let music_dir = PathBuf::from("tests/data/library/usb_drive");
    let info = filesystem::get_library(&music_dir).unwrap();
    assert_eq!(
        info,
        filesystem::SeratoLibraryInfo {
            path: PathBuf::from("tests/data/library/usb_drive"),
            database_path: PathBuf::from("tests/data/library/usb_drive/_Serato_/database V2"),
            crates: vec![
                (
                    String::from("80s Mashup"),
                    PathBuf::from(
                        "tests/data/library/usb_drive/_Serato_/Subcrates/80s Mashup.crate"
                    )
                ),
                (
                    String::from("French House"),
                    PathBuf::from(
                        "tests/data/library/usb_drive/_Serato_/Subcrates/French House.crate"
                    )
                ),
            ],
        }
    );
}
