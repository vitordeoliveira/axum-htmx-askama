use crate::{error::Result, model::*};
use askama::Template;
use axum::{
    extract::{FromRef, Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{delete, post},
    Form, Router,
};
use serde::Deserialize;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
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

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/addtodo", post(add_todo_item))
        .route("/deletetodo/:id", delete(remove_todo_item))
        .route("/activetodo/:id", post(active_todo))
        .with_state(app_state)
}

#[derive(Debug, Deserialize)]
struct AddTodoRequest {
    value: String,
}

#[derive(Template)]
#[template(path = "todo-item.html")]
struct TodoItem {
    todo: Todo,
}

async fn add_todo_item(
    State(mc): State<ModelController>,
    Form(todo): Form<AddTodoRequest>,
) -> Result<impl IntoResponse> {
    tracing::info!("add_todo_item");

    if todo.value.is_empty() {
        return Err(crate::error::Error::InternalServerError);
    }

    let todo = mc.add_todos(todo.value).await?;

    let template = TodoItem { todo };

    Ok(HtmlTemplate(template))
}

async fn remove_todo_item(
    State(mc): State<ModelController>,
    Path(id): Path<u16>,
) -> Result<impl IntoResponse> {
    mc.delete_todo(id).await?;

    Ok(())
}

async fn active_todo(
    State(mc): State<ModelController>,
    Path(id): Path<u16>,
) -> Result<impl IntoResponse> {
    let todo = mc.toggle_todo(id).await?;

    let template = TodoItem { todo };

    Ok(HtmlTemplate(template))
}
