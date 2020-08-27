mod get_todo_by_id_endpoint;
use get_todo_by_id_endpoint::get_todo_by_id_endpoint;

mod get_todo_by_id;
use get_todo_by_id::{get_todo_by_id, GetTodoByIdInput, GetTodoByIdOutput};

mod todo_repository;

mod todo;
use todo::Todo;

mod user;
use user::User;

mod todo_notes_error;
use todo_notes_error::TodoNotesError;

use actix_web::{middleware, web, App, HttpServer};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::env;

fn db_pool() -> Result<Pool, TodoNotesError> {
    let mut cfg = Config::new();
    cfg.host = Some("ec2-52-202-66-191.compute-1.amazonaws.com".to_string());
    cfg.dbname = Some("daidkbnekdtkt0".to_string());
    cfg.user = Some("xqdxhbeckoyiao".to_string());
    cfg.port = Some(5432);
    cfg.password =
        Some("e814c1fa10fd137bce5a0ee7ecaa23b62cb4e9cedd7d17a96eda2898f8edf2cf".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let connector = MakeTlsConnector::new(connector);
    Ok(cfg.create_pool(connector)?)
}

#[actix_rt::main]
async fn main() -> Result<(), TodoNotesError> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let pool = db_pool()?;
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(web::resource("/api/todos/{id}").to(get_todo_by_id_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?)
}
