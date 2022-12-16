use diesel::prelude::*;

use crate::models::{Game, ViewGame};

pub fn view_all_games(conn: &mut SqliteConnection) -> Vec<ViewGame> {
  use crate::schema::game::dsl::*;
  game
    .select((id, title, developer, publisher, series, platform, tagsStr))
    .load::<ViewGame>(conn)
    .expect("Error loading posts")
}

pub fn find_all_games(conn: &mut SqliteConnection) -> Vec<Game> {
  use crate::schema::game::dsl::*;
  game.load::<Game>(conn).expect("Error loading posts")
}

// find_game

// find_game_row (?)

// find_random_games

// find_game_page_keyset

// find_add_app

// update_games

// save_game

// remove_game

// find_games_with_tag

// chunk_find_by_ids

// apply_flat_game_filters

// do_where_title

// do_where_field

// apply_tag_filters

// get_game_query
