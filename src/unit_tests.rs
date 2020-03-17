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
