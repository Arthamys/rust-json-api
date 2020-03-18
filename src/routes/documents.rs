use crate::repository::{DbConn, documents, revisions};
use crate::models::document::DocumentContent;
use rocket_contrib::json::{JsonValue, Json};
use rocket::response::status::Created;
use crate::error::ApiError;

#[get("/documents")]
pub fn get_all_documents(
    conn: DbConn,
) -> Result<JsonValue, ApiError> {
    let documents = documents::get_all(&conn)?;
    Ok(json!({"documents": documents}))
}

#[post("/documents/<title>", format = "json", data = "<new_document>")]
pub fn new_document(
    new_document: Json<DocumentContent>,
    title: String,
    conn: DbConn,
) -> Result<Created<JsonValue>, ApiError> {
    let new_doc = new_document.into_inner();

    let document = documents::create_or_get(&title, &conn)?;
    revisions::create(document.id, &new_doc.content, &conn)?;
    let resource_url = format!("http://localhost:8000/{}", document.id);// TODO find a better way to set Location header
    Ok(Created(resource_url, Some(json!({"document": document}))))
}

