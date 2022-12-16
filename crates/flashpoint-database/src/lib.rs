use diesel::prelude::*;

pub mod game;
pub mod models;
pub mod schema;

pub fn establish_connection(db_path: &str) -> SqliteConnection {
  SqliteConnection::establish(db_path).unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}
