use diesel::prelude::*;

pub mod models;
pub mod schema;
use models::*;

fn establish_connection(db_path: &str) -> SqliteConnection {
  SqliteConnection::establish(db_path).unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn get_games(db_path: &str) -> Vec<Game> {
  use self::schema::game::dsl::*;
  let connection = &mut establish_connection(db_path);
  game.limit(5).load::<Game>(connection).expect("Error loading posts")
}
