use askama::Template;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};

use model::Todo;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

use crate::model::ModelController;

mod model;
mod web;

fn routes_static() -> Router {
    println!("->> {:<12} - routes_static", "CALLED");
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate {
    title: String,
    todos: Vec<Todo>,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert(header::SERVER, "axum".parse().unwrap());
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

async fn handle_main(State(mc): State<ModelController>) -> Result<impl IntoResponse, ()> {
    let todos = mc.get_todos().await?;
    let hello = HelloTemplate {
        title: "RUST AXUM ASKAMA HTMX TODO".to_string(),
        todos,
    };

    Ok((StatusCode::OK, Html(hello.render().unwrap())))

    // let val = (StatusCode::OK, hello).into_response();

    // Ok(HtmlTemplate(hello))
}
