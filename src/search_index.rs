use crate::Error;
use std::{
    path::{Path, PathBuf},
    process::Command
};

/// Finds not recursive
pub fn ls_search_index(dir: &Path) -> Result<Option<PathBuf>, Error> {
    let search_index: Option<_> = dir.read_dir()?.find_map(|e| -> Option<_> {
        let e = e.ok()?;
        let name = e.file_name().into_string().ok()?;
        name.starts_with("search-index").then(|| e.path())
    });
    Ok(search_index)
}

pub fn find_std() -> Result<Option<PathBuf>, Error> {
    let output = Command::new("rustup").args(&["doc", "--path"]).output()?;
    let out = unsafe { String::from_utf8_unchecked(output.stdout) };
    let file = PathBuf::from(out);
    let dir = file.parent().unwrap();
    ls_search_index(dir)
}

pub fn find_local() -> Result<Option<PathBuf>, Error> {
    let meta = match metadata() {
        Ok(x) => x,
        Err(_) => return Ok(None)
    };
    let dir = meta.target_directory.join_os("doc");
    if !dir.is_dir() {
        return Ok(None);
    }
    ls_search_index(&dir)
}

pub async fn search_indexes() -> Result<Vec<PathBuf>, Error> {
    let async_find_std = tokio::spawn(async { find_std() });
    let async_find_local = tokio::spawn(async { find_local() });
    let (std, local) = tokio::join!(async_find_std, async_find_local);
    let mut res = Vec::with_capacity(2);
    if let Some(std) = std?? {
        res.push(std);
    }
    if let Some(local) = local?? {
        res.push(local);
    }
    Ok(res)
}

fn metadata() -> Result<cargo_metadata::Metadata, Error> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    cmd.other_options(vec![String::from("--offline")]);
    Ok(cmd.exec()?)
}
