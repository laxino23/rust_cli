use std::{
    net::SocketAddr,
    path::{Path as stdPath, PathBuf},
    sync::Arc,
};

use anyhow::Result as aResult;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    directory: PathBuf,
}

pub async fn process_http_server(directory: PathBuf, port: u16) -> aResult<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving directory {} on port {}", directory.display(), port);
    let state = HttpServeState { directory };
    // axum router
    let router: Router = Router::new()
        .route("/{*wildcard}", get(file_handler))
        .with_state(Arc::new(state));
    // tcp listener
    let listener = TcpListener::bind(addr).await?;
    // serve
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let full_path = stdPath::new(&state.directory).join(path);

    if full_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(full_path.clone()).await
        {
            info!("read file {} bytes", content.len());
            (StatusCode::OK, content)
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file: {:?}", full_path),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("File not found: {:?}", full_path),
        )
    }
}
