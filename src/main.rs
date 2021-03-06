mod api;
mod commands;
mod model;
mod utils;

use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Arxiv CLI to search and download papers")]
enum ArxivCli {
    Download {
        /// ID of arxiv paper to download
        arxiv_id: String,
    },
    Search {
        /// Search query
        search_query: String,

        /// Sort order of returned papers
        #[structopt(default_value = "submittedDate", long)]
        sort_by: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = ArxivCli::from_args();

    match args {
        ArxivCli::Download { arxiv_id } => commands::download_command(arxiv_id).await?,
        ArxivCli::Search {
            search_query,
            sort_by,
        } => commands::search_command(search_query, sort_by).await?,
    }

    Ok(())
}
