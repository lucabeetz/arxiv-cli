use std::fs;
use std::io::Write;

use color_eyre::eyre::Result;
use reqwest;

pub async fn download_pdf(arxiv_id: &str, out_path: &str) -> Result<()> {
    let pdf_url = format!("http://de.arxiv.org/pdf/{}.pdf", arxiv_id);
    let body = reqwest::get(pdf_url).await?.bytes().await?;

    let mut file = fs::File::create(out_path)?;
    file.write_all(&body)?;

    Ok(())
}
