#[macro_use]
extern crate serde;

use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path
};
use string_cache::DefaultAtom as Atom;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not supported format of search-index.json")]
    InvalidFormat(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error)
}

// TODO: data parallel
// TODO: CRLF
// one crate per one line
pub fn read_search_index<P: AsRef<Path>>(src: P) -> Result<Vec<(String, doc::Crate)>, Error> {
    let file = File::open(src.as_ref())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    lines.next(); // remove first line
    lines
        .map(|l| l.map_err(Error::from))
        .filter(|l| {
            if let Ok(l) = &l {
                l != "}');" && l != "initSearch(searchIndex);"
            } else {
                true
            }
        })
        .map(|l| l.and_then(parse_line))
        .collect()
}

fn parse_line(line: String) -> Result<(String, doc::Crate), Error> {
    let mut line = {
        let mut line = line;
        line.pop(); // last backslash
        if &line[(line.len() - 1)..] == "," {
            line.pop(); // last commma
        }
        line
    };
    let colon_idx = line
        .find(':')
        .ok_or_else(|| Error::InvalidFormat(line.clone()))?;
    let (mut name_colon, mut body) = {
        let body = line.split_off(colon_idx + 1);
        (line, body)
    };
    let mut quoted_name = {
        let _colon = name_colon.split_off(colon_idx);
        name_colon
    };
    let name = {
        quoted_name.pop();
        quoted_name.split_off(1)
    };
    println!("{}", &name);
    let body = unescape::unescape(&body).ok_or_else(|| Error::InvalidFormat(body.clone()))?;
    let krate: doc::Crate = serde_json::from_str(&body).unwrap();
    Ok((name, krate))
}

// pub fn fix_json<S: AsRef<str>>(json: S) -> String {
//    let mut is_escape = false;
//    let mut is_string = false;
//    let mut buffer = String::with_capacity(1024);

//    for chr in json.as_ref().chars() {
//        match chr {
//            'N' if !is_string => {
//                buffer.push_str("null");
//                continue;
//            }
//            '"' if !is_escape => is_string = !is_string,
//            '\\' if !is_escape => is_escape = true,
//            _ => is_escape = false
//        };
//        buffer.push(chr);
//    }

//    buffer
//}

pub mod doc {
    use string_cache::DefaultAtom as Atom;

    #[derive(Debug, Deserialize)]
    pub struct Crate {
        // t, n, q, d, f, i, p
        //#[serde(rename = "i")]
        // items: Vec<IndexItem>,
        //#[serde(rename = "p")]
        // paths: Vec<Parent>
        doc: Atom
    }
}
