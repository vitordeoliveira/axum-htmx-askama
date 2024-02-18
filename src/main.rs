use std::sync::{Arc, Mutex};

use askama::Template;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};

use serde::Deserialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct AppState {
    todos: Mutex<Vec<Option<Todo>>>,
}

#[derive(Debug, Clone)]
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

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    let router = Router::new()
        .route("/", get(handle_main))
        .route("/todolist", post(add_todo_item))
        .route("/gettodos", get(get_todos))
        .with_state(app_state);

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

#[derive(Template)]
#[template(path = "todoitem.html")]
struct TodoList {
    todos: Vec<Todo>,
}

async fn handle_main() -> impl IntoResponse {
    let hello = HelloTemplate {
        title: "RUST AXUM ASKAMA HTMX TODO".to_string(),
    };
    HtmlTemplate(hello)
}

#[derive(Debug, Deserialize)]
struct TodoRequest {
    todo: String,
}

// #[instrument(skip(state))]
async fn add_todo_item(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    if todo.todo.is_empty() {
        return Err(());
    }

    info!("just printing;");
    let mut todos = state.todos.lock().unwrap();
    let newid = if todos.is_empty() {
        0
    } else {
        todos.last().unwrap().clone().map_or(0, |todo| todo.id + 1)
    };

    // let newid = if todos.last().is_some() {
    //     todos.last().unwrap().clone().unwrap().id + 1
    // } else {
    //     0
    // };

    todos.push(Some(Todo::new(newid, todo.todo, false)));

    let collect: Vec<Todo> = todos.clone().into_iter().flatten().collect();

    let template = TodoList { todos: collect };
    Ok(HtmlTemplate(template))
}

async fn remove_todo_item(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut todos = state.todos.lock().unwrap();
}

// #[tracing::instrument]
async fn get_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();

    let collect: Vec<Todo> = todos.clone().into_iter().flatten().collect();

    let template = TodoList { todos: collect };
    HtmlTemplate(template)
}
