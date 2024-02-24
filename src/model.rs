use std::env;

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::Result;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: sqlx::types::Uuid,
    pub value: String,
    pub active: bool,
}

impl Todo {
    pub async fn get_todos(mc: &ModelController) -> Result<Vec<Self>> {
        let rows = sqlx::query_as::<_, Todo>("SELECT * FROM todo")
            .fetch_all(mc.db())
            .await
            .unwrap();

        Ok(rows)
    }

    pub async fn add_todos(mc: &ModelController, value: String) -> Result<Self> {
        let rows = sqlx::query_as::<_, Todo>("INSERT INTO todo(value) VALUES ($1) RETURNING *")
            .bind(value)
            .fetch_one(mc.db())
            .await?;

        Ok(rows)
    }

    pub async fn delete_todo(mc: &ModelController, id: sqlx::types::Uuid) -> Result<()> {
        sqlx::query_as::<_, Todo>("DELETE FROM todo WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(mc.db())
            .await?;

        Ok(())
    }

    async fn toggle_todo(mc: &ModelController, id: uuid::Uuid) -> Result<Self> {
        let rows = sqlx::query_as::<_, Todo>(
            "UPDATE todo SET active = NOT active WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_one(mc.db())
        .await?;

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

        Ok(Self { db: pool })
    }
    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

// Model Controller
// Clone here just clone the ARC not the Vector
#[derive(Clone)]
pub struct ModelController {
    db: Pool<Postgres>,
}

impl ModelController {
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
