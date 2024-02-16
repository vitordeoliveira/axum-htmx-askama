use askama::Template;
use axum::{
    http::{self, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("error".into()))
        .init();

    tracing::info!("initializing router...");

    let router = Router::new().route("/", get(handle_main));
    let port = 8000_u16;

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

pub fn into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => {
            let headers = [(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(T::MIME_TYPE),
            )];

            (headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String, // the field name should match the variable name
    title: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
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
        name: "nicolas".to_string(),
        title: "nicolas".to_string(),
    };
    // into_response(&hello)
    HtmlTemplate(hello)
}
