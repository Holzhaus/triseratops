//! High-level interface for parsing Serato libraries.
//!
//! ```
//! use std::path::PathBuf;
//! use triseratops::library::{Library, Track};
//!
//! fn read_library(music_dir: &PathBuf) {
//!     let library = Library::read_from_path(music_dir).unwrap();
//!     for subcrate_name in library.subcrates() {
//!         let tracks: Vec<&Track> = library.subcrate(&subcrate_name).unwrap().collect();
//!         println!("Subcrate '{}': {} tracks", subcrate_name, tracks.len());
//!     }
//! }
//! ```

use super::database;
use crate::error::Error;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

/// File name of the main database file
const DATABASE_FILENAME: &str = "database V2";
/// File extension of the crate database files
const CRATE_EXTENSION: &str = "crate";
/// Name of the Serato directory inside the library directory
const SERATO_DIR: &str = "_Serato_";
/// Name of the directory containing subcrates inside the Serato directory
const SUBCRATE_DIR: &str = "Subcrates";

#[derive(Clone, Debug)]
pub struct Track {
    pub file_path: PathBuf,
    pub file_type: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub comment: Option<String>,
    pub composer: Option<String>,
    pub grouping: Option<String>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub missing: bool,
    pub beatgrid_locked: bool,
}

impl Track {
    /// Creates a new, empty Track object.
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::new(),
            file_type: None,
            title: None,
            artist: None,
            album: None,
            genre: None,
            comment: None,
            composer: None,
            grouping: None,
            label: None,
            key: None,
            missing: false,
            beatgrid_locked: false,
        }
    }

    /// Creates a new Track object from a list of database fields.
    pub fn from_fields(fields: Vec<database::Field>) -> Result<Self, Error> {
        let mut track = Self::new();
        for field in fields {
            match field {
                database::Field::FilePath(file_path) => {
                    track.file_path = file_path;
                }
                database::Field::FileType(file_type) => {
                    track.file_type = Some(file_type);
                }
                database::Field::Album(album) => {
                    track.album = Some(album);
                }
                database::Field::Artist(artist) => {
                    track.artist = Some(artist);
                }
                database::Field::Comment(comment) => {
                    track.comment = Some(comment);
                }
                database::Field::Composer(composer) => {
                    track.composer = Some(composer);
                }
                database::Field::Grouping(grouping) => {
                    track.grouping = Some(grouping);
                }
                database::Field::Label(label) => {
                    track.label = Some(label);
                }
                database::Field::Key(key) => {
                    track.key = Some(key);
                }
                database::Field::Missing(missing) => {
                    track.missing = missing;
                }
                database::Field::BeatgridLocked(beatgrid_lock) => {
                    track.beatgrid_locked = beatgrid_lock;
                }
                _ => (),
            }
        }

        Ok(track)
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

/// DAO that reads Serato libraries from the file system.
pub struct Library {
    path: PathBuf,
    tracks: HashMap<PathBuf, Track>,
}

impl Library {
    /// Read the library in the given path.
    pub fn read_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = fs::canonicalize(path)?;
        let tracks = HashMap::new();
        let mut library = Library { path, tracks };
        library.reload()?;

        Ok(library)
    }

    fn serato_path(&self) -> PathBuf {
        self.path.join(SERATO_DIR)
    }

    /// Reload the library database from the hard disk.
    pub fn reload(&mut self) -> Result<(), Error> {
        let database_path = self.serato_path().join(DATABASE_FILENAME);
        let mut file = BufReader::new(File::open(database_path)?);
        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let mut tracks = HashMap::new();
        let fields = database::parse(&data)?;
        for field in fields {
            if let database::Field::Track(t) = field {
                let track = Track::from_fields(t)?;
                tracks.insert(track.file_path.clone(), track);
            }
        }
        self.tracks = tracks;

        Ok(())
    }

    /// Get all tracks in the library.
    pub fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.tracks.values()
    }

    /// Get the track struct for the given path.
    pub fn track(&self, file_path: &PathBuf) -> Option<&Track> {
        self.tracks.get(file_path)
    }

    /// Get a list of subcrate names.
    pub fn subcrates(&self) -> impl Iterator<Item = String> {
        let crates_path = self.serato_path().join(SUBCRATE_DIR);
        crates_path
            .read_dir()
            .into_iter()
            .flatten()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter_map(|x| crate_name_from_path(&x).ok())
    }

    /// Get a list of tracks from the subcrate with the given name.
    pub fn subcrate(&self, name: &str) -> Result<impl Iterator<Item = &Track>, Error> {
        let filename = format!("{}.{}", name, CRATE_EXTENSION);
        let crate_path = self.serato_path().join(SUBCRATE_DIR).join(filename);
        let mut file = BufReader::new(File::open(crate_path)?);
        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let fields = database::parse(&data)?;
        let tracks = fields.into_iter().filter_map(move |field| {
            if let database::Field::Track(track_fields) = field {
                for track_field in track_fields {
                    if let database::Field::TrackPath(path) = track_field {
                        return self.track(&path);
                    }
                }
            }

            None
        });
        Ok(tracks)
    }
}

fn crate_name_from_path(path: &PathBuf) -> Result<String, Error> {
    if !path.is_file() {
        return Err(Error::IOError(io::Error::new(
            io::ErrorKind::Other,
            "crate path is not a file",
        )));
    }

    if let Some(ext) = path.extension() {
        if ext != CRATE_EXTENSION {
            return Err(Error::IOError(io::Error::new(
                io::ErrorKind::Other,
                "crate path has no .crate extension",
            )));
        }
        if let Some(crate_name_osstr) = path.file_stem() {
            if let Some(crate_name) = crate_name_osstr.to_str() {
                return Ok(crate_name.to_string());
            }
        }
    }

    Err(Error::IOError(io::Error::new(
        io::ErrorKind::Other,
        "Failed to create crate name",
    )))
}
