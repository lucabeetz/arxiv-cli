mod api;

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

    let out_path = format!("{}.pdf", &args.arxiv_id);
    api::download_pdf(&args.arxiv_id, &out_path).await?;

    Ok(())
}
