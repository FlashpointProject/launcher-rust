use crate::models::TagCategory;
use crate::schema::tag_category;
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

pub fn find_tag_categories(conn: &mut SqliteConnection) -> Vec<TagCategory> {
  use crate::schema::tag_category::dsl::*;
  tag_category
    .load::<TagCategory>(conn)
    .expect("Error loading tag categories")
}

pub fn create_tag_category(
  conn: &mut SqliteConnection,
  new_category: InsertableTagCategory,
) -> Result<TagCategory, diesel::result::Error> {
  diesel::insert_into(tag_category::table)
    .values(&new_category)
    .execute(conn)?;
  // TODO: Broadcast changes?
  // Find and return the newly created category
  tag_category::table
    .filter(tag_category::name.eq(new_category.name))
    .first(conn)
}

pub fn save_tag_category(
  conn: &mut SqliteConnection,
  category: InsertableTagCategory,
) -> Result<usize, diesel::result::Error> {
  diesel::update(tag_category::table)
    .set(&category)
    .execute(conn)
}

pub fn get_tag_category(
  conn: &mut SqliteConnection,
  category_id: i32,
) -> Result<TagCategory, diesel::result::Error> {
  tag_category::table
    .filter(tag_category::id.eq(category_id))
    .first(conn)
}

pub fn get_tag_category_by_name(
  conn: &mut SqliteConnection,
  name: String,
) -> Result<TagCategory, diesel::result::Error> {
  tag_category::table
    .filter(tag_category::name.eq(name))
    .first(conn)
}

pub fn delete_tag_category(
  conn: &mut SqliteConnection,
  category_id: i32,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(tag_category::table.filter(tag_category::id.eq(category_id))).execute(conn)
}

// find_game_tags

// get_tag_by_id

// get_tag_by_name

// add_alias_to_tag

// find_primary_aliases

// broadcast_tag_categories

// get_filter_ids_query (?)
