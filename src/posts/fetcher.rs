use super::{
    filesystem::FileSystemPostFetcher,
    github::GithubPostFetcher,
    model::{FrontMatter, Post},
};
use anyhow::Result;

#[derive(Clone)]
pub enum PostFetcher {
    Github(GithubPostFetcher),
    FileSystem(FileSystemPostFetcher),
}

impl PostFetcher {
    pub async fn fetch(&self, slug: &str) -> Result<Post> {
        let raw_post = match self {
            PostFetcher::Github(fetcher) => fetcher.fetch(slug).await,
            PostFetcher::FileSystem(fetcher) => fetcher.fetch(slug).await,
        }?;

        let data = fronma::parser::parse::<FrontMatter>(&raw_post)
            .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;

        return Ok(Post {
            metadata: data.headers,
            content: data.body.to_string(),
        });
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        match self {
            PostFetcher::Github(fetcher) => fetcher.list().await,
            PostFetcher::FileSystem(fetcher) => fetcher.list().await,
        }
    }
}
