use diesel::prelude::*;

pub mod game;
pub mod models;
pub mod schema;
pub mod tag;

#[macro_export]
macro_rules! with_db {
  ($dbpath:expr, $func:expr, $($args:expr),*) => {{
    $func(&mut establish_connection($dbpath), $($args),*)
  }};
  ($dbpath:expr, $func:expr) => {{
    $func(&mut establish_connection($dbpath))
  }};
}

pub fn establish_connection(db_path: &str) -> SqliteConnection {
  SqliteConnection::establish(db_path).unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}
