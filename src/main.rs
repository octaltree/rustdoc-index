use rustdoc_index::*;

fn main() -> Result<(), Error> {
    // let cs = read_search_index("/home/octaltree/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/search-index1.53.0.js")?;
    let cs = read_search_index("/home/octaltree/workspace/fust/target/doc/search-index.js")?;
    println!("{:?}", cs);
    Ok(())
}
