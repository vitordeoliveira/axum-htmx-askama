use askama::Template;
use axum::{
    extract::FromRef,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};

use crate::model::ModelManager;

use self::home::{active_todo, add_todo_item, handle_main, remove_todo_item};

pub mod home;
pub mod notfound;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelManager,
}

pub fn routes(mc: ModelManager) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/", get(handle_main))
        .route("/addtodo", post(add_todo_item))
        .route("/deletetodo/:id", delete(remove_todo_item))
        .route("/activetodo/:id", post(active_todo))
        .with_state(app_state)
}

struct HtmlTemplate<T>(T, Option<StatusCode>);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert(header::SERVER, "axum".parse().unwrap());

        let statuscode = match self.1 {
            Some(status) => status,
            None => StatusCode::OK,
        };

        match self.0.render() {
            Ok(html) => (statuscode, headers, Html(html)).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
