use super::endpoints::{get_todo_by_id_endpoint, home};
use crate::{
    common::TodoNotesError,
    queries::{GetTodoByIdInput, GetTodoByIdOutput, Query},
};
use actix_web::{middleware, web, App, HttpServer};
use std::env;

pub async fn main<
    T: 'static + Clone + Send + Query<GetTodoByIdInput, Option<GetTodoByIdOutput>>,
>(
    get_todo_by_id: T,
) -> Result<(), TodoNotesError> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    Ok(HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(get_todo_by_id.clone())
            .service(web::resource("/").to(home))
            .service(web::resource("/todos/{id}").to(get_todo_by_id_endpoint::<T>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?)
}
