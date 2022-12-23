use diesel::helper_types::IntoBoxed;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::models::GameRelation;
use crate::models::{Game, TagAlias, ViewGame};
use crate::schema::game;
use crate::schema::game_tags_tag;
use crate::schema::tag_alias;
use crate::types::{DbState, FilterOpts};

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

/// if pageSize is Some, order_by_ascending will be treated as the key.
fn get_game_query<'a>(
  filters: &FilterOpts,
  order_by_ascending: Option<(&GameRelation, bool)>,
  offset: Option<i64>,
  page_size: Option<i64>,
) -> diesel::helper_types::IntoBoxed<'a, game::table, Sqlite> {
  let mut query = game::table.into_boxed();
  if let Some(_playlist_id) = &filters.playlist_id {
    todo!()
  }
  if let Some(search) = &filters.search_query {
    // Whitelist tends to be more restrictive, do it first.
    for filter in &search.whitelist {
      query = filter.filter_column(query, true);
    }
    for filter in &search.blacklist {
      query = filter.filter_column(query, false);
    }
    for filter in &search.generic_whitelist {
      query = apply_generic_filter(query, filter, true);
    }
    for filter in &search.generic_blacklist {
      query = apply_generic_filter(query, filter, false);
    }
  }
  if let Some((order_by, ascending)) = order_by_ascending {
    if let Some(page_size) = page_size {
      query = order_by.page(query, ascending, page_size)
    } else {
      query = order_by.order_query(query, ascending);
    }
  }
  if let Some(offset) = offset {
    query = query.offset(offset);
  }
  query
}

/// Applies a single filter to four game columns: title, alterateTitles, publisher, and developer.
/// If whitelist, these are LIKE filters, OR'd together. Otherwise, these are NOT LIKE filters, AND'd together.
fn apply_generic_filter<'a, 'b>(
  q: IntoBoxed<'a, game::table, Sqlite>,
  val: &str,
  whitelist: bool,
) -> IntoBoxed<'b, game::table, Sqlite>
where
  'a: 'b,
{
  let parens_val = "%".to_owned() + val + "%";
  if whitelist {
    q.filter(
      game::title
        .like(parens_val.clone())
        .or(game::alternateTitles.like(parens_val.clone()))
        .or(game::publisher.like(parens_val.clone()))
        .or(game::developer.like(parens_val)),
    )
  } else {
    q.filter(
      game::title
        .not_like(parens_val.clone())
        .and(game::alternateTitles.not_like(parens_val.clone()))
        .and(game::publisher.not_like(parens_val.clone()))
        .and(game::developer.not_like(parens_val)),
    )
  }
}
