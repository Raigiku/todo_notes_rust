use actix_web::{
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use std::{env, error::Error, fmt, io};

#[derive(Debug, Serialize)]
pub enum TodoNotesError {
    UserError { field: String, message: String },
    InternalError,
}

impl Error for TodoNotesError {}

impl fmt::Display for TodoNotesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for TodoNotesError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TodoNotesError::UserError { .. } => StatusCode::BAD_REQUEST,
            TodoNotesError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<tokio_postgres::Error> for TodoNotesError {
    fn from(e: tokio_postgres::Error) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}

impl From<native_tls::Error> for TodoNotesError {
    fn from(e: native_tls::Error) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}

impl From<env::VarError> for TodoNotesError {
    fn from(e: env::VarError) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}

impl From<deadpool_postgres::PoolError> for TodoNotesError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}

impl From<io::Error> for TodoNotesError {
    fn from(e: io::Error) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}

impl From<deadpool_postgres::config::ConfigError> for TodoNotesError {
    fn from(e: deadpool_postgres::config::ConfigError) -> Self {
        TodoNotesError::UserError {
            field: "".to_string(),
            message: e.to_string(),
        }
        // TodoNotesError::InternalError
    }
}
