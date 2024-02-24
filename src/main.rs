use std::env;

use axum::{
    routing::{get, get_service},
    Router,
};

use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

use dotenv::dotenv;

use axum_htmx_askama::{error::Result, model::ModelController, view::home::handle_main, web};

fn routes_static() -> Router {
    println!("->> {:<12} - routes_static", "CALLED");
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("error".into()))
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    tracing::info!("initializing router...");

    sqlx::migrate!().run(&pool).await?;
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_todos::routes(mc.clone());
    // .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let router = Router::new()
        .route("/", get(handle_main))
        .with_state(mc.clone())
        .nest("/api", routes_apis)
        .fallback_service(routes_static());

    let port = 8000_u16;

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
