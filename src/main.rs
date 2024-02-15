use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::{prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    let subscriber = Registry::default().with(fmt_layer).with(filter_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Starting up");
    tracing::warn!("Are you sure?");
    tracing::error!("This is an error!")
}
