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
    ItemNotFound
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

pub async fn location_from_line(line: &str) -> Result<Option<String>, Error> {
    let (fst, snd) = {
        let mut a = line.split_whitespace();
        a.next()
            .and_then(|fst| a.next().map(|snd| (fst, snd)))
            .ok_or(LocationError::InvalidFormat)
    }?;
    let ty = ItemType::from_str(snd).map_err(LocationError::from)?;
    let path_components = fst.split("::").collect::<Vec<_>>();
    let krate_name = *path_components.get(0).ok_or(LocationError::InvalidFormat)?;
    let search_indexes = crate::search_index::search_indexes().await?;
    let doc_dirs: Vec<_> = search_indexes
        .iter()
        .map(|file| file.parent().unwrap())
        .collect();
    let doc_dir = match doc_dirs.iter().find(|dir| dir.join(krate_name).is_dir()) {
        Some(p) => *p,
        None => return Ok(None)
    };
    let (file, rest) = find_file(doc_dir, &path_components).ok_or(LocationError::FileNotFound)?;
    let url = item_url(&file, rest, ty);
    Ok(Some(url))
}

fn item_url(file: &Path, rest: &[&str], ty: ItemType) -> String {
    if rest.is_empty() {
        format!("file://{}", file.display())
    } else {
        let top = rest[0];
        format!("file://{}#{}.{}", file.display(), ty.as_str(), top)
    }
}

// TODO: escaping??
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
