use crate::FlashpointService;
use chrono::NaiveDateTime;
use flashpoint_config::types::*;
use flashpoint_database::{
  models::{Game, TagAlias, TagCategory, ViewGame},
  tag::InsertableTagCategory,
};
use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

pub type WebsocketRegister<RecType, ResType> = Box<
  dyn Fn(
      MutexGuard<FlashpointService>,
      RecType,
    ) -> Result<WebsocketRes<ResType>, Box<dyn std::error::Error>>
    + Send,
>;
// pub type WebsocketRegisterAlone<RecType, ResType> = Box<dyn Fn(RecType) -> ResType + Send>;

#[derive(Debug, Serialize)]
pub struct InitDataRes {
  pub config: Config,
  pub prefs: Preferences,
  #[cfg(feature = "services")]
  pub services_info: Services,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddRecData {
  pub first: i32,
  pub second: i32,
}

#[derive(Debug, Serialize)]
pub struct WebsocketRes<T> {
  pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct AddRecv {
  pub first: i32,
  pub second: i32,
}

#[derive(Debug, Serialize)]
pub struct TagRes {
  pub id: i32,
  pub date_modified: NaiveDateTime,
  pub category_id: Option<i32>,
  pub description: Option<String>,
  pub primary_alias: TagAlias,
  pub aliases: Vec<TagAlias>,
}

pub struct WebsocketRegisters {
  pub init_data: WebsocketRegister<(), InitDataRes>,
  pub view_all_games: WebsocketRegister<(), Vec<ViewGame>>,
  pub all_games: WebsocketRegister<(), Vec<Game>>,
  pub all_tag_categories: WebsocketRegister<(), Vec<TagCategory>>,
  pub create_tag_category: WebsocketRegister<InsertableTagCategory, TagCategory>,
  pub find_tag_by_name: WebsocketRegister<String, TagRes>,
  pub add: WebsocketRegister<AddRecv, i32>,
}

// #[macro_export]
// macro_rules! ws_execute_async {
//   // String rec type
//   ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr, String) => {
//     ws_execute_async!($func_data.as_str().unwrap().to_string(), $register, $res_str, $fp_service);
//   };
//   // JSON rec type
//   ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr, $rectype:ident) => {
//     let data_str = serde_json::to_string($func_data).unwrap();
//     let data: $rectype = serde_json::from_str(data_str.as_str()).unwrap();
//     ws_execute_async!(data, $register, $res_str, $fp_service);
//   };
//   // No Data
//   ($register:expr, $res_str:expr, $fp_service:expr) => {
//     ws_execute_async!((), $register, $res_str, $fp_service);
//   };
//   // Data already deserialized
//   ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr) => {
//     let res = ($register)($fp_service, $func_data).await;
//     $res_str = serde_json::to_string(&res).unwrap();
//   };
// }

#[macro_export]
macro_rules! ws_execute {
  // String rec type
  ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr, String) => {
    ws_execute!(
      $func_data.as_str().unwrap().to_string(),
      $register,
      $res_str,
      $fp_service
    );
  };
  // JSON rec type
  ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr, $rectype:ident) => {
    let data_str = serde_json::to_string($func_data)?; // TODO: Make safe
    let data: $rectype = serde_json::from_str(data_str.as_str())?; // TODO: Make safe
    ws_execute!(data, $register, $res_str, $fp_service);
  };
  // No Data
  ($register:expr, $res_str:expr, $fp_service:expr) => {
    ws_execute!((), $register, $res_str, $fp_service);
  };
  // Data already deserialized
  ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr) => {
    let mut fp_service = $fp_service.lock().unwrap();
    if !fp_service.initialized {
      fp_service.init();
    }
    let res = ($register)(fp_service, $func_data);
    $res_str = serde_json::to_string(&res?)?; // TODO: Make safe
  };
}

// #[macro_export]
// macro_rules! ws_execute_alone {
//   // String rec type
//   ($func_data:expr, $register:expr, $res_str:expr, String) => {
//     ws_execute_alone!($func_data.as_str()?.to_string(), $register, $res_str);
//   };
//   // JSON rec type
//   ($func_data:expr, $register:expr, $res_str:expr, $rectype:ident) => {
//     let data_str = serde_json::to_string($func_data)?;
//     let data: $rectype = serde_json::from_str(data_str.as_str())?: ws_execute_alone!(data, $register, $res_str);
//   };
//   // No Data
//   ($register:expr, $res_str:expr) => {
//     ws_execute_alone!((), $register, $res_str);
//   };
//   // Data already deserialized
//   ($func_data:expr, $register:expr, $res_str:expr) => {
//     let res = ($register)($func_data);
//     $res_str = serde_json::to_string(&res)?;
//   };
// }
