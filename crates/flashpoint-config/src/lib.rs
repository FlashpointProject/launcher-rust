pub mod types;

#[cfg(test)]
mod tests {
  #[test]
  fn load_default_config() {
    let config: Result<crate::types::Config, serde_json::Error> = serde_json::from_str("{}");
    assert!(config.is_ok())
  }

  #[test]
  fn load_default_prefs() {
    let prefs: Result<crate::types::Preferences, serde_json::Error> = serde_json::from_str("{}");
    assert!(prefs.is_ok())
  }
}
