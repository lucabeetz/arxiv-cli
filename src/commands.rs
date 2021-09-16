use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use colored::Colorize;
use std::io;

use crate::api::ArxivQuery;
use crate::utils::select_tag_and_download;

pub async fn search_command(search_query: String, sort_by: String) -> Result<()> {
    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_search_query(&search_query);
    query.set_sort_by(&sort_by);

    let mut feed = query.run().await?;

    if feed.papers.is_empty() {
        println!("No papers found matching query \"{}\"", search_query);
        return Ok(());
    }

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

    match feed.papers.get(0) {
        Some(paper) => select_tag_and_download(paper).await?,
        _ => eprintln!("No paper found with id: {}", arxiv_id),
    }

    Ok(())
}
