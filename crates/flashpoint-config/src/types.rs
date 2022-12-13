use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackProcessInfo {
  #[serde(default)]
  name: String,
  #[serde(default)]
  mad4fp: bool,
  path: String,
  filename: String,
  #[serde(default)]
  arguments: Vec<String>,
  #[serde(default)]
  kill: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainWindow {
  #[serde(default)]
  pub x: u32,
  #[serde(default)]
  pub y: u32,
  #[serde(default)]
  pub width: u32,
  #[serde(default)]
  pub height: u32,
  #[serde(default)]
  pub maximized: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum BrowsePageLayout {
  List = 0,
  Grid = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
  Silent = 5,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppPathOverride {
  pub path: String,
  pub r#override: String,
  pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagFilterGroup {
  pub name: String,
  pub description: String,
  pub enabled: bool,
  pub tags: Vec<String>,
  pub categories: Vec<String>,
  pub child_filters: Vec<String>,
  pub extreme: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsCurate {
  pub prev: Vec<String>,
  pub next: Vec<String>,
  pub load: Vec<String>,
  pub new_cur: Vec<String>,
  pub delete_curs: Vec<String>,
  pub export_curs: Vec<String>,
  pub export_data_packs: Vec<String>,
  pub import_curs: Vec<String>,
  pub refresh: Vec<String>,
  pub run: Vec<String>,
  pub run_mad4fp: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shortcuts {
  pub curate: ShortcutsCurate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(default = "flashpoint_path")]
  pub flashpoint_path: String,
  #[serde(default)]
  pub use_custom_titlebar: bool,
  #[serde(default = "bool_true")]
  pub start_server: bool,
  #[serde(default = "server")]
  pub server: String,
  #[serde(default = "back_port_min")]
  pub back_port_min: u16,
  #[serde(default = "back_port_max")]
  pub back_port_max: u16,
  #[serde(default = "images_port_min")]
  pub images_port_min: u16,
  #[serde(default = "images_port_max")]
  pub images_port_max: u16,
  #[serde(default = "logs_base_url")]
  pub logs_base_url: String,
  #[serde(default)]
  pub updates_enabled: bool,
  #[serde(default = "gotd_url")]
  pub gotd_url: String,
  #[serde(default)]
  pub gotd_show_all: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
  #[serde(default = "image_folder_path")]
  pub image_folder_path: String,
  #[serde(default = "logo_folder_path")]
  pub logo_folder_path: String,
  #[serde(default = "playlist_folder_path")]
  pub playlist_folder_path: String,
  #[serde(default = "json_folder_path")]
  pub json_folder_path: String,
  #[serde(default = "htdocs_folder_path")]
  pub htdocs_folder_path: String,
  #[serde(default = "platforms_folder_path")]
  pub platforms_folder_path: String,
  #[serde(default = "theme_folder_path")]
  pub theme_folder_path: String,
  #[serde(default = "logosets_folder_path")]
  pub logo_sets_folder_path: String,
  #[serde(default = "metaedits_folder_path")]
  pub meta_edits_folder_path: String,
  #[serde(default = "extensions_folder_path")]
  pub extensions_path: String,
  #[serde(default = "data_packs_folder_path")]
  pub data_packs_folder_path: String,
  #[serde(default = "browse_page_game_scale")]
  pub browse_page_game_scale: f32,
  #[serde(default)]
  pub browse_page_show_extreme: bool,
  #[serde(default = "bool_true")]
  pub enable_editing: bool,
  #[serde(default = "fallback_language")]
  pub fallback_language: String,
  #[serde(default = "language")]
  pub current_language: String,
  #[serde(default = "browse_page_layout")]
  pub browse_page_layout: BrowsePageLayout,
  #[serde(default = "bool_true")]
  pub browse_page_show_left_sidebar: bool,
  #[serde(default = "bool_true")]
  pub browse_page_show_right_sidebar: bool,
  #[serde(default = "sidebar_width")]
  pub browse_page_right_sidebar_width: u32,
  #[serde(default = "sidebar_width")]
  pub browse_page_left_sidebar_width: u32,
  #[serde(default = "sidebar_width")]
  pub curate_page_left_sidebar_width: u32,
  #[serde(default = "bool_true")]
  pub show_developer_tab: bool,
  #[serde(default = "current_theme")]
  pub current_theme: String,
  #[serde(default)]
  pub current_logo_set: String,
  #[serde(default)]
  pub last_selected_library: String,
  #[serde(default = "games_order_by")]
  pub games_order_by: String,
  #[serde(default = "games_order")]
  pub games_order: String,
  #[serde(default = "default_library")]
  pub default_library: String,
  #[serde(default = "main_window")]
  pub main_window: MainWindow,
  #[serde(default = "bool_true")]
  pub save_imported_curations: bool,
  #[serde(default = "bool_true")]
  pub keep_archive_key: bool,
  #[serde(default = "bool_true")]
  pub symlink_curation_content: bool,
  #[serde(default)]
  pub on_demand_images: bool,
  #[serde(default = "on_demand_base_url")]
  pub on_demand_base_url: String,
  #[serde(default = "browser_mode_proxy")]
  pub browser_mode_proxy: String,
  #[serde(default = "show_log_source")]
  pub show_log_source: HashMap<LogLevel, bool>,
  #[serde(default)]
  pub show_log_level: HashMap<u32, bool>,
  #[serde(default)]
  pub excluded_random_libraries: Vec<String>,
  #[serde(default)]
  pub app_path_overrides: Vec<AppPathOverride>,
  #[serde(default)]
  pub tag_filters: Vec<TagFilterGroup>,
  #[serde(default)]
  pub tag_filters_in_curate: bool,
  #[serde(default)]
  pub native_platforms: Vec<String>,
  #[serde(default)]
  pub disable_extreme_games: bool,
  #[serde(default)]
  pub show_broken_games: bool,
  #[serde(default = "shortcuts")]
  pub shortcuts: Shortcuts,
  #[serde(default = "online_manual")]
  pub online_manual: String,
  #[serde(default)]
  pub offline_manual: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Services {
  pub server: Vec<BackProcessInfo>,
  pub daemon: Vec<BackProcessInfo>,
  pub start: Vec<BackProcessInfo>,
  pub stop: Vec<BackProcessInfo>,
  pub watch: Vec<String>,
}

fn image_folder_path() -> String {
  "Data/Images".to_string()
}

fn logo_folder_path() -> String {
  "Data/Logos".to_string()
}

fn playlist_folder_path() -> String {
  "Data/Playlists".to_string()
}

fn json_folder_path() -> String {
  "Data".to_string()
}
fn htdocs_folder_path() -> String {
  "Legacy/htdocs".to_string()
}

fn platforms_folder_path() -> String {
  "Data/Platforms".to_string()
}

fn theme_folder_path() -> String {
  "Data/Themes".to_string()
}

fn logosets_folder_path() -> String {
  "Data/LogoSets".to_string()
}

fn metaedits_folder_path() -> String {
  "Data/MetaEdits".to_string()
}

fn extensions_folder_path() -> String {
  "Data/Extensions".to_string()
}

fn data_packs_folder_path() -> String {
  "Data/Games".to_string()
}

fn browse_page_game_scale() -> f32 {
  0.087
}

fn fallback_language() -> String {
  "en".to_string()
}

fn language() -> String {
  "<auto>".to_string()
}

fn main_window() -> MainWindow {
  MainWindow {
    width: 1280,
    height: 720,
    maximized: false,
    x: 0,
    y: 0,
  }
}

fn shortcuts() -> Shortcuts {
  Shortcuts {
    curate: ShortcutsCurate {
      prev: vec!["ctrl+arrowup".to_string(), "cmd+arrowup".to_string()],
      next: vec!["ctrl+arrowdown".to_string(), "cmd+arrowdown".to_string()],
      load: vec!["ctrl+o".to_string(), "cmd+o".to_string()],
      new_cur: vec!["ctrl+n".to_string(), "cmd+n".to_string()],
      delete_curs: vec!["ctrl+delete".to_string(), "cmd+delete".to_string()],
      export_curs: vec!["ctrl+s".to_string(), "cmd+s".to_string()],
      export_data_packs: vec!["ctrl+shift+s".to_string(), "cmd+shift+s".to_string()],
      import_curs: vec!["ctrl+i".to_string(), "cmd+i".to_string()],
      refresh: vec!["ctrl+r".to_string(), "cmd+r".to_string()],
      run: vec!["ctrl+t".to_string(), "cmd+t".to_string()],
      run_mad4fp: vec!["ctrl+shift+t".to_string(), "cmd+shift+t".to_string()],
    },
  }
}

fn show_log_source() -> HashMap<LogLevel, bool> {
  let mut map = HashMap::new();
  map.insert(LogLevel::Info, true);
  map.insert(LogLevel::Warn, true);
  map.insert(LogLevel::Error, true);
  map.insert(LogLevel::Debug, false);
  map.insert(LogLevel::Silent, false);
  map
}

fn default_library() -> String {
  "arcade".to_string()
}

fn browse_page_layout() -> BrowsePageLayout {
  BrowsePageLayout::List
}

fn sidebar_width() -> u32 {
  320
}

fn current_theme() -> String {
  "Metal\\theme.css".to_string()
}

fn games_order_by() -> String {
  "title".to_string()
}

fn games_order() -> String {
  "ASC".to_string()
}

fn on_demand_base_url() -> String {
  "https://infinity.unstable.life/Flashpoint/Data/Images/".to_string()
}

fn browser_mode_proxy() -> String {
  "localhost:22500".to_string()
}

fn online_manual() -> String {
  "https://flashpointproject.github.io/manual/".to_string()
}

fn bool_true() -> bool {
  true
}

fn flashpoint_path() -> String {
  "./".to_string()
}

fn logs_base_url() -> String {
  "https://logs.unstable.life/".to_string()
}

fn server() -> String {
  "Apache Webserver".to_string()
}

fn back_port_min() -> u16 {
  12001
}

fn back_port_max() -> u16 {
  12100
}

fn images_port_min() -> u16 {
  12101
}

fn images_port_max() -> u16 {
  12200
}

fn gotd_url() -> String {
  "https://download.unstable.life/gotd.json".to_string()
}
