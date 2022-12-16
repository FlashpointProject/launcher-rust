use diesel::prelude::*;

pub mod game;
pub mod models;
pub mod schema;
pub mod tag;
pub use diesel::sqlite::SqliteConnection;

pub fn establish_connection(db_path: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
  SqliteConnection::establish(db_path)
}
