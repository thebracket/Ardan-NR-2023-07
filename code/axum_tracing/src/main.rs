use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Tracing
    use tracing_subscriber::fmt::format::FmtSpan;

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Add span events
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // Display debug-level info
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Axum App
    use tower_http::trace::{self, TraceLayer};
    let app = Router::new().route("/", get(say_hello_text)).layer(
        TraceLayer::new_for_http()
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}
