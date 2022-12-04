#![feature(decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{serve::StaticFiles, json::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    name: String
}

#[get("/user")]
fn user() -> JsonValue {
    json!(User {name: "Marcus".to_string()})
}

fn main() { 
    rocket::ignite()
        .mount("/", routes![user])
        .mount("/", StaticFiles::from("public"))
        .launch();
}
