pub use self::error::{Error, Result};

use axum::Router;
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod error;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .merge(routes_home())
    .fallback_service(routes_static());

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn routes_home() -> Router {
    Router::new()
    .route(
        "/",
        get(handler_home),
    )
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("public")))
}

async fn handler_home() -> impl IntoResponse {
    Html(format!("<html><head><script src=\"/js/htmx-1.9.10.min.js\"></script></head><body><h1>Coming soon!</h1><a href=\"https://youtube.com/BraydenGirard\">YouTube</a><br><a rel=\"me\" href=\"https://mastodon.social/@braydengirard\">Mastodon</a></body></html>"))
}