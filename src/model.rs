extern crate serde;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Feed {
    #[serde(rename = "entry", default)]
    pub papers: Vec<Paper>,
}

#[derive(Deserialize, Debug)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub summary: String,

    #[serde(rename = "author", default)]
    pub authors: Vec<Author>,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
}
