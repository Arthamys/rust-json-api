use diesel::PgConnection;
use std::sync::Mutex;
use crate::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

// we use a mutex to synchronize the tests, so that they don't
// override the correct state of the database during the test.
// This leads to longer testing phases (as we run tests almost)
// in sequence.
lazy_static! {
    static ref DB_LOCK: Mutex<()> = Mutex::new(());
}

#[test]
fn get_hello_world() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "Hello, world!".to_string());
}

#[test]
fn get_all_documents() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let _lock = DB_LOCK.lock();
    mock_data(&rocket());

    let response = client.get("/documents").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response
        .content_type()
        .expect("no content in response for /documents route"),
        ContentType::JSON
        );
}

#[test]
fn insert_new_document() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let _lock = DB_LOCK.lock();
    let db_conn = super::repository::DbConn::get_one(&rocket()).expect("could not connect to database");
    clear_documents(&db_conn.0);

    let response = client.post("/documents/new_document")
        .header(ContentType::JSON)
        .body("{\"content\":\"This is the content of my latest document\"}")
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response
        .content_type()
        .expect("no content in response from POST /documents/<title>;"),
        ContentType::JSON);
    // TODO make sure the response body matches the request body
}

#[cfg(test)]
fn mock_data(rocket: &rocket::Rocket) {
    use crate::schema::documents;
    use crate::models::NewDocument;
    use diesel::RunQueryDsl;

    let db_conn = super::repository::DbConn::get_one(&rocket).expect("could not connect to database");
    clear_documents(&db_conn.0);
    diesel::insert_into(documents::table)
        .values(NewDocument {title: "First Document"})
        .execute(&db_conn.0).expect("could not insert new document");
    diesel::insert_into(documents::table)
        .values(NewDocument {title: "Second Document"})
        .execute(&db_conn.0).expect("could not insert new document");
}

#[cfg(test)]
fn clear_documents(db_conn: &PgConnection) {
    use diesel::delete;
    use diesel::RunQueryDsl;
    use crate::schema::documents;

    delete(documents::table).execute(db_conn).expect("could not clear documents table");
}
