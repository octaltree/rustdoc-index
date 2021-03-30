use rustdoc_index::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let std =  read_search_index_and_show("/home/octaltree/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/search-index1.53.0.js");
    let local =
        read_search_index_and_show("/home/octaltree/workspace/fust/target/doc/search-index.js");
    let (a, b) = tokio::join!(std, local);
    a?;
    b?;
    Ok(())
}
