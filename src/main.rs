mod common;
use common::TodoNotesError;
use infrastructure::persistence::postgres::GetTodoByIdPg;
use sqlx::PgPool;
use std::{env, sync::Arc};

mod commands;
mod infrastructure;
mod models;
mod queries;

#[actix_rt::main]
async fn main() -> Result<(), TodoNotesError> {
    let db_url = &env::var("DATABASE_URL")?;
    let db_pool = Arc::new(PgPool::new(db_url).await?);
    let get_todo_by_id = GetTodoByIdPg::new(Arc::clone(&db_pool));

    infrastructure::presentation::api_actix_web::main(get_todo_by_id).await
    // infrastructure::presentation::terminal::main(get_todo_by_id).await
}
