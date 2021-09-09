mod api;
mod model;

use api::ArxivQuery;
use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    arxiv_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::from_args();

    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_id_list(args.arxiv_id);

    let feed = query.run().await?;

    for paper in feed.papers {
        paper.download_pdf("papers").await?;
    }

    Ok(())
}
