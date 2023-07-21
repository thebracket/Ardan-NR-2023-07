use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    // Simple output
    //let subscriber = tracing_subscriber::FmtSubscriber::new();

    // Detailed Output
    let subscriber = tracing_subscriber::fmt()
        .json()
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
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Hello World!");
    tracing::warn!("Hello World!");
    tracing::error!("Hello World!");
    do_something();
    do_something_async().await;
}

#[tracing::instrument]
fn do_something() {
    tracing::info!("Doing something");
    for n in 0..3 {
        do_something_else(n);
    }
}

#[tracing::instrument]
fn do_something_else(n: i32) {
    tracing::info!("Doing something else: {n}");
}

#[tracing::instrument]
async fn do_something_async() {
    tracing::info!("We're in an async context");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    tracing::info!("Finished waiting");
}
