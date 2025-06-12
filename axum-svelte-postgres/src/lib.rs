use axum::{Router, routing::get};
pub use {{project-name}}::Result;
use tokio::net::TcpListener;

pub async fn run() -> crate::Result {
    let router = Router::new().route("/", get(async || "Hello, World"));
    let tcp = TcpListener::bind("127.0.0.1:5050").await?;

    axum::serve(tcp, router).await?;

    Ok(())
}
