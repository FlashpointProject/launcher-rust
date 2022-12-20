use diesel::SqliteConnection;

use crate::game::GameFilter;

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

/// An opaque structure that holds the current database state.
pub struct DbState {
  pub(crate) conn: SqliteConnection,
}

pub struct ParsedSearch {
  pub generic_blacklist: Vec<String>,
  pub generic_whitelist: Vec<String>,
  pub blacklist: Vec<GameFilter>,
  pub whitelist: Vec<GameFilter>,
}

pub struct FilterOpts {
  pub search_limit: Option<i64>,
  pub playlist_id: Option<String>,
  pub search_query: Option<ParsedSearch>,
}
