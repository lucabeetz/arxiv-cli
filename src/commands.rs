use color_eyre::eyre::Result;

use crate::api::ArxivQuery;

pub async fn search_command(search_query: String, sort_by: String) -> Result<()> {
    let mut query = ArxivQuery::new(String::from("http://export.arxiv.org/api/query?"));
    query.set_search_query(&search_query);
    query.set_sort_by(&sort_by);

    let feed = query.run().await?;

    println!(
        "Displaying {} papers matching the query '{}'\n",
        feed.papers.len(),
        &search_query
    );
    for (index, paper) in feed.papers.iter().enumerate() {
        println!("[{}] {}", index, paper.title);
        println!("-----> {}", paper.id);
        println!("{}\n", paper.summary);
    }

    Ok(())
}
