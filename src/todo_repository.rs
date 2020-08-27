use crate::{Todo, TodoNotesError};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub async fn get_todo_by_id(
    db_client: &deadpool_postgres::Client,
    id: Uuid,
) -> Result<Option<Todo>, TodoNotesError> {
    let rows = db_client
        .query("SELECT * FROM todo WHERE id=$1", &[&id])
        .await?;
    let some_value = rows.first();
    some_value
        .map(|value| {
            let text: String = value.try_get(1)?;
            let created_at: DateTime<Utc> = value.try_get(2)?;
            let user_id: Uuid = value.try_get(3)?;
            Todo::new(id, text, created_at, user_id)
        })
        .transpose()
}
