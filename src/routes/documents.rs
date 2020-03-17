use crate::repository::{DbConn, documents};
use rocket_contrib::json::JsonValue;
use crate::error::ApiError;

#[get("/documents")]
pub fn get_all_documents(
    conn: DbConn,
) -> Result<JsonValue, ApiError> {
    let documents = documents::get_all(&conn)?;
    Ok(json!({"documents": documents}))
}

