use crate::model::Paper;
use color_eyre::eyre::Result;
use std::io::prelude::*;
use std::{
    fs::{self, OpenOptions},
    io,
    path::Path,
};

pub async fn select_tag_and_download(paper: &Paper) -> Result<()> {
    // Create list of existing tags by reading directory names in current directory
    let mut available_tags: Vec<String> = Vec::new();
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .expect("Unable to parse file name")
                .to_str()
                .expect("Unable to parse OsStr to str")
                .to_string();

            available_tags.push(dir_name);
        }
    }

    // Display existing tags and ask for desired tag
    println!(
        "Enter paper tag, existing tags: {}",
        available_tags.join(", ")
    );
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let tag_dir = line.trim();

    // Download paper
    fs::create_dir_all(tag_dir)?;
    paper.download_pdf(tag_dir).await?;

    // Append paper info to markdown file
    let mut md_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new(tag_dir).join(Path::new("paper_index.md")))?;

    write!(md_file, "{}\n{}\n\n", paper.title, paper.id)?;

    Ok(())
}
