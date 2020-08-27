use crate::{get_todo_by_id, GetTodoByIdInput, GetTodoByIdOutput, TodoNotesError};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
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
    user: String,
}

impl GetTodoByIdResponse {
    pub fn new(output: GetTodoByIdOutput) -> Self {
        Self {
            id: *output.id(),
            text: output.text().clone(),
            created_at: *output.created_at(),
            user: format!("127.0.0.1:8080/users/{}", output.user_id()),
        }
    }
}

pub async fn get_todo_by_id_endpoint(
    request: web::Path<GetTodoByIdRequest>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, TodoNotesError> {
    let input = GetTodoByIdInput::new(request.id.clone())?;
    let db_client = db_pool.get().await?;
    let some_output = get_todo_by_id(&db_client, input).await?;
    if let Some(output) = some_output {
        let response = GetTodoByIdResponse::new(output);
        Ok(HttpResponse::Ok()
            .set_header("content-type", "application/json")
            .json(response))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
