use crate::{
    common::TodoNotesError,
    models::Todo,
    queries::{GetTodoByIdInput, GetTodoByIdOutput, Query},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetTodoByIdPg {
    db_pool: Arc<PgPool>,
}

impl GetTodoByIdPg {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Query<GetTodoByIdInput, Option<GetTodoByIdOutput>> for GetTodoByIdPg {
    async fn execute(
        &self,
        input: GetTodoByIdInput,
    ) -> Result<Option<GetTodoByIdOutput>, TodoNotesError> {
        Ok(sqlx::query("SELECT * FROM todo WHERE id=$1")
            .bind(input.id())
            .map(|row: PgRow| {
                let id: Uuid = row.try_get(0)?;
                let text: String = row.try_get(1)?;
                let created_at: DateTime<Utc> = row.try_get(2)?;
                let user_id: Uuid = row.try_get(3)?;
                Todo::new(id, text, created_at, user_id)
            })
            .fetch_optional(self.db_pool.as_ref())
            .await?
            .transpose()?
            .map(GetTodoByIdOutput::new))
    }
}
