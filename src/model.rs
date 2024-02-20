use std::sync::{Arc, Mutex};

use serde::Deserialize;

#[derive(Debug)]
struct AppState {
    todos: Mutex<Vec<Option<Todo>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Todo {
    id: u16,
    value: String,
    active: bool,
}

// impl Todo {
//     fn new(id: u16, value: String, active: bool) -> Self {
//         Self { id, value, active }
//     }
// }

// constructor
impl ModelController {
    pub async fn new() -> Result<Self, ()> {
        Ok(Self {
            todos_store: Arc::default(),
        })
    }
}

// Model Controller
// Clone here just clone the ARC not the Vector
#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
}

impl ModelController {
    pub async fn get_todos(&self) -> Result<Vec<Todo>, ()> {
        let store = self.todos_store.lock().unwrap();
        let todos = store.iter().filter_map(|i| i.clone()).collect();
        Ok(todos)
    }
}
