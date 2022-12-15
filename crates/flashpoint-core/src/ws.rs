use crate::FlashpointService;
use flashpoint_config::types::*;
use flashpoint_database::models::Game;
use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

pub type WebsocketRegister<RecType, ResType> = Box<dyn Fn(MutexGuard<FlashpointService>, RecType) -> ResType + Send>;

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
pub struct StringRes {
  pub data: String,
}
#[derive(Debug, Serialize)]
pub struct NumberRes {
  pub data: i32,
}

#[derive(Debug, Serialize)]
pub struct GameVecRes {
  pub data: Vec<Game>,
}

pub struct WebsocketRegisters {
  pub init_data: WebsocketRegister<(), InitDataRes>,
  pub all_games: WebsocketRegister<(), GameVecRes>,
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
    ws_execute!($func_data.as_str().unwrap().to_string(), $register, $res_str, $fp_service);
  };
  // JSON rec type
  ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr, $rectype:ident) => {
    let data_str = serde_json::to_string($func_data).unwrap(); // TODO: Make safe
    let data: $rectype = serde_json::from_str(data_str.as_str()).unwrap(); // TODO: Make safe
    ws_execute!(data, $register, $res_str, $fp_service);
  };
  // No Data
  ($register:expr, $res_str:expr, $fp_service:expr) => {
    ws_execute!((), $register, $res_str, $fp_service);
  };
  // Data already deserialized
  ($func_data:expr, $register:expr, $res_str:expr, $fp_service:expr) => {
    if !$fp_service.initialized {
      $fp_service.init();
    }
    let res = ($register)($fp_service, $func_data);
    $res_str = serde_json::to_string(&res).unwrap(); // TODO: Make safe
  };
}
