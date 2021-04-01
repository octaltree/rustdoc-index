use rustdoc_index::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "listdoc")]
    _void: String,
    #[structopt(subcommand)]
    pub sub: Option<SubCommand>
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Show,
    Location(Location)
}

#[derive(Debug, StructOpt)]
struct Location {
    #[structopt(name = "line")]
    #[structopt(help = "A line of list")]
    line: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt: Opt = Opt::from_args();
    run(opt).await
}

async fn run(opt: Opt) -> Result<(), Error> {
    match opt.sub.unwrap_or(SubCommand::Show) {
        SubCommand::Show => list().await,
        SubCommand::Location(args) => location(args).await
    }
}

async fn list() -> Result<(), Error> {
    for search_index in search_index::search_indexes().await?.into_iter() {
        read_search_index_and_show(search_index)?;
    }
    Ok(())
}

async fn location(args: Location) -> Result<(), Error> {
    let url = location::location_from_line(&args.line).await?;
    if let Some(url) = url {
        println!("{}", url);
    }
    Ok(())
}
