use super::database;
use crate::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub struct Track {
    pub file_path: PathBuf,
    pub file_type: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub comment: String,
    pub grouping: String,
    pub label: String,
    pub key: String,
    pub missing: bool,
    pub beatgrid_locked: bool,
}

pub struct Library {
    path: PathBuf,
    fields: Vec<database::Field>,
}

impl Library {
    pub fn read_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        let fields = vec![];
        let mut library = Library { path, fields };
        library.reload()?;

        Ok(library)
    }

    pub fn reload(&mut self) -> Result<(), Error> {
        let serato_path = self.path.join("_Serato_");
        let database_path = serato_path.join("database V2");
        let mut file = BufReader::new(File::open(database_path)?);
        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let fields = database::parse(&data)?;
        self.fields = fields;

        Ok(())
    }
}
