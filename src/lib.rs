#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub const RUSTFMT_VERSION: &str = "rustfmt 1.4.36-nightly (7de6968 2021-02-07)";

use rayon::prelude::*;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter},
    path::Path
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not supported format of search-index.json")]
    InvalidFormat(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error)
}

pub async fn read_search_index_and_show<P: AsRef<Path>>(src: P) -> Result<(), Error> {
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
                    l != "}');" && l != "initSearch(searchIndex);"
                } else {
                    true
                }
            })
            .map(|l: Result<String, Error>| l.and_then(parse_line))
            .map(|r: Result<_, Error>| r.map(|(_name, krate)| krate.n))
    };
    // show
    {
        // let out = stdout();
        // let mut out = BufWriter::new(out.lock());
        doc.try_for_each(|r: Result<Vec<String>, Error>| -> Result<(), _> {
            r.and_then(|ss| -> Result<_, _> {
                for x in ss {
                    println!("{:?}", x);
                }
                Ok(())
            })
        })?
    }

    Ok(())
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
    println!("{}", &name);
    let krate: doc::Crate = serde_json::from_str(&body).unwrap();
    Ok((name, krate))
}

pub mod doc {
    use string_cache::DefaultAtom as Atom;

    // t, n, q, d, i, f are items array
    #[derive(Debug, Deserialize)]
    pub struct Crate {
        doc: Atom,
        p: Vec<(usize, String)>,

        f: Vec<Option<Types>>,
        t: Vec<usize>,
        pub n: Vec<String>,
        q: Vec<String>,
        d: Vec<String>,
        i: Vec<usize>
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum Types {
        OnlyArgs((Vec<(String, usize)>,)),
        WithResponse(Vec<(String, usize)>, ResponseType)
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum ResponseType {
        Single((String, usize)),
        Complex(Vec<(String, usize)>)
    }
}
