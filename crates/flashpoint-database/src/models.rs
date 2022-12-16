use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewGame {
  pub id: String,
  pub title: String,
  pub developer: String,
  pub publisher: String,
  pub series: String,
  pub platform: String,
  pub tags_str: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
  pub id: String,
  pub parent_game_id: Option<String>,
  pub title: String,
  pub alternate_titles: String,
  pub series: String,
  pub developer: String,
  pub publisher: String,
  pub date_added: NaiveDateTime,
  pub date_modified: NaiveDateTime,
  pub platform: String,
  pub broken: bool,
  pub extreme: bool,
  pub play_mode: String,
  pub status: String,
  pub notes: String,
  pub source: String,
  pub application_path: String,
  pub launch_command: String,
  pub release_date: String,
  pub version: String,
  pub original_description: String,
  pub language: String,
  pub library: String,
  pub order_title: String,
  pub active_data_id: Option<i32>,
  pub active_data_on_disk: bool,
  pub tags_str: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalApp {
  pub id: String,
  pub application_path: String,
  pub auto_run_before: bool,
  pub launch_command: String,
  pub name: String,
  pub wait_for_exit: bool,
  pub parent_game_id: Option<String>,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
  pub id: i32,
  pub game_id: Option<String>,
  pub title: String,
  pub date_added: NaiveDateTime,
  pub sha256: String,
  pub crc32: i32,
  pub present_on_disk: bool,
  pub path: Option<String>,
  pub size: i32,
  pub parameters: Option<String>,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
  pub id: String,
  pub title: String,
  pub description: String,
  pub author: String,
  pub icon: Option<String>,
  pub library: String,
  pub extreme: bool,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistGame {
  pub id: i32,
  pub playlist_id: Option<String>,
  pub order: i32,
  pub notes: String,
  pub game_id: Option<String>,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
  pub id: i32,
  pub name: String,
  pub date_added: NaiveDateTime,
  pub last_updated: NaiveDateTime,
  pub source_file_url: String,
  pub base_url: String,
  pub count: i32,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceData {
  pub id: i32,
  pub source_id: Option<i32>,
  pub sha256: String,
  pub url_path: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
  pub id: i32,
  pub date_modified: NaiveDateTime,
  pub primary_alias_id: Option<i32>,
  pub category_id: Option<i32>,
  pub description: Option<String>,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagAlias {
  pub id: i32,
  pub tag_id: Option<i32>,
  pub name: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagCategory {
  pub id: i32,
  pub name: String,
  pub color: String,
  pub description: Option<String>,
}
