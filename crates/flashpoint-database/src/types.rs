use diesel::SqliteConnection;

#[derive(Debug)]
pub enum DbErrors {
  Connection(diesel::ConnectionError),
  ReadFailed,
}

impl std::fmt::Display for DbErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DbErrors::Connection(e) => {
        write!(f, "database connection error: {}", e)
      }
      DbErrors::ReadFailed => {
        write!(f, "database read failure")
      }
    }
  }
}

impl std::error::Error for DbErrors {}

/// The data required for database state initialization.
#[derive(Debug)]
pub struct InitData {
  pub db_path: String,
}

/// An opaque structure that holds the current database state.
pub struct DbState {
  pub(crate) conn: SqliteConnection,
}
