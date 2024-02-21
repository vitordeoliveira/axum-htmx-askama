use std::sync::{Arc, Mutex};

use serde::Deserialize;
use tracing::info;

use crate::error::{Error, Result};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Todo {
    pub id: u16,
    pub value: String,
    pub active: bool,
}

// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
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
    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let store = self.todos_store.lock().unwrap();
        let todos = store.iter().filter_map(|i| i.clone()).collect();
        Ok(todos)
    }

    pub async fn add_todos(&self, value: String) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let newid = store.len() as u16;

        let todo = Todo {
            id: newid,
            active: false,
            value,
        };

        store.push(Some(todo.clone()));

        Ok(todo)
    }

    pub async fn delete_todo(&self, id: u16) -> Result<()> {
        info!("delete_todo");
        let mut store = self.todos_store.lock().unwrap();

        store.retain(|i| i.as_ref().unwrap().id != id);

        Ok(())
    }

    pub async fn toggle_todo(&self, id: u16) -> Result<Todo> {
        info!("toggle_todo");

        let mut store = self.todos_store.lock().unwrap();

        // if let Some(todo) = store
        //     .iter_mut()
        //     .find(|t| t.is_some() && t.as_ref().unwrap().id == id)
        // {
        //     todo.as_mut().unwrap().active = !todo.as_ref().unwrap().active;
        //     return Ok(todo.as_ref().unwrap().clone());
        // }
        //
        // Err(())

        let mut changed: Option<Todo> = None;

        store.iter_mut().for_each(|t| {
            if let Some(todo) = t.as_mut() {
                if todo.id == id {
                    todo.active = !todo.active;
                    changed = Some(todo.clone());
                }
            }
        });

        match changed {
            Some(todo) => Ok(todo),
            None => Err(Error::TodoNotFound { id: (1) }),
        }
    }
}
