use crate::posts;
use crate::posts::fetcher::PostFetcher;
use crate::posts::github::GithubPostFetcher;

#[derive(Clone)]
pub struct AppState {
    pub post_fetcher: PostFetcher,
}

pub fn setup_app_state() -> AppState {
    let post_fetcher_type =
        std::env::var("POST_FETCHER_TYPE").unwrap_or_else(|_| "github".to_string());

    let post_fetcher = match post_fetcher_type.as_str() {
        "github" => PostFetcher::Github(GithubPostFetcher::new("noah-guillory", "blog-posts")),
        "filesystem" => PostFetcher::FileSystem(posts::filesystem::FileSystemPostFetcher::new(
            "/Users/noah/Projects/blog-posts",
        )),
        _ => panic!("Invalid POST_FETCHER_TYPE. Use 'github' or 'filesystem'."),
    };

    return AppState {
        post_fetcher: post_fetcher,
    };
}
