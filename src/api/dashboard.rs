use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::Embed;
use tracing::info;

/// Embed the dashboard static assets at compile time.
#[derive(Embed)]
#[folder = "dashboard/"]
struct DashboardAssets;

/// Serve the embedded dashboard on a random available port.
/// Returns the actual port the server bound to.
pub async fn start_dashboard_server(api_port: u16) -> anyhow::Result<u16> {
    let app = Router::new()
        // Serve a dynamic config script that tells the SPA where the API is
        .route("/config.js", get(move || async move {
            let js = format!(
                "window.__FLUXX_API_BASE__ = 'http://localhost:{}/v1';",
                api_port
            );
            Response::builder()
                .header(header::CONTENT_TYPE, "application/javascript")
                .body(js)
                .unwrap()
        }))
        // Serve all static assets, fallback to index.html for SPA routing
        .fallback(get(serve_embedded));

    // Bind to port 0 = OS picks a random available port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
    let port = listener.local_addr()?.port();

    info!("Dashboard: Serving on http://localhost:{}", port);

    tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    Ok(port)
}

/// Serve an embedded static file, or index.html for SPA fallback.
async fn serve_embedded(Path(path): Path<String>) -> impl IntoResponse {
    // Try to find the exact file
    if let Some(file) = DashboardAssets::get(&path) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        return Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
            .body(axum::body::Body::from(file.data.to_vec()))
            .unwrap()
            .into_response();
    }

    // SPA fallback: serve index.html with config.js injected
    serve_index_html().into_response()
}

/// Serve the root index.html with the config script injected.
pub async fn serve_root() -> impl IntoResponse {
    serve_index_html()
}

fn serve_index_html() -> Response {
    match DashboardAssets::get("index.html") {
        Some(file) => {
            let html = String::from_utf8_lossy(&file.data);
            // Inject config.js script before the closing </head> tag
            let injected = html.replace(
                "</head>",
                r#"<script src="/config.js"></script></head>"#,
            );
            Response::builder()
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .header(header::CACHE_CONTROL, "no-cache")
                .body(axum::body::Body::from(injected))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("Dashboard not found"))
            .unwrap(),
    }
}
