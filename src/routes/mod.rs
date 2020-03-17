#[get("/hello")]
pub fn hello_world() -> String {
    "Hello, world!".to_string()
}
