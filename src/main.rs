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
    controller.run_server().await?;

    Ok(())
}
