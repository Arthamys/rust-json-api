use diesel::result::Error as DieselError;
use rocket::response::{self, Response, Responder};
use rocket::request::Request;
use rocket::http::{Status, ContentType};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
    DocumentNotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::DatabaseError => f.write_str("Internal Server Error"),
            ApiError::DocumentNotFound(ref title) => f.write_str(&format!("Document '{}' not found", title)),
        }
    }
}

impl std::error::Error for ApiError {}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        use std::io::Cursor;

        let resp_status = match self {
            ApiError::DatabaseError => Status::new(500, "Internal Error"),
            ApiError::DocumentNotFound(_) => Status::new(404, "Not Found"),
        };
        let error_msg = format!("{}", self);
        let body = json!({"error": error_msg}).to_string();
        Response::build()
            .status(resp_status)
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(body))
            .ok()
    }
}

impl From<DieselError> for ApiError {
    fn from(_e: DieselError) -> Self {
        ApiError::DatabaseError
    }
}
