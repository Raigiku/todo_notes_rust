use crate::{todo_repository, Todo, TodoNotesError};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct GetTodoByIdInput {
    id: Uuid,
}

impl GetTodoByIdInput {
    pub fn new(id: String) -> Result<Self, TodoNotesError> {
        let id = Uuid::parse_str(&id).map_err(|_| TodoNotesError::UserError {
            field: "id".to_string(),
            message: "todo id is not a uuid".to_string(),
        })?;
        Ok(Self { id })
    }
}

pub struct GetTodoByIdOutput {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
    user_id: Uuid
}

impl GetTodoByIdOutput {
    pub fn new(todo: Todo) -> Self {
        Self {
            id: *todo.id(),
            text: todo.text().clone(),
            created_at: *todo.created_at(),
            user_id: *todo.user_id()
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
}

pub async fn get_todo_by_id(
    input: GetTodoByIdInput,
) -> Result<Option<GetTodoByIdOutput>, TodoNotesError> {
    let some_todo = todo_repository::get_todo_by_id(input.id).await?;
    Ok(some_todo.map(GetTodoByIdOutput::new))
}
