use crate::models::Todo;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetTodoByIdOutput {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
    user_id: Uuid,
}

impl GetTodoByIdOutput {
    pub fn new(todo: Todo) -> Self {
        Self {
            id: *todo.id(),
            text: todo.text().clone(),
            created_at: *todo.created_at(),
            user_id: *todo.user_id(),
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
