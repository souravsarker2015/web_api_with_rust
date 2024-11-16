--------------------    ***   --------------------

--------------------    ###   --------------------


--------------------    *** create project:  --------------------

The first thing we need to do is generate our project.

Generate a new project
cargo new --lib diesel_demo
cd diesel_demo
--------------------    ###   create project:   end --------------------


--------------------    ***  for sqlite setup need to install:  --------------------


sudo apt update
sudo apt install sqlite3 libsqlite3-dev


cargo install diesel_cli --no-default-features --features sqlite

diesel setup --database-url sqlite ./database.sqlite
touch ./database.sqlite

--------------------    ###   --------------------


--------------------    ***  .env  --------------------
DATABASE_URL=/home/sourov/snap/snap_store_/a/web_api_with_rust/rocket-app/database.sqlite




--------------------    ###  .env end --------------------


--------------------    ***  diesel.toml  --------------------
`

# For documentation on how to configure this file,
# see https://diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]

[migrations_directory]
dir = "/home/sourov/snap/snap_store_/a/web_api_with_rust/rocket-app/migrations"

`
--------------------    ###   --------------------

bash run: diesel setup


--------------------    ***   generate migration --------------------

diesel migration generate create_rustaceans

create table in up.sql

diesel migration run

diesel migration list

diesel migration revert


--------------------    ###   --------------------


--------------------    ***   cargo.toml --------------------

`
[package]
name = "rocket-app"
version = "0.1.0"
edition = "2021"

[dependencies]
#rocket = "0.5.0-rc"
rocket = { version = "0.5.0-rc.3", features = ["json"] }
serde_json = "1.0"
base64 = "0.20.0"
#rocket = "0.5.0-rc.3"
#base64 = "0.20.0"
#diesel = { version = "2.1.1", features = ["sqlite", "chrono"] }
#dotenvy = "0.15" # Optional, for using .env files

diesel = { version = "2.2.0", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }
# build libsqlite3 as part of the build process
# uncomment this line if you run into setup issues
# libsqlite3-sys = { version = "0.30", features = ["bundled"] }
dotenvy = "0.15"
 `

--------------------    ###   --------------------