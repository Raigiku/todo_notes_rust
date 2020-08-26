use crate::{get_todo_by_id, GetTodoByIdInput, GetTodoByIdOutput, TodoNotesError};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetTodoByIdRequest {
    id: String,
}

#[derive(Serialize)]
struct GetTodoByIdResponse {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
    links: Vec<String>,
}

impl GetTodoByIdResponse {
    pub fn new(output: GetTodoByIdOutput) -> Self {
        Self {
            id: *output.id(),
            text: output.text().clone(),
            created_at: *output.created_at(),
            links: vec!["das".to_string()],
        }
    }
}

pub async fn get_todo_by_id_endpoint(
    request: web::Path<GetTodoByIdRequest>,
) -> Result<impl Responder, TodoNotesError> {
    let input = GetTodoByIdInput::new(request.id.clone())?;
    let some_output = get_todo_by_id(input).await?;
    if let Some(output) = some_output {
        let response = GetTodoByIdResponse::new(output);
        Ok(HttpResponse::Ok()
            .set_header("content-type", "application/json")
            .json(response))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
