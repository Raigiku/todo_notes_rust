use crate::common::TodoNotesError;
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

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
