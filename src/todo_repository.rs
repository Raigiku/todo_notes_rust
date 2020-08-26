use crate::{Todo, TodoNotesError};
use chrono::{DateTime, Utc};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::env;
use uuid::Uuid;

pub async fn get_todo_by_id(id: Uuid) -> Result<Option<Todo>, TodoNotesError> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let connector = MakeTlsConnector::new(connector);
    let (client, connection) =
        tokio_postgres::connect(&env::var("POSTGRES_HEROKU_URI")?, connector).await?;

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM todo WHERE id=$1", &[&id])
        .await?;
    let some_value = rows.first();
    some_value
        .map(|value| {
            let text: String = value.try_get("text")?;
            let created_at: DateTime<Utc> = value.try_get("created_at")?;
            Todo::new(id, text, created_at)
        })
        .transpose()
}
