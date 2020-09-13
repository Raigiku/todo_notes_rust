use crate::infrastructure::presentation::api_actix_web::models::Link;
use actix_web::{HttpResponse, Responder};

pub async fn home() -> impl Responder {
    let links = vec![Link::new(
        "gets a todo by id".to_string(),
        "/users/{id}".to_string(),
    )];
    HttpResponse::Ok()
        .set_header("content-type", "application/json")
        .json(links)
}
