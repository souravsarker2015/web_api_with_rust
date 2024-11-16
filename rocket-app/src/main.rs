#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_sync_db_pools;

use rocket::serde::json::{json, Value};
use rocket_sync_db_pools::database;
use diesel::prelude::*;
use crate::models::{Rustacean, NewRustacean};
mod models;
mod schema;
use crate::schema::rustacean::dsl::*;
use rocket::response::status;
#[database("sqlite")]
struct DbConnect(diesel::SqliteConnection);

#[catch(404)]
fn not_found() -> Value {
    json!({"error": "Not found"})
}

// Get all rustaceans from the database
#[get("/rustaceans")]
async fn get_rustaceans(db: DbConnect) -> Value {
    let result = db.run(|conn| {
        rustacean
            .load::<Rustacean>(conn)
            .map_err(|err| format!("Error loading rustaceans: {:?}", err))
    })
        .await;

    match result {
        Ok(rustaceans) => json!(rustaceans),
        Err(err) => json!({"error": err}),
    }
}

// Get a single rustacean by ID
#[get("/rustaceans/<rustacean_id>")]
async fn view_rustacean(rustacean_id: i32, db: DbConnect) -> Value {
    let result = db.run(move |conn| {
        rustacean
            .filter(id.eq(rustacean_id))
            .first::<Rustacean>(conn)
            .optional()
            .map_err(|_| "Error finding rustacean")
    })
        .await;

    match result {
        Ok(Some(dev)) => json!(dev),
        Ok(None) => json!({"error": "Rustacean not found"}),
        Err(_) => json!({"error": "Failed to retrieve rustacean"}),
    }
}

// Create a new rustacean
#[post("/rustaceans", format = "json", data = "<new_dev>")]
async fn create_rustacean(new_dev: rocket::serde::json::Json<NewRustacean>, db: DbConnect) -> Value {
    let new_dev = new_dev.into_inner();

    let result = db.run(move |conn| {
        diesel::insert_into(rustacean)
            .values(&new_dev)
            .execute(conn)
            .map_err(|_| "Error creating Rustacean")
    })
        .await;

    match result {
        Ok(_) => json!({"message": "Rustacean created successfully"}),
        Err(_) => json!({"error": "Failed to create Rustacean"}),
    }
}


#[put("/rustaceans/<rustacean_id>", format = "json", data = "<updated_dev>")]
async fn update_rustacean(rustacean_id: i32, updated_dev: rocket::serde::json::Json<NewRustacean>, db: DbConnect) -> Value {
    let updated_dev = updated_dev.into_inner();

    let result = db.run(move |conn| {
        diesel::update(rustacean.filter(id.eq(rustacean_id)))
            .set((name.eq(updated_dev.name), email.eq(updated_dev.email)))
            .execute(conn)
            .map_err(|_| "Error updating Rustacean")
    })
        .await;

    match result {
        Ok(_) => json!({"message": "Rustacean updated successfully"}),
        Err(_) => json!({"error": "Failed to update Rustacean"}),
    }
}

#[delete("/rustaceans/<rustacean_id>")]
async fn delete_rustacean(rustacean_id: i32, db: DbConnect) -> Result<status::NoContent, Value> {
    let result = db.run(move |conn| {
        diesel::delete(rustacean.filter(id.eq(rustacean_id)))
            .execute(conn)
            .map_err(|_| "Error deleting Rustacean")
    })
        .await;

    match result {
        Ok(deleted) if deleted > 0 => Ok(status::NoContent),  // Only return NoContent if something was deleted
        Ok(_) => Err(json!({"error": "Rustacean not found"})),  // Handle case where nothing was deleted
        Err(_) => Err(json!({"error": "Failed to delete Rustacean"})),  // Generic error
    }
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConnect::fairing())
        .launch()
        .await;
}


// #[macro_use]
// extern crate rocket;
// extern crate diesel;
// extern crate rocket_sync_db_pools;
//
// use rocket::response::status;
// use rocket::serde::json::{Value, json};
//
// use rocket_sync_db_pools::database;
//
// #[database("sqlite")]
// struct DbConnect(diesel::SqliteConnection);
//
//
// // #[get("/")]
// // // fn hello() -> &'static str {
// // fn hello() -> Value {
// //     json!("hello world")
// // }
//
// #[catch(404)]
// fn not_found() -> Value {
//     json!("not found")
// }
//
//
// #[get("/db_rust_devs")]
// fn get_rust_devs_from_db(_db: DbConnect) -> Value {
//     json!([{"id": 1, "name": "sojol saha"}, {"id": 2, "name": "sojol saha again"}])
// }
//
// #[get("/rust_devs")]
// fn get_rust_devs() -> Value {
//     json!([{"id": 1, "name": "sojol saha"}, {"id": 2, "name": "sojol saha again"}])
// }
//
// #[get("/rust_devs/<id>")]
// fn view_rust_devs(id: i32) -> Value {
//     json!([{"id":id,"name":"sojol saha"}])
// }
//
// #[post("/rust_devs", format = "json")]
// fn create_rust_devs() -> Value {
//     json!({"id":1,"name":"sojol saha"})
// }
//
// #[put("/rust_devs/<id>")]
// fn update_rust_devs(id: i32) -> Value {
//     json!({"id":id,"name":"sojol saha"})
// }
//
// #[delete("/rust_devs/<_id>")]
// fn delete_rust_devs(_id: i32) -> status::NoContent {
//     status::NoContent
// }
//
// #[rocket::main]
// async fn main() {
//     let _ = rocket::build()
//         .mount("/", routes![
//             get_rust_devs_from_db,
//             get_rust_devs,
//             view_rust_devs,
//             create_rust_devs,
//             update_rust_devs,
//             delete_rust_devs
//         ])
//         .register("/", catchers![not_found])
//         .attach(DbConnect::fairing())
//         .launch()
//         .await;
// }
