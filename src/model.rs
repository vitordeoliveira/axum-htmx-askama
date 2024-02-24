use std::{
    env,
    sync::{Arc, Mutex},
};

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use tracing::info;

use crate::error::{Error, Result};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Todo {
    pub id: u16,
    pub value: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Todo1 {
    pub id: sqlx::types::Uuid,
    pub value: String,
    pub active: bool,
}

impl Todo1 {
    pub async fn get_todos(mc: &ModelController) -> Result<Vec<Self>> {
        let rows = sqlx::query_as!(Todo1, "SELECT * FROM todo")
            .fetch_all(mc.db())
            .await
            .unwrap();

        Ok(rows)
    }
}

// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self {
            todos_store: Arc::default(),
            db: pool,
            test: 10,
        })
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

// Model Controller
// Clone here just clone the ARC not the Vector
#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
    db: Pool<Postgres>,
    test: u16,
}

impl ModelController {
    pub async fn get_todos(&self) -> Result<Vec<Todo1>> {
        let todos = Todo1::get_todos(self).await?;
        Ok(todos)
    }

    pub async fn add_todos(&self, value: String) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        // TODO: add on the database
        // let db = self.db();

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
