use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{posts::model::FrontMatter, web::app_state::AppState};

// Define an Askama template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post_contents: &'a str,
    metadata: FrontMatter,
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

    let rendered_post = markdown::to_html(&post.content);
    let template = PostTemplate {
        post_contents: &rendered_post,
        metadata: post.metadata,
    };
    Html(template.render().unwrap())
}
