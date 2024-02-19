use std::sync::Arc;

use axum::{extract::State, Router};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post())
}

async fn get_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();

    let collect: Vec<Todo> = todos.clone().into_iter().flatten().collect();

    let template = TodoList { todos: collect };
    HtmlTemplate(template)
}
