use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::rustacean)]
pub struct Rustacean {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::rustacean)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
