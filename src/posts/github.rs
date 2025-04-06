use anyhow::Result;

use super::model::{FrontMatter, Post};

#[derive(Clone)]
pub struct GithubPostFetcher {
    pub owner: &'static str,
    pub repo: &'static str,
}

impl GithubPostFetcher {
    pub fn new(owner: &'static str, repo: &'static str) -> Self {
        Self { owner, repo }
    }

    fn get_download_url(&self, slug: &str) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/main/{}.md",
            self.owner, self.repo, slug
        )
    }

    pub async fn fetch(&self, slug: &str) -> Result<String> {
        let contents = reqwest::get(self.get_download_url(slug))
            .await?
            .text()
            .await?;

        Ok(contents)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
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
                        markdown_files.push(format!(
                            "{}/{}",
                            year_dir,
                            file.name.trim_end_matches(".md")
                        ));
                    }
                }
            }
        }

        tracing::info!("Fetched posts: {:?}", markdown_files);

        Ok(markdown_files)
    }
}
