use diesel::{QueryResult, RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::pg::PgConnection;
use crate::schema::documents; // documents table ORM
use crate::models::{Document, NewDocument}; // Data object

/// Retrieve all the entries of the documents table
pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Document>> {
    documents::table
        .load::<Document>(conn)
}

/// persist the new document, or get it's definition if it already exists
pub fn create_or_get(doc_title: &str, conn: &PgConnection) -> QueryResult<Document> {
    match documents::table
        .filter(documents::title.eq(doc_title))
        .first::<Document>(conn) {
            Ok(doc) => Ok(doc),
            Err(_) => {
                let new_document = &NewDocument {
                    title: doc_title,
                };
                diesel::insert_into(documents::table)
                    .values(new_document)
                    .get_result::<Document>(conn)
            }
        }
}
