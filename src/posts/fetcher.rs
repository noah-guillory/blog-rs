use anyhow::{Ok, Result};
use axum::http::request;
use tracing_subscriber::fmt::format;

pub trait PostFetcher {
    async fn fetch(&self, slug: &str) -> Result<String>;
    async fn list(&self) -> Result<Vec<String>>;
}

#[derive(Clone)]
pub struct GithubPostFetcher<'a> {
    owner: &'a str,
    repo: &'a str,
}

impl GithubPostFetcher<'_> {
    pub fn new<'a>(owner: &'a str, repo: &'a str) -> GithubPostFetcher<'a> {
        GithubPostFetcher { owner, repo }
    }
}

fn get_download_url(owner: &str, repo: &str, slug: &str) -> String {
    format!("https://raw.githubusercontent.com/{owner}/{repo}/main/{slug}.md")
}

impl PostFetcher for GithubPostFetcher<'_> {
    async fn fetch(&self, slug: &str) -> Result<String> {
        let contents = reqwest::get(get_download_url(self.owner, self.repo, slug))
            .await?
            .text()
            .await?;

        return Ok(contents);
    }

    async fn list(&self) -> Result<Vec<String>> {
        let root_directories = octocrab::instance()
            .repos(self.owner, self.repo)
            .get_content()
            .path("")
            .r#ref("main")
            .send()
            .await?;

        let mut markdown_files = Vec::new();

        for item in root_directories.items {
            if item.r#type == "dir" {
                let year_dir = item.name;
                let year_files = octocrab::instance()
                    .repos(self.owner, self.repo)
                    .get_content()
                    .path(&year_dir)
                    .r#ref("main")
                    .send()
                    .await?;

                for file in year_files.items {
                    if file.name.ends_with(".md") {
                        markdown_files.push(format!("{}/{}", year_dir, file.name));
                    }
                }
            }
        }

        Ok(markdown_files)
    }
}

struct FileSystemPostFetcher {
    base_path: String,
}

impl FileSystemPostFetcher {
    pub fn new(base_path: String) -> FileSystemPostFetcher {
        FileSystemPostFetcher { base_path }
    }

    fn get_file_path(&self, slug: &str) -> String {
        format!("{}/{}.md", self.base_path, slug)
    }
}

impl PostFetcher for FileSystemPostFetcher {
    async fn fetch(&self, slug: &str) -> Result<String> {
        let file_path = self.get_file_path(slug);
        let contents = tokio::fs::read_to_string(file_path).await?;
        Ok(contents)
    }

    async fn list(&self) -> Result<Vec<String>> {
        let mut markdown_files = Vec::new();
        let entries = tokio::fs::read_dir(&self.base_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".md") {
                        markdown_files.push(name.to_string());
                    }
                }
            }
        }

        Ok(markdown_files)
    }
}
