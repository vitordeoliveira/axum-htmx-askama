use axum::Router;

use tracing_subscriber::EnvFilter;

use dotenv::dotenv;

use axum_htmx_askama::{
    controller::Controller, error::Result, model::ModelManager, view::notfound::handler_404,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("error".into()))
        .init();

    let mc = ModelManager::new().await?;
    let controller = Controller::new(mc.clone());

    let router = Router::new()
        .nest("/", controller.view)
        .nest("/api", controller.data)
        .fallback(handler_404);

    let port = 8000_u16;

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
