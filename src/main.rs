use axum::{Router, extract::MatchedPath, http::Request, routing::get};
use handlers::post_handler;
use posts::{fetcher::PostFetcher, github::GithubPostFetcher};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod handlers;
pub mod posts;

use crate::handlers::index_handler;

#[derive(Clone)]
struct AppState {
    post_fetcher: PostFetcher,
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the app state with a post fetcher

    let app_state = AppState {
        post_fetcher: PostFetcher::Github(GithubPostFetcher::new("noah-guillory", "blog-posts")),
        // post_fetcher: PostFetcher::FileSystem(posts::filesystem::FileSystemPostFetcher::new(
        //     "/Users/noah/Projects/blog-posts",
        // )),
    };

    // Build the application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/post/{year}/{slug}", get(post_handler))
        .with_state(app_state)
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    // Define the address to run the server on
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Run the server
    axum::serve(listener, app).await.unwrap();
}
