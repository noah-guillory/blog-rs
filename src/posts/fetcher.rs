use super::{filesystem::FileSystemPostFetcher, github::GithubPostFetcher};
use anyhow::Result;

#[derive(Clone)]
pub enum PostFetcher {
    Github(GithubPostFetcher),
    FileSystem(FileSystemPostFetcher),
}

impl PostFetcher {
    pub async fn fetch(&self, slug: &str) -> Result<String> {
        match self {
            PostFetcher::Github(fetcher) => fetcher.fetch(slug).await,
            PostFetcher::FileSystem(fetcher) => fetcher.fetch(slug).await,
        }
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        match self {
            PostFetcher::Github(fetcher) => fetcher.list().await,
            PostFetcher::FileSystem(fetcher) => fetcher.list().await,
        }
    }
}
