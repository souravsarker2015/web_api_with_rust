#[macro_use]
extern crate rocket;

use rocket::response::status;

// use rocket::serde::json::Json;
use rocket::serde::json::{Value, json};

// #[get("/")]
// // fn hello() -> &'static str {
// fn hello() -> Value {
//     json!("hello world")
// }

#[get("/rust_devs")]  // Original route
fn get_rust_devs() -> Value {
    json!([{"id": 1, "name": "sojol saha"}, {"id": 2, "name": "sojol saha again"}])
}

#[get("/rust_devs/<id>")]
fn view_rust_devs(id: i32) -> Value {
    json!([{"id":id,"name":"sojol saha"}])
}

#[post("/rust_devs", format = "json")]
fn create_rust_devs() -> Value {
    json!({"id":1,"name":"sojol saha"})
}

#[put("/rust_devs/<id>")]
fn update_rust_devs(id: i32) -> Value {
    json!({"id":id,"name":"sojol saha"})
}

#[delete("/rust_devs/<_id>")]
fn delete_rust_devs(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        // .mount("/", routes![hello,get_rust_devs,view_rust_devs,create_rust_devs,update_rust_devs,delete_rust_devs])
        .mount("/", routes![
            get_rust_devs,
            view_rust_devs,
            create_rust_devs,
            update_rust_devs,
            delete_rust_devs
        ])
        .launch()
        .await;
}
