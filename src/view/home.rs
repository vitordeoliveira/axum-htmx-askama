use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;

use crate::{
    error::{self, Result},
    model::{todo::Todo, ModelController},
    view::HtmlTemplate,
};

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate {
    title: String,
    todos: Vec<Todo>,
}

pub async fn handle_main(State(mc): State<ModelController>) -> Result<impl IntoResponse> {
    let todos = mc.get_todos().await?;

    let hello = HelloTemplate {
        title: "RUST AXUM ASKAMA HTMX TODO".to_string(),
        todos,
    };

    let html = match hello.render() {
        Ok(html) => html,
        Err(_) => return Err(error::Error::InternalServer),
    };

    Ok((StatusCode::OK, Html(html)))
}

#[derive(Debug, Deserialize)]
pub struct AddTodoRequest {
    value: String,
}

#[derive(Template)]
#[template(path = "todo-item.html")]
struct TodoItem {
    todo: Todo,
}

pub async fn add_todo_item(
    State(mc): State<ModelController>,
    Form(todo): Form<AddTodoRequest>,
) -> Result<impl IntoResponse> {
    tracing::info!("add_todo_item");

    if todo.value.is_empty() {
        return Err(crate::error::Error::InternalServer);
    }

    let todo = mc.add_todos(todo.value).await?;

    let template = TodoItem { todo };

    Ok(HtmlTemplate(template, Some(StatusCode::OK)))
}

pub async fn remove_todo_item(
    State(mc): State<ModelController>,
    Path(id): Path<sqlx::types::Uuid>,
) -> Result<impl IntoResponse> {
    mc.delete_todo(id).await?;

    Ok(())
}

pub async fn active_todo(
    State(mc): State<ModelController>,
    Path(id): Path<sqlx::types::Uuid>,
) -> Result<impl IntoResponse> {
    let todo = mc.toggle_todo(id).await?;

    let template = TodoItem { todo };

    Ok(HtmlTemplate(template, Some(StatusCode::OK)))
}
