use std::sync::{Arc, Mutex};

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Form, Router,
};

use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

use crate::model::ModelController;

mod model;
mod web;

#[derive(Debug)]
struct AppState {
    todos: Mutex<Vec<Option<Todo>>>,
}

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    id: u16,
    value: String,
    active: bool,
}

impl Todo {
    fn new(id: u16, value: String, active: bool) -> Self {
        Self { id, value, active }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("error".into()))
        .init();

    tracing::info!("initializing router...");

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_todos::routes(mc.clone());
    // .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    let router = Router::new()
        .route("/", get(handle_main))
        .nest("/api", routes_apis);

    // .with_state(app_state);

    let port = 8000_u16;

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    title: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert(header::SERVER, "axum".parse().unwrap());
        // This is how send a custom event from server to HTMX
        // headers.insert("HX-Trigger", "myevent".parse().unwrap());
        match self.0.render() {
            Ok(html) => (StatusCode::OK, headers, Html(html)).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn handle_main() -> impl IntoResponse {
    let hello = HelloTemplate {
        title: "RUST AXUM ASKAMA HTMX TODO".to_string(),
    };
    HtmlTemplate(hello)
}
