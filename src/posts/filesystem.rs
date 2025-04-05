use anyhow::Result;
use tokio::fs;

#[derive(Clone)]
pub struct FileSystemPostFetcher {
    pub base_path: &'static str,
}

impl FileSystemPostFetcher {
    pub fn new(base_path: &'static str) -> Self {
        Self { base_path }
    }

    fn get_file_path(&self, slug: &str) -> String {
        format!("{}/{}.md", self.base_path, slug)
    }

    pub async fn fetch(&self, slug: &str) -> Result<String> {
        let file_path = self.get_file_path(slug);
        let contents = fs::read_to_string(file_path).await?;
        Ok(contents)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        let mut markdown_files = Vec::new();
        let mut entries = fs::read_dir(&self.base_path).await?;

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
