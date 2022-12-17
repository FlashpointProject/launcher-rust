use crate::models::{Tag, TagAlias, TagCategory};
use crate::schema::{tag, tag_alias, tag_category};
use crate::types::DbState;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = tag_category)]
pub struct InsertableTagCategory {
  pub name: String,
  pub color: String,
  pub description: Option<String>,
}

// find_tags

// create_tag

// delete_tag

// save_tag

// save_tag_alias

// merge_tags

// cleanup_tag_aliases (still needed?)

// find_tag_suggestions

pub fn find_tag_categories(state: &mut DbState) -> Vec<TagCategory> {
  use crate::schema::tag_category::dsl::*;
  tag_category
    .load::<TagCategory>(&mut state.conn)
    .expect("Error loading tag categories")
}

pub fn create_tag_category(
  state: &mut DbState,
  new_category: InsertableTagCategory,
) -> Result<TagCategory, diesel::result::Error> {
  diesel::insert_into(tag_category::table)
    .values(&new_category)
    .execute(&mut state.conn)?;
  // TODO: Broadcast changes?
  // Find and return the newly created category
  tag_category::table
    .filter(tag_category::name.eq(new_category.name))
    .first(&mut state.conn)
}

pub fn save_tag_category(
  state: &mut DbState,
  category: InsertableTagCategory,
) -> Result<usize, diesel::result::Error> {
  diesel::update(tag_category::table)
    .set(&category)
    .execute(&mut state.conn)
}

pub fn get_tag_category(
  state: &mut DbState,
  category_id: i32,
) -> Result<TagCategory, diesel::result::Error> {
  tag_category::table
    .filter(tag_category::id.eq(category_id))
    .first(&mut state.conn)
}

pub fn get_tag_category_by_name(
  state: &mut DbState,
  name: String,
) -> Result<TagCategory, diesel::result::Error> {
  tag_category::table
    .filter(tag_category::name.eq(name))
    .first(&mut state.conn)
}

pub fn delete_tag_category(
  state: &mut DbState,
  category_id: i32,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(tag_category::table.filter(tag_category::id.eq(category_id)))
    .execute(&mut state.conn)
}

// find_game_tags

// get_tag_by_id

// get_tag_by_name

pub fn find_tag_by_name(
  state: &mut DbState,
  name: String,
) -> Result<(Tag, Vec<TagAlias>), diesel::result::Error> {
  // Load tag
  let alias = tag_alias::table
    .filter(tag_alias::name.eq(name))
    .first::<TagAlias>(&mut state.conn)?;
  let tag_obj = tag::table
    .filter(tag::id.eq(alias.tag_id.unwrap()))
    .first::<Tag>(&mut state.conn)?;

  // Load aliases
  let aliases = tag_alias::table
    .filter(tag_alias::tagId.eq(tag_obj.id))
    .load::<TagAlias>(&mut state.conn)?;

  Ok((tag_obj, aliases))
}

// add_alias_to_tag

// find_primary_aliases

// broadcast_tag_categories

// get_filter_ids_query (?)
