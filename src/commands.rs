use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use colored::Colorize;
use std::io::prelude::*;
use std::{
    fs::{self, OpenOptions},
    io,
    path::Path,
};

use crate::api::ArxivQuery;
use crate::model::Paper;

pub async fn search_command(search_query: String, sort_by: String) -> Result<()> {
    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_search_query(&search_query);
    query.set_sort_by(&sort_by);

    let mut feed = query.run().await?;

    // Display papers matching query
    println!(
        "Displaying {} papers matching the query \"{}\"\n",
        feed.papers.len(),
        &search_query
    );
    for (index, paper) in feed.papers.iter_mut().enumerate() {
        paper.summary.truncate(512);
        paper.summary += "...";

        println!("{} {}", index.to_string().blue(), paper.title.green());
        println!("> {}", paper.id.cyan());
        println!(
            "> Published: {}, Updated: {}",
            paper.published.yellow(),
            paper.updated.yellow()
        );
        println!("{}", "----- Abstract -----".blue());
        println!("{}\n", paper.summary);
    }

    // Select paper
    println!(
        "Enter ID of paper to download 0-{}: ",
        feed.papers.len() - 1
    );
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let paper_index: usize = line.trim().parse()?;

    if let Some(paper) = feed.papers.get(paper_index) {
        select_tag_and_download(paper).await?;
    }

    Ok(())
}

pub async fn download_command(arxiv_id: String) -> Result<()> {
    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_id_list(&arxiv_id);

    let feed = query.run().await?;

    if let Some(paper) = feed.papers.get(0) {
        select_tag_and_download(paper).await?;
    }

    Ok(())
}

async fn select_tag_and_download(paper: &Paper) -> Result<()> {
    // Create list of existing tags
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

    // Select tag
    println!("Enter paper tag, existing tags: {:?}", available_tags);
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let tag_dir = line.trim();

    fs::create_dir_all(tag_dir)?;
    paper.download_pdf(tag_dir).await?;

    let mut md_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new(tag_dir).join(Path::new("paper_index.md")))?;

    write!(md_file, "{}\n{}\n\n", paper.title, paper.id)?;

    Ok(())
}
