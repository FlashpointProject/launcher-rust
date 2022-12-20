use diesel::prelude::*;

// TODO: make all the functions return Result enums instead of expect()ing.
pub mod game;
pub mod models;
//pub mod models_expanded;
pub mod schema;
pub mod tag;
pub mod types;
use types::{DbErrors, DbState};

fn establish_connection(db_path: &str) -> Result<SqliteConnection, DbErrors> {
  match SqliteConnection::establish(db_path) {
    Ok(conn) => Ok(conn),
    Err(e) => Err(DbErrors::Connection(e)),
  }
}

pub fn initialize(db_path: &str) -> Result<DbState, DbErrors> {
  let conn = establish_connection(db_path)?;
  Ok(DbState { conn })
}
