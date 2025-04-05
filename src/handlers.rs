use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{AppState, posts::fetcher::PostFetcher};

// Define an Askama template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    body: &'a str,
}

pub async fn index_handler(State(state): State<AppState>) -> Html<String> {
    let template = IndexTemplate {};

    let all_posts: Vec<String> = state.post_fetcher.list().await.unwrap();
    tracing::info!("Fetched posts: {:?}", all_posts);
    Html(template.render().unwrap())
}

pub async fn post_handler(
    State(state): State<AppState>,
    Path((year, slug)): Path<(u32, String)>,
) -> Html<String> {
    tracing::info!("Fetching post for year: {}, slug: {}", year, slug);
    let post = state
        .post_fetcher
        .fetch(format!("{year}/{slug}").as_str())
        .await
        .unwrap();

    let rendered_post = markdown::to_html(&post);
    let template = PostTemplate {
        body: &rendered_post,
    };
    Html(template.render().unwrap())
}
