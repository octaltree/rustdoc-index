#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub const RUSTDOC_VERSION: &str = "rustdoc 1.53.0-nightly (132b4e5d1 2021-04-13)";
pub mod doc;
pub mod location;
pub mod search_index;

use rayon::prelude::*;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
    path::Path
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not supported format of search-index.json")]
    InvalidFormat(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
    #[error(transparent)]
    Metadata(#[from] cargo_metadata::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Location(#[from] location::LocationError)
}

pub fn read_search_index_and_show<P: AsRef<Path>>(src: P) -> Result<(), Error> {
    // read
    let doc = {
        let file = File::open(src.as_ref())?;
        let reader = BufReader::new(file);
        // one crate per one line
        let mut lines = reader.lines();
        lines.next(); // remove first line
        lines
            .par_bridge()
            .map(|l| l.map_err(Error::from))
            .filter(|l| {
                if let Ok(l) = &l {
                    l != "}');"
                        && l != "initSearch(searchIndex);"
                        && l != "if (window.initSearch) {window.initSearch(searchIndex)};"
                } else {
                    true
                }
            })
            .map(|l: Result<String, Error>| l.and_then(parse_line))
    };
    // show
    {
        doc.try_for_each(|r: Result<(String, doc::Crate), Error>| -> Result<(), _> {
            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            r.and_then(|(_name, krate)| -> Result<_, _> {
                for path in krate.items() {
                    writeln!(out, "{}", path)?;
                }
                Ok(())
            })
        })
        .unwrap();
    }

    Ok(())
}

/// Parses one line `"name":{..},`
pub fn parse_line(line: String) -> Result<(String, doc::Crate), Error> {
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
    let (mut name_colon, body) = {
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
    let body = unescape::unescape(&body).ok_or_else(|| Error::InvalidFormat(body.clone()))?;
    let krate: doc::Crate = serde_json::from_str(&body)?;
    Ok((name, krate))
}
