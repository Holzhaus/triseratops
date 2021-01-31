use std::path::PathBuf;

pub type CrateInfo = (String, PathBuf);

#[derive(Debug, PartialEq)]
pub struct SeratoLibraryInfo {
    pub path: PathBuf,
    pub database_path: PathBuf,
    pub crates: Vec<CrateInfo>,
}

pub fn get_library(path: &PathBuf) -> Option<SeratoLibraryInfo> {
    if !path.is_dir() {
        return None;
    }

    let serato_path = path.join("_Serato_");
    if !serato_path.exists() {
        return None;
    }

    let database_path = serato_path.join("database V2");
    if !database_path.exists() {
        return None;
    }

    let crates_path = serato_path.join("Subcrates");
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
                if ext != "crate" {
                    continue;
                }
                if let Some(crate_name_osstr) = crate_path.file_stem() {
                    if let Some(crate_name) = crate_name_osstr.to_str() {
                        crates.push((crate_name.to_string(), crate_path));
                    }
                }
            }
        }
    }

    crates.sort();

    Some(SeratoLibraryInfo {
        path: path.to_path_buf(),
        database_path,
        crates,
    })
}
