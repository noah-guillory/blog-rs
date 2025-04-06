use crate::posts::model::PublicationStatus;

use super::{
    filesystem::FileSystemPostFetcher,
    github::GithubPostFetcher,
    model::{FrontMatter, Post},
};
use anyhow::Result;
use fronma::parser::ParsedData;

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

        let parsed_data = match fronma::parser::parse::<FrontMatter>(&raw_post) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!(
                    "Failed to parse front matter: {:?}  Setting to default values",
                    e
                );
                ParsedData {
                    body: raw_post.as_str(),
                    headers: FrontMatter {
                        title: slug.to_string(),
                        status: PublicationStatus::Draft,
                    },
                }
            }
        };

        let rendered_post = markdown::to_html(parsed_data.body);

        return Ok(Post {
            slug: slug.to_string(),
            metadata: parsed_data.headers,
            content: rendered_post,
        });
    }

    pub async fn list(&self) -> Result<Vec<Post>> {
        let slugs = match self {
            PostFetcher::Github(fetcher) => fetcher.list().await?,
            PostFetcher::FileSystem(fetcher) => fetcher.list().await?,
        };

        let posts = futures::future::try_join_all(slugs.into_iter().map(|slug| {
            let fetcher = self.clone();
            async move { fetcher.fetch(&slug).await }
        }))
        .await?;

        tracing::info!("Fetched posts: {:?}", posts);

        Ok(posts)
    }
}
