/// Local error type.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    Tokio(#[from] tokio::io::Error),
    Axum(#[from] axum::Error),
    Json(#[from] serde_json::Error),
    Sqlx(#[from] sqlx::Error),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
