use diesel::{QueryResult, RunQueryDsl};
use diesel::pg::PgConnection;
use crate::schema::documents; // documents table ORM
use crate::models::Document; // Data object

/// Retrieve all the entries of the documents table
pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Document>> {
    documents::table
        .load::<Document>(conn)
}
