use flashpoint_config::types::{Config, Preferences, Services};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub mod signals;
use signals::*;

#[derive(Clone, Copy)]
pub enum InitLoad {
  Services,
  Database,
  Playlists,
  Extensions,
  ExecMappings,
  Curate,
}

signal!(ExitSignal<ExitRecv, i32>);
signal!(OnDidConnectSignal<OnDidConnectRecv, u8>);
signal!(InitLoadSignal<InitLoadRecv, InitLoad>);

pub struct FlashpointSignals {
  pub exit_code: ExitSignal,
  pub init_load: InitLoadSignal,
}

pub struct FlashpointService {
  pub base_path: String,
  pub config: Config,
  pub prefs: Preferences,
  #[cfg(feature = "services")]
  pub services_info: Services,
  pub signals: FlashpointSignals,
}

impl FlashpointService {
  pub async fn new(base_path_str: String) -> Self {
    let base_path = Path::new(&base_path_str);

    let config_path = base_path
      .join("config.json")
      .as_os_str()
      .to_str()
      .unwrap()
      .to_string();
    println!("Config Path: {}", config_path);
    let config = load_config_file(&config_path).await.unwrap();

    let prefs_path = base_path
      .parent()
      .unwrap()
      .join("preferences.json")
      .as_os_str()
      .to_str()
      .unwrap()
      .to_string();
    println!("Prefs Path: {}", prefs_path);
    let prefs = load_prefs_file(&prefs_path).await.unwrap();

    Self {
      base_path: base_path_str.clone(),
      config,
      prefs: prefs.clone(),
      #[cfg(feature = "services")]
      services_info: load_services(&base_path_str, &prefs).await.unwrap(),
      signals: FlashpointSignals {
        exit_code: ExitSignal::new(),
        init_load: InitLoadSignal::new(),
      },
    }
  }

  pub fn init(&mut self) {
    // TODO
    self.signals.init_load.emit(InitLoad::Services);
    // TODO
    self.signals.init_load.emit(InitLoad::Database);
    // TODO
    self.signals.init_load.emit(InitLoad::Playlists);
    // TODO
    self.signals.init_load.emit(InitLoad::Extensions);
    // TODO
    self.signals.init_load.emit(InitLoad::ExecMappings);
    // TODO
    self.signals.init_load.emit(InitLoad::Curate);
  }

  pub fn exit(&self) {
    self.signals.exit_code.emit(0);
  }
}

#[cfg(feature = "services")]
async fn load_services(
  base_path: &str,
  prefs: &Preferences,
) -> Result<Services, Box<dyn std::error::Error>> {
  let p = Path::new(&base_path);
  let services_path = p
    .parent()
    .unwrap()
    .join(prefs.json_folder_path.clone())
    .join("services.json")
    .as_os_str()
    .to_str()
    .unwrap()
    .to_string();
  println!("Services Path: {}", services_path);
  let services = load_services_file(&services_path).await.unwrap();
  Ok(services)
}

#[cfg(feature = "services")]
async fn load_services_file(path: &str) -> Result<Services, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let services: Services = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
  Ok(services)
}

async fn load_config_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let config: Config = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
  Ok(config)
}

async fn load_prefs_file(path: &str) -> Result<Preferences, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let prefs: Preferences = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
  Ok(prefs)
}
