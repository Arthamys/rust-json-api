#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

#[cfg(test)]
mod unit_tests;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
            routes![
            routes::hello_world,
            ])
}
