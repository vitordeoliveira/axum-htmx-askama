use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::Result;

use self::todo::Todo;

pub mod todo;

#[derive(Clone)]
pub struct ModelManager {
    db: Pool<Postgres>,
}

// constructor
impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { db: pool })
    }
    pub(self) fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

impl ModelManager {
    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = Todo::get_todos(self).await?;
        Ok(todos)
    }

    pub async fn add_todos(&self, value: String) -> Result<Todo> {
        let todo = Todo::add_todos(self, value.clone()).await?;
        Ok(todo)
    }

    pub async fn delete_todo(&self, id: sqlx::types::Uuid) -> Result<()> {
        Todo::delete_todo(self, id).await?;
        Ok(())
    }

    pub async fn toggle_todo(&self, id: sqlx::types::Uuid) -> Result<Todo> {
        let todo = Todo::toggle_todo(self, id).await?;
        Ok(todo)
    }
}
