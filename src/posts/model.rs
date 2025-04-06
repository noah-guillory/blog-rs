use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PublicationStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
}

#[derive(Deserialize, Debug)]
pub struct FrontMatter {
    pub title: String,
    pub status: PublicationStatus,
}

#[derive(Debug)]
pub struct Post {
    pub metadata: FrontMatter,
    pub slug: String,
    pub content: String,
}
