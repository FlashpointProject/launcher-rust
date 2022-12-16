use diesel::prelude::*;

use crate::models::{Game, ViewGame};
use crate::schema::game::dsl::*;
use crate::types::DbState;

pub fn view_all_games(state: &mut DbState) -> Vec<ViewGame> {
  game
    .select((id, title, developer, publisher, series, platform, tagsStr))
    .load::<ViewGame>(&mut state.conn)
    .expect("Error loading posts")
}

pub fn find_all_games(state: &mut DbState) -> Vec<Game> {
  game
    .load::<Game>(&mut state.conn)
    .expect("Error loading posts")
}

pub fn find_game(state: &mut DbState, game_id: String) -> Result<Game, diesel::result::Error> {
  game.find(game_id).first(&mut state.conn)
}

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
