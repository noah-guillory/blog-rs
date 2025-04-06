use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{
    posts::model::{FrontMatter, Post},
    web::app_state::AppState,
};

// Define an Askama template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    all_posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post_contents: &'a str,
    metadata: FrontMatter,
}

pub async fn index_handler(State(state): State<AppState>) -> Html<String> {
    let all_posts = state.post_fetcher.list().await;

    tracing::info!("Fetched posts: {:?}", all_posts);
    let all_posts = match all_posts {
        Ok(posts) => posts,
        Err(e) => {
            tracing::error!("Error fetching posts: {:?}", e);
            return Html("Error fetching posts".to_string());
        }
    };

    let template = IndexTemplate {
        all_posts: all_posts,
    };

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

    let template = PostTemplate {
        post_contents: &post.content,
        metadata: post.metadata,
    };
    Html(template.render().unwrap())
}
