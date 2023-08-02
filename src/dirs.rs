use std::fs::{self, DirEntry};
use std::io;

pub fn get_dirs(path: &str) -> io::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?;
    let dirs: Vec<DirEntry> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.metadata().map(|m| m.is_dir()).unwrap_or(false))
        .collect();

    Ok(dirs)
}

pub fn has_git_dir(dir: &DirEntry) -> bool {
    dir.path().join(".git").is_dir()
}

pub fn get_git_dirs(path: &str) -> io::Result<Vec<DirEntry>> {
    let dirs = get_dirs(path)?;
    let git_dirs: Vec<DirEntry> = dirs.into_iter()
        .filter(has_git_dir)
        .collect();

    Ok(git_dirs)
}
