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
            } else if entry.file_type().await?.is_dir() {
                let year_dir = entry.file_name();
                let year_path = format!("{}/{}", self.base_path, year_dir.to_string_lossy());
                let mut year_entries = fs::read_dir(year_path).await?;

                while let Some(year_entry) = year_entries.next_entry().await? {
                    if year_entry.file_type().await?.is_file() {
                        if let Some(name) = year_entry.file_name().to_str() {
                            if name.ends_with(".md") {
                                markdown_files.push(format!(
                                    "{}/{}",
                                    year_dir.to_string_lossy(),
                                    name.trim_end_matches(".md")
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(markdown_files)
    }
}
