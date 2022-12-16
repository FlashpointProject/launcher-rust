use diesel::prelude::*;

pub mod game;
pub mod models;
pub mod schema;
pub mod tag;
pub mod types;
use types::{DbErrors, DbState, InitData};

fn establish_connection(db_path: &str) -> Result<SqliteConnection, DbErrors> {
  match SqliteConnection::establish(db_path) {
    Ok(conn) => Ok(conn),
    Err(e) => Err(DbErrors::Connection(e)),
  }
}

pub fn initialize(data: InitData) -> Result<DbState, types::DbErrors> {
  let conn = establish_connection(&data.db_path)?;
  Ok(DbState { conn })
}
