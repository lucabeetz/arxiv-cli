use color_eyre::eyre::Result;
use colored::Colorize;

use crate::api::ArxivQuery;

pub async fn search_command(search_query: String, sort_by: String) -> Result<()> {
    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_search_query(&search_query);
    query.set_sort_by(&sort_by);

    let mut feed = query.run().await?;

    println!(
        "Displaying {} papers matching the query '{}'\n",
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

    Ok(())
}
