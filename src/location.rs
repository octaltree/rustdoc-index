use crate::{
    doc::{ItemType, ParseItemTypeError},
    Error
};
use std::{
    path::{Path, PathBuf},
    str::FromStr
};

#[derive(Debug, Error)]
pub enum LocationError {
    #[error("Invalid format")]
    InvalidFormat,
    #[error(transparent)]
    ParseItemTypeError(#[from] ParseItemTypeError),
    #[error("File not found")]
    FileNotFound,
    #[error("Item not found")]
    ItemNotFound,
    #[error("Doc dir not found")]
    DocNotFound
}

pub const FILETYPE: &[ItemType] = &[
    ItemType::Struct,
    ItemType::Union,
    ItemType::Enum,
    ItemType::Function,
    ItemType::Typedef,
    ItemType::Static,
    ItemType::Trait,
    ItemType::Macro,
    ItemType::Primitive,
    ItemType::Constant,
    ItemType::Keyword,
    ItemType::ProcAttribute,
    ItemType::ProcDerive
];

pub const STD_CRATES: &[&str] = &["alloc", "core", "proc_macro", "std", "test"];

pub async fn location_from_line(line: &str) -> Result<String, Error> {
    let (path_components, ty) = parse_line(line)?;
    let (krate_name, tail): (_, &[&str]) = split_krate(&path_components)?;
    let search_index: PathBuf = find_search_index(krate_name)?;
    let doc_dir: &Path = search_index.parent().unwrap();
    let krate_dir: PathBuf = cd_krate_dir(doc_dir, krate_name)?;
    let (file, rest) = find_file(&krate_dir, tail).ok_or(LocationError::FileNotFound)?;
    let url = item_url(&file, rest, ty);
    Ok(url)
}

fn parse_line(line: &str) -> Result<(Vec<&str>, ItemType), Error> {
    let (fst, snd) = {
        let mut a = line.split_whitespace();
        a.next()
            .and_then(|fst| a.next().map(|snd| (fst, snd)))
            .ok_or(LocationError::InvalidFormat)
    }?;
    let ty = ItemType::from_str(snd).map_err(LocationError::from)?;
    let path_components = fst.split("::").collect::<Vec<_>>();
    Ok((path_components, ty))
}

fn split_krate<'a, 'b>(
    path_components: &'a [&'b str]
) -> Result<(&'b str, &'a [&'b str]), LocationError> {
    let krate_name = *path_components.get(0).ok_or(LocationError::InvalidFormat)?;
    Ok((krate_name, &path_components[1..]))
}

fn find_search_index(krate_name: &str) -> Result<PathBuf, Error> {
    let search_index: PathBuf = if is_std_crates(krate_name) {
        crate::search_index::find_std()
    } else {
        crate::search_index::find_local()
    }?
    .ok_or(LocationError::DocNotFound)?;
    Ok(search_index)
}

#[inline]
fn is_std_crates(name: &str) -> bool { STD_CRATES.iter().any(|c| *c == name) }

fn cd_krate_dir(doc_dir: &Path, krate_name: &str) -> Result<PathBuf, LocationError> {
    let krate_dir: PathBuf = Some(doc_dir.join(krate_name))
        .filter(|p| p.is_dir())
        .ok_or(LocationError::DocNotFound)?;
    Ok(krate_dir)
}

fn item_url(file: &Path, rest: &[&str], ty: ItemType) -> String {
    if rest.is_empty() {
        format!("file://{}", file.display())
    } else {
        let top = rest[0];
        format!("file://{}#{}.{}", file.display(), ty.as_str(), top)
    }
}

// TODO: conflicted primitive
// 可能性があるパスを列挙してその中で存在するもの
fn find_file<'a, 'b>(
    dir: &Path,
    path_components: &'a [&'b str]
) -> Option<(PathBuf, &'a [&'b str])> {
    let (cd, mut rest) = step_into_module(dir, path_components);
    if rest.is_empty() {
        return Some((cd.join("index.html"), rest));
    }
    let top = rest[0];
    let found = FILETYPE
        .iter()
        .map(|ty| cd.join(format!("{}.{}.html", ty.as_str(), top)))
        .find(|p| p.is_file())?;
    rest = &rest[1..];
    Some((found, rest))
}

fn step_into_module<'a, 'b>(
    dir: &Path,
    path_components: &'a [&'b str]
) -> (PathBuf, &'a [&'b str]) {
    let mut cd: PathBuf = dir.into();
    let mut rest: &[&str] = path_components;
    while !rest.is_empty() {
        let top = rest[0];
        let attempt = cd.join(top);
        if !attempt.is_dir() {
            break;
        }
        rest = &rest[1..];
        cd = attempt;
    }
    (cd, rest)
}
