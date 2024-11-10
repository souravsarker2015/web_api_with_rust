#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest};
use rocket::response::status;
use rocket::request::Outcome;
// use rocket::serde::json::Json;
use rocket::serde::json::{Value, json};
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}
#[catch(401)]
fn unauthorized() -> &'static str {
    "Unauthorized access! Please provide valid credentials."
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }
        BasicAuth::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_str: &str) -> Option<BasicAuth> {
        let decode = base64::decode(base64_str).ok()?;
        let decode_str = String::from_utf8(decode).ok()?;
        let split = decode_str.split(":").collect::<Vec<&str>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }
        // Outcome::Failure((Status::Unauthorized, ()))
        // Outcome::Forward(())
        Outcome::Forward(())

    }
}
// #[get("/")]
// // fn hello() -> &'static str {
// fn hello() -> Value {
//     json!("hello world")
// }

#[catch(404)]
fn not_found() -> Value {
    json!("not found")
}

#[get("/rust_devs")]  // Original route
fn get_rust_devs(_auth: BasicAuth) -> Value {
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
        .register("/", catchers![unauthorized,not_found])
        .launch()
        .await;
}
