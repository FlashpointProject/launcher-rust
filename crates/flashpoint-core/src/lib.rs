use flashpoint_config::types::{Config, Preferences};

pub struct FlashpointService {
  pub base_path: String,
  pub config: Config,
  pub prefs: Preferences,
}

impl FlashpointService {
  pub fn new(base_path: String, config: Config, prefs: Preferences) -> Self {
    Self {
      base_path,
      config,
      prefs
    }
  }
}