extern crate serato_tags;

use serato_tags::library::filesystem;
use std::path::PathBuf;

#[test]
fn library_dir_detection() {
    let music_dir = PathBuf::from("tests/data/library");
    let db_path = filesystem::get_serato_database(&music_dir).unwrap();
    assert_eq!(
        db_path,
        PathBuf::from("tests/data/library/_Serato_/database V2")
    );
}
