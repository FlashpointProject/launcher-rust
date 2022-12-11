use std::{io::BufReader, fs::File, path::Path};

use flashpoint_core::FlashpointService;
use flashpoint_config::types::{Config, Preferences};

fn main() {
  let base_path = Path::new(r"C:\Users\colin\Downloads\Flashpoint 11 Infinity\Launcher");
  
  let config_path = base_path.join("config.json").as_os_str().to_str().unwrap().to_string();
  println!("Config Path: {}", config_path);
  let config = load_config_file(&config_path).unwrap();
  
  let prefs_path = base_path.parent().unwrap().join("preferences.json").as_os_str().to_str().unwrap().to_string();
  println!("Prefs Path: {}", prefs_path);
  let prefs = load_prefs_file(&prefs_path).unwrap();

  let fp_service = FlashpointService::new(base_path.to_str().unwrap().to_string(), config, prefs);
  println!("Loaded Flashpoint Service");
}

fn load_config_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let config: Config = serde_json::from_reader(reader)?;
  Ok(config)
}

fn load_prefs_file(path: &str) -> Result<Preferences, Box<dyn std::error::Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let prefs: Preferences = serde_json::from_reader(reader)?;
  Ok(prefs)
}
