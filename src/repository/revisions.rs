use diesel::{QueryResult, RunQueryDsl};
use diesel::pg::PgConnection;
use chrono::NaiveDateTime;
use crate::models::revision::{Revision, NewRevision};
use crate::schema::revisions;

pub fn create(
    doc_id: i32,
    content: &str,
    conn: &PgConnection,
) -> QueryResult<Revision> {
    let first_revision = &NewRevision {
        document_id: doc_id,
        content: &content,
        creation_date: &chrono::Utc::now().naive_utc(),
    };
    diesel::insert_into(revisions::table)
        .values(first_revision)
        .get_result::<Revision>(conn)
}

