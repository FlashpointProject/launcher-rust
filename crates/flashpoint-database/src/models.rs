use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct TestGame {
  pub id: String,
  pub parent_game_id: Option<String>,
  pub title: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
  pub id: String,
  pub parent_game_id: Option<String>,
  pub title: String,
  // pub alternateTitles: String,
  // pub series: String,
  // pub developer: String,
  // pub publisher: String,
  // pub date_added: NaiveDateTime,
  // pub date_modified: NaiveDateTime,
  // pub platform: String,
  // pub broken: bool,
  // pub extreme: bool,
  // pub play_mode: String,
  // pub status: String,
  // pub notes: String,
  // pub source: String,
  // pub application_path: String,
  // pub launch_command: String,
  // pub release_date: String,
  // pub version: String,
  // pub original_description: String,
  // pub library: String,
  // pub order_title: String,
  // pub active_data_id: Option<i32>,
  // pub active_data_on_disk: bool,
  // pub tags_str: String,
}
