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
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/todos/{id}").to(get_todo_by_id_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
