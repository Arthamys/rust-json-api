#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod error;
mod models;
mod routes;
mod schema;
mod repository;

#[cfg(test)]
mod unit_tests;

/// Create the rocket instance that represents our REST API
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
            routes![
            routes::hello_world,
            routes::documents::get_all_documents,
            routes::documents::new_document,
            ])
        .attach(repository::DbConn::fairing())
}
