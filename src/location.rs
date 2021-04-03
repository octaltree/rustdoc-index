use crate::{
    doc::{ItemType, ParseItemTypeError, FILETYPE},
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

pub const STD_CRATES: &[&str] = &["alloc", "core", "proc_macro", "std", "test"];

pub async fn location_from_line(line: &str) -> Result<String, Error> {
    let (path_components, ty) = parse_line(line)?;
    let (krate_name, tail): (_, &[&str]) = split_krate(&path_components)?;
    let search_index: PathBuf = find_search_index(krate_name)?;
    find(&search_index, krate_name, tail, ty)
}

fn find(
    search_index: &Path,
    krate_name: &str,
    tail: &[&str],
    ty: ItemType
) -> Result<String, Error> {
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
    let search_index: PathBuf = if is_std_krate(krate_name) {
        crate::search_index::find_std()
    } else {
        crate::search_index::find_local()
    }?
    .ok_or(LocationError::DocNotFound)?;
    Ok(search_index)
}

#[inline]
fn is_std_krate(name: &str) -> bool { STD_CRATES.iter().any(|c| *c == name) }

fn cd_krate_dir(doc_dir: &Path, krate_name: &str) -> Result<PathBuf, LocationError> {
    let krate_dir: PathBuf = Some(doc_dir.join(krate_name))
        .filter(|p| p.is_dir())
        .ok_or(LocationError::DocNotFound)?;
    Ok(krate_dir)
}

// TODO: conflicted primitive
fn find_file<'a, 'b>(
    dir: &Path,
    path_components: &'a [&'b str]
) -> Option<(PathBuf, &'a [&'b str])> {
    let (cd, mut rest) = step_into_module(dir, path_components);
    if rest.is_empty() {
        return Some((cd.join("index.html"), rest));
    }
    let top = rest[0];
    rest = &rest[1..];
    let found = FILETYPE
        .iter()
        .map(|ty| cd.join(format!("{}.{}.html", ty.as_str(), top)))
        .find(|p| p.is_file())?;
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

fn item_url(file: &Path, rest: &[&str], ty: ItemType) -> String {
    if let Some(id) = item_id(rest, ty) {
        format!("file://{}#{}", file.display(), id)
    } else {
        format!("file://{}", file.display())
    }
}

fn item_id(rest: &[&str], ty: ItemType) -> Option<String> {
    if rest.is_empty() {
        return None;
    }
    println!("{:?} {:?}", rest, ty);
    if rest.len() == 1 {
        return if ty == ItemType::StructField && rest[0].parse::<i32>().is_ok() {
            None
        } else {
            Some(format!("{}.{}", ty.as_str(), rest[0]))
        };
    }
    if rest.len() == 2 {
        if ty == ItemType::StructField {
            return if rest[1].parse::<i32>().is_ok() {
                Some(format!("variant.{}", rest[0]))
            } else {
                Some(format!("variant.{}.field.{}", rest[0], rest[1]))
            };
        }
        // TODO: unknown
        return Some(format!("{}.{}", ty.as_str(), rest[1]));
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Lines},
        path::PathBuf,
        process::{Child, ChildStdout, Command, Stdio}
    };

    #[tokio::test]
    async fn file_exists_for_every_line() {
        let mut source = source();
        let search_indexes = crate::search_index::search_indexes().await.unwrap();
        for line in list(&mut source) {
            let line = line.unwrap();
            file_exists_for_every_line_impl(&search_indexes, &line, false);
            // file_exists_for_every_line_impl(&search_indexes, &line, true);
        }
    }

    #[tokio::test]
    async fn item_exists_for_every_line() {
        let mut source = source();
        let search_indexes = crate::search_index::search_indexes().await.unwrap();
        for line in list(&mut source) {
            let line = line.unwrap();
            file_exists_for_every_line_impl(&search_indexes, &line, true);
        }
    }

    fn source() -> Child {
        let child = Command::new("./target/debug/cargo-listdoc")
            .args(&["listdoc", "show"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
    }

    fn list(child: &mut Child) -> Lines<BufReader<ChildStdout>> {
        let stdout = child.stdout.take().unwrap();
        BufReader::new(stdout).lines()
    }

    fn file_exists_for_every_line_impl(search_indexes: &[PathBuf], line: &str, check_item: bool) {
        let (path_components, ty) = parse_line(line).unwrap();
        let (krate_name, tail): (_, &[&str]) = split_krate(&path_components).unwrap();
        if is_std_krate(krate_name) {
            return;
        }
        let maybe_file = search_indexes
            .iter()
            .find_map(|s| find(s, krate_name, tail, ty).ok());
        let file = match maybe_file {
            None => panic!("Not found {}", line),
            Some(x) => x
        };
        if check_item {
            item_exists(&file);
        }
    }

    fn item_exists(url: &str) {
        let idx = match url.find('#') {
            Some(x) => x,
            None => return
        };
        println!("{}", url);
        let (file, item) = url.split_at(idx);
        let item = &item[1..];
        let file = file.strip_prefix("file://").unwrap();
        let id = format!(r#"id="{}""#, item);
        let contents = std::fs::read_to_string(file).unwrap();
        if !contents.contains(&id) {
            panic!("Not found {} in {}", id, file);
        }
    }

    // fn lines(s: String) -> Vec<String> {
    //    let mut lines = Vec::new();
    //    let mut buf = s;
    //    while let Some(idx) = buf.find('\n') {
    //        let new = buf.split_off(idx);
    //        lines.push(buf);
    //        buf = new;
    //    }
    //    if !buf.is_empty() {
    //        lines.push(buf);
    //    }
    //    lines
    //}
}
