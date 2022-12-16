use diesel::prelude::*;

pub mod models;
pub mod schema;
use models::*;

fn establish_connection(db_path: &str) -> SqliteConnection {
  SqliteConnection::establish(db_path).unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn view_all_games(db_path: &str) -> Vec<ViewGame> {
  use self::schema::game::dsl::*;
  let connection = &mut establish_connection(db_path);
  game
    .select((id, title, developer, publisher, series, platform, tagsStr))
    .load::<ViewGame>(connection)
    .expect("Error loading posts")
}

pub fn all_games(db_path: &str) -> Vec<Game> {
  use self::schema::game::dsl::*;
  let connection = &mut establish_connection(db_path);
  game.load::<Game>(connection).expect("Error loading posts")
}
