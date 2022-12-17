use diesel::prelude::*;

use crate::models::{Game, TagAlias, ViewGame};
use crate::schema::game;
use crate::schema::game_tags_tag;
use crate::schema::tag_alias;
use crate::types::DbState;

pub fn view_all_games(state: &mut DbState) -> Vec<ViewGame> {
  game::table
    .select((
      game::id,
      game::title,
      game::developer,
      game::publisher,
      game::series,
      game::platform,
      game::tagsStr,
    ))
    .load::<ViewGame>(&mut state.conn)
    .expect("Error loading posts")
}

pub fn find_all_games(state: &mut DbState) -> Vec<Game> {
  game::table
    .load::<Game>(&mut state.conn)
    .expect("Error loading posts")
}

pub fn find_game(state: &mut DbState, game_id: String) -> Result<Game, diesel::result::Error> {
  game::table.find(game_id).first(&mut state.conn)
}

// find_game_row (?)

// find_random_games

// find_game_page_keyset

// find_add_app

// update_games

pub fn save_game(state: &mut DbState, g: Game) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(game::table)
    .values(&g)
    .execute(&mut state.conn)
}

pub fn remove_game(state: &mut DbState, game_id: String) -> Result<usize, diesel::result::Error> {
  diesel::delete(game::table.find(game_id)).execute(&mut state.conn)
}

pub fn find_games_with_tag(state: &mut DbState, tag_str: String) -> Vec<Game> {
  let alias = tag_alias::table
    .filter(tag_alias::name.eq(tag_str))
    .first::<TagAlias>(&mut state.conn)
    .expect("Tag alias not found");

  let game_ids = game_tags_tag::table
    .filter(game_tags_tag::tagId.eq(alias.tag_id.expect("Tried to unwrap broken alias")))
    .select(game_tags_tag::gameId);

  game::table
    .filter(game::id.eq_any(game_ids))
    .load::<Game>(&mut state.conn)
    .expect("Error loading posts")
  // TODO: Do we attach the other fields?
}

// chunk_find_by_ids

// apply_flat_game_filters

// do_where_title

// do_where_field

// apply_tag_filters

// get_game_query
