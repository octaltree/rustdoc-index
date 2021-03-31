use rustdoc_index::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let async_find_std = tokio::spawn(async { search_index::find_std() });
    let async_find_local = tokio::spawn(async { search_index::find_local() });
    let (std, local) = tokio::join!(async_find_std, async_find_local);
    if let Some(std) = std?? {
        read_search_index_and_show(std)?;
    }
    if let Some(local) = local?? {
        read_search_index_and_show(local)?;
    }
    Ok(())
}
