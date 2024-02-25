use crate::error::Result;

use super::ModelController;

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
        // let insert_query = format!("INSERT INTO todo(value) VALUES ('{value}') RETURNING *");
        // let rows = sqlx::query_as::<_, Todo>(&insert_query)
        //     .fetch_one(mc.db())
        //     .await?;

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

    pub async fn toggle_todo(mc: &ModelController, id: uuid::Uuid) -> Result<Self> {
        let rows = sqlx::query_as::<_, Todo>(
            "UPDATE todo SET active = NOT active WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_one(mc.db())
        .await?;

        Ok(rows)
    }
}
