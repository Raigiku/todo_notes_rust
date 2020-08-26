use crate::TodoNotesError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Todo {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(id: Uuid, text: String, created_at: DateTime<Utc>) -> Result<Self, TodoNotesError> {
        let text = Self::validate_text(text)?;
        Ok(Self {
            id,
            text,
            created_at,
        })
    }

    fn validate_text(text: String) -> Result<String, TodoNotesError> {
        if text.len() > 200 {
            Err(TodoNotesError::UserError {
                field: "text".to_string(),
                message: "text length should be less than 200 characters".to_string(),
            })
        } else {
            Ok(text)
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
}
