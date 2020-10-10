use crate::domain::config::Config;
use std::fs::File;
use std::io::Read;

pub trait ConfigService {
  fn get_config(&self) -> &Config;
}

pub struct FileConfigService {
  config: Option<Config>,
}

impl FileConfigService {
  pub fn new() -> FileConfigService {
    FileConfigService {
      config: Option::None,
    }
  }

  pub fn load_config(&mut self, fname: &str) -> Result<(), std::io::Error> {
    let mut content = String::new();
    File::open(fname)?.read_to_string(&mut content)?;
    self.config = serde_json::from_str(&content)?;

    Ok(())
  }
}

impl ConfigService for FileConfigService {
  fn get_config(&self) -> &Config {
    self.config.as_ref().unwrap()
  }
}
