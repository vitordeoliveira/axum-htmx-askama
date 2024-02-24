use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{self, Result},
    model::{ModelController, Todo},
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
