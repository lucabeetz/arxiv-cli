use std::io::Write;
use std::{fs, path::Path};

use color_eyre::eyre::Result;
use quick_xml;
use reqwest;

use crate::model::{Feed, Paper};

#[derive(Default)]
pub struct ArxivQuery {
    pub base_url: String,
    pub search_query: Option<String>,
    pub id_list: Option<String>,
}

impl ArxivQuery {
    pub fn new(base_url: String) -> Self {
        ArxivQuery {
            base_url,
            ..ArxivQuery::default()
        }
    }

    pub fn set_search_query(&mut self, search_query: String) {
        self.search_query = Some(search_query);
    }

    pub fn set_id_list(&mut self, id_list: String) {
        self.id_list = Some(id_list);
    }

    pub async fn run(&self) -> Result<Feed> {
        let body = reqwest::get(self.to_url()).await?.text().await?;
        let feed = quick_xml::de::from_str(&body)?;

        Ok(feed)
    }

    fn to_url(&self) -> String {
        let mut queries = Vec::new();

        if let Some(search_query) = &self.search_query {
            queries.push(format!("search_query={}", search_query));
        }

        if let Some(id_list) = &self.id_list {
            queries.push(format!("id_list={}", id_list));
        }

        format!("{}{}", self.base_url, queries.join("&"))
    }
}

impl Paper {
    pub async fn download_pdf(&self, out_dir: &str) -> Result<()> {
        let arxiv_id = String::from(*self.id.split('/').collect::<Vec<_>>().last().unwrap());
        let pdf_url = format!("http://de.arxiv.org/pdf/{}.pdf", arxiv_id);
        let body = reqwest::get(pdf_url).await?.bytes().await?;

        // Create output dir and save paper to PDF
        fs::create_dir_all(out_dir)?;
        let file_name = format!("{}.pdf", self.title).replace(" ", "_");
        let out_path = Path::new(out_dir).join(Path::new(&file_name));
        let mut file = fs::File::create(out_path)?;
        file.write_all(&body)?;

        Ok(())
    }
}