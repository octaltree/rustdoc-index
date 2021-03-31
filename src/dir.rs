use crate::Error;
use std::{
    path::{Path, PathBuf},
    process::Command
};

pub fn find_std() -> Result<Option<PathBuf>, Error> {
    let output = Command::new("rustup").args(&["doc", "--path"]).output()?;
    let out = unsafe { String::from_utf8_unchecked(output.stdout) };
    let file = PathBuf::from(out);
    let dir = file.parent().unwrap();
    find_search_index(dir)
}

/// Not recursive
pub fn find_search_index(dir: &Path) -> Result<Option<PathBuf>, Error> {
    let search_index: Option<_> = dir.read_dir()?.find_map(|e| -> Option<_> {
        let e = e.ok()?;
        let name = e.file_name().into_string().ok()?;
        name.starts_with("search-index").then(|| e.path())
    });
    Ok(search_index)
}

pub fn find_target() {}
