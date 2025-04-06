use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum PublicationStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
}

#[derive(Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub status: PublicationStatus,
}

pub struct Post {
    pub metadata: FrontMatter,
    pub content: String,
}
