use std::path::PathBuf;

pub fn get_serato_database(dir: &PathBuf) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }

    let mut path = dir.join("_Serato_");
    if !path.exists() {
        return None;
    }

    path.push("database V2");
    if !path.exists() {
        return None;
    }

    Some(path)
}
