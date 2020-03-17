use crate::rocket;
use rocket::local::Client;
use rocket::http::Status;

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
    mock_data(&rocket());

    let response = client.get("/documents").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response
        .content_type()
        .expect("no content in response for /documents route"),
        rocket::http::ContentType::JSON
        );
}

#[cfg(test)]
fn mock_data(rocket: &rocket::Rocket) {
    use crate::schema::documents;
    use crate::models::document::NewDocument;
    use diesel::delete;
    use diesel::RunQueryDsl;

    let db_conn = super::repository::DbConn::get_one(&rocket).expect("could not connect to database");
    delete(documents::table).execute(&db_conn.0).expect("could not clear documents table");
    diesel::insert_into(documents::table)
        .values(NewDocument {title: "First Document".to_string()})
        .execute(&db_conn.0).expect("could not insert new document");
    diesel::insert_into(documents::table)
        .values(NewDocument {title: "Second Document".to_string()})
        .execute(&db_conn.0).expect("could not insert new document");


}
