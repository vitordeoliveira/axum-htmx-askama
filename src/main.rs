#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Starting up");
    tracing::warn!("Are you sure?");
    tracing::error!("This is an error!")
}
