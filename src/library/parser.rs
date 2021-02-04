use super::database;
use crate::error::Error;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::{Path, PathBuf};

const DATABASE_FILENAME: &str = "database V2";
const CRATE_EXTENSION: &str = ".crate";
const SERATO_DIR: &str = "_Serato_";
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

pub struct Library {
    path: PathBuf,
    tracks: HashMap<PathBuf, Track>,
}

impl Library {
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

    pub fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.tracks.values()
    }

    pub fn track(&self, file_path: &PathBuf) -> Option<&Track> {
        self.tracks.get(file_path)
    }

    pub fn subcrates(&self) -> Vec<String> {
        let crates_path = self.serato_path().join(SUBCRATE_DIR);
        let mut crates = vec![];
        if let Ok(entries) = crates_path.read_dir() {
            for entry in entries {
                let entry = match entry.ok() {
                    Some(x) => x,
                    None => {
                        continue;
                    }
                };

                let crate_path = entry.path();
                if !crate_path.is_file() {
                    continue;
                }

                if let Some(ext) = crate_path.extension() {
                    if ext != CRATE_EXTENSION {
                        continue;
                    }
                    if let Some(crate_name_osstr) = crate_path.file_stem() {
                        if let Some(crate_name) = crate_name_osstr.to_str() {
                            crates.push(crate_name.to_string());
                        }
                    }
                }
            }
        }

        crates
    }

    pub fn subcrate(&self, name: String) -> Result<Vec<&Track>, Error> {
        let filename = name.add(CRATE_EXTENSION);
        let crate_path = self.serato_path().join(SUBCRATE_DIR).join(filename);
        let mut file = BufReader::new(File::open(crate_path)?);
        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let mut tracks = vec![];

        let fields = database::parse(&data)?;
        for field in fields {
            if let database::Field::Track(track_fields) = field {
                for track_field in track_fields {
                    if let database::Field::TrackPath(path) = track_field {
                        if let Some(track) = self.track(&path) {
                            tracks.push(track);
                        }
                    }
                }
            }
        }

        Ok(tracks)
    }
}
